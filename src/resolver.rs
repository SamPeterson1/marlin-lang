use std::{collections::HashMap, rc::Rc};

use crate::{error::{Diagnostic, DiagnosticType}, expr::{assignment_expr::AssignmentExpr, binary_expr::BinaryExpr, block_expr::BlockExpr, break_expr::BreakExpr, call_expr::CallExpr, declaration_expr::DeclarationExpr, get_address_expr::GetAddressExpr, get_char_expr::GetCharExpr, if_expr::IfExpr, literal_expr::LiteralExpr, loop_expr::LoopExpr, put_char_expr::PutCharExpr, static_array_expr::StaticArrayExpr, struct_initializer_expr::StructInitializerExpr, unary_expr::UnaryExpr, var_expr::VarExpr, ExprVisitable, ExprVisitor}, item::{FunctionItem, Item, ItemVisitor, StructItem}, logger::Logger, types::{parsed_type::{ParsedFunctionType, ParsedType, ParsedTypeName}, resolved_type::{FunctionType, PointerType, ResolvedType, StructType}}};

struct VarDeclaration {
    is_defined: bool,
    is_argument: bool,
    declaration_type: ResolvedType,
    id: i32,
}

#[derive(Debug)]
pub struct ResolvedVar {
    pub value_type: ResolvedType,
    pub is_argument: bool,
    pub id: i32,
}

pub struct SymbolTable {
    pub types: HashMap<String, ResolvedType>,
    pub variables: HashMap<VarExpr, ResolvedVar>,
    pub functions: HashMap<String, ResolvedType>,
    logger: Logger,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            types: HashMap::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
            logger: Logger::new("SymbolTable"),
        }
    }

    pub fn resolve(&mut self, items: &[Box<dyn Item>]) -> Vec<Diagnostic> {
        let mut errors = Vec::new();

        let mut type_resolver = TypeResolver::new(self);

        errors.append(&mut type_resolver.resolve(items));

        let mut variable_resolver = VariableResolver::new(self);

        errors.append(&mut variable_resolver.resolve(items));
        
        errors
    }

    pub fn insert_variable(&mut self, expr: VarExpr, var: ResolvedVar) {
        self.variables.insert(expr, var);
    }

    pub fn get_variable(&self, expr: &VarExpr) -> &ResolvedVar {
        self.variables.get(expr).unwrap()
    }

    pub fn get_function(&self, name: &str) -> &ResolvedType {
        self.functions.get(name).unwrap()
    }

    pub fn insert_type(&mut self, type_name: String, resolved_type: ResolvedType) {
        self.types.insert(type_name, resolved_type);
    }

    pub fn insert_function(&mut self, function_name: String, resolved_type: ResolvedType) {
        self.functions.insert(function_name, resolved_type);
    }

    pub fn has_type_name(&self, type_name: &str) -> bool {
        self.types.contains_key(type_name)
    }

    pub fn resolve_struct_item(&self, struct_item: &StructItem) -> ResolvedType {
        let mut member_types = Vec::new();

        for (member_name, member_type) in struct_item.members.iter() {
            member_types.push((member_name.to_string(), self.get_resolved_type(member_type).unwrap()));
        }

        ResolvedType::Struct(StructType::new(member_types))
    }

    pub fn get_resolved_type(&self, parsed_type: &ParsedType) -> Result<ResolvedType, Diagnostic> {
       Ok(match parsed_type {
            ParsedType::Integer => ResolvedType::Integer,
            ParsedType::Double => ResolvedType::Double,
            ParsedType::Boolean => ResolvedType::Boolean,
            ParsedType::Empty => ResolvedType::Empty,
            ParsedType::TypeName(ParsedTypeName {name, position}) => {
                let name_string = (*name).to_string();
                let resolved_type = self.types.get(&name_string);

                if let Some(resolved_type) = resolved_type {
                    self.logger.log_detailed_info(&format!("Resolved type {} to {:?}", name, resolved_type));
                    return Ok(resolved_type.clone());
                } else {
                    self.logger.log_brief_error(&format!("Unknown type name {:?}", name));
                    return Err(Diagnostic::new(1, DiagnosticType::Error, *position, format!("Unknown type name {:?}", name)));
                }
            }
            ParsedType::Function(parsed_function_type) => {
                let mut arg_types = Vec::new();

                for parsed_arg_type in &*parsed_function_type.arg_types {
                    arg_types.push(self.get_resolved_type(parsed_arg_type)?.clone())
                }

                let ret_type = self.get_resolved_type(&parsed_function_type.ret_type)?;

                ResolvedType::Function(FunctionType {
                    arg_types: Rc::new(arg_types),
                    ret_type: Rc::new(ret_type)
                })
            },
            ParsedType::Pointer(pointer_type) => ResolvedType::Pointer(PointerType {
                pointee: Rc::new(self.get_resolved_type(&pointer_type.pointee)?)
            })
        })
    }
}

