pub mod assignment_expr;
pub mod binary_expr;
pub mod block_expr;
pub mod break_expr;
pub mod call_expr;
pub mod declaration_expr;
pub mod get_char_expr;
pub mod if_expr;
pub mod literal_expr;
pub mod loop_expr;
pub mod put_char_expr;
pub mod new_array_expr;
pub mod unary_expr;
pub mod var_expr;
pub mod function_item;
pub mod struct_item;
pub mod constructor_item;
pub mod main_item;
pub mod arguments;
pub mod parsed_type;
pub mod function_prototype;
pub mod parameters;
pub mod program;
pub mod constructor_call;

use assignment_expr::AssignmentExpr;
use binary_expr::BinaryExpr;
use block_expr::BlockExpr;
use break_expr::BreakExpr;
use call_expr::CallExpr;
use declaration_expr::DeclarationExpr;
use erased_serde::serialize_trait_object;
use get_char_expr::GetCharExpr;
use if_expr::IfExpr;
use literal_expr::LiteralExpr;
use loop_expr::LoopExpr;
use put_char_expr::PutCharExpr;
use serde::Serialize;
use serde_json;
use unary_expr::UnaryExpr;
use var_expr::VarExpr;

use crate::{ast::{constructor_call::{ConstructorCallExpr}, constructor_item::ConstructorItem, function_item::FunctionItem, main_item::MainItem, new_array_expr::NewArrayExpr, struct_item::StructItem}, token::{PositionRange, Positioned}};

pub trait ASTVisitable: AcceptsASTVisitor<()> {}

pub trait AcceptsASTVisitor<T> {
    fn accept_visitor(&self, visitor: &mut dyn ASTVisitor<T>) -> T;
}

pub trait ASTNode: ASTVisitable + Positioned + erased_serde::Serialize {}
serialize_trait_object!(ASTNode);

#[derive(Debug)]
pub struct ASTWrapper<E> {
    pub data: E,
    pub position: PositionRange,
}

impl<E> ASTWrapper<E> {
    pub fn new(data: E, position: PositionRange) -> Self {
        Self {
            data,
            position,
        }
    }
}

impl<E: Serialize> Serialize for ASTWrapper<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        
        // Create a map to hold all our fields
        let mut map = serializer.serialize_map(None)?;
        
        // Add ! field FIRST (alphabetically before all other fields)
        let full_type_name = std::any::type_name::<E>();
        map.serialize_entry("!", &full_type_name)?;
        
        // Serialize the data and flatten its fields into our map (middle entries)
        let data_value = serde_json::to_value(&self.data).map_err(serde::ser::Error::custom)?;
        if let serde_json::Value::Object(data_map) = data_value {
            for (key, value) in data_map {
                map.serialize_entry(&key, &value)?;
            }
        }
        
        // Add z_position field LAST (alphabetically after all other fields)
        map.serialize_entry("z_position", &self.position)?;
        
        map.end()
    }
}

impl<E> Positioned for ASTWrapper<E> {
    fn get_position(&self) -> &PositionRange {
        &self.position
    }
}

pub trait ASTVisitor<T> {
    fn visit_binary(&mut self, _node: &ASTWrapper<BinaryExpr>) -> T { unimplemented!() }
    fn visit_unary(&mut self, _node: &ASTWrapper<UnaryExpr>) -> T { unimplemented!() }
    fn visit_literal(&mut self, _node: &ASTWrapper<LiteralExpr>) -> T { unimplemented!() }
    fn visit_var(&mut self, _node: &ASTWrapper<VarExpr>) -> T { unimplemented!() }
    fn visit_if(&mut self, _node: &ASTWrapper<IfExpr>) -> T { unimplemented!() }
    fn visit_assignment(&mut self, _node: &ASTWrapper<AssignmentExpr>) -> T { unimplemented!() }
    fn visit_declaration(&mut self, _node: &ASTWrapper<DeclarationExpr>) -> T { unimplemented!() }
    fn visit_block(&mut self, _node: &ASTWrapper<BlockExpr>) -> T { unimplemented!() }
    fn visit_loop(&mut self, _node: &ASTWrapper<LoopExpr>) -> T { unimplemented!() }
    fn visit_break(&mut self, _node: &ASTWrapper<BreakExpr>) -> T { unimplemented!() }
    fn visit_call(&mut self, _node: &ASTWrapper<CallExpr>) -> T { unimplemented!() }
    fn visit_constructor_call(&mut self, _node: &ASTWrapper<ConstructorCallExpr>) -> T { unimplemented!() }
    fn visit_new_array(&mut self, _node: &ASTWrapper<NewArrayExpr>) -> T { unimplemented!() }
    fn visit_put_char(&mut self, _node: &ASTWrapper<PutCharExpr>) -> T { unimplemented!() }
    fn visit_get_char(&mut self, _node: &ASTWrapper<GetCharExpr>) -> T { unimplemented!() }
    fn visit_function(&mut self, _node: &ASTWrapper<FunctionItem>) -> T { unimplemented!() }
    fn visit_struct(&mut self, _node: &ASTWrapper<StructItem>) -> T { unimplemented!() }
    fn visit_constructor(&mut self, _node: &ASTWrapper<ConstructorItem>) -> T { unimplemented!() }
    fn visit_main(&mut self, _node: &ASTWrapper<MainItem>) -> T { unimplemented!() }
}

#[macro_export]
macro_rules! impl_ast_node {
    ($Name: ident, $VisitFunction: ident) => {
        impl crate::ast::ASTNode for crate::ast::ASTWrapper<$Name> {}
        impl crate::ast::ASTVisitable for crate::ast::ASTWrapper<$Name> {}

        impl<T> crate::ast::AcceptsASTVisitor<T> for crate::ast::ASTWrapper<$Name> {
            fn accept_visitor(&self, visitor: &mut dyn crate::ast::ASTVisitor<T>) -> T {
                visitor.$VisitFunction(self)
            }
        }
    }
}