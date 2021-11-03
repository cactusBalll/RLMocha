use crate::{RLFuncStru, RLResult, RLVal, RLenv, ReplEnv};
use std::collections::VecDeque;
//captured链 调用链混淆
#[inline]
fn eval_lambda(
    f: &RLFuncStru,
    mut para: VecDeque<RLVal>,
    env: *mut RLenv,
    repl: *mut ReplEnv,
) -> RLResult {
    let mut g = f.clone();
    unsafe {
        g.env = (*repl).new_env();
        //复制env中绑定的变量
        for (k, v) in (*f.env).env.iter() {
            (*g.env).env.insert(k.clone(), v.clone());
        }
        (*g.env).outer_scope = env;
        (*g.env).captured = (*f.env).captured;//bugfixed: 计算闭包时没有复制捕获列表
    }
    let partial = para.len() < g.para.len();

    for para_name in g.para.iter() {
        if let Some(x) = para.pop_front() {
            unsafe {
                (*g.env).env.insert(para_name.clone(), x);
            }
        } else {
            break;
        }
    }
    //把已经绑定的参数从参数列表中消去
    for _i in 0..para.len() {
        g.para.pop_front();
    }
    //返回部分绑定的函数
    if partial {
        return Ok(RLVal::RLFunc(g));
    }
    //绑定可变参数
    if para.len() > g.para.len() {
        if g.isvla {
            unsafe {
                (*g.env).env.insert(g.vla_para.clone(), RLVal::Qexpr(para));
            }
        } else {
            return Err("语义:`eval_lamda`:λ表达式错误的参数数量".to_string());
        }
    }
    return eval_expr(*g.body.clone(), g.env, repl);
}
pub unsafe fn get_var(
    s: &String,
    env: *mut RLenv,
    repl: *mut ReplEnv,
) -> Result<&mut RLVal, String> {
    //println!("Debug:get_var,getting {}", s);
    //println!("getting from...{:?}",(*env).env);
    match (*env).env.get_mut(s) {
        Some(x) => Ok(x),
        None => {
            if (*env).captured != 0x0 as *mut RLenv {
                //println!("getting from captured...{:?}",(*env).env);
                match get_var(s, (*env).captured, repl) {
                    Ok(x) => {
                        return Ok(x);
                    }
                    Err(_s) => {}
                }
            }
            if (*env).outer_scope != 0x0 as *mut RLenv {
                match get_var(s, (*env).outer_scope, repl) {
                    Ok(x) => {
                        return Ok(x);
                    }
                    Err(_s) => {}
                }
            }
            if let Some(x) = (*(*repl).global).env.get_mut(s) {
                return Ok(x);
            } else {
                return Err(format!("语义:`get_var`:变量未定义:尝试获取`{}`", s));
            }
            /*let mut p = (*env).outer_scope;
            loop {
                if p == 0x0 as *mut RLenv {
                    if let Some(x) = (*(*repl).global).env.get_mut(s) {
                        return Ok(x);
                    } else {
                        return Err(format!("语义:`get_var`:变量未定义:尝试获取`{}`", s));
                    }
                }
                if let Some(x) = (*p).env.get_mut(s) {
                    return Ok(x);
                } else {
                    if (*p).captured != 0x0 as *mut RLenv {
                        match get_var(s, (*p).captured, repl) {
                            Ok(x) => {
                                return Ok(x);
                            }
                            Err(_s) => {}
                        }
                    }
                    p = (*p).outer_scope;
                }
            }*/
        }
    }
}

pub fn eval_expr(expr: RLVal, env: *mut RLenv, repl: *mut ReplEnv) -> RLResult {
    match expr {
        RLVal::Symbol(s) => Ok(RLVal::Symbol(s)),
        RLVal::Number(x) => Ok(RLVal::Number(x)),
        RLVal::RLStr(s) => Ok(RLVal::RLStr(s)),

        RLVal::Sexpr(v) => {
            if v.len() == 0 {
                //空的Sexpr,其语义就是自己
                return Ok(RLVal::Sexpr(v));
            } else {
                let mut mediate = VecDeque::new();
                for item in v {
                    let tresult = eval_expr(item, env, repl)?;
                    match tresult {
                        RLVal::Symbol(s) => unsafe {
                            let mut rlv = get_var(&s, env, repl)?;
                            while let RLVal::Symbol(s) = rlv {
                                rlv = get_var(s, env, repl)?;
                            }
                            mediate.push_back(rlv.clone());
                        },
                        _ => {
                            mediate.push_back(tresult);
                        }
                    }
                    //println!("Debug:{:?}", mediate);
                }
                if mediate.len() == 1 {
                    return Ok(mediate.pop_front().unwrap());
                }
                let sym_func = mediate.pop_front().unwrap();
                match sym_func {
                    RLVal::Symbol(_s) => {
                        return Err("语义:`eval_expr`:带入symbol后运算符仍为symbol".to_string());
                    }
                    RLVal::BuiltinFunc(f) => f(mediate, env, repl),
                    RLVal::RLFunc(f) => {
                        return eval_lambda(&f, mediate, env, repl);
                    }
                    _ => {
                        return Err(
                            "语义:`eval_expr`:无效的Sexpr,Sexpr头元素为λ或者内置函数".to_string()
                        );
                    }
                }
            }
        }
        RLVal::Qexpr(v) => Ok(RLVal::Qexpr(v)),
        RLVal::RLFunc(f) => Ok(RLVal::RLFunc(f)),
        RLVal::BuiltinFunc(f) => Ok(RLVal::BuiltinFunc(f)),
        //注释在语法分析已被丢弃,不可能匹配到
        RLVal::Comment(_s) => Ok(RLVal::Qexpr(VecDeque::new())),
    }
}