pub struct TypeResolver<'a> {
    symbol_table: &'a mut SymbolTable,
    unresolved_types: HashMap<String, i32>,
    type_dependencies: HashMap<String, Vec<String>>,
    unresolved_struct_declarations: HashMap<String, StructItem>,
    logger: Logger,
    diagnostics: Vec<Diagnostic>,
}

impl TypeResolver<'_> {
    pub fn new(symbol_table: &mut SymbolTable) -> TypeResolver<'_> {
        TypeResolver {
            symbol_table,
            unresolved_types: HashMap::new(),
            type_dependencies: HashMap::new(),
            unresolved_struct_declarations: HashMap::new(),
            logger: Logger::new("TypeResolver"),
            diagnostics: Vec::new(),
        }
    }

    pub fn resolve(&mut self, items: &[Box<dyn Item>]) -> Vec<Diagnostic> {
        for item in items {
            item.accept_visitor(self);
        }

        println!("Unresolved types: {:?}", self.unresolved_types);
        println!("Type dependencies: {:?}", self.type_dependencies);

        Vec::new()
    }

    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.logger.log_brief_error(&format!("Pushing diagnostic: {}", diagnostic));
        self.diagnostics.push(diagnostic);
    }

    fn err_unknown_type(&mut self, type_name: &str) {
        self.logger.log_brief_error(&format!("Unknown type name {}", type_name));
    }

    fn resolve_struct(&mut self, type_name: String) {
        self.logger.log_brief_info(&format!("Fully resolved struct {}", type_name));

        let struct_expr = self.unresolved_struct_declarations.remove(&type_name).unwrap();
            
        let resolved_type = self.symbol_table.resolve_struct_item(&struct_expr);

        self.symbol_table.insert_type(type_name.clone(), resolved_type);

        if let Some(dependencies) = self.type_dependencies.remove(&type_name) {
            for dependency in dependencies {
                let n_dependencies = self.unresolved_types.get_mut(&dependency).unwrap();

                *n_dependencies -= 1;

                self.logger.log_detailed_info(&format!("Struct {:?} now has {:?} dependencies after resolving {:?}", dependency, *n_dependencies, type_name));

                if *n_dependencies == 0 {
                    self.resolve_struct(dependency);
                }
            }
        }
    }
}

impl ItemVisitor<()> for TypeResolver<'_> {
    fn visit_struct(&mut self, expr: &StructItem) -> () {
        let mut n_dependencies = 0;

        let struct_name = expr.name.clone();

        self.logger.log_brief_info(&format!("Resolving struct {:?}", struct_name));

        for (_, member_type) in expr.members.iter() {
            if let ParsedType::TypeName(ParsedTypeName {name: type_name, ..}) = member_type {
                if !self.symbol_table.has_type_name(type_name) {
                    let dependencies= self.type_dependencies.entry(type_name.to_string()).or_insert_with(|| Vec::new());
                    dependencies.push(struct_name.to_string());

                    self.logger.log_brief_info(&format!("Struct {:?} depends on unresolved type {:?}", struct_name, type_name));
                    
                    n_dependencies += 1;

                    continue;
                }
            }
        }

        self.logger.log_brief_info(&format!("Struct {:?} has {:?} dependencies", struct_name, n_dependencies));

        self.unresolved_struct_declarations.insert(struct_name.to_string(), expr.clone());

        if n_dependencies != 0 {
            self.unresolved_types.insert(struct_name.to_string(), n_dependencies);
        } else {
            self.resolve_struct(struct_name.to_string());
        }
    }
    
    fn visit_function(&mut self, item: &FunctionItem) -> () { 
        let resolved_type = self.symbol_table.get_resolved_type(&ParsedType::Function(ParsedFunctionType {
            arg_types: Rc::new(item.args.iter().map(|arg| arg.1.clone()).collect()),
            ret_type: Rc::new(item.ret_type.clone())
        }));

        match resolved_type {
            Ok(resolved_type) => {
                self.logger.log_detailed_info(&format!("Resolved function {:?} to {:?}", item.name, resolved_type));
                self.symbol_table.insert_function(item.name.to_string(), resolved_type);
            },
            Err(diagnostic) => {
                self.push_diagnostic(diagnostic);
            }
        }
    }
}

pub struct VariableResolver<'a> {
    symbol_table: &'a mut SymbolTable,
    num_scopes: usize,
    scopes: Vec<HashMap<String, VarDeclaration>>,
    logger: Logger,
    diagnostics: Vec<Diagnostic>,
}

impl VariableResolver<'_> {
    pub fn new(symbol_table: &mut SymbolTable) -> VariableResolver<'_> {
        VariableResolver {
            symbol_table,
            num_scopes: 1,
            scopes: vec![HashMap::new()],
            logger: Logger::new("VariableResolver"),
            diagnostics: Vec::new(),
        }
    }

    pub fn resolve(mut self, items: &[Box<dyn Item>]) -> Vec<Diagnostic> {
        println!("Resolving");

        for item in items {
            item.accept_visitor(&mut self);
        }

        self.diagnostics
    }

    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.logger.log_brief_error(&format!("Pushing diagnostic: {}", diagnostic));
        self.diagnostics.push(diagnostic);
    }

    fn clear_scopes(&mut self) {
        self.num_scopes = 1;
        self.scopes = vec![HashMap::new()];
    }

}

