use std::{any::type_name, collections::{HashMap, HashSet}, rc::Rc};

use crate::{environment::{FunctionType, ParsedType, ResolvedType, StructType, Value}, error::Diagnostic, expr::{AssignmentExpr, BinaryExpr, BlockExpr, BreakExpr, CallExpr, DeclarationExpr, EmptyExpr, Expr, ExprVisitor, IfExpr, InputExpr, LiteralExpr, LoopExpr, PrintExpr, RandExpr, StructExpr, UnaryExpr, VarExpr}};

struct VarDeclaration {
    is_defined: bool,
    declaration_type: ResolvedType,
}

#[derive(Debug)]
struct ResolvedVar {
    dist: usize,
    value_type: ResolvedType
}

pub struct SymbolTable {
    pub types: HashMap<String, ResolvedType>,
    pub variables: HashMap<VarExpr, ResolvedVar>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            types: HashMap::new(),
            variables: HashMap::new()
        }
    }

    pub fn resolve(&mut self, exprs: &[Box<dyn Expr>]) -> Vec<Diagnostic> {
        let mut errors = Vec::new();

        let mut type_resolver = TypeResolver::new(self);

        errors.append(&mut type_resolver.resolve(exprs));

        let mut variable_resolver = VariableResolver::new(self);

        errors.append(&mut variable_resolver.resolve(exprs));
        
        errors
    }

    pub fn insert_variable(&mut self, expr: VarExpr, var: ResolvedVar) {
        self.variables.insert(expr, var);
    }
    
    pub fn get_variable_dist(&self, expr: &VarExpr) -> usize {
        self.variables.get(expr).unwrap().dist
    }

    pub fn get_variable_type(&self, expr: &VarExpr) -> &ResolvedType {
        &self.variables.get(expr).unwrap().value_type
    }

    pub fn insert_type(&mut self, type_name: String, resolved_type: ResolvedType) {
        println!("Resolved type name {:?} to {:?}", type_name, resolved_type);

        self.types.insert(type_name, resolved_type);
    }

    pub fn has_type_name(&self, type_name: &str) -> bool {
        self.types.contains_key(type_name)
    }

    pub fn resolve_struct_expr(&self, struct_expr: &StructExpr) -> ResolvedType {
        let mut member_types = HashMap::new();

        for (member_name, member_type) in struct_expr.members.iter() {
            member_types.insert(member_name.to_string(), self.get_resolved_type(member_type));
        }

        ResolvedType::Struct(StructType {
            member_types: Rc::new(member_types)
        })
    }

    pub fn get_resolved_type(&self, parsed_type: &ParsedType) -> ResolvedType {
        match parsed_type {
            ParsedType::Integer => ResolvedType::Integer,
            ParsedType::Float => ResolvedType::Float,
            ParsedType::Double => ResolvedType::Double,
            ParsedType::Boolean => ResolvedType::Boolean,
            ParsedType::String => ResolvedType::String,
            ParsedType::Empty => ResolvedType::Empty,
            ParsedType::TypeName(name) => self.types.get(name).unwrap().clone(),
            ParsedType::Function(parsed_function_type) => {
                let mut arg_types = Vec::new();

                for parsed_arg_type in &*parsed_function_type.arg_types {
                    arg_types.push(self.get_resolved_type(parsed_arg_type).clone())
                }

                let ret_type = self.get_resolved_type(&parsed_function_type.ret_type);

                ResolvedType::Function(FunctionType {
                    arg_types: Rc::new(arg_types),
                    ret_type: Rc::new(ret_type)
                })
            }
        }
    }
}

pub struct TypeResolver<'a> {
    symbol_table: &'a mut SymbolTable,
    unresolved_types: HashMap<String, i32>,
    type_dependencies: HashMap<String, Vec<String>>,
    unresolved_struct_declarations: HashMap<String, StructExpr>,
}

impl TypeResolver<'_> {
    pub fn new(symbol_table: &mut SymbolTable) -> TypeResolver<'_> {
        TypeResolver {
            symbol_table,
            unresolved_types: HashMap::new(),
            type_dependencies: HashMap::new(),
            unresolved_struct_declarations: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, exprs: &[Box<dyn Expr>]) -> Vec<Diagnostic> {
        for expr in exprs {
            expr.accept_visitor(self);
        }

        println!("Unresolved types: {:?}", self.unresolved_types);
        println!("Type dependencies: {:?}", self.type_dependencies);
        println!("Unresolved struct declarations: {:?}", self.unresolved_struct_declarations);

        Vec::new()
    }

    fn resolve_struct(&mut self, type_name: String) {
        println!("resolving {:?}", type_name);

        let struct_expr = self.unresolved_struct_declarations.remove(&type_name).unwrap();
            
        let resolved_type = self.symbol_table.resolve_struct_expr(&struct_expr);

        self.symbol_table.insert_type(type_name.clone(), resolved_type);

        if let Some(dependencies) = self.type_dependencies.remove(&type_name) {
            for dependency in dependencies {
                let n_dependencies = self.unresolved_types.get_mut(&dependency).unwrap();

                *n_dependencies -= 1;

                if *n_dependencies == 0 {
                    self.resolve_struct(dependency);
                }
            }
        }
    }
}

