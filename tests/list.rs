mod util;

#[test]
fn test_simple_unordered_list() {
  let md = r##"
- list1
- list2
- list3
"##;
  let expected = r##"
 list1
 list2
 list3
"##;
  util::assert_cmp(md, expected);
}

#[test]
fn test_simple_ordered_list() {
  let md = r##"
  1. one
  2. two
  3. three
"##;
let expected = r##"
 1. one
 2. two
 3. three
"##;
  util::assert_cmp(md, expected);
}

#[test]
fn test_nested_unordered_list() {
  let md = r##"
- list1
  - list2
    - list3
- list4
  - list5
"##;
  let expected = r##"
 list1
  list2
   list3
 list4
  list5
"##;
  util::assert_cmp(md, expected);
}
