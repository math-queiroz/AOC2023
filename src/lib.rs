extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn day(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);
    function.sig.ident = Ident::new("solution", function.sig.ident.span());
    let args = &parse_macro_input!(attr as AttributeArgs)[..];
    let (day, name) = match args {
        [NestedMeta::Lit(Lit::Int(d)), NestedMeta::Lit(Lit::Str(n))] => (d, n),
        _ => panic!("Invalid filename on aoc::day macro"),
    };
    TokenStream::from(quote! {
        #function
        fn main() {
            let (input, line_ending) = get_input();
            let t = ::std::time::Instant::now();
            let solution = solution(input, line_ending);
            let elapsed = t.elapsed();
            print!("Day{}: {}\n{:?}\nSolution took {:.2?}", #day, #name, solution, elapsed);
        }
        fn get_input<'a>() -> (String, &'a str) {
            let path_name = format!("input/{:0>2}.txt", #day);
            let raw_input = std::fs::read_to_string(path_name).expect("Wrong file name!");
            let input = raw_input.trim_end().to_string();
            let Some(lf_pos) = input.find('\n') else { return (input, "\n") };
            let Some(lf_pre) = input.as_bytes().get(lf_pos-1) else { return (input, "\n") };
            if *lf_pre == b'\r' { (input, "\r\n")}
            else { (input, "\n")}
        }
    })
}

#[proc_macro_attribute]
pub fn asserts(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);
    let args = &parse_macro_input!(attr as AttributeArgs)[..];
    let (v1, v2) = match args {
        [NestedMeta::Lit(Lit::Str(v1)), NestedMeta::Lit(Lit::Str(v2))] => (v1, v2),
        _ => panic!("Expected two strings on aoc::assert macro"),
    };
    TokenStream::from(quote! {
        #function
        #[cfg(test)]
        mod test {
            use super::*;
            #[test]
            fn solutions() {
                let (input, line_ending) = get_input();
                let solution = solution(input, line_ending);
                assert_eq!(#v1, solution.0.to_string(), "Solution to part 1");
                assert_eq!(#v2, solution.1.to_string(), "Solution to part 2");
            }
        }
    })
}
