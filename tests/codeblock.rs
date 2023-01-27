mod util;

#[test]
fn test_unnamed_block() {
  let md = r##"```
fn main() {
  println!("Hello, world!");
}
```"##;
  let expected = r##"code:
 fn main() {
   println!("Hello, world!");
 }
"##;
  util::assert_cmp(md, expected);
}

#[test]
fn test_named_block() {
  let md = r##"```hello.rs
fn main() {
  println!("Hello, world!");
}
```"##;
  let expected = r##"code:hello.rs
 fn main() {
   println!("Hello, world!");
 }
"##;
  util::assert_cmp(md, expected);
}
