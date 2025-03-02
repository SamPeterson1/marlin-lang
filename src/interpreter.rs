extern crate rand;
use std::{collections::HashMap, io::{self, Write}, rc::Rc};
use std::str::FromStr;

use rand::Rng;

use crate::{environment::{Env, EnvRef, Function, ParsedType, Struct, Value, ValueRef}, expr::{AssignmentExpr, BinaryExpr, BlockExpr, BreakExpr, CallExpr, DeclarationExpr, EmptyExpr, Expr, ExprVisitor, IfExpr, InputExpr, LiteralExpr, LoopExpr, PrintExpr, RandExpr, StructExpr, StructInitializerExpr, UnaryExpr, VarExpr}, resolver::SymbolTable};
pub struct Interpreter {
    break_value: Option<ValueRef>,
    symbol_table: SymbolTable,
    env: EnvRef
}

impl Interpreter {
    pub fn new(symbol_table: SymbolTable) -> Interpreter {
        Interpreter {
            break_value: None, 
            symbol_table,
            env: Env::new()
        }
    }

    pub fn interpret(&mut self, exprs: &[Box<dyn Expr>]) -> ValueRef {
        let mut result = ValueRef::new(Value::Empty);
        
        for expr in exprs {
            result = self.interpret_expr(&**expr);
        }
    
        result
    }

    fn interpret_expr(&mut self, expr: &(impl Expr + ?Sized)) -> ValueRef {
        expr.accept_visitor(self)
    }
}


impl ExprVisitor<ValueRef> for Interpreter {
    fn visit_empty(&mut self, _expr: &EmptyExpr) -> ValueRef {
        ValueRef::new(Value::Empty)
    }

