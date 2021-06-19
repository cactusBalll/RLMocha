extern crate proc_macro;
use proc_macro::TokenStream;
extern crate chrono;
use chrono::prelude::*;
#[cfg(test)]
mod test{
    #[test]
    fn t1(){
        println!("{}",super::Local::now());
    }
}
#[proc_macro]
pub fn compile_time(_item: TokenStream) -> TokenStream {
    let time_str = format!("\"{}\"",Local::now());
    time_str.as_str().parse().unwrap()
}
