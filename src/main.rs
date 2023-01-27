use anyhow;
use std::fs;
use pulldown_cmark::{Parser, Options};
use obs2scrap::scrapbox;

fn main() -> anyhow::Result<()> {
  let md_content = fs::read_to_string("sample/sample1.md")?;

  let parser = Parser::new_ext(&md_content, Options::all());
  let mut output = String::new();
  scrapbox::push_scrapbox(&mut output, parser);

  println!("{}", output);

  Ok(())
}
