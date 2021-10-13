use quote::ToTokens;
use syn::{Error, Expr, Lit};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

use crate::result::{OnError, Message, OnOk, OnExpr, ResultMacro, WhenExpr};
use crate::result::kw;

impl Parse for ResultMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            when: parse_expression_when(input)?,
            ok: parse_expression_ok(input)?,
            debug: parse_expression_debug(input)?,
            error: parse_expression_error(input)?,
        })
    }
}

fn decode_expr_type(expr: Expr) -> &'static str {
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

fn parse_expression(input: ParseStream, message: &str) -> syn::Result<Expr> {
    let expr = <Expr>::parse(input)?;

    match expr {
        Expr::Block(_) | Expr::TryBlock(_) => {
            // nothing else to process
        }
        Expr::Async(_) | Expr::Await(_) |
        Expr::Call(_) | Expr::Cast(_) |
        Expr::Field(_) | Expr::Group(_) |
        Expr::If(_) | Expr::Index(_) |
        Expr::Macro(_) | Expr::Match(_) |
        Expr::MethodCall(_) | Expr::Reference(_) |
        Expr::Try(_) =>
            parse_optional_semicolon(input)?,
        _ =>
            return Err(Error::new(expr.span(), format!("unsupported {} expression", message)))
    }

    Ok(expr)
}

fn parse_expression_debug(input: ParseStream) -> syn::Result<Option<Message>> {
    if input.peek(Token![@]) && input.peek2(kw::debug) {
        <Token![@]>::parse(input)?;
        <kw::debug>::parse(input)?;

        let message = parse_message(input, "debug", "err")?;

        parse_optional_semicolon(input)?;

        Ok(Some(message))
    } else {
        Ok(None)
    }
}

fn parse_expression_error(input: ParseStream) -> syn::Result<Option<OnError>> {
    if input.peek(Token![@]) && input.peek2(kw::error) {
        <Token![@]>::parse(input)?;
        <kw::error>::parse(input)?;

        let message = parse_message(input, "error", "err").ok();

        let expr = if !input.is_empty() && !input.peek(Token![@]) {
            if message.is_some() {
                <Token![;]>::parse(input)?;
            }

            if !input.is_empty() {
                let expr = parse_expression(input, "error")?;
                let captured = utils::search_for_ident(expr.to_token_stream(), "err");

                Some(OnExpr { expr, captured })
            } else {
                None
            }
        } else {
            parse_optional_semicolon(input)?;

            None
        };

        Ok(Some(OnError { message, expr }))
    } else {
        Ok(None)
    }
}

fn parse_expression_ok(input: ParseStream) -> syn::Result<Option<OnOk>> {
    if input.peek(Token![@]) && input.peek2(kw::ok) {
        <Token![@]>::parse(input)?;
        <kw::ok>::parse(input)?;

        Ok(Some(
            match parse_message(input, "ok", "ok") {
                Ok(message) => {
                    parse_optional_semicolon(input)?;
                    OnOk::Message(message)
                }
                Err(_) => {
                    let expr = parse_expression(input, "ok")?;
                    let captured = utils::search_for_ident(expr.to_token_stream(), "ok");

                    OnOk::Expr(OnExpr { captured, expr })
                }
            }
        ))
    } else {
        Ok(None)
    }
}

fn parse_expression_when(input: ParseStream) -> syn::Result<WhenExpr> {
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
        _ => return Err(Error::new(expr.span(), format!("{:?} is not a supported when expression", decode_expr_type(expr))))
    };

    Ok(WhenExpr { expr, tried })
}

fn parse_message(input: ParseStream, section: &str, ident: &str) -> syn::Result<Message> {
    let literal = <Lit>::parse(input)?;

    match literal {
        Lit::Str(_) | Lit::ByteStr(_) => {}
        _ => return Err(Error::new(literal.span(), format!("{} expects a string literal", section)))
    }

    let mut exprs = Vec::new();
    let mut captured = false;

    while input.peek(Token![,]) {
        <Token![,]>::parse(input)?;

        let expr = <Expr>::parse(input)?;

        if expr.to_token_stream().to_string() == ident {
            captured = true;
        }

        match expr {
            Expr::Array(_) | Expr::Await(_) |
            Expr::Binary(_) | Expr::Block(_) |
            Expr::Box(_) | Expr::Call(_) |
            Expr::Cast(_) | Expr::Field(_) |
            Expr::If(_) | Expr::Index(_) |
            Expr::Lit(_) | Expr::Macro(_) |
            Expr::Match(_) | Expr::MethodCall(_) |
            Expr::Path(_) | Expr::Reference(_) |
            Expr::Try(_) | Expr::TryBlock(_) |
            Expr::Tuple(_) | Expr::Unary(_) |
            Expr::Unsafe(_) => {}
            _ => return Err(Error::new(expr.span(), format!("unsupported {} expression", section)))
        }

        exprs.push(expr);
    }

    Ok(Message {
        args: if exprs.is_empty() { None } else { Some(exprs) },
        captured,
        fmt: literal,
    })
}

fn parse_optional_semicolon(input: ParseStream) -> syn::Result<()> {
    if let Some(punct) = input.cursor().punct() {
        if punct.0.as_char() == ';' {
            <Token![;]>::parse(input)?;
        }
    }

    Ok(())
}

mod utils {
    use proc_macro2::{TokenStream, TokenTree};
    use syn::{Expr, Stmt};

    pub(super) fn does_block_contain_try(expr: &Expr) -> bool {
        if let Expr::Block(block_expr) = expr {
            if (&block_expr.block.stmts)
                .into_iter()
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

    pub fn search_for_ident(stream: TokenStream, checked: &str) -> bool {
        for each in stream {
            match each {
                TokenTree::Group(group) => {
                    return search_for_ident(group.stream(), checked);
                }
                TokenTree::Ident(ident) if ident.to_string() == checked => {
                    return true;
                }
                _ => {}
            }
        }

        false
    }
}