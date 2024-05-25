use parse::tokenize::Tokenizer;

mod parse;

fn main() {
    let sample = std::fs::read_to_string("example/hello.rg").expect("failed to load example/hello.rg");
    let mut tokenizer = Tokenizer::new(sample.as_str());
    while let Some(res) = tokenizer.next() {
        match res {
            Ok((span, lexeme)) => println!("{:?} {:?}", span, lexeme),
            Err(e) => {
                let raw = tokenizer.get_span(&e.span);
                println!("{:?} {:?} {:?}", e.kind, e.span, raw);
                break;
            },
        }
    }
}
