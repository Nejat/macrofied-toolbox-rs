use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Error, Expr, Lit};
use syn::parse::{Parse, ParseStream, Peek};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Paren;

use crate::common::{Message, OnSuccess, WhenExpr};
use crate::common::models::OnExpr;
use crate::common::parse::utils::search_for_ident;

mod kw {
    custom_keyword![debug];
    custom_keyword![when];
}

pub fn decode_expr_type(expr: &Expr) -> &'static str {
    match expr {
        Expr::Array(_) => "array",
        Expr::Assign(_) => "assign",
        Expr::AssignOp(_) => "assign-op",
        Expr::Async(_) => "async",
        Expr::Await(_) => "await",
        Expr::Binary(_) => "binary",
        Expr::Block(_) => "block",
        Expr::Box(_) => "box",
        Expr::Break(_) => "break",
        Expr::Call(_) => "call",
        Expr::Cast(_) => "cast",
        Expr::Closure(_) => "closure",
        Expr::Continue(_) => "continue",
        Expr::Field(_) => "field",
        Expr::ForLoop(_) => "for-loop",
        Expr::Group(_) => "group",
        Expr::If(_) => "if",
        Expr::Index(_) => "index",
        Expr::Let(_) => "let",
        Expr::Lit(_) => "lit",
        Expr::Loop(_) => "loop",
        Expr::Macro(_) => "macro",
        Expr::Match(_) => "match",
        Expr::MethodCall(_) => "method call",
        Expr::Paren(_) => "paren",
        Expr::Path(_) => "path",
        Expr::Range(_) => "range",
        Expr::Reference(_) => "reference",
        Expr::Repeat(_) => "repeat",
        Expr::Return(_) => "return",
        Expr::Struct(_) => "struct",
        Expr::Try(_) => "try",
        Expr::TryBlock(_) => "try-block",
        Expr::Tuple(_) => "tuple",
        Expr::Type(_) => "type",
        Expr::Unary(_) => "unary",
        Expr::Unsafe(_) => "unsafe",
        Expr::Verbatim(_) => "verbatim",
        Expr::While(_) => "while",
        Expr::Yield(_) => "yield",
        Expr::__TestExhaustive(_) => unimplemented!()
    }
}

pub fn parse_expression_success<T>(
    input: ParseStream, token: T, section: &str, capture: Option<String>,
) -> syn::Result<Option<OnSuccess>>
    where T: Peek
{
    if input.peek(Token![@]) && input.peek2(token) {
        <Token![@]>::parse(input)?;
        <Ident>::parse(input)?;

        let mut captured = capture;

        if input.peek(Paren) {
            let capture_span = input.span();
            let content;

            parenthesized!(content in input);

            let fields: Punctuated<Ident, Token![,]> = content.parse_terminated(Ident::parse)?;

            if fields.len() != 1 {
                return Err(Error::new(
                    capture_span,
                    format!("\"{}\" is not valid, use only one value", fields.to_token_stream()),
                ));
            }

            captured = Some(fields[0].to_string());

            <Token![=>]>::parse(input)?;
        }

        Ok(Some(
            if let Ok(message) = parse_message(input, section, &captured) {
                parse_optional_semicolon(input)?;
                OnSuccess::Message(message)
            } else {
                let expr = parse_expression(input, section)?;

                OnSuccess::Expr(OnExpr { captured, expr })
            }
        ))
    } else {
        Ok(None)
    }
}

pub fn parse_expression(input: ParseStream, section: &str) -> syn::Result<Expr> {
    let expr = <Expr>::parse(input)?;

    match expr {
        Expr::Block(_) | Expr::TryBlock(_) | Expr::Unsafe(_) => {}
        Expr::Assign(_) | Expr::AssignOp(_) |
        Expr::Await(_) | Expr::Binary(_) |
        Expr::Box(_) | Expr::Break(_) |
        Expr::Call(_) | Expr::Cast(_) |
        Expr::Closure(_) | Expr::Continue(_) |
        Expr::Field(_) | Expr::Group(_) |
        Expr::If(_) | Expr::Index(_) |
        Expr::Macro(_) | Expr::Match(_) |
        Expr::MethodCall(_) | Expr::Path(_) |
        Expr::Reference(_) | Expr::Repeat(_) |
        Expr::Return(_) | Expr::Try(_) |
        Expr::Tuple(_) | Expr::Unary(_) =>
            parse_optional_semicolon(input)?,
        _ =>
            return Err(Error::new(
                expr.span(),
                format!(
                    "{:?} is not a supported {} expression, try placing it into a code block",
                    decode_expr_type(&expr), section
                ),
            ))
    }

    Ok(expr)
}

