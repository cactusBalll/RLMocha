use crate::{RLResult, RLVal, RLenv, ReplEnv};
use std::collections::VecDeque;
pub fn pow(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let (base,p);
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            base = x;
        } else {
            return Err(
                "builtin function math extension `pow`:参数错误:参数1应为Number".to_string(),
            );
        }
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            p = x;
        } else {
            return Err(
                "builtin function math extension `pow`:参数错误:参数2应为Number".to_string(),
            );
        }
        return Ok(RLVal::Number(f64::powf(base,p)))
    } else {
        return Err("builtin function math extension `pow`:参数数量错误".to_string());
    }
}
pub fn cos(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        let xx;
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            xx = x;
        } else {
            return Err(
                "builtin function math extension `cos`:参数错误:参数1应为Number".to_string(),
            );
        }
        return Ok(RLVal::Number(f64::cos(xx)))
    } else {
        return Err("builtin function math extension `cos`:参数数量错误".to_string());
    }
}
pub fn sin(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        let xx;
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            xx = x;
        } else {
            return Err(
                "builtin function math extension `sin`:参数错误:参数1应为Number".to_string(),
            );
        }
        return Ok(RLVal::Number(f64::sin(xx)))
    } else {
        return Err("builtin function math extension `sin`:参数数量错误".to_string());
    }
}
pub fn tan(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        let xx;
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            xx = x;
        } else {
            return Err(
                "builtin function math extension `tan`:参数错误:参数1应为Number".to_string(),
            );
        }
        return Ok(RLVal::Number(f64::tan(xx)))
    } else {
        return Err("builtin function math extension `tan`:参数数量错误".to_string());
    }
}
pub fn sqrt(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        let xx;
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            xx = x;
        } else {
            return Err(
                "builtin function math extension `sqrt`:参数错误:参数1应为Number".to_string(),
            );
        }
        return Ok(RLVal::Number(f64::sqrt(xx)))
    } else {
        return Err("builtin function math extension `sqrt`:参数数量错误".to_string());
    }
}
pub fn ln(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        let xx;
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            xx = x;
        } else {
            return Err(
                "builtin function math extension `ln`:参数错误:参数1应为Number".to_string(),
            );
        }
        return Ok(RLVal::Number(f64::ln(xx)))
    } else {
        return Err("builtin function math extension `ln`:参数数量错误".to_string());
    }
}