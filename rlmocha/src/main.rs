extern crate rlmocha_proc_macro;
use std::collections::{HashMap, LinkedList, VecDeque};
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
#[cfg(test)]
mod tst;
const GCSIZE: usize = 5000;
type RLResult = Result<RLVal, String>;
#[derive(Clone)]
pub enum RLVal {
    Symbol(String),
    Number(f64),
    RLStr(String),
    RLFunc(RLFuncStru),
    BuiltinFunc(fn(VecDeque<RLVal>, *mut RLenv, *mut ReplEnv) -> RLResult),
    Comment(String),
    Sexpr(VecDeque<RLVal>),
    Qexpr(VecDeque<RLVal>),
}
impl PartialEq for RLVal {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RLVal::Symbol(s) => {
                if let RLVal::Symbol(s2) = other {
                    s == s2
                } else {
                    false
                }
            }
            RLVal::Number(x) => {
                if let RLVal::Number(x2) = other {
                    f64::abs(x - x2) < 1e-6
                } else {
                    false
                }
            }
            RLVal::RLStr(s) => {
                if let RLVal::RLStr(s2) = other {
                    s == s2
                } else {
                    false
                }
            }
            RLVal::RLFunc(f) => {
                if let RLVal::RLFunc(f2) = other {
                    f == f2
                } else {
                    false
                }
            }
            RLVal::BuiltinFunc(f) => {
                if let RLVal::BuiltinFunc(f2) = other {
                    f == f2
                } else {
                    false
                }
            }
            RLVal::Comment(s) => {
                if let RLVal::Comment(s2) = other {
                    s == s2
                } else {
                    false
                }
            }
            RLVal::Sexpr(v) => {
                if let RLVal::Sexpr(v2) = other {
                    v == v2
                } else {
                    false
                }
            }
            RLVal::Qexpr(v) => {
                if let RLVal::Qexpr(v2) = other {
                    v == v2
                } else {
                    false
                }
            }
        }
    }
}
impl fmt::Debug for RLVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RLVal::Symbol(s) => {
                write!(f, "{}", s)
            }
            RLVal::Number(x) => {
                write!(f, "{}", x)
            }
            RLVal::RLStr(s) => {
                write!(f, "\"{}\"", s)
            }
            RLVal::RLFunc(_func) => {
                write!(f, "<lambda>")
            }
            RLVal::BuiltinFunc(_func) => {
                write!(f, "<builtin function>")
            }
            RLVal::Comment(s) => {
                write!(f, "<comment>(\"{}\")", s)
            }
            RLVal::Sexpr(v) => {
                write!(f, "Sexpr({:?})", v)
            }
            RLVal::Qexpr(v) => {
                write!(f, "Qexpr({:?})", v)
            }
        }
    }
}
impl fmt::Display for RLVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RLVal::Symbol(s) => {
                write!(f, "{}", s)
            }
            RLVal::Number(x) => {
                write!(f, "{}", x)
            }
            RLVal::RLStr(s) => {
                write!(f, "\"{}\"", s)
            }
            RLVal::RLFunc(_func) => {
                write!(f, "<lambda>")
            }
            RLVal::BuiltinFunc(_func) => {
                write!(f, "<builtin function>")
            }
            RLVal::Comment(s) => {
                write!(f, "<comment>(\"{}\")", s)
            }
            RLVal::Sexpr(v) => {
                write!(f, "(")?;
                for val in v.iter() {
                    write!(f, "{} ", val)?;
                }
                write!(f, ")")
            }
            RLVal::Qexpr(v) => {
                write!(f, "{{")?;
                for val in v.iter() {
                    write!(f, "{} ", val)?;
                }
                write!(f, "}}")
            }
        }
    }
}

