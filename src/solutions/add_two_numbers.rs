#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn _add_two_numbers(_l1: Option<Box<ListNode>>, _l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if _l1.is_none() && _l2.is_none() { return None }
    return None
}