use crate::ast::*;

pub struct CodeGen {

}

impl CodeGen {
    pub fn new() -> Self {
        let context = Context::create();
        let module = context.create_module("main_module");
        let builder = context.create_builder();

        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function("main", fn_type, None);
    
        let entry = context.append_basic_block(function, "entry");
        builder.position_at_end(entry);
    
        builder.build_return(Some(&i32_type.const_int(42, false)));
    
        println!("{}", module.print_to_string().to_string());

        Self {

        }
    }
}

impl ASTVisitor<'_, ()> for CodeGen {
    fn visit_binary(&mut self, node: &BinaryExpr) {
        
    }

    fn visit_unary(&mut self, node: &UnaryExpr) {
        
    }

    fn visit_literal(&mut self, node: &LiteralExpr) {
        
    }

    fn visit_member_access(&mut self, node: &MemberAccess) {
        
    }

    fn visit_var(&mut self, node: &VarExpr) {
        
    }

    fn visit_if(&mut self, node: &IfExpr) {
        
    }

    fn visit_assignment(&mut self, node: &AssignmentExpr) {
        
    }

    fn visit_delete(&mut self, node: &DeleteExpr) {
        
    }

    fn visit_declaration(&mut self, node: &DeclarationExpr) {
        
    }

    fn visit_block(&mut self, node: &BlockExpr) {
        for expr in &node.exprs {
            expr.accept_visitor(self);
        }
    }

    fn visit_loop(&mut self, node: &LoopExpr) {
        
    }

    fn visit_exit(&mut self, node: &ExitExpr) {
        
    }

    fn visit_constructor_call(&mut self, node: &ConstructorCallExpr) {
        
    }

    fn visit_new_array(&mut self, node: &NewArrayExpr) {
        
    }

    fn visit_impl(&mut self, node: &ImplItem) {
        
    }

    fn visit_function(&mut self, node: &FunctionItem) {
        
    }

    fn visit_struct(&mut self, node: &StructItem) {
        
    }

    fn visit_constructor(&mut self, node: &ConstructorItem) {
        
    }

    fn visit_main(&mut self, node: &MainItem) {
        node.body.accept_visitor(self);
    }

    fn visit_program(&mut self, node: &Program) {
        for item in &node.items {
            item.accept_visitor(self);
        }
    }
}