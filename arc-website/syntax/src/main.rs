use clap::{App, Arg, ArgMatches, SubCommand};
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use std::io;
use std::process;

pub fn make_app() -> App<'static, 'static> {
    App::new("syntax-preprocessor")
        .about("A mdbook preprocessor which adds syntax highlighting for arc-lang")
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let preprocessor = Syntax::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(&renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

pub struct Syntax;

impl Syntax {
    pub fn new() -> Syntax {
        Syntax
    }
}

impl Preprocessor for Syntax {
    fn name(&self) -> &str {
        "syntax-preprocessor"
    }

    fn run(&self, _: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let grammar_regex = regex::Regex::new(r"(?s)```arc-lang\n(.*?)```").unwrap();

        let comment_regex = regex::Regex::new(r"#[^{].*").unwrap();
        let comment_subst = r#"<i style="color:gray">${0}</i>"#;

        book.for_each_mut(|item| {
            if let mdbook::BookItem::Chapter(ch) = item {
                ch.content = grammar_regex
                    .replace_all(&ch.content, |caps: &regex::Captures<'_>| {
                        let s = caps.get(1).unwrap().as_str();
                        let s = comment_regex.replace_all(&s, comment_subst);
                        format!("<pre><code>{}</code></pre>", s)
                    })
                    .into_owned();
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
