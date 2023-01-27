use pulldown_cmark::{Parser, Options};
use obs2scrap::scrapbox;

pub fn convert(md: &str) -> String {
  let parser = Parser::new_ext(md, Options::all());
  let mut output = String::new();
  scrapbox::push_scrapbox(&mut output, parser);

  output
}

pub fn assert_cmp(md: &str, expected: &str) {
  assert_eq!(expected.trim(), convert(md).trim());
}