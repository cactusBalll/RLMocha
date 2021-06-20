use crate::{RLFuncStru, RLResult, RLVal, RLenv, ReplEnv};
use std::collections::VecDeque;
use std::fs;
const RLVAL_TRUE: RLVal = RLVal::Number(1.);
const RLVAL_FALSE: RLVal = RLVal::Number(0.);
pub fn add(xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    let mut sum = 0.;
    for x in xs {
        if let RLVal::Number(xn) = x {
            sum += xn;
        } else {
            return Err("builtin function `add`:只能<Number>类型相加".to_string());
        }
    }
    Ok(RLVal::Number(sum))
}
pub fn mul(xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    let mut sum = 1.;
    for x in xs {
        if let RLVal::Number(xn) = x {
            sum *= xn;
        } else {
            return Err("builtin function `add`:只能<Number>类型相加".to_string());
        }
    }
    Ok(RLVal::Number(sum))
}
pub fn div(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let a;
        let b;
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            a = x;
        } else {
            return Err("builtin function `div`:参数错误".to_string());
        }
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            b = x;
        } else {
            return Err("builtin function `div`:参数错误".to_string());
        }
        if b == 0. {
            return Err("builtin function `div`:除零错误".to_string());
        } else {
            return Ok(RLVal::Number(a / b));
        }
    } else {
        return Err("builtin function `div`:参数错误".to_string());
    }
}
//取负
pub fn minus(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            Ok(RLVal::Number(-x))
        } else {
            return Err("builtin function `minus`:参数错误".to_string());
        }
    } else {
        return Err("builtin function `minus`:参数错误".to_string());
    }
}
//就是map，将一个函数作用与一个Qexpr，返回Qexpr
//pub fn _map(mut xs: VecDeque<RLVal>,_env:*mut RLenv,_repl:*mut ReplEnv) -> RLResult {
//}
pub fn lambda(mut xs: VecDeque<RLVal>, env: *mut RLenv, repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let para;
        let body;
        if let RLVal::Qexpr(x) = xs.pop_front().unwrap() {
            para = x;
        } else {
            return Err("builtin function `lambda`:参数错误".to_string());
        }
        if let RLVal::Qexpr(x) = xs.pop_front().unwrap() {
            body = x;
        } else {
            return Err("builtin function `lamda`:参数错误".to_string());
        }
        let mut vla = false;
        let mut vla_para = String::new();
        let mut paravec: VecDeque<String> = VecDeque::new();
        //处理参数部分
        for i in 0..para.len() {
            match &para[i] {
                RLVal::Symbol(x) => {
                    //{x & xs}可变参数
                    if x == "&" {
                        vla = true;
                        continue;
                    } else if vla {
                        vla_para = x.clone();
                        break;
                    } else {
                        paravec.push_back(x.clone());
                    }
                }
                _ => {
                    return Err("builtin function `lamda`:参数错误,非法参数列表".to_string());
                }
            }
        }
        unsafe {
            let nenv = (*repl).new_env();
            (*nenv).captured = env;
            Ok(RLVal::RLFunc(RLFuncStru {
                para: paravec,
                body: Box::new(RLVal::Sexpr(body)),
                env: nenv,
                global: (*repl).global,
                isvla: vla,
                vla_para: vla_para,
            }))
        }
    } else {
        return Err("builtin function `lamda`:参数错误".to_string());
    }
}
pub fn def(mut xs: VecDeque<RLVal>, _env: *mut RLenv, repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let name;
        let val;
        if let RLVal::Qexpr(mut x) = xs.pop_front().unwrap() {
            if let RLVal::Symbol(s) = x.pop_front().unwrap() {
                name = s;
            } else {
                return Err("builtin function `def`:参数错误".to_string());
            }
        } else {
            return Err("builtin function `def`:参数错误".to_string());
        }
        val = xs.pop_front().unwrap();
        //        println!("Debug:env[{}] = {:?}",name,val);
        unsafe {
            (*(*repl).global).env.insert(name, val);
        }
    } else {
        return Err("builtin function `def`:参数错误".to_string());
    }
    Ok(RLVal::Sexpr(VecDeque::new()))
}
pub fn head(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::Qexpr(mut x) = xs.pop_front().unwrap() {
            let mut ret = VecDeque::new();
            ret.push_back(x.pop_front().unwrap_or(RLVal::Sexpr(VecDeque::new())));
            Ok(RLVal::Qexpr(ret))
        } else {
            return Err("builtin function `head`:参数错误".to_string());
        }
    } else {
        return Err("builtin function `head`:参数错误".to_string());
    }
}
pub fn tail(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::Qexpr(mut x) = xs.pop_front().unwrap() {
            x.pop_front();
            Ok(RLVal::Qexpr(x))
        } else {
            return Err("builtin function `tail`:参数错误".to_string());
        }
    } else {
        return Err("builtin function `tail`:参数错误".to_string());
    }
}
pub fn list(xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    Ok(RLVal::Qexpr(xs))
}
//接收一个Qexpr，并将其作为Sexpr求值
pub fn eval(mut xs: VecDeque<RLVal>, env: *mut RLenv, repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::Qexpr(x) = xs.pop_front().unwrap() {
            crate::evaluation::eval_expr(RLVal::Sexpr(x), env, repl)
        } else {
            return Err("builtin function `eval`:参数错误".to_string());
        }
    } else {
        return Err("builtin function `eval`:参数错误".to_string());
    }
}
pub fn join(xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    let mut ret = VecDeque::new();
    for x in xs {
        if let RLVal::Qexpr(mut v) = x {
            ret.append(&mut v);
        } else {
            return Err("builtin function `join`:参数错误".to_string());
        }
    }
    Ok(RLVal::Qexpr(ret))
}
///排序函数 小于
pub fn lt(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let a;
        let b;
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            a = x;
        } else {
            return Err("builtin function `lt(<)`:参数错误".to_string());
        }
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            b = x;
        } else {
            return Err("builtin function `lt(<)`:参数错误".to_string());
        }
        if a < b {
            Ok(RLVAL_TRUE.clone())
        } else {
            Ok(RLVAL_FALSE.clone())
        }
    } else {
        return Err("builtin function `lt(<)`:参数错误".to_string());
    }
}
///排序函数 大于
pub fn gt(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let a;
        let b;
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            a = x;
        } else {
            return Err("builtin function `gt(>)`:参数错误".to_string());
        }
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            b = x;
        } else {
            return Err("builtin function `gt(>)`:参数错误".to_string());
        }
        if a > b {
            Ok(RLVAL_TRUE.clone())
        } else {
            Ok(RLVAL_FALSE.clone())
        }
    } else {
        return Err("builtin function `gt(>)`:参数错误".to_string());
    }
}
///排序函数 等于 类型只有浮点数...
pub fn eq(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let a = xs.pop_front().unwrap();
        let b = xs.pop_front().unwrap();
        if a == b {
            Ok(RLVAL_TRUE.clone())
        } else {
            Ok(RLVAL_FALSE.clone())
        }
    } else {
        return Err("builtin function `eq(=)`:参数错误".to_string());
    }
}
pub fn condition(mut xs: VecDeque<RLVal>, env: *mut RLenv, repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 3 {
        let a;
        let b;
        let c;
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            a = x;
        } else {
            return Err("builtin function `condition(if)`:参数错误".to_string());
        }
        if let RLVal::Qexpr(x) = xs.pop_front().unwrap() {
            b = x;
        } else {
            return Err("builtin function `condition(if)`:参数错误".to_string());
        }
        if let RLVal::Qexpr(x) = xs.pop_front().unwrap() {
            c = x;
        } else {
            return Err("builtin function `condition(if)`:参数错误".to_string());
        }
        let cond = RLVal::Number(a);
        if cond == RLVAL_TRUE || a > 0. {
            crate::evaluation::eval_expr(RLVal::Sexpr(b), env, repl)
        } else if cond == RLVAL_FALSE || a < 0. {
            crate::evaluation::eval_expr(RLVal::Sexpr(c), env, repl)
        } else {
            return Err(
                "builtin function `condition(if)`:参数错误:条件要求是布尔值(Number(1)/Number(0))"
                    .to_string(),
            );
        }
    } else {
        return Err("builtin function `condition(if)`:参数错误".to_string());
    }
}
pub fn rl_print(xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    for x in xs {
        println!("RLmocha>> {}", x);
    }
    Ok(RLVal::Sexpr(VecDeque::new()))
}
pub fn rl_error(xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    for x in xs {
        println!("RLmocha:err>> {}", x);
    }
    Err("builtin function `rl_error`:运行时错误".to_string())
}
///从文件加载
pub fn load_file(mut xs: VecDeque<RLVal>, _env: *mut RLenv, repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::RLStr(s) = xs.pop_front().unwrap() {
            let str_loaded = match fs::read_to_string(s) {
                Ok(s_read) => s_read,
                Err(_e) => {
                    return Err("builtin function `load_file(load)`:IO错误:加载文件失败".to_string())
                }
            };
            unsafe {
                let results = (*repl).run_file(str_loaded);
                for result in results {
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(format!(
                                "builtin function `load_file(load)`:解析错误:\n\t{}",
                                e
                            ))
                        }
                    }
                }
            }
        } else {
            return Err("builtin function `load_file(load)`:参数错误".to_string());
        }
    } else {
        return Err("builtin function `load_file(load)`:参数错误".to_string());
    }
    Ok(RLVal::Sexpr(VecDeque::new()))
}
pub fn eval_string(mut xs: VecDeque<RLVal>, _env: *mut RLenv, repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::RLStr(s) = xs.pop_front().unwrap() {
            unsafe {
                let result = (*repl).run_file(s);
                //打印运行结果
                let mut cnt_line = 0;
                for res in result {
                    match res {
                        Ok(_) => {}
                        Err(s) => {
                            println!("builtin function `eval_string`:在计算第{}个表达式时发生错误:{}", cnt_line, s);
                        }
                    }
                    cnt_line += 1;
                }
            }
        } else {
            return Err("builtin function `eval_string`:参数错误:参数1应为Str".to_string());
        }
    } else {
        return Err("builtin function `eval_string`:参数数量错误".to_string());
    }
    Ok(RLVal::Sexpr(VecDeque::new()))
}
pub fn _gc(_xs: VecDeque<RLVal>, _env: *mut RLenv, repl: *mut ReplEnv) -> RLResult {
    unsafe { (*repl).run_garbage_collector() }
}
///定义局部变量
pub fn put(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let name;
        let val;
        if let RLVal::Qexpr(mut x) = xs.pop_front().unwrap() {
            if let RLVal::Symbol(s) = x.pop_front().unwrap() {
                name = s;
            } else {
                return Err("builtin function `put`:参数错误".to_string());
            }
        } else {
            return Err("builtin function `put`:参数错误".to_string());
        }
        val = xs.pop_front().unwrap();
        //        println!("Debug:env[{}] = {:?}",name,val);
        unsafe {
            (*_env).env.insert(name, val);
        }
    } else {
        return Err("builtin function `put`:参数错误".to_string());
    }
    Ok(RLVal::Sexpr(VecDeque::new()))
}
///Qexpr的最后一个元素
pub fn last(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::Qexpr(mut x) = xs.pop_front().unwrap() {
            Ok(x.pop_back().unwrap_or(RLVal::Sexpr(VecDeque::new())))
        } else {
            return Err("builtin function `last`:参数错误".to_string());
        }
    } else {
        return Err("builtin function `last`:参数错误".to_string());
    }
}
///Qexpr除最后一个元素之前元素
pub fn init(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::Qexpr(mut x) = xs.pop_front().unwrap() {
            x.pop_back();
            Ok(RLVal::Qexpr(x))
        } else {
            return Err("builtin function `last`:参数错误".to_string());
        }
    } else {
        return Err("builtin function `last`:参数错误".to_string());
    }
}