pub fn parse_expression_debug(
    input: ParseStream, ident: &Option<String>,
) -> syn::Result<Option<Message>> {
    if input.peek(Token![@]) && input.peek2(kw::debug) {
        <Token![@]>::parse(input)?;
        <kw::debug>::parse(input)?;

        let message = parse_message(input, "debug", ident)?;

        parse_optional_semicolon(input)?;

        Ok(Some(message))
    } else {
        Ok(None)
    }
}

pub fn parse_expression_when(input: ParseStream) -> syn::Result<WhenExpr> {
    if input.peek(Token![@]) {
        <Token![@]>::parse(input)?;
        <kw::when>::parse(input)?;
    }

    let mut expr = <Expr>::parse(input)?;
    let mut tried = false;

    match expr {
        Expr::Await(_) | Expr::Call(_) |
        Expr::Cast(_) | Expr::Field(_) |
        Expr::Group(_) | Expr::If(_) |
        Expr::Index(_) | Expr::Macro(_) |
        Expr::Match(_) | Expr::MethodCall(_) |
        Expr::Path(_) | Expr::Reference(_) =>
            parse_optional_semicolon(input)?,
        Expr::Try(try_expr) => {
            tried = true;
            expr = try_expr.expr.as_ref().clone();
            parse_optional_semicolon(input)?;
        }
        Expr::Block(_) if utils::does_block_contain_try(&expr) =>
            return Err(Error::new(expr.span(), "block can not contain a try expression")),
        Expr::Block(_) => {}
        _ => return Err(Error::new(
            expr.span(),
            format!("{:?} is not a supported when expression", decode_expr_type(&expr)),
        ))
    };

    Ok(WhenExpr { expr, tried })
}

pub fn parse_message(
    input: ParseStream, section: &str, ident: &Option<String>,
) -> syn::Result<Message> {
    let literal = <Lit>::parse(input)?;

    match literal {
        Lit::Str(_) | Lit::ByteStr(_) => {}
        _ => return Err(Error::new(literal.span(), format!("{} expects a string literal", section)))
    }

    let mut exprs = Vec::new();
    let mut captured = None;

    while input.peek(Token![,]) {
        <Token![,]>::parse(input)?;

        let expr = <Expr>::parse(input)?;

        if let Some(checked) = &ident {
            if search_for_ident(expr.to_token_stream(), checked) {
                captured = Some(checked.to_string());
            }
        }

        match expr {
            Expr::Array(_) | Expr::Await(_) |
            Expr::Binary(_) | Expr::Block(_) |
            Expr::Call(_) | Expr::Cast(_) |
            Expr::Field(_) | Expr::Group(_) |
            Expr::If(_) | Expr::Index(_) |
            Expr::Lit(_) | Expr::Macro(_) |
            Expr::Match(_) | Expr::MethodCall(_) |
            Expr::Paren(_) | Expr::Path(_) |
            Expr::Range(_) | Expr::Reference(_) |
            Expr::Repeat(_) | Expr::Try(_) |
            Expr::TryBlock(_) | Expr::Tuple(_) |
            Expr::Unary(_) | Expr::Unsafe(_) => {}
            _ => return Err(Error::new(
                expr.span(),
                format!("{:?} is not a supported {} expression", decode_expr_type(&expr), section)
            ))
        }

        exprs.push(expr);
    }

    Ok(Message {
        args: if exprs.is_empty() { None } else { Some(exprs) },
        captured,
        fmt: literal,
    })
}

pub fn parse_optional_semicolon(input: ParseStream) -> syn::Result<()> {
    if let Some(punct) = input.cursor().punct() {
        if punct.0.as_char() == ';' {
            <Token![;]>::parse(input)?;
        }
    }

    Ok(())
}

pub mod utils {
    #[cfg(any(feature = "option", feature = "result"))]
    use proc_macro2::{TokenStream, TokenTree};
    use syn::{Expr, Stmt};

    pub(super) fn does_block_contain_try(expr: &Expr) -> bool {
        if let Expr::Block(block_expr) = expr {
            if (&block_expr.block.stmts)
                .iter()
                .any(
                    |stmt| {
                        match stmt {
                            Stmt::Expr(expr) |
                            Stmt::Semi(expr, _) =>
                                if let Expr::Try(_) = expr {
                                    return true;
                                },
                            _ => {}
                        }
                        false
                    }
                ) {
                return true;
            }
        }

        false
    }

    #[cfg(any(feature = "option", feature = "result"))]
    pub fn search_for_ident(stream: TokenStream, checked: &str) -> bool {
        for each in stream {
            match each {
                TokenTree::Group(group) => {
                    return search_for_ident(group.stream(), checked);
                }
                TokenTree::Ident(ident) if ident == checked => {
                    return true;
                }
                _ => {}
            }
        }

        false
    }
}