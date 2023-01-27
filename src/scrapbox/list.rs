pub struct ListHandler {
  lists: Vec<List>,
}

struct List {
  /// Index of ordered list.
  /// If `order` is zero, the list is unordered.
  /// If `order` is not zero, list is ordered and `order` means the index of the list.
  order: u64,
}

impl ListHandler {
  pub fn new() -> Self {
    Self { lists: Vec::new() }
  }

  pub fn start_list(&mut self, order_start: Option<u64>) -> String {
    let index = order_start.unwrap_or(0);
    self.lists.push(List { order: index });

    if self.lists.len() >= 2 {
      "\n".into()
    } else {
      "".into()
    }
  }

  pub fn end_list(&mut self) {
    if self.lists.pop().is_none() {
      panic!("ListHandler::end_list: list is empty");
    }
  }

  pub fn get(&mut self) -> String {
    let depth = self.lists.len();
    let list = self.lists.last_mut().unwrap();
    let s = if list.order == 0 {
      " ".repeat(depth)
    } else {
      list.order += 1;
      format!("{}{}. ", " ".repeat(depth), list.order - 1)
    };

    s
  }

  pub fn depth(&self) -> usize {
    self.lists.len()
  }
}
