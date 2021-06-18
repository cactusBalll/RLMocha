use crate::{RLResult, RLToken, RLVal};
use std::collections::VecDeque;

///指示当前token
static mut PTR: usize = 0;
///语法分析
pub fn parse(token_stream: Vec<RLToken>) -> Vec<RLResult> {
    let mut ret = Vec::new();
    unsafe {
        while PTR < token_stream.len() {
            match &token_stream[PTR] {
                RLToken::Str(s) => {
                    ret.push(Ok(RLVal::RLStr(s.clone())));
                    PTR += 1;
                }
                RLToken::Symbol(s) => {
                    ret.push(Ok(RLVal::Symbol(s.clone())));
                    PTR += 1;
                }
                RLToken::Number(x) => {
                    ret.push(Ok(RLVal::Number(*x)));
                    PTR += 1;
                }
                RLToken::Comment(_s) => {
                    PTR += 1;
                }
                RLToken::BracketSL => {
                    ret.push(sexpr(&token_stream));
                }
                RLToken::BracketML => {
                    ret.push(qexpr(&token_stream));
                }
                //右括号应该在sexpr(),qexpr()函数中被吃掉
                RLToken::BracketSR | RLToken::BracketMR => {}
            }
        }
        PTR = 0;
    }

    ret
}
///单行一串项,没有括号,当作Sexpr,用于repl
pub fn parse_one_line(token_stream: Vec<RLToken>) -> RLResult {
    let mut ret: VecDeque<RLVal> = VecDeque::new();
    unsafe {
        while PTR < token_stream.len() {
            match &token_stream[PTR] {
                RLToken::Str(s) => {
                    ret.push_back(RLVal::RLStr(s.clone()));
                    PTR += 1;
                }
                RLToken::Symbol(s) => {
                    ret.push_back(RLVal::Symbol(s.clone()));
                    PTR += 1;
                }
                RLToken::Number(x) => {
                    ret.push_back(RLVal::Number(*x));
                    PTR += 1;
                }
                RLToken::Comment(_s) => {
                    PTR += 1;
                }
                RLToken::BracketSL => {
                    ret.push_back(sexpr(&token_stream)?);
                }
                RLToken::BracketML => {
                    ret.push_back(qexpr(&token_stream)?);
                }
                RLToken::BracketSR | RLToken::BracketMR => {
                    break;
                }
            }
        }
        PTR = 0;
    }
    Ok(RLVal::Sexpr(ret))
}
fn sexpr(token_stream: &Vec<RLToken>) -> RLResult {
    let mut ret: VecDeque<RLVal> = VecDeque::new();
    unsafe {
        PTR += 1;
        while PTR < token_stream.len() {
            match &token_stream[PTR] {
                RLToken::Str(s) => {
                    ret.push_back(RLVal::RLStr(s.clone()));
                    PTR += 1;
                }
                RLToken::Symbol(s) => {
                    ret.push_back(RLVal::Symbol(s.clone()));
                    PTR += 1;
                }
                RLToken::Number(x) => {
                    ret.push_back(RLVal::Number(*x));
                    PTR += 1;
                }
                RLToken::Comment(_s) => {
                    PTR += 1;
                }
                RLToken::BracketSL => {
                    ret.push_back(sexpr(token_stream)?);
                }
                RLToken::BracketML => {
                    ret.push_back(qexpr(token_stream)?);
                }
                RLToken::BracketSR | RLToken::BracketMR => {
                    break;
                }
            }
        }
        //吃掉右括号
        PTR += 1;
    }

    Ok(RLVal::Sexpr(ret))
}
fn qexpr(token_stream: &Vec<RLToken>) -> RLResult {
    let mut ret: VecDeque<RLVal> = VecDeque::new();
    unsafe {
        PTR += 1;
        while PTR < token_stream.len() {
            match &token_stream[PTR] {
                RLToken::Str(s) => {
                    ret.push_back(RLVal::RLStr(s.clone()));
                    PTR += 1;
                }
                RLToken::Symbol(s) => {
                    ret.push_back(RLVal::Symbol(s.clone()));
                    PTR += 1;
                }
                RLToken::Number(x) => {
                    ret.push_back(RLVal::Number(*x));
                    PTR += 1;
                }
                RLToken::Comment(_s) => {
                    PTR += 1;
                }
                RLToken::BracketSL => {
                    ret.push_back(sexpr(token_stream)?);
                }
                RLToken::BracketML => {
                    ret.push_back(qexpr(token_stream)?);
                }
                RLToken::BracketSR | RLToken::BracketMR => {
                    break;
                }
            }
        }
        //吃掉右括号
        PTR += 1;
    }
    Ok(RLVal::Qexpr(ret))
}
