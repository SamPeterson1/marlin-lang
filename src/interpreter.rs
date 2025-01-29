extern crate rand;
use std::{borrow::Borrow, cell::{Ref, RefCell}, collections::HashMap, io::{self, Write}, rc::Rc};
use std::str::FromStr;

use rand::Rng;

use crate::{environment::{Env, EnvRef, Function, Type, TypedValue, Value}, expr::{AssignmentExpr, BinaryExpr, BlockExpr, BreakExpr, CallExpr, DeclarationExpr, EmptyExpr, Expr, ExprVisitor, IfExpr, InputExpr, LiteralExpr, LoopExpr, PrintExpr, RandExpr, UnaryExpr, VarExpr}, resolver::Resolver};

pub struct Interpreter {
    break_value: Option<Rc<Value>>,
    resolver: Resolver,
    env: EnvRef
}

impl Interpreter {
    pub fn new(resolver: Resolver) -> Interpreter {
        Interpreter {
            break_value: None, 
            resolver,
            env: Env::new()
        }
    }

    pub fn interpret(&mut self, exprs: &[Box<dyn Expr>]) -> Rc<Value> {
        let mut result = Rc::new(Value::Empty);
        
        for expr in exprs {
            result = self.interpret_expr(&**expr);
        }
    
        result
    }

    fn interpret_expr(&mut self, expr: &(impl Expr + ?Sized)) -> Rc<Value> {
        expr.accept_visitor(self)
    }
}


impl ExprVisitor<Rc<Value>> for Interpreter {
    fn visit_empty(&mut self, _expr: &EmptyExpr) -> Rc<Value> {
        Rc::new(Value::Empty)
    }

    fn visit_binary(&mut self, expr: &BinaryExpr) -> Rc<Value> {
        let left = self.interpret_expr(&*expr.left);
        let right = self.interpret_expr(&*expr.right);
        
        expr.operator.interpret(left, right)
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> Rc<Value> {
        let operand = self.interpret_expr(&*expr.expr);
        
        expr.operator.interpret(operand)
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> Rc<Value> {
        if let Value::Function(function) = &expr.value.value {
            let mut function = Function::clone(&function);

            function.env = self.env.clone();

            Rc::new(Value::Function(function))
        } else {
            Rc::new(expr.value.value.clone())
        }
    }

    fn visit_var(&mut self, expr: &VarExpr) -> Rc<Value> {
        self.env.get_value(expr, &self.resolver)
    }

    fn visit_if(&mut self, expr: &IfExpr) -> Rc<Value> {
        let condition = self.interpret_expr(&*expr.condition);
    
        match condition.as_ref() {
            Value::Bool(x) => {
                if *x {
                    self.interpret_expr(&*expr.success)
                } else {
                    if let Some(fail) = &expr.fail {
                        self.interpret_expr(&**fail)
                    } else {
                        Rc::new(Value::Empty)
                    }
                }
            },
            _ => panic!("Cannot use non-boolean value in if statement")
        }
    }

    fn visit_assignment(&mut self, expr: &AssignmentExpr) -> Rc<Value> {
        let value = self.interpret_expr(&*expr.expr);
        self.env.put_value(&expr.asignee.identifier, &value, false);
        
        value
    }

    fn visit_declaration(&mut self, expr: &DeclarationExpr) -> Rc<Value> {
        let value = self.interpret_expr(&*expr.expr);
        self.env.put_value(&expr.identifier, &value, true);
        
        Rc::new(Value::Empty)
    }

    fn visit_block(&mut self, expr: &BlockExpr) -> Rc<Value> {
        let prev_scope = self.env.clone();
        self.env = Env::new_with_parent(&self.env);

        let mut value = Rc::new(Value::Empty);
    
        for expr in &expr.exprs {
            value = self.interpret_expr(&**expr);

            if let Some(_) = self.break_value {
                self.env = prev_scope;

                return value;
            }
        }
        
        self.env = prev_scope;

        value
    }

    fn visit_print(&mut self, expr: &PrintExpr) -> Rc<Value> {
        let result = self.interpret_expr(&*expr.expr);
        
        println!("{:?}", *result.as_ref());
    
        result
    }

    fn visit_rand(&mut self, expr: &RandExpr) -> Rc<Value> {
        if let Value::Int(min) = *self.interpret_expr(&*expr.min).as_ref() {
            if let Value::Int(max) = *self.interpret_expr(&*expr.max).as_ref() {
                let num = rand::thread_rng().gen_range(min..max);
                Rc::new(Value::Int(num))
            } else {
                panic!("Called random with non-numeric bounds");
            }
        } else {
            panic!("Called random with non-numeric bounds");
        }
    }

    fn visit_loop(&mut self, expr: &LoopExpr) -> Rc<Value> {
        let prev_scope = self.env.clone();
        self.env = Env::new_with_parent(&self.env);
        
        if let Some(initial) = &expr.initial {
           self.interpret_expr(&**initial);
        }
    
        let value = loop {
            let execute = match &expr.condition {
                Some(condition) => self.interpret_expr(&**condition),
                None => Rc::new(Value::Bool(true))
            };

            if let Value::Bool(execute) = *execute.as_ref() {
                if execute {
                    self.interpret_expr(&*expr.body);
                    
                    if let Some(increment) = &expr.increment {
                       self.interpret_expr(&**increment);
                    }
                } else {
                    break Rc::new(Value::Empty);
                }
            } else {
                panic!("Non-boolean expression used as condition in for loop");
            }

            let mut break_value: Option<Rc<Value>> = None;
            std::mem::swap(&mut break_value, &mut self.break_value);

            if let Some(break_value) = break_value {
                break break_value;
            }
        };

        self.env = prev_scope;

        value
    }

    fn visit_break(&mut self, expr: &BreakExpr) -> Rc<Value> {
        self.break_value = Some(self.interpret_expr(&*expr.expr));

        Rc::new(Value::Empty)
    }

    fn visit_input(&mut self, expr: &InputExpr) -> Rc<Value> {
        if let Value::String(prompt) = self.interpret_expr(&*expr.prompt).as_ref() {
            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let input_trim = input.trim();

            match &expr.return_type {
                Type::Integer => Rc::new(Value::Int(i32::from_str(input_trim).unwrap())),
                Type::Float => Rc::new(Value::Float(f32::from_str(input_trim).unwrap())),
                Type::Double => Rc::new(Value::Double(f64::from_str(input_trim).unwrap())),
                Type::String => Rc::new(Value::String(String::from_str(input_trim).unwrap())),
                _ => panic!("Invalid input type")
            }
        } else {
            panic!("Expected expression type String for input expression");
        }
    }

    fn visit_call(&mut self, expr: &CallExpr) -> Rc<Value> {
        let tmp = self.interpret_expr(&*expr.func_expr);
        let function: &Value = tmp.as_ref();

        let mut arg_env = match function {
            Value::Function(function) => Env::new_with_parent(&function.env),
            _ => panic!()
        };

        for (i, expr) in expr.args.iter().enumerate() {
            if let Value::Function(function) = function {
                let identifier = &function.args[i];
                let value = self.interpret_expr(&**expr);

                arg_env.put_value(&identifier, &value, true);
            } else {
                panic!();
            }
        }

        let caller_env = self.env.clone();
        self.env = arg_env;

        let value = match function {
            Value::Function(function) => {
                self.interpret_expr(&**function.body)
            },
            _ => panic!("Cannot call non-function value")
        };

        self.env = caller_env;


        value
    }

}