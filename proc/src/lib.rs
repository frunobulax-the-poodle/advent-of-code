use proc_macro::TokenStream;
use syn::*;

#[proc_macro_attribute]
pub fn main(_attrs: TokenStream, items: TokenStream) -> TokenStream {
    let fun: ItemFn = parse2(items.into()).unwrap();
    let stmts = &fun.block.stmts;
    let new: proc_macro2::TokenStream = parse_quote! {
        fn main() -> Result<(), Box<dyn std::error::Error>> {
            let v: Vec<_> = file!()
                .rsplit('/')
                .next()
                .unwrap()
                .split('.')
                .next()
                .unwrap()
                .split('-')
                .collect();
            let (year, day) = (v[0], v[1]);
            let p = format!("input/{}/{:02}", year, day);
            let path = std::path::Path::new(&p);
            std::fs::create_dir_all(path.parent().expect("parnets"))?;
            if !path.exists() {
                // Fetch input if it does not exist
                std::process::Command::new("./fetch.sh")
                    .args([year.to_string(), day.to_string()])
                    .output()?;
            }
            let mut input = std::fs::read_to_string(path)?;
            // Remove trailing newline
            input.truncate(input.len() - 1);

            let now = std::time::Instant::now();
            let (p1, p2) = { #(#stmts)* };
            let time = now.elapsed().as_millis();

            println!("++++++++++ DAY {} ++++++++++", day);
            println!("Part 1\n{}\n", p1);
            println!("Part 2\n{}\n", p2);
            println!("Time\n{}ms\n", time);
            Ok(())
        }
    };
    new.into()
}
