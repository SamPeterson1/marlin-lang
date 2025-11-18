use crate::{error::{Diagnostic, DiagnosticType, PARSER_ERR_GROUP}, token::{PositionRange, TokenType}};

const ERR_EXPECTED_STRUCT_NAME: u32 = 1;
const ERR_EXPECTED_MEMBER_NAME: u32 = 2;
const ERR_EXPECTED_FN_NAME: u32 = 4;
const ERR_UNEXPECTED_TOKEN: u32 = 5;
const ERR_EXPECTED_ITEM: u32 = 6;
const ERR_EXPECTED_ARG_TYPE: u32 = 7;
const ERR_EXPECTED_ARG_NAME: u32 = 8;
const ERR_EXPECTED_RETURN_TYPE: u32 = 9;
const ERR_EXPECTED_DECLARATION_TYPE: u32 = 10;
const ERR_EXPECTED_DECLARATION_NAME: u32 = 11;
const ERR_EXPECTED_VAR: u32 = 12;

fn new_diagnostic(sub_code: u32, position: PositionRange, msg: String) -> Diagnostic {
    Diagnostic::new(PARSER_ERR_GROUP + sub_code, DiagnosticType::Error, position, msg)
}

pub fn err_expected_struct_name(position: PositionRange) -> Diagnostic {
    let msg = String::from("expected struct name");
    new_diagnostic(ERR_EXPECTED_STRUCT_NAME, position, msg)
}

pub fn err_expected_member_name(position: PositionRange) -> Diagnostic {
    let msg = String::from("expected member name");
    new_diagnostic(ERR_EXPECTED_MEMBER_NAME, position, msg)
}

pub fn err_expected_fn_name(position: PositionRange) -> Diagnostic {
    let msg = String::from("expected function name");
    new_diagnostic(ERR_EXPECTED_FN_NAME, position, msg)
}

pub fn err_unexpected_token(position: PositionRange) -> Diagnostic {
    let msg = String::from("unexpected token");
    new_diagnostic(ERR_UNEXPECTED_TOKEN, position, msg)
}

pub fn err_expected_item(position: PositionRange) -> Diagnostic {
    let msg = String::from("expected item");
    new_diagnostic(ERR_EXPECTED_ITEM, position, msg)
}

pub fn err_expected_token(position: PositionRange, token_type: TokenType) -> Diagnostic {
    let msg = format!("expected {}", token_type);
    new_diagnostic(ERR_UNEXPECTED_TOKEN, position, msg)
}

pub fn err_expected_arg_type(position: PositionRange) -> Diagnostic {
    let msg = String::from("expected argument type");
    new_diagnostic(ERR_EXPECTED_ARG_TYPE, position, msg)
}

pub fn err_expected_arg_name(position: PositionRange) -> Diagnostic {
    let msg = String::from("expected argument name");
    new_diagnostic(ERR_EXPECTED_ARG_NAME, position, msg)
}

pub fn err_expected_return_type(position: PositionRange) -> Diagnostic {
    let msg = String::from("expected return type");
    new_diagnostic(ERR_EXPECTED_RETURN_TYPE, position, msg)
}

pub fn err_expected_declaration_type(position: PositionRange) -> Diagnostic {
    let msg = String::from("expected declaration type");
    new_diagnostic(ERR_EXPECTED_DECLARATION_TYPE, position, msg)
}

pub fn err_expected_declaration_name(position: PositionRange) -> Diagnostic {
    let msg = String::from("expected declaration name");
    new_diagnostic(ERR_EXPECTED_DECLARATION_NAME, position, msg)
}

pub fn err_expected_var(position: PositionRange) -> Diagnostic {
    let msg = String::from("expected variable");
    new_diagnostic(ERR_EXPECTED_VAR, position, msg)
}