///token枚举
#[derive(Debug)]
pub enum RLToken {
    Str(String),
    Symbol(String),
    Number(f64),
    Comment(String),
    BracketSL,
    BracketML,
    BracketSR,
    BracketMR,
}
pub struct RLenv {
    pub env: HashMap<String, RLVal>,
    pub outer_scope: *mut RLenv,
    pub captured: *mut RLenv,
    pub marked: bool,
}
impl RLenv {
    fn new() -> RLenv {
        RLenv {
            env: HashMap::new(),
            outer_scope: 0x0 as *mut RLenv,
            captured: 0x0 as *mut RLenv,
            marked: false,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct RLFuncStru {
    pub para: VecDeque<String>,
    pub body: Box<RLVal>,
    pub env: *mut RLenv,
    pub global: *mut RLenv,
    pub isvla: bool,
    pub vla_para: String,
}

mod builtin;
mod builtin_str;
mod evaluation;
mod parse_ast;
mod tokenize;
pub struct ReplEnv {
    global: *mut RLenv,
    gc_list: LinkedList<Box<RLenv>>,
}
impl ReplEnv {
    fn new() -> ReplEnv {
        let mut gc_list = LinkedList::new();
        let mut glob = Box::new(RLenv::new());
        let glob_ptr = glob.as_mut() as *mut RLenv;
        gc_list.push_back(glob);
        ReplEnv {
            global: glob_ptr,
            gc_list: gc_list,
        }
    }
    fn load_builtin_func(&mut self) {
        unsafe {
            (*self.global)
                .env
                .insert("+".to_string(), RLVal::BuiltinFunc(builtin::add));
            (*self.global)
                .env
                .insert("*".to_string(), RLVal::BuiltinFunc(builtin::mul));
            (*self.global)
                .env
                .insert("\\".to_string(), RLVal::BuiltinFunc(builtin::lambda));
            (*self.global)
                .env
                .insert("def".to_string(), RLVal::BuiltinFunc(builtin::def));
            (*self.global)
                .env
                .insert("head".to_string(), RLVal::BuiltinFunc(builtin::head));
            (*self.global)
                .env
                .insert("tail".to_string(), RLVal::BuiltinFunc(builtin::tail));
            (*self.global)
                .env
                .insert("list".to_string(), RLVal::BuiltinFunc(builtin::list));
            (*self.global)
                .env
                .insert("eval".to_string(), RLVal::BuiltinFunc(builtin::eval));
            (*self.global)
                .env
                .insert("join".to_string(), RLVal::BuiltinFunc(builtin::join));
            (*self.global)
                .env
                .insert("<".to_string(), RLVal::BuiltinFunc(builtin::lt));
            (*self.global)
                .env
                .insert(">".to_string(), RLVal::BuiltinFunc(builtin::gt));
            (*self.global)
                .env
                .insert("==".to_string(), RLVal::BuiltinFunc(builtin::eq));
            (*self.global)
                .env
                .insert("if".to_string(), RLVal::BuiltinFunc(builtin::condition));
            (*self.global)
                .env
                .insert("print".to_string(), RLVal::BuiltinFunc(builtin::rl_print));
            (*self.global)
                .env
                .insert("error".to_string(), RLVal::BuiltinFunc(builtin::rl_error));
            (*self.global)
                .env
                .insert("load".to_string(), RLVal::BuiltinFunc(builtin::load_file));
            (*self.global)
                .env
                .insert("gc".to_string(), RLVal::BuiltinFunc(builtin::_gc));
            (*self.global)
                .env
                .insert("=".to_string(), RLVal::BuiltinFunc(builtin::put));
            (*self.global)
                .env
                .insert("last".to_string(), RLVal::BuiltinFunc(builtin::last));
            (*self.global)
                .env
                .insert("init".to_string(), RLVal::BuiltinFunc(builtin::init));
            (*self.global)
                .env
                .insert("`-`".to_string(), RLVal::BuiltinFunc(builtin::minus));
            (*self.global)
                .env
                .insert("/".to_string(), RLVal::BuiltinFunc(builtin::div));
            (*self.global).env.insert(
                "str.slice".to_string(),
                RLVal::BuiltinFunc(builtin_str::slice),
            );
            (*self.global).env.insert(
                "str.len".to_string(),
                RLVal::BuiltinFunc(builtin_str::strlen),
            );
            (*self.global)
                .env
                .insert("str.+".to_string(), RLVal::BuiltinFunc(builtin_str::concat));
            (*self.global).env.insert(
                "str.split".to_string(),
                RLVal::BuiltinFunc(builtin_str::split),
            );
            (*self.global).env.insert(
                "str.to_ascii".to_string(),
                RLVal::BuiltinFunc(builtin_str::to_ascii),
            );
            (*self.global).env.insert(
                "str.from_ascii".to_string(),
                RLVal::BuiltinFunc(builtin_str::from_ascii),
            );
            (*self.global).env.insert(
                "str.format".to_string(),
                RLVal::BuiltinFunc(builtin_str::format),
            );
            (*self.global).env.insert(
                "str.parse".to_string(),
                RLVal::BuiltinFunc(builtin_str::parse_str_to_number),
            );
        }
    }
    fn run_line(&mut self, input: String) -> RLResult {
        let tokens = tokenize::lexical_analyze(input)?;
        let ast = parse_ast::parse_one_line(tokens)?;
        let result = evaluation::eval_expr(ast, self.global, self as *mut ReplEnv);
        if self.gc_list.len() > GCSIZE {
            unsafe {
                self.run_garbage_collector()?;
            }
        }
        return result;
    }
    fn run_file(&mut self, input: String) -> Vec<RLResult> {
        let tokens = match tokenize::lexical_analyze(input) {
            Ok(x) => x,
            Err(e) => {
                let mut ret = Vec::new();
                ret.push(Err(e));
                return ret;
            }
        };
        let ast = parse_ast::parse(tokens);
        let mut result = Vec::new();
        let mut cnt = 0;
        let mut last_gc = 0;
        for expr in ast {
            match expr {
                Ok(expr_ok) => {
                    result.push(evaluation::eval_expr(
                        expr_ok,
                        self.global,
                        self as *mut ReplEnv,
                    ));
                }
                Err(s) => {
                    result.push(Err(s));
                }
            }
            if self.gc_list.len() > GCSIZE && cnt - last_gc > 10 {
                unsafe {
                    if let Err(s) = self.run_garbage_collector() {
                        result.push(Err(format!("垃圾回收器:{}", s)));
                    }
                }
                last_gc = cnt;
            }
            cnt += 1;
            //unsafe {println!("Debug:{:?}",(*self.global).env);}
        }
        result
    }
    fn new_env(&mut self) -> *mut RLenv {
        let mut nenv = Box::new(RLenv::new());
        let ret = nenv.as_mut() as *mut RLenv;
        self.gc_list.push_back(nenv);
        return ret;
    }
    unsafe fn gc_mark(&mut self, env: *mut RLenv) {
        //栈模拟，防止递归栈溢出
        let mut to_mark = Vec::new();
        to_mark.push(env);
        while to_mark.len() != 0 {
            let env = to_mark.pop().unwrap();
            for variable in (*env).env.values() {
                match variable {
                    RLVal::RLFunc(f) => {
                        (*f.env).marked = true;
                        //防止多次访问跑不出来 (*(*f.env).captured).marked == false
                        if (*f.env).captured != 0x0 as *mut RLenv
                            && (*(*f.env).captured).marked == false
                        {
                            to_mark.push((*f.env).captured);
                        }
                        let mut p = (*f.env).outer_scope;
                        while p != 0x0 as *mut RLenv {
                            (*p).marked = true;
                            if (*p).captured != 0x0 as *mut RLenv
                                && (*(*f.env).captured).marked == false
                            {
                                to_mark.push((*p).captured);
                            }
                            p = (*p).outer_scope;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    unsafe fn run_garbage_collector(&mut self) -> RLResult {
        let collected_env;
        //标记
        (*self.global).marked = true;
        self.gc_mark(self.global);
        //清理
        let mut temp_container = Vec::new();
        let lenth_of_gc_list = self.gc_list.len();
        loop {
            match self.gc_list.pop_front() {
                Some(v) => {
                    if (*v).marked {
                        temp_container.push(v);
                    }
                }
                None => break,
            }
        }
        collected_env = lenth_of_gc_list - temp_container.len();
        loop {
            match temp_container.pop() {
                Some(v) => {
                    self.gc_list.push_back(v);
                }
                None => break,
            }
        }
        //清除标记
        for variable in self.gc_list.iter_mut() {
            (*variable).marked = false;
        }
        Ok(RLVal::Number(collected_env as f64))
    }
}
fn main() {
    let mut args = env::args();
    args.next();
    if let Some(x) = args.next() {
        let file_to_intetpret = fs::read_to_string(x).expect("main:无法打开文件");
        let mut environment = ReplEnv::new();
        environment.load_builtin_func();
        let result = environment.run_file(file_to_intetpret);
        //打印运行结果
        let mut cnt_line = 0;
        for res in result {
            match res {
                Ok(_) => {}
                Err(s) => {
                    println!("在计算第{}个表达式时发生错误:{}", cnt_line, s);
                }
            }
            cnt_line += 1;
        }
    } else {
        let time_str = rlmocha_proc_macro::compile_time!();
        println!("RLmocha REPL env (build at {}):",time_str);
        let mut environment = ReplEnv::new();
        environment.load_builtin_func();
        print!("RLmocha>");
        io::stdout().flush().expect("main:控制台错误");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            match stdin.read_line(&mut buffer) {
                Ok(_) => {
                    if buffer == "exit\n" || buffer == "exit\r\n" || buffer == "exit" {
                        break;
                    } else {
                        match environment.run_line(buffer) {
                            Ok(v) => {
                                println!("{}", v)
                            }
                            Err(s) => {
                                println!("错误:{}", s)
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("从控制台读取出错:{}", e);
                }
            }
            print!("RLmocha>");
            io::stdout().flush().expect("main:控制台错误");
        }
    }
}
