use std::collections::HashMap;

use crate::{environment::{Type, Value}, error::Diagnostic, expr::{AssignmentExpr, BinaryExpr, BlockExpr, BreakExpr, CallExpr, DeclarationExpr, EmptyExpr, Expr, ExprVisitor, IfExpr, InputExpr, LiteralExpr, LoopExpr, PrintExpr, RandExpr, UnaryExpr, VarExpr}};

struct VarDeclaration {
    is_defined: bool,
    declaration_type: Type,
}

#[derive(Debug)]
struct ResolvedVar {
    dist: usize,
    value_type: Type
}

pub struct Resolver {
    num_scopes: usize,
    scopes: Vec<HashMap<String, VarDeclaration>>,
    resolved: HashMap<VarExpr, ResolvedVar>,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            num_scopes: 1,
            scopes: vec![HashMap::new()],
            resolved: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, exprs: &[Box<dyn Expr>]) -> Vec<Diagnostic> {        
        for expr in exprs {
            self.resolve_expr(&**expr);
        }

        Vec::new()
    }

    pub fn get_dist(&self, expr: &VarExpr) -> usize {
        self.resolved.get(expr).unwrap().dist
    }

    pub fn get_type(&self, expr: &VarExpr) -> &Type {
        &self.resolved.get(expr).unwrap().value_type
    }

    fn resolve_expr(&mut self, expr: &(impl Expr + ?Sized)) {
        expr.accept_visitor(self)
    }
}

impl Resolver {
    fn declare(&mut self, name: &str, value_type: &Type) {
        let declaration = VarDeclaration {
            is_defined: false,
            declaration_type: value_type.clone()
        };

        self.scopes[self.num_scopes - 1].insert(name.to_string(), declaration);
    }

    fn define(&mut self, name: &str) {
        let declaration = self.scopes[self.num_scopes - 1].get_mut(name).unwrap();
        declaration.is_defined = true;
    }

    fn push_scope(&mut self) {
        self.num_scopes += 1;
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.num_scopes -= 1;
        self.scopes.pop();
    }

    fn resolve_var(&mut self, var_expr: &VarExpr) {
        let mut found = false;

        for i in (0..self.num_scopes).rev() {
            if let Some(declaration) = self.scopes[i].get(&*var_expr.identifier) {
                if declaration.is_defined {
                    let resolved_var = ResolvedVar {
                        dist: self.num_scopes - i - 1,
                        value_type: declaration.declaration_type.clone()
                    };
                    
                    found = true;
                    self.resolved.insert(VarExpr::clone(var_expr), resolved_var);
                    break;
                } else {
                    panic!("Cannot reference variable name in initializer");
                }
            }
        }

        if !found {
        panic!("Unknown variable name {:?}", var_expr.identifier);
        }
    }
}

impl ExprVisitor<()> for Resolver {
    fn visit_empty(&mut self, _expr: &EmptyExpr) {}

    fn visit_binary(&mut self, expr: &BinaryExpr) {
        self.resolve_expr(&*expr.left);
        self.resolve_expr(&*expr.right);
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) {
        self.resolve_expr(&*expr.expr);
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) {
        if let Value::Function(function) = &expr.value.value {
            self.push_scope();

            if let Type::Function(function_type) = &expr.value.value_type {
                let arg_types = &function_type.arg_types;

                for (i, arg) in function.args.iter().enumerate() {                   
                    self.declare(arg, arg_types.get(i).unwrap());
                    self.define(arg);
                }
    
                self.resolve_expr(&**function.body);
    
                self.pop_scope();
            }

            
        }
    }

    fn visit_var(&mut self, expr: &VarExpr) {
        self.resolve_var(expr)
    }

    fn visit_if(&mut self, expr: &IfExpr) {
        println!("Depth before if: {:?}", self.num_scopes);
        self.resolve_expr(&*expr.condition);
        println!("Depth after condition: {:?}", self.num_scopes);
        self.resolve_expr(&*expr.success);

        println!("Depth after if: {:?}", self.num_scopes);

        if let Some(fail) = &expr.fail {
            self.resolve_expr(&**fail);
        }
    }

    fn visit_assignment(&mut self, expr: &AssignmentExpr) {
        self.resolve_expr(&*expr.asignee);
        self.resolve_expr(&*expr.expr);
    }

    fn visit_declaration(&mut self, expr: &DeclarationExpr) {
        self.declare(&expr.identifier, &expr.declaration_type);

        //Allow recursive funtions
        if let Type::Function(_) = &expr.declaration_type {
            self.define(&expr.identifier);
        }

        self.resolve_expr(&*expr.expr);

        self.define(&expr.identifier);
    }

    fn visit_block(&mut self, expr: &BlockExpr) {
        self.push_scope();
        for expr in &expr.exprs {
            self.resolve_expr(&**expr);
        }
        self.pop_scope();
    }

    fn visit_print(&mut self, expr: &PrintExpr) {
        self.resolve_expr(&*expr.expr);
    }

    fn visit_rand(&mut self, expr: &RandExpr) {
        self.resolve_expr(&*expr.min);
        self.resolve_expr(&*expr.max);
    }

    fn visit_loop(&mut self, expr: &LoopExpr) {
        self.push_scope();
        
        if let Some(initial) = &expr.initial {
            self.resolve_expr(&**initial);
        }
        
        if let Some(condition) = &expr.condition {
            self.resolve_expr(&**condition);
        }

        if let Some(increment) = &expr.increment {
            self.resolve_expr(&**increment);
        }

        self.resolve_expr(&*expr.body);
        self.pop_scope();
    }

    fn visit_break(&mut self, expr: &BreakExpr) {
        self.resolve_expr(&*expr.expr);
    }

    fn visit_input(&mut self, expr: &InputExpr) {
        self.resolve_expr(&*expr.prompt);
    }

    fn visit_call(&mut self, expr: &CallExpr) {
        self.resolve_expr(&*expr.func_expr);

        for arg in &expr.args {
            self.resolve_expr(&**arg);
        }
    }
}