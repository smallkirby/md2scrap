mod util;

#[test]
fn test_heading() {
  let md = r##"# Heading1
normal sentence"##;
  let expected = r##"[****** Heading1]
normal sentence"##;
  util::assert_cmp(md, expected);

  let md = r##"##### Heading5
normal sentence"##;
  let expected = r##"[** Heading5]
normal sentence"##;
  util::assert_cmp(md, expected);
}

#[test]
fn test_strong() {
  let md = r##"**strong**"##;
  let expected = r##"[* strong]"##;
  util::assert_cmp(md, expected);
}

#[test]
fn test_strikethrough() {
  let md = r##"~~strikethrough~~"##;
  let expected = r##"[- strikethrough]"##;
  util::assert_cmp(md, expected);
}
