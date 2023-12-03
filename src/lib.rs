extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn puzzle(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);
    function.sig.ident = Ident::new("solve", function.sig.ident.span());
    let args = &parse_macro_input!(attr as AttributeArgs)[..];
    let file_name = match args {
        [NestedMeta::Lit(Lit::Str(file_name))] => file_name,
        _ => panic!("Inject_input macro expected a number"),
    };
    TokenStream::from(quote! {
        fn get_input() -> (String, &'static str) {
            let path_name = format!("input/{}", #file_name);
            let raw_input = std::fs::read_to_string(path_name).expect("Wrong file name!");
            let input = raw_input.trim_end().to_string();
            let Some(lf_pos) = input.find('\n') else { return (input, "\n") };
            let Some(lf_pre) = input.as_bytes().get(lf_pos-1) else { return (input, "\n") };
            if *lf_pre == b'\r' { (input, "\r\n")}
            else { (input, "\n")}
        }
        #function
        fn main() {
            let t = ::std::time::Instant::now();
            let (input, line_ending) = get_input();
            let solution = solve(input, line_ending);
            let elapsed = t.elapsed();
            println!("{:?}\nSolution took {:.2?}", solution, elapsed);
        }
    })
}

#[proc_macro_attribute]
pub fn assert(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);
    let args = &parse_macro_input!(attr as AttributeArgs)[..];
    let (v1, v2) = match args {
        [NestedMeta::Lit(Lit::Str(v1)), NestedMeta::Lit(Lit::Str(v2))] => (v1, v2),
        _ => panic!("Expected expected numbers"),
    };
    TokenStream::from(quote! {
        #function
        #[cfg(test)]
        mod solutions {
            use super::*;
            #[test]
            fn part1() {
                let (input, line_ending) = get_input();
                let solution = solve(input, line_ending);
                assert_eq!(#v1, solution.0.to_string(), "Solution to part 1");
            }
            #[test]
            fn part2() {
                let (input, line_ending) = get_input();
                let solution = solve(input, line_ending);
                assert_eq!(#v2, solution.1.to_string(), "Solution to part 2");
            }
        }
    })
}