impl ExprVisitor<()> for TypeResolver<'_> {
    fn visit_empty(&mut self, _expr: &EmptyExpr) -> () { }
    fn visit_binary(&mut self, _expr: &BinaryExpr) -> () { }
    fn visit_unary(&mut self, _expr: &UnaryExpr) -> () { }
    fn visit_literal(&mut self, _expr: &LiteralExpr) -> () { }
    fn visit_var(&mut self, _expr: &VarExpr) -> () { }
    fn visit_if(&mut self, _expr: &IfExpr) -> () { }
    fn visit_assignment(&mut self, _expr: &AssignmentExpr) -> () { }
    fn visit_declaration(&mut self, _expr: &DeclarationExpr) -> () { }
    fn visit_block(&mut self, _expr: &BlockExpr) -> () { }
    fn visit_print(&mut self, _expr: &PrintExpr) -> () { }
    fn visit_rand(&mut self, _expr: &RandExpr) -> () { }
    fn visit_loop(&mut self, _expr: &LoopExpr) -> () { }
    fn visit_break(&mut self, _expr: &BreakExpr) -> () { }
    fn visit_input(&mut self, _expr: &InputExpr) -> () { }
    fn visit_call(&mut self, _expr: &CallExpr) -> () { }

    fn visit_struct(&mut self, expr: &StructExpr) -> () {
        let mut n_dependencies = 0;

        let struct_name = expr.name.clone();

        for (_, member_type) in expr.members.iter() {
            if let ParsedType::TypeName(type_name) = member_type {
                if !self.symbol_table.has_type_name(type_name) {
                    let dependencies: &mut Vec<String> = self.type_dependencies.entry(type_name.to_string()).or_insert_with(|| Vec::new());
                    dependencies.push(struct_name.to_string());

                    n_dependencies += 1;

                    continue;
                }
            }
        }

        self.unresolved_struct_declarations.insert(struct_name.to_string(), expr.clone());

        if n_dependencies != 0 {
            self.unresolved_types.insert(struct_name.to_string(), n_dependencies);
        } else {
            self.resolve_struct(struct_name.to_string());
        }
    }
}

pub struct VariableResolver<'a> {
    symbol_table: &'a mut SymbolTable,
    num_scopes: usize,
    scopes: Vec<HashMap<String, VarDeclaration>>,
}

impl VariableResolver<'_> {
    pub fn new(symbol_table: &mut SymbolTable) -> VariableResolver<'_> {
        VariableResolver {
            symbol_table,
            num_scopes: 1,
            scopes: vec![HashMap::new()],
        }
    }

    pub fn resolve(&mut self, exprs: &[Box<dyn Expr>]) -> Vec<Diagnostic> {
        for expr in exprs {
            self.resolve_expr(&**expr);
        }

        Vec::new()
    }

    fn resolve_expr(&mut self, expr: &(impl Expr + ?Sized)) {
        expr.accept_visitor(self)
    }
}

impl VariableResolver<'_> {
    fn declare(&mut self, name: &str, value_type: &ResolvedType) {
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
                    self.symbol_table.insert_variable(VarExpr::clone(var_expr), resolved_var);
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

impl ExprVisitor<()> for VariableResolver<'_> {
    fn visit_empty(&mut self, _expr: &EmptyExpr) {}

    fn visit_binary(&mut self, expr: &BinaryExpr) {
        self.resolve_expr(&*expr.left);
        self.resolve_expr(&*expr.right);
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) {
        self.resolve_expr(&*expr.expr);
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) {
        if let Value::Function(function) = &*expr.value {
            self.push_scope();

            if let ResolvedType::Function(function_type) = &self.symbol_table.get_resolved_type(&expr.parsed_type) {
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
        self.declare(&expr.identifier, &self.symbol_table.get_resolved_type(&expr.declaration_type));

        //Allow recursive funtions
        if let ParsedType::Function(_) = &expr.declaration_type {
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

    fn visit_struct(&mut self, expr: &StructExpr) {

    }
}