    fn visit_binary(&mut self, expr: &BinaryExpr) -> ValueRef {
        let left = self.interpret_expr(&*expr.left);
        let right = self.interpret_expr(&*expr.right);
        
        expr.operator.interpret(left, right)
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> ValueRef {
        let operand = self.interpret_expr(&*expr.expr);
        
        expr.operator.interpret(operand)
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> ValueRef {
        if let Value::Function(function) = &*expr.value.as_ref() {
            let mut function = Function::clone(&function);

            function.env = self.env.clone();

            ValueRef::new(Value::Function(function))
        } else {
            expr.value.clone()
        }
    }

    fn visit_var(&mut self, expr: &VarExpr) -> ValueRef {
        let mut value = self.env.get_value(expr, &self.symbol_table);

        for member_access in expr.member_accesses.iter() {
            let member;
            if let Value::Struct(struct_value) = &*value.as_ref() {
                member = struct_value.member_values.get(member_access).unwrap().clone();
            } else {
                panic!("Cannot access field of non-struct type");
            }

            value = member;
        }

        value
    }

    fn visit_if(&mut self, expr: &IfExpr) -> ValueRef {
        let condition = self.interpret_expr(&*expr.condition);
    
        let value = match *condition.as_ref() {
            Value::Bool(x) => {
                if x {
                    self.interpret_expr(&*expr.success)
                } else {
                    if let Some(fail) = &expr.fail {
                        self.interpret_expr(&**fail)
                    } else {
                        ValueRef::new(Value::Empty)
                    }
                }
            },
            _ => panic!("Cannot use non-boolean value in if statement")
        };

        value
    }

    fn visit_assignment(&mut self, expr: &AssignmentExpr) -> ValueRef {
        let assignment_value = self.interpret_expr(&*expr.expr);
        let n_member_accesses = expr.asignee.member_accesses.len();

        if n_member_accesses == 0 {
            self.env.put_value_ref(&expr.asignee.identifier, assignment_value.clone(), false);
        } else {
            let mut assignee_value = self.env.get_value(&expr.asignee, &self.symbol_table);

            for member_access in expr.asignee.member_accesses[0..(n_member_accesses-1)].iter() {
                let assignee_member;
                if let Value::Struct(struct_value) = &*assignee_value.as_ref() {
                    assignee_member = struct_value.member_values.get(member_access).unwrap().clone()
                } else {
                    panic!("Cannot access member of non-struct value");
                }

                assignee_value = assignee_member;
            }

            let last_member_access = expr.asignee.member_accesses.get(n_member_accesses - 1).unwrap();

            if let Value::Struct(struct_value) = &mut *assignee_value.as_mut() {
                struct_value.member_values.insert(last_member_access.clone(), assignment_value.clone());
            };
        }

        assignment_value
    }

    fn visit_declaration(&mut self, expr: &DeclarationExpr) -> ValueRef {
        let value = self.interpret_expr(&*expr.expr);
        self.env.put_value_ref(&expr.identifier, value, true);
        
        ValueRef::new(Value::Empty)
    }

    fn visit_block(&mut self, expr: &BlockExpr) -> ValueRef {
        let prev_scope = self.env.clone();
        self.env = Env::new_with_parent(&self.env);

        let mut value = ValueRef::new(Value::Empty);
    
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

    fn visit_print(&mut self, expr: &PrintExpr) -> ValueRef {
        let result = self.interpret_expr(&*expr.expr);
        
        match &*result.as_ref() {
            Value::Int(x) => println!("{}", x),
            Value::Float(x) => println!("{}", x),
            Value::Double(x) => println!("{}", x),
            Value::String(x) => println!("{}", x),
            Value::Bool(x) => println!("{}", x),
            Value::Struct(x) => println!("{:?}", x),
            Value::Empty => (),
            _ => panic!("Cannot print value")
        }
    
        result
    }

    fn visit_rand(&mut self, expr: &RandExpr) -> ValueRef {
        if let Value::Int(min) = *self.interpret_expr(&*expr.min).as_ref() {
            if let Value::Int(max) = *self.interpret_expr(&*expr.max).as_ref() {
                let num = rand::thread_rng().gen_range(min..max);
                ValueRef::new(Value::Int(num))
            } else {
                panic!("Called random with non-numeric bounds");
            }
        } else {
            panic!("Called random with non-numeric bounds");
        }
    }

    fn visit_loop(&mut self, expr: &LoopExpr) -> ValueRef {
        let prev_scope = self.env.clone();
        self.env = Env::new_with_parent(&self.env);
        
        if let Some(initial) = &expr.initial {
           self.interpret_expr(&**initial);
        }
    
        let value = loop {
            let execute = match &expr.condition {
                Some(condition) => self.interpret_expr(&**condition),
                None => ValueRef::new(Value::Bool(true))
            };

            if let Value::Bool(execute) = *execute.as_ref() {
                if execute {
                    self.interpret_expr(&*expr.body);
                    
                    if let Some(increment) = &expr.increment {
                       self.interpret_expr(&**increment);
                    }
                } else {
                    break ValueRef::new(Value::Empty);
                }
            } else {
                panic!("Non-boolean expression used as condition in for loop");
            }

            let mut break_value: Option<ValueRef> = None;
            std::mem::swap(&mut break_value, &mut self.break_value);

            if let Some(break_value) = break_value {
                break break_value;
            }
        };

        self.env = prev_scope;

        value
    }

    fn visit_break(&mut self, expr: &BreakExpr) -> ValueRef {
        self.break_value = Some(self.interpret_expr(&*expr.expr));

        ValueRef::new(Value::Empty)
    }

    fn visit_input(&mut self, expr: &InputExpr) -> ValueRef {
        if let Value::String(prompt) = &*self.interpret_expr(&*expr.prompt).as_ref() {
            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let input_trim = input.trim();

            match expr.return_type {
                ParsedType::Integer => ValueRef::new(Value::Int(i64::from_str(input_trim).unwrap())),
                ParsedType::Float => ValueRef::new(Value::Float(f32::from_str(input_trim).unwrap())),
                ParsedType::Double => ValueRef::new(Value::Double(f64::from_str(input_trim).unwrap())),
                ParsedType::String => ValueRef::new(Value::String(String::from_str(input_trim).unwrap())),
                _ => panic!("Invalid input type")
            }
        } else {
            panic!("Expected expression type String for input expression");
        }
    }

    fn visit_call(&mut self, expr: &CallExpr) -> ValueRef {
        let tmp = self.interpret_expr(&*expr.func_expr);

        let mut arg_env = match &*tmp.as_ref() {
            Value::Function(function) => Env::new_with_parent(&function.env),
            _ => panic!()
        };

        for (i, expr) in expr.args.iter().enumerate() {
            if let Value::Function(function) = &*tmp.as_ref() {
                let identifier = &function.args[i];
                let value = self.interpret_expr(&**expr);

                arg_env.put_value_ref(&identifier, value, true);
            } else {
                panic!();
            }
        }

        let caller_env = self.env.clone();
        self.env = arg_env;

        let value = match &*tmp.as_ref() {
            Value::Function(function) => {
                self.interpret_expr(&**function.body)
            },
            _ => panic!("Cannot call non-function value")
        };

        self.env = caller_env;


        value
    }

    fn visit_struct(&mut self, _expr: &StructExpr) -> ValueRef {
        ValueRef::new(Value::Empty)
    }

    fn visit_struct_initializer(&mut self, expr: &StructInitializerExpr) -> ValueRef {
        let mut member_values = HashMap::new();

        for (key, value) in expr.member_inits.iter() {
            member_values.insert(key.clone(), value.accept_visitor(self));
        }

        ValueRef::new(Value::Struct(Struct {
            member_values
        }))
    }

}