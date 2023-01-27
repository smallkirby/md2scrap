use obs2scrap::scrapbox;
use pulldown_cmark::{Options, Parser};

pub fn convert(md: &str) -> String {
  let parser = Parser::new_ext(md, Options::all());
  let option = scrapbox::option::ScrapboxOption::default();
  let mut output = String::new();
  scrapbox::push_scrapbox(&mut output, parser, option);

  output
}

pub fn assert_cmp(md: &str, expected: &str) {
  assert_eq!(expected.trim(), convert(md).trim());
}
