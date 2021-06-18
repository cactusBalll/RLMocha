use crate::RLToken;
#[cfg(test)]
mod lexical_test {
    #[test]
    fn simple_expr0() {
        let test_str = "def {add-mul} (\\ {x y} {+ x (* x y)})".to_string();
        let token_stream = super::lexical_analyze(test_str);
        println!("{:?}", token_stream);
        assert_eq!(0, 0);
    }
    #[test]
    fn simple_expr1() {
        let test_str = "def {add-mul} (\\ {x y ;23333\n} {+ x (* x y)});23333\n".to_string();
        let token_stream = super::lexical_analyze(test_str);
        println!("{:?}", token_stream);
        assert_eq!(0, 0);
    }
    #[test]
    fn number_expr0() {
        let test_str = "(+ (+ 2 3) (+ 3 4))".to_string();
        let token_stream = super::lexical_analyze(test_str);
        println!("{:?}", token_stream);
        assert_eq!(0, 0);
    }
    #[test]
    fn lambda_expr0() {
        let test_str = "( \\ {x y} {+ x (* x y)}) 2 3 ".to_string();
        let token_stream = super::lexical_analyze(test_str);
        println!("{:?}", token_stream);
        assert_eq!(0, 0);
    }
    #[test]
    fn whole0() {
        let test_str = r"
        ; Function Definitions
        (def {fun} (\ {f b} {
          def (head f) (\ (tail f) b)
        }))
        (fun {add-mul x y} {
            (+ x (* x x y))
        })
        (print fun)
        (print (add-mul 2 3))"
            .to_string();
        let token_stream = super::lexical_analyze(test_str);
        println!("{:?}", token_stream);
        assert_eq!(0, 0);
    }
}
fn state0(
    ret: &mut Vec<RLToken>,
    nxt_char: char,
    state: &mut i32,
    _ptr: &mut usize,
    tstr: &mut String,
) -> Result<(), String> {
    match nxt_char {
        //字符串
        '\"' => {
            *state = 3;
            tstr.clear();
        }
        //数字
        '0'..='9' | '-' => {
            tstr.clear();
            tstr.push(nxt_char);
            *state = 5;
        }
        //括号..
        '(' => {
            ret.push(RLToken::BracketSL);
        }
        ')' => {
            ret.push(RLToken::BracketSR);
        }
        '{' => {
            ret.push(RLToken::BracketML);
        }
        '}' => {
            ret.push(RLToken::BracketMR);
        }
        //注释
        ';' => {
            *state = 8;
        }
        //空字符
        ' ' | '\r' | '\n' | '\t' => {}
        //标识符
        _ => {
            *state = 1;
            tstr.clear();
            tstr.push(nxt_char);
        }
    }
    Ok(())
}
fn state1(
    ret: &mut Vec<RLToken>,
    nxt_char: char,
    state: &mut i32,
    ptr: &mut usize,
    tstr: &mut String,
) -> Result<(), String> {
    match nxt_char {
        ' ' | '\r' | '\n' | '\t' => {
            ret.push(RLToken::Symbol(tstr.clone()));
            tstr.clear();
            *state = 0;
        }
        ')' | '}' | ';' => {
            ret.push(RLToken::Symbol(tstr.clone()));
            tstr.clear();
            *state = 0;
            //防止把下一个标签给吃掉
            *ptr -= 1;
        }
        //非法
        '\"' | '(' | '{' => {
            return Err(format!("词法错误:标识符不合法,在第{}个字符处", ptr));
        }
        _ => {
            tstr.push(nxt_char);
            *state = 2;
        }
    }
    Ok(())
}
fn state2(
    ret: &mut Vec<RLToken>,
    nxt_char: char,
    state: &mut i32,
    ptr: &mut usize,
    tstr: &mut String,
) -> Result<(), String> {
    match nxt_char {
        ' ' | '\r' | '\n' | '\t' => {
            ret.push(RLToken::Symbol(tstr.clone()));
            tstr.clear();
            *state = 0;
        }
        ')' | '}' | ';' => {
            ret.push(RLToken::Symbol(tstr.clone()));
            tstr.clear();
            *state = 0;
            //防止把下一个标签给吃掉
            *ptr -= 1;
        }
        //非法
        '\"' | '(' | '{' => {
            return Err(format!("词法错误:标识符不合法,在第{}个字符处", ptr));
        }
        _ => {
            tstr.push(nxt_char);
        }
    }
    Ok(())
}
fn state3(
    ret: &mut Vec<RLToken>,
    nxt_char: char,
    state: &mut i32,
    _ptr: &mut usize,
    tstr: &mut String,
) -> Result<(), String> {
    match nxt_char {
        '\"' => {
            ret.push(RLToken::Str(tstr.clone()));
            tstr.clear();
            *state = 0;
        }
        _ => {
            tstr.push(nxt_char);
        }
    }
    Ok(())
}
fn state5(
    ret: &mut Vec<RLToken>,
    nxt_char: char,
    state: &mut i32,
    ptr: &mut usize,
    tstr: &mut String,
) -> Result<(), String> {
    match nxt_char {
        '0'..='9' => {
            tstr.push(nxt_char);
        }
        '.' => {
            tstr.push(nxt_char);
            *state = 6;
        }
        _ => {
            //println!("tstr:{}",tstr);
            if tstr == "-" {
                ret.push(RLToken::Symbol("-".to_string()));
                tstr.clear();
                *state = 0;
                *ptr -= 1;
                return Ok(());
            }
            let num: f64 = match tstr.parse() {
                Ok(x) => x,
                Err(_) => return Err(format!("词法错误,错误的数字,无法转换,在第{}个字符处", ptr)),
            };
            tstr.clear();
            *state = 0;
            *ptr -= 1;
            ret.push(RLToken::Number(num));
        }
    }
    Ok(())
}
fn state6(
    ret: &mut Vec<RLToken>,
    nxt_char: char,
    state: &mut i32,
    ptr: &mut usize,
    tstr: &mut String,
) -> Result<(), String> {
    match nxt_char {
        '0'..='9' => {
            tstr.push(nxt_char);
        }

        _ => {
            //println!("tstr:{}",tstr);
            let num: f64 = match tstr.parse() {
                Ok(x) => x,
                Err(_) => return Err(format!("词法错误,错误的数字,无法转换,在第{}个字符处", ptr)),
            };
            tstr.clear();
            *state = 0;
            *ptr -= 1;
            ret.push(RLToken::Number(num));
        }
    }
    Ok(())
}
fn state8(
    ret: &mut Vec<RLToken>,
    nxt_char: char,
    state: &mut i32,
    _ptr: &mut usize,
    tstr: &mut String,
) -> Result<(), String> {
    match nxt_char {
        '\r' | '\n' => {
            ret.push(RLToken::Comment(tstr.clone()));
            tstr.clear();
            *state = 0;
        }
        _ => {
            tstr.push(nxt_char);
        }
    }
    Ok(())
}
///词法分析
pub fn lexical_analyze(str_to_ana: String) -> Result<Vec<RLToken>, String> {
    let mut ret = Vec::new();
    //转化为char数组
    let mut characters = Vec::new();
    for c in str_to_ana.chars() {
        characters.push(c);
    }
    //有限自动机
    let mut ptr = 0;
    let mut state = 0;
    let mut tstr = String::new();
    loop {
        if ptr >= characters.len() {
            break;
        }
        let nxt_char = characters[ptr];
        match state {
            0 => {
                state0(&mut ret, nxt_char, &mut state, &mut ptr, &mut tstr)?;
            }
            1 => {
                state1(&mut ret, nxt_char, &mut state, &mut ptr, &mut tstr)?;
            }
            2 => {
                state2(&mut ret, nxt_char, &mut state, &mut ptr, &mut tstr)?;
            }
            3 => {
                state3(&mut ret, nxt_char, &mut state, &mut ptr, &mut tstr)?;
            }
            5 => {
                state5(&mut ret, nxt_char, &mut state, &mut ptr, &mut tstr)?;
            }
            6 => {
                state6(&mut ret, nxt_char, &mut state, &mut ptr, &mut tstr)?;
            }
            8 => {
                state8(&mut ret, nxt_char, &mut state, &mut ptr, &mut tstr)?;
            }
            _ => {}
        }
        ptr += 1;
    }
    Ok(ret)
}
