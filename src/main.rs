use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::Config;
use regex::Regex;
use std::env;
use std::io;
use std::process;
use std::vec;

const NAME: &str = "abbr";

#[derive(Debug)]
struct Abbreviation {
    abbr: String,
    expanded: String,
}

struct Abbr {
    list: Vec<Abbreviation>,
}

impl Abbr {
    fn new(ctx: &PreprocessorContext) -> Self {
        Self {
            list: get_abbr_table(&ctx.config).unwrap_or_else(|| vec![]),
        }
    }
}

impl Preprocessor for Abbr {
    fn name(&self) -> &str {
        NAME
    }

    fn run(
        &self,
        ctx: &mdbook::preprocess::PreprocessorContext,
        mut book: mdbook::book::Book,
    ) -> mdbook::errors::Result<mdbook::book::Book> {
        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chap) = item {
                for abbr in &self.list {
                    // Find and replace all abbreviations defined in config
                    let re = Regex::new(format!("\\b({})\\b", abbr.abbr).as_str()).unwrap();
                    let replacement =
                        format!("<abbr title=\"{}\">{}</abbr>", abbr.expanded, abbr.abbr);

                    chap.content = re
                        .replace_all(chap.content.as_str(), replacement.as_str())
                        .into_owned();
                }
            };
        });

        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}

/// Get the abbreviation -> expansion config
fn get_abbr_table(config: &Config) -> Option<Vec<Abbreviation>> {
    let preprocessor_config = config.get("preprocessor")?;
    let abbr_config = preprocessor_config.get("abbr")?;
    let abbr_table = abbr_config.get("list")?;

    Some(
        abbr_table
            .as_table()
            .unwrap()
            .iter()
            .map(|(k, v)| Abbreviation {
                abbr: k.clone(),
                expanded: String::from(v.as_str().unwrap()),
            })
            .collect(),
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if &args[1] == "supports" {
            // Support all renderers
            process::exit(0);
        }
    }

    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin()).expect("Failed to parse input");
    let preprocessor = Abbr::new(&ctx);
    let processed_book = preprocessor.run(&ctx, book);

    serde_json::to_writer(
        io::stdout(),
        &processed_book.expect("Failed to process book"),
    )
    .expect("Failed to serialize book");
}
