use obs2scrap::scrapbox;
use pulldown_cmark::Parser;
use std::fs;

fn main() -> anyhow::Result<()> {
  let md_content = fs::read_to_string("sample/sample1.md")?;

  let parser = Parser::new(&md_content);
  let options = scrapbox::option::ScrapboxOption::default();
  let mut output = String::new();
  scrapbox::push_scrapbox(&mut output, parser, options);

  println!("{}", output);

  Ok(())
}
