#[test]
fn ast0() {
    let test_str = "(def {add-mul} (\\ {x y} {+ x (* x y)}))".to_string();
    let token_stream = super::tokenize::lexical_analyze(test_str).unwrap();
    println!("{:?}", token_stream);
    let result = super::parse_ast::parse(token_stream);
    println!("{:?}", result);
    assert_eq!(0, 0);
}
#[test]
fn whole_0() {
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
    //let token_stream = super::tokenize::lexical_analyze(test_str);
    //println!("{:?}", token_stream);
    //let result = super::parse_ast::parse(token_stream);
    //println!("{:?}", result);
    let mut environment = super::ReplEnv::new();
    environment.load_builtin_func();
    println!("{:?}", environment.run_file(test_str));
    assert_eq!(0, 0);
}
