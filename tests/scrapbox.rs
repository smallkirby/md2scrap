use pulldown_cmark::{Parser, Options};
use obs2scrap::scrapbox;

fn convert(md: &str) -> String {
  let parser = Parser::new_ext(md, Options::all());
  let mut output = String::new();
  scrapbox::push_scrapbox(&mut output, parser);

  output
}

fn assert_cmp(md: &str, expected: &str) {
  assert_eq!(expected, convert(md).trim_end());
}

#[test]
fn test_heading() {
  let md = r##"# Heading1
normal sentence"##;

  let expected = r##"[****** Heading1]
normal sentence"##;

  assert_cmp(md, expected);
}