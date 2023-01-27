mod util;

#[test]
fn test_inline_link() {
  let md = r##"[link](https://example.com)"##;
  let expected = r##"[https://example.com link]"##;
  util::assert_cmp(md, expected);
}

#[test]
fn test_inline_link_empty_link() {
  let md = r##"[link]()"##;
  let expected = r##"[ link]"##;
  util::assert_cmp(md, expected);
}
