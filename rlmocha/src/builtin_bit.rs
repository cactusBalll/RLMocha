use crate::{RLResult, RLVal, RLenv, ReplEnv};
use std::collections::VecDeque;
pub fn not(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            if x < 0.{
                return Err(
                    "builtin function bit extension `not`:参数错误:参数1应大于0".to_string(),
                );
            }
            let xi = x as u32;
            let yi = !xi;
            return Ok(RLVal::Number(yi as f64));
        } else {
            return Err(
                "builtin function bit extension `not`:参数错误:参数1应为Number".to_string(),
            );
        }
    } else {
        return Err("builtin function bit extension `not`:参数数量错误".to_string());
    }
}
pub fn and(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let (xi1, xi2);
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            if x < 0.{
                return Err(
                    "builtin function bit extension `and`:参数错误:参数1应大于0".to_string(),
                );
            }
            xi1 = x as u32;
        } else {
            return Err(
                "builtin function bit extension `and`:参数错误:参数1应为Number".to_string(),
            );
        }
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            if x < 0.{
                return Err(
                    "builtin function bit extension `and`:参数错误:参数2应大于0".to_string(),
                );
            }
            xi2 = x as u32;
        } else {
            return Err(
                "builtin function bit extension `and`:参数错误:参数1应为Number".to_string(),
            );
        }
        let yi = xi1 & xi2;
        return Ok(RLVal::Number(yi as f64));
    } else {
        return Err("builtin function bit extension `and`:参数数量错误".to_string());
    }
}
pub fn or(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let (xi1, xi2);
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            if x < 0.{
                return Err(
                    "builtin function bit extension `or`:参数错误:参数1应大于0".to_string(),
                );
            }
            xi1 = x as u32;
        } else {
            return Err(
                "builtin function bit extension `or`:参数错误:参数1应为Number".to_string(),
            );
        }
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            if x < 0.{
                return Err(
                    "builtin function bit extension `or`:参数错误:参数2应大于0".to_string(),
                );
            }
            xi2 = x as u32;
        } else {
            return Err(
                "builtin function bit extension `or`:参数错误:参数1应为Number".to_string(),
            );
        }
        let yi = xi1 | xi2;
        return Ok(RLVal::Number(yi as f64));
    } else {
        return Err("builtin function bit extension `or`:参数数量错误".to_string());
    }
}
pub fn xor(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let (xi1, xi2);
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            if x < 0.{
                return Err(
                    "builtin function bit extension `xor`:参数错误:参数1应大于0".to_string(),
                );
            }
            xi1 = x as u32;
        } else {
            return Err(
                "builtin function bit extension `xor`:参数错误:参数1应为Number".to_string(),
            );
        }
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            if x < 0.{
                return Err(
                    "builtin function bit extension `xor`:参数错误:参数2应大于0".to_string(),
                );
            }
            xi2 = x as u32;
        } else {
            return Err(
                "builtin function bit extension `xor`:参数错误:参数1应为Number".to_string(),
            );
        }
        let yi = xi1 ^ xi2;
        return Ok(RLVal::Number(yi as f64));
    } else {
        return Err("builtin function bit extension `xor`:参数数量错误".to_string());
    }
}
pub fn lsh(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let (xi1, xi2);
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            if x < 0.{
                return Err(
                    "builtin function bit extension `lsh`:参数错误:参数1应大于0".to_string(),
                );
            }
            xi1 = x as u32;
        } else {
            return Err(
                "builtin function bit extension `lsh`:参数错误:参数1应为Number".to_string(),
            );
        }
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            if x < 0.{
                return Err(
                    "builtin function bit extension `lsh`:参数错误:参数2应大于0".to_string(),
                );
            }
            xi2 = x as u32;
        } else {
            return Err(
                "builtin function bit extension `lsh`:参数错误:参数1应为Number".to_string(),
            );
        }
        let yi = xi1 << xi2;
        return Ok(RLVal::Number(yi as f64));
    } else {
        return Err("builtin function bit extension `lsh`:参数数量错误".to_string());
    }
}
pub fn rsh(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let (xi1, xi2);
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            if x < 0.{
                return Err(
                    "builtin function bit extension `rsh`:参数错误:参数1应大于0".to_string(),
                );
            }
            xi1 = x as u32;
        } else {
            return Err(
                "builtin function bit extension `rsh`:参数错误:参数1应为Number".to_string(),
            );
        }
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            if x < 0.{
                return Err(
                    "builtin function bit extension `rsh`:参数错误:参数2应大于0".to_string(),
                );
            }
            xi2 = x as u32;
        } else {
            return Err(
                "builtin function bit extension `rsh`:参数错误:参数1应为Number".to_string(),
            );
        }
        let yi = xi1 >> xi2;
        return Ok(RLVal::Number(yi as f64));
    } else {
        return Err("builtin function bit extension `rsh`:参数数量错误".to_string());
    }
}
