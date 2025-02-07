use std::{collections::HashMap, rc::Rc};

use crate::{expr::{Expr, VarExpr}, resolver::SymbolTable};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ParsedType {
    Integer, Float, Double, 
    Boolean, String, Empty,
    TypeName(String),
    Function(ParsedFunctionType)
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParsedFunctionType {
    pub arg_types: Rc<Vec<ParsedType>>,
    pub ret_type: Rc<ParsedType>
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ResolvedType {
    Integer, Float, Double, 
    Boolean, String, Empty,
    Function(FunctionType),
    Struct(StructType),
}

impl ResolvedType {
    pub fn is_numeric(&self) -> bool {
        *self == ResolvedType::Integer || *self == ResolvedType::Float || *self == ResolvedType::Double
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StructType {
    pub member_types: Rc<HashMap<String, ResolvedType>>
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FunctionType {
    pub arg_types: Rc<Vec<ResolvedType>>,
    pub ret_type: Rc<ResolvedType>
}

#[derive(Debug)]
pub struct Function {
    pub body: Rc<Box<dyn Expr>>,
    pub args: Rc<Vec<String>>,
    pub env: EnvRef
}

impl Clone for Function {
    fn clone(&self) -> Function {
        Function {
            body: Rc::clone(&self.body),
            args: Rc::clone(&self.args),
            env: self.env.clone()
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Int (i32),
    Float (f32),
    Double (f64),
    Bool (bool),
    String (String),
    Function (Function),
    Empty
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            Value::Int(x) => Value::Int(*x),
            Value::Float(x) => Value::Float(*x),
            Value::Double(x) => Value::Double(*x),
            Value::Bool(x) => Value::Bool(*x),
            Value::String(x) => Value::String(x.clone()),
            Value::Function(x) => Value::Function(x.clone()),
            Value::Empty => Value::Empty
        }
    }

}

impl Into<TypedValue> for Value {
    fn into(self) -> TypedValue {
        match self {
            Value::Int(_) => TypedValue::new(ResolvedType::Integer, self),
            Value::Float(_) => TypedValue::new(ResolvedType::Float, self),
            Value::Double(_) => TypedValue::new(ResolvedType::Double, self),
            Value::Bool(_) => TypedValue::new(ResolvedType::Boolean, self),
            Value::String(_) => TypedValue::new(ResolvedType::String, self),
            Value::Function(_) => panic!("Cannot infer type from function value"),
            Value::Empty => TypedValue::empty()
        }
    }
}

impl Value {
    pub fn as_int(&self) -> i32 { let Value::Int(x) = self else { unreachable!() }; *x }
    pub fn as_float(&self) -> f32 { let Value::Float(x) = self else { unreachable!() }; *x }
    pub fn as_double(&self) -> f64 { let Value::Double(x) = self else { unreachable!() }; *x }
    pub fn as_bool(&self) -> bool { let Value::Bool(x) = self else { unreachable!() }; *x }
    #[allow(dead_code)]
    pub fn as_string(&self) -> &str { let Value::String(x) = self else { unreachable!() }; x }
}

#[derive(Debug)]
pub struct TypedValue {
    pub value_type: ResolvedType,
    pub value: Value
}

impl TypedValue {
    pub fn new(value_type: ResolvedType, value: Value) -> TypedValue {
        TypedValue {
            value_type,
            value
        }
    }

    pub fn empty() -> TypedValue {
        TypedValue {
            value_type: ResolvedType::Empty,
            value: Value::Empty
        }
    }

}

impl AsRef<i32> for Value {
    fn as_ref(&self) -> &i32 {
        match &self {
            Value::Int(x) => x,
            _ => panic!("Tried to borrow non-integer value as integer")
        }
    }
}

impl AsMut<i32> for Value {
    fn as_mut(&mut self) -> &mut i32 {
        match self {
            Value::Int(x) => x,
            _ => panic!("Tried to borrow non-integer value as integer")
        }
    }
}

impl AsRef<f32> for Value {
    fn as_ref(&self) -> &f32 {
        match &self {
            Value::Float(x) => x,
            _ => panic!("Tried to borrow non-float value as float")
        }
    }
}

impl AsMut<f32> for Value {
    fn as_mut(&mut self) -> &mut f32 {
        match self {
            Value::Float(x) => x,
            _ => panic!("Tried to borrow non-float value as float")
        }
    }
}

impl AsRef<f64> for Value {
    fn as_ref(&self) -> &f64 {
        match self {
            Value::Double(x) => x,
            _ => panic!("Tried to borrow non-double value as double")
        }
    }
}

impl AsMut<f64> for Value {
    fn as_mut(&mut self) -> &mut f64 {
        match self {
            Value::Double(x) => x,
            _ => panic!("Tried to borrow non-double value as double")
        }
    }
}

impl AsRef<Function> for Value {
    fn as_ref(&self) -> &Function {
        match self {
            Value::Function(function) => function,
            _ => panic!("Tried to borrow non-function value as function")
        }
    }
}

impl AsMut<Function> for Value {
    fn as_mut(&mut self) -> &mut Function {
        match self {
            Value::Function(function) => function,
            _ => panic!("Tried borrow non-function value as function")
        }
    }
}

pub struct Env {
    pub vars: HashMap<String, Rc<Value>>,
    parent: EnvRef,
}

#[derive(Clone, Debug)]
pub struct EnvRef {
    env: Option<*mut Env>
}

impl EnvRef {
    fn new(env: Env) -> EnvRef {
        EnvRef {
            env: Some(Box::into_raw(Box::new(env)))
        }
    }

    pub fn new_none() -> EnvRef {
        EnvRef {
            env: None
        }
    }

    fn is_some(&self) -> bool {
        self.env.is_some()
    }

    pub fn as_env(&self) -> &Env {
        unsafe {
            &*self.env.unwrap()
        }
    }

    fn as_env_mut(&self) -> &mut Env {
        unsafe {
            &mut *self.env.unwrap()
        }
    }

    #[allow(dead_code)]
    pub fn prev_scope(&self) -> EnvRef {
        self.as_env().parent.clone()
    }

    pub fn put_value(&mut self, identifier: &str, value: &Rc<Value>, is_declaration: bool) {
        if is_declaration || self.as_env().vars.contains_key(identifier) {
            self.as_env_mut().vars.insert(identifier.to_string(), value.clone());
        } else if self.as_env().parent.is_some() {
            self.as_env_mut().parent.put_value(identifier, value, is_declaration);
        } else {
            panic!("Unknown variable name {:?}", identifier);
        }
    }

    pub fn get_value(&self, var_expr: &VarExpr, symbol_table: &SymbolTable) -> Rc<Value> {
        let mut env = self.as_env();
    
        for _ in 0..symbol_table.get_variable_dist(var_expr) {
            env = env.parent.as_env();
        }

        env.vars.get(&*var_expr.identifier).unwrap().clone()
    }
}

impl Env {
    pub fn new() -> EnvRef {
        EnvRef::new(Env {
            vars: HashMap::new(),
            parent: EnvRef::new_none(),
        })

    }

    pub fn new_with_parent(parent: &EnvRef) -> EnvRef {
        EnvRef::new(Env {
            vars: HashMap::new(),
            parent: parent.clone()
        })
    }
}