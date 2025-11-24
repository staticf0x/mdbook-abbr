use log::debug;
use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::config::Config;
use mdbook_preprocessor::{parse_input, Preprocessor, PreprocessorContext};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::io;
use std::process;
use std::vec;

const NAME: &str = "abbr";

#[derive(Debug, Deserialize)]
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
        _ctx: &PreprocessorContext,
        mut book: Book,
    ) -> mdbook_preprocessor::errors::Result<Book> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chap) = item {
                debug!("Processing chapter: {}", chap.name);
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

    fn supports_renderer(&self, _renderer: &str) -> mdbook_preprocessor::errors::Result<bool> {
        Ok(true)
    }
}

/// Get the abbreviation -> expansion config
fn get_abbr_table(config: &Config) -> Option<Vec<Abbreviation>> {
    let abbr_table: HashMap<String, String> = config
        .get::<HashMap<String, String>>("preprocessor.abbr.list")
        .expect("Couldn't get preprocessor.abbr.list")?;

    debug!(
        "Found {} abbreviations in preprocessor.abbr.list",
        abbr_table.len()
    );

    Some(
        abbr_table
            .iter()
            .into_iter()
            .map(|(key, value)| Abbreviation {
                abbr: key.to_string(),
                expanded: value.to_string(),
            })
            .collect::<Vec<Abbreviation>>(),
    )
}

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if &args[1] == "supports" {
            // Support all renderers
            process::exit(0);
        }
    }

    debug!("Running mdbook-abbr preprocessor");

    let (ctx, book) = parse_input(io::stdin()).expect("Failed to parse input");
    let preprocessor = Abbr::new(&ctx);
    let processed_book = preprocessor.run(&ctx, book);

    serde_json::to_writer(
        io::stdout(),
        &processed_book.expect("Failed to process book"),
    )
    .expect("Failed to serialize book");
}