impl VariableResolver<'_> {
    fn declare(&mut self, id: i32, is_argument: bool, name: &str, value_type: &ResolvedType) {
        let declaration = VarDeclaration {
            is_defined: false,
            id,
            is_argument,
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
                        id: declaration.id,

                        is_argument: declaration.is_argument,
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

impl ItemVisitor<()> for VariableResolver<'_> {
    fn visit_struct(&mut self, item: &StructItem) {}

    fn visit_function(&mut self, item: &FunctionItem) {
        self.clear_scopes();

        for (i, (arg_name, arg_type)) in item.args.iter().enumerate() {
            match self.symbol_table.get_resolved_type(arg_type) {
                Ok(value_type) => {
                    self.logger.log_detailed_info(&format!("Resolved function argument {:?} of type {:?}", arg_name, value_type));
                    self.declare(i as i32, true, arg_name, &value_type);
                    self.define(arg_name);
                },
                Err(diagnostic) => {
                    self.logger.log_brief_error(&format!("Error resolving function argument {:?} of type {}", arg_name, arg_type));
                    self.push_diagnostic(diagnostic);
                }
            }
        }

        item.expr.accept_visitor(self);
    }
}

impl ExprVisitor<()> for VariableResolver<'_> {
    fn visit_binary(&mut self, expr: &BinaryExpr) {
        expr.left.accept_visitor(self);
        expr.right.accept_visitor(self);
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) {
        expr.expr.accept_visitor(self);
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) { }

    fn visit_var(&mut self, expr: &VarExpr) {
        self.resolve_var(expr);

        for array_access in expr.array_accesses.iter() {
            array_access.accept_visitor(self);
        }
    }

    fn visit_if(&mut self, expr: &IfExpr) {
        expr.condition.accept_visitor(self);
        expr.success.accept_visitor(self);
        
        if let Some(fail) = &expr.fail {
            fail.accept_visitor(self);
        }
    }

    fn visit_assignment(&mut self, expr: &AssignmentExpr) {
        expr.asignee.accept_visitor(self);
        expr.expr.accept_visitor(self);
    }

    fn visit_declaration(&mut self, expr: &DeclarationExpr) {
        match self.symbol_table.get_resolved_type(&expr.declaration_type) {
            Ok(value_type) => {
                self.logger.log_detailed_info(&format!("Resolved declaration of {:?} of type {}", expr.identifier, value_type));
                self.declare(expr.id, false, &expr.identifier, &value_type);
            },
            Err(diagnostic) => {
                self.logger.log_brief_error(&format!("Error resolving declaration of {:?} of type {}", expr.identifier, expr.declaration_type));
                self.push_diagnostic(diagnostic);
            }
        }

        expr.expr.accept_visitor(self);

        self.define(&expr.identifier);
    }

    fn visit_block(&mut self, expr: &BlockExpr) {
        self.push_scope();
        for expr in &expr.exprs {
            expr.accept_visitor(self);
        }
        self.pop_scope();
    }

    fn visit_loop(&mut self, expr: &LoopExpr) {
        self.push_scope();
        
        if let Some(initial) = &expr.initial {
            initial.accept_visitor(self);
        }
        
        if let Some(condition) = &expr.condition {
            condition.accept_visitor(self);
        }

        if let Some(increment) = &expr.increment {
            increment.accept_visitor(self);
        }

        expr.body.accept_visitor(self);
        self.pop_scope();
    }

    fn visit_break(&mut self, expr: &BreakExpr) {
        expr.expr.accept_visitor(self);
    }

    fn visit_call(&mut self, expr: &CallExpr) {
        if !self.symbol_table.functions.contains_key(&*expr.function) {
            self.logger.log_brief_error(&format!("Unknown function {:?}", expr.function));
            self.push_diagnostic(Diagnostic::new(1, DiagnosticType::Error, expr.position, format!("Unknown function {:?}", expr.function)));
            return;
        }

        for arg in &expr.args {
            arg.accept_visitor(self);
        }
    }

    fn visit_struct_initializer(&mut self, expr: &StructInitializerExpr) {
        for (_, value) in expr.member_inits.iter() {
            value.accept_visitor(self);
        }
    }

    fn visit_get_address(&mut self, expr: &GetAddressExpr) -> () {
        expr.var_expr.accept_visitor(self);
    }

    fn visit_static_array(&mut self, expr: &StaticArrayExpr) -> () {

    }

    fn visit_get_char(&mut self, expr: &GetCharExpr) -> () {
        
    }

    fn visit_put_char(&mut self, expr: &PutCharExpr) -> () {
        expr.expr.accept_visitor(self);
    }
}