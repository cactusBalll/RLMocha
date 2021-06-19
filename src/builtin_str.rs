use crate::{RLFuncStru, RLResult, RLVal, RLenv, ReplEnv};
use std::collections::VecDeque;
pub fn slice(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 3 {
        let str_to_slice;
        let l;
        let r;
        if let RLVal::RLStr(s) = xs.pop_front().unwrap() {
            str_to_slice = s;
        } else {
            return Err("builtin function str extension `slice`:参数错误:参数1应为Str".to_string());
        }
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            l = x;
            if l < 0. {
                return Err(
                    "builtin function str extension `slice`:参数错误:参数2应大于0".to_string(),
                );
            }
        } else {
            return Err(
                "builtin function str extension `slice`:参数错误:参数2应为Number".to_string(),
            );
        }
        if let RLVal::Number(x) = xs.pop_front().unwrap() {
            r = x;
            if r < 0. {
                return Err(
                    "builtin function str extension `slice`:参数错误:参数3应大于0".to_string(),
                );
            }
        } else {
            return Err(
                "builtin function str extension `slice`:参数错误:参数3应为Number".to_string(),
            );
        }
        let (ll, rr);
        if l > str_to_slice.len() as f64 {
            ll = str_to_slice.len();
        } else {
            ll = l as usize;
        }
        if r > str_to_slice.len() as f64 {
            rr = str_to_slice.len();
        } else {
            rr = r as usize;
        }
        return Ok(RLVal::RLStr((&str_to_slice[ll..rr]).to_string()));
    } else {
        return Err("builtin function str extension `slice`:参数数量错误".to_string());
    }
}
pub fn strlen(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::RLStr(s) = xs.pop_front().unwrap() {
            return Ok(RLVal::Number(s.len() as f64));
        } else {
            return Err(
                "builtin function str extension `strlen`:参数1类型错误,应为Str".to_string(),
            );
        }
    } else {
        return Err("builtin function str extension `strlen`:参数数量错误".to_string());
    }
}
pub fn concat(xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    let mut ret = String::new();
    for x in xs {
        if let RLVal::RLStr(s) = x {
            ret.push_str(&s);
        } else {
            return Err(
                "builtin function str extension `concat`:参数类型错误，只能拼接字符串".to_string(),
            );
        }
    }
    Ok(RLVal::RLStr(ret))
}
pub fn split(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let (str_to_split, c);
        if let RLVal::RLStr(s) = xs.pop_front().unwrap() {
            str_to_split = s;
        } else {
            return Err(
                "builtin function str extension `split`:参数类型错误，只能分割字符串".to_string(),
            );
        }
        if let RLVal::RLStr(s) = xs.pop_front().unwrap() {
            c = s;
            if c.len() > 1 {
                return Err(
                    "builtin function str extension `split`:参数类型错误，只能以字符为界限分割"
                        .to_string(),
                );
            }
        } else {
            return Err(
                "builtin function str extension `split`:参数类型错误，只能以字符为界限分割"
                    .to_string(),
            );
        }
        let mut ret = VecDeque::new();
        ret.extend(str_to_split.split(&c).map(|x| RLVal::RLStr(x.to_string())));
        Ok(RLVal::Qexpr(ret))
    } else {
        return Err("builtin function str extension `split`:参数数量错误".to_string());
    }
}
pub fn to_ascii(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::RLStr(s) = xs.pop_front().unwrap() {
            let mut ret = VecDeque::new();
            for c in s.chars() {
                if c.is_ascii() {
                    ret.push_back(RLVal::Number(c as u8 as f64));
                } else {
                    ret.push_back(RLVal::Number(0.));
                }
            }
            return Ok(RLVal::Qexpr(ret));
        } else {
            return Err(
                "builtin function str extension `to_ascii`:参数1类型错误，只能转换Str".to_string(),
            );
        }
    } else {
        return Err("builtin function str extension `to_ascii`:参数数量错误".to_string());
    }
}
pub fn from_ascii(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::Qexpr(v) = xs.pop_front().unwrap(){
            let mut  ret = String::new();
            for x_w in v.iter(){
                if let RLVal::Number(x) = x_w{
                    let x = *x as i32;
                    //在ascii范围内
                    if 0 <= x && x < 128{
                        ret.push(x as u8 as char);
                    }
                }
            }
            Ok(RLVal::RLStr(ret))
        }else{
            return Err("builtin function str extension `from_ascii`:参数1类型错误".to_string());
        }
    }else {
        return Err("builtin function str extension `from_ascii`:参数数量错误".to_string());
    }
}
pub fn format(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult{
    if xs.len() == 1 {
        Ok(RLVal::RLStr(format!("{}",xs.pop_front().unwrap())))
    }else {
        return Err("builtin function str extension `format`:参数数量错误".to_string());
    }
}
pub fn parse_str_to_number(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv)-> RLResult{
    if xs.len() == 1 {
        if let RLVal::RLStr(str_to_parse) = xs.pop_front().unwrap(){
            let x:f64 = str_to_parse.parse().unwrap_or(0.);
            return Ok(RLVal::Number(x));
        }else{
            return Err("builtin function str extension `parse_str_to_number`:参数1类型错误，应为Str".to_string());
        }
    }else {
        return Err("builtin function str extension `parse_str_to_number`:参数数量错误".to_string());
    }
}
