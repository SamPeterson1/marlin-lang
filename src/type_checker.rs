use crate::{environment::{Type, Value}, error::Diagnostic, expr::{AssignmentExpr, BinaryExpr, BlockExpr, BreakExpr, CallExpr, DeclarationExpr, EmptyExpr, Expr, ExprVisitor, IfExpr, InputExpr, LiteralExpr, LoopExpr, PrintExpr, RandExpr, UnaryExpr, VarExpr}, resolver::Resolver};

pub struct TypeChecker<'a> {
    resolver: &'a Resolver,
    loop_types: Vec<Option<Type>>,
    current_loop_idx: Option<usize>
}

impl<'a> TypeChecker<'a> {
    pub fn new(resolver: &'a Resolver) -> TypeChecker {
        TypeChecker {
            resolver,
            loop_types: Vec::new(),
            current_loop_idx: None
        }
    }

    pub fn check_types(&mut self, exprs: &[Box<dyn Expr>]) -> Vec<Diagnostic> {        
        for expr in exprs {
            println!("Check types");
            expr.accept_visitor(self);
        }

        Vec::new()
    }
}

impl ExprVisitor<Type> for TypeChecker<'_> {
    fn visit_empty(&mut self, _expr: &EmptyExpr) -> Type {
        Type::Empty
    }


    fn visit_binary(&mut self, expr: &BinaryExpr) -> Type {
        let left_type = expr.left.accept_visitor(self);
        let right_type = expr.right.accept_visitor(self);

        println!("{:?} {:?} {:?}", left_type, expr.operator, right_type);

        expr.operator.interpret_type(left_type, right_type).unwrap()
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> Type {
        let operand_type = expr.expr.accept_visitor(self);

        expr.operator.interpret_type(operand_type).unwrap()
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> Type {
        if let Value::Function(function) = &expr.value.value {
            function.body.accept_visitor(self);
        }

        expr.value.value_type.clone()
    }

    fn visit_var(&mut self, expr: &VarExpr) -> Type {
        self.resolver.get_type(expr).clone()
    }

    fn visit_if(&mut self, expr: &IfExpr) -> Type {
        let condition_type = expr.condition.accept_visitor(self);

        if condition_type != Type::Boolean {
            panic!("Invalid type {:?} for if condition", condition_type);
        }

        let success_type = expr.success.accept_visitor(self);

        if let Some(fail) = &expr.fail {
            let fail_type = fail.accept_visitor(self);

            println!("{:?}, {:?}", success_type, fail_type);
            
            if success_type != fail_type {
                panic!("Mismatched types {:?}, {:?} for if branches", success_type, fail_type);
            }
        }

        success_type
    }

    fn visit_assignment(&mut self, expr: &AssignmentExpr) -> Type {
        let var_type = self.resolver.get_type(&expr.asignee);

        let value_type = expr.expr.accept_visitor(self);

        println!("assigning {:?} to {:?}", var_type, value_type);

        if *var_type != value_type {
            panic!("Mismatched types {:?}, {:?} for assignment", var_type, value_type);
        }

        var_type.clone()
    }

    fn visit_declaration(&mut self, expr: &DeclarationExpr) -> Type {
        let value_type = expr.expr.accept_visitor(self);

        println!("declaring {:?} as {:?}", expr.identifier, value_type);

        if value_type != expr.declaration_type {
            panic!("Mismatched types {:?}, {:?} for assignment", value_type, expr.declaration_type);
        }

        value_type
    }

    fn visit_block(&mut self, expr: &BlockExpr) -> Type {
        let mut block_type = Type::Empty;

        for expr in &expr.exprs {
            block_type = expr.accept_visitor(self);
        }

        block_type
    }

    fn visit_print(&mut self, expr: &PrintExpr) -> Type {
        expr.expr.accept_visitor(self)
    }

    fn visit_rand(&mut self, expr: &RandExpr) -> Type {
        let min_type = expr.min.accept_visitor(self);
        let max_type = expr.max.accept_visitor(self);

        if min_type != Type::Integer || max_type != Type::Integer {
            panic!("Invalid types {:?}, {:?} for rand expression", min_type, max_type);
        }

        Type::Integer
    }

    fn visit_loop(&mut self, expr: &LoopExpr) -> Type {
        let loop_idx = self.loop_types.len();
        self.current_loop_idx = Some(loop_idx);
        self.loop_types.push(None);

        expr.body.accept_visitor(self);

        return match &self.loop_types[loop_idx] {
            Some(t) => t.clone(),
            None => Type::Empty
        }
    }

    fn visit_break(&mut self, expr: &BreakExpr) -> Type {
        let expr_type = expr.expr.accept_visitor(self);

        if self.current_loop_idx.is_none() {
            panic!("Break expression outside of loop");
        }

        let current_loop_idx = *self.current_loop_idx.as_ref().unwrap();

        match &self.loop_types[current_loop_idx] {
            Some(t) => {
                if *t != expr_type {
                    panic!("Mismatched types {:?}, {:?} for break expression", t, expr_type);
                } else {
                    t.clone()
                }
            },
            None => {
                self.loop_types[current_loop_idx] = Some(expr_type.clone());
                expr_type
            }
        }
    }

    fn visit_input(&mut self, expr: &InputExpr) -> Type {
        expr.return_type.clone()
    }

    fn visit_call(&mut self, expr: &CallExpr) -> Type {
        let expr_type = expr.func_expr.accept_visitor(self);

        println!("calling {:?}", expr_type);

        if let Type::Function(function_type) = &expr_type {
            if function_type.arg_types.len() != expr.args.len() {
                panic!("Mismatched number of arguments {:?}, {:?} for function call", function_type.arg_types.len(), expr.args.len());
            }


            expr.args.iter().enumerate().for_each(|(i, arg)| {

                let arg_type = arg.accept_visitor(self);
                println!("Function arg {:?} given {:?}", function_type.arg_types[i], arg_type);

                if arg_type != function_type.arg_types[i] {
                    panic!("Mismatched types {:?}, {:?} for function call", arg_type, function_type.arg_types[i]);
                }
            });
        }

        match expr_type {
            Type::Function(function_type) => (*function_type.ret_type).clone(),
            _ => panic!("Invalid type {:?} for function call", &expr_type)
        }
    }
}