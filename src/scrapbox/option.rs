pub struct ScrapboxOption {
  pub newline_before_heading: bool,
}

impl Default for ScrapboxOption {
  fn default() -> Self {
    Self {
      newline_before_heading: true,
    }
  }
}
