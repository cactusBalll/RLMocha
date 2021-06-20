# RLMocha

#### 介绍
用rust近乎复制了[这里](http://buildyourownlisp.com/)的lispy。
参照的中文版[这里](https://www.abnerchou.me/BuildYourOwnLispCn/)，RLMocha：R for rust，L for Lisp，Mocha for Mocha。
#### 安装
编译源码需要安装[rust](https://www.rust-lang.org/zh-CN/),可能需要MSVC，也就是大概需要装VisualStudio。
在根目录执行```cargo run```编译并运行，进入交互式环境。
```rust```和其包管理```Cargo```在官网有详细教程。也可以执行```rustup doc``` 打开本地文档。（rust yyds）

也可以下载build子目录下的可执行文件。（毕竟VS太大了）

#### 使用说明
```
RLMocha [filename] 运行文件
RLMocha 进入交互式环境
exit 退出交互式环境
```
语法和`lispy`几乎相同(根本就是一模一样的),可以参照[这里](https://www.abnerchou.me/BuildYourOwnLispCn/)
有一些扩充,如：
```
(load "std.mc")
```
在当前环境加载文件
#### 示例
RLMocha是函数式语言，动态强类型（大概算是，毕竟没有隐式类型转换）。有GC（性能极差，单次扫描标记清理，不可暂停）。
##### Hello World
```lisp
(print "Hello World")
```
##### 概述
类型：
```rust
pub enum RLVal {
    Symbol(String),//符号 aaa
    Number(f64),//数 3.14
    RLStr(String),//字符串 "aaa" 
    RLFunc(RLFuncStru),//闭包 (\{}0)
    BuiltinFunc(fn(VecDeque<RLVal>, *mut RLenv, *mut ReplEnv) -> RLResult),//内置函数 def,\,+,*,str.slice...
    Comment(String),//注释 ;...
    Sexpr(VecDeque<RLVal>),//S-表达式 (a b c)
    Qexpr(VecDeque<RLVal>),//Q-表达式 {a b c}
}
```
概括的解释
```lisp
;注释
(load "std.mc");加载std.mc文件，加载后才能使用其中函数。
(+ 2 3);=5,S-表达式的会被直接计算，即其语义是计算结果（前缀表达式，必须加括号）
{+ 2 3};={+ 2 3},Q-表达式的语义是自己，即阻止计算
(\ {a b} {+ a b});一个lambda表达式，本质上'\'是内置函数，接收两个Q-expr，第一个是参数列表，第二个是函数体
((\ {a b} {+ a b}) 2 3);=5,带入计算
```
##### 闭包(函数)
```lisp
def {add-one} (\ {a} {+ a 1})
(add-one)
;>><lambda>
(add-one 4)
;>>5
(def {fun} (\ {f b} {
  def (head f) (\ (tail f) b)
}))
(fun {add-one a}{
    + a 1
})
```
##### 内置函数
```./rlmocha/src```中所有带builtin的文件是内置函数的实现。
```
RLmocha REPL env (built at 2021-06-20 16:31:25.776832700 +08:00,version 0.1.0):
RLmocha>str.slice "abcdefg" 2 3
"c"
RLmocha>str.slice "abcdef" 0 3
"abc"
RLmocha>str.strlen "abcdef"
错误:语义:`get_var`:变量未定义:尝试获取`str.strlen`
RLmocha>str.len "abcdef"
6
RLmocha>str.concat "abc" def" "efg"
错误:词法错误:标识符不合法,在第20个字符处
RLmocha>str.concat "abc" "def" "efg"
错误:语义:`get_var`:变量未定义:尝试获取`str.concat`
RLmocha>str.+ "abc" "def" "efg"
"abcdefefg"
RLmocha>str.split "abc efg efs" " "
{"abc" "efg" "efs" }
RLmocha>str.to_ascii "abcd34"
{97 98 99 100 51 52 }
RLmocha>str.from_ascii {97 98 99 100 51 52 }
"abcd34"
RLmocha>str.format 345
"345"
RLmocha>str.format { {3 {3 4}}}
"{{3 {3 4 } } }"
RLmocha>str.parse "34.5"
34.5
RLmocha>str.parse "dwddawd"
0
RLmocha>file.read "std.mc"
""
RLmocha>file.read "../std.mc"
";RLMocha 基础库
; 原子
(def {true} 1)
(def {false} 0)
(def {nil} {}).....
RLmocha>file.write "23333.txt" "23333"
()
RLmocha>file.input "what?"
input>> what?
<<233
"233
"
RLmocha>bit.and 1 1
1
RLmocha>bit.not 1
4294967294
RLmocha>bit.xor 1 0
1
RLmocha>bit.or 0 0
0
RLmocha>math.ln 2.7
0.9932517730102834
RLmocha>math.pow 2 10
1024
```
具体使用参见源码，其余map，foldl，head，tail等函数和lispy完全相同，可以去上述网站查看。
#### 总结
大概是用于rust入门练手的项目。。。
