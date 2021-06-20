use crate::{RLResult, RLVal, RLenv, ReplEnv};
use std::collections::VecDeque;
use std::{
    fs,
    io::{self, Write},
};
pub fn read_to_string(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::RLStr(s) = xs.pop_front().unwrap() {
            let str_loaded = match fs::read_to_string(s) {
                Ok(s_read) => s_read,
                Err(_e) => "".to_string(),
            };
            return Ok(RLVal::RLStr(str_loaded));
        } else {
            return Err(
                "builtin function file extension `read_to_string`:参数错误:参数1应为Str"
                    .to_string(),
            );
        }
    } else {
        return Err("builtin function file extension `read_to_string`:参数数量错误".to_string());
    }
}
pub fn write_to_file(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 2 {
        let (target, content);
        if let RLVal::RLStr(s) = xs.pop_front().unwrap() {
            target = s;
        } else {
            return Err(
                "builtin function file extension `write_to_file`:参数错误:参数1应为Str".to_string(),
            );
        }
        if let RLVal::RLStr(s) = xs.pop_front().unwrap() {
            content = s;
        } else {
            return Err(
                "builtin function file extension `write_to_file`:参数错误:参数2应为Str".to_string(),
            );
        }
        match fs::write(target, content) {
            Ok(_) => {
                return Ok(RLVal::Sexpr(VecDeque::new()));
            }
            Err(_e) => {
                return Err(
                    "builtin function file extension `write_to_file`:IO错误:写入文件失败"
                        .to_string(),
                );
            }
        }
    } else {
        return Err("builtin function file extension `write_to_file`:参数数量错误".to_string());
    }
}
pub fn input(mut xs: VecDeque<RLVal>, _env: *mut RLenv, _repl: *mut ReplEnv) -> RLResult {
    if xs.len() == 1 {
        if let RLVal::RLStr(s) = xs.pop_front().unwrap() {
            println!("input>> {}", s);
            print!("<<");
            io::stdout()
                .flush()
                .expect("builtin function file extension `input`:error:控制台错误");
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => {
                    return Ok(RLVal::RLStr(buffer.trim().to_string()));
                }
                Err(e) => {
                    return Err(format!(
                        "builtin function file extension `input`:从控制台读取出错:{}",
                        e
                    ));
                }
            }
        } else {
            return Err(
                "builtin function file extension `input`:参数错误:参数1应为Str".to_string(),
            );
        }
    } else {
        return Err("builtin function file extension `input`:参数数量错误".to_string());
    }
}
