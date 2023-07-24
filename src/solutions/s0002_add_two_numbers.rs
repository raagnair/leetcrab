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

/// This is an iterative solution. Even though the problem implores a recursive one,
/// we don't know how long the ListNode chain can be.
///
/// Pseudo code:
/// Create pointers (left, right) to both input nodes
/// Create an empty "head" node that we will add to as we count our sum
/// Create a pointer that represents the "previous" node in our iteration
///   this pointer starts out by pointing to our "head"
/// Track whether we have an addition overflow, start it off as false
///
/// While our two pointers are not empty
///   Start counting a sum for our latest pointers
///
///   +1 if we had a remainder from our last iteration
///   if left points to something, add its value to sum, point left to left.next
///   if right points to something, add its value to sum, point right to right.next
///
///   recalculate remainder as true only if sum is greater or equal to 10
///   then, create a new ListNode with value as sum % 10 as current.next
///   set current to current.next
/// End of While loop
///
/// At this point, both left and right pointers are exhausted.
/// In the event that there's a remainder, add one last ListNode with value of 1.
///
/// Return "head".next.
///
/// Time complexity: O(N) where N is the larger of the two input lists
/// Space complexity: O(N) where N is the larger of the two input lists
///
#[allow(dead_code)]
pub fn add_two_numbers(
  l1: Option<Box<ListNode>>,
  l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut left = l1;
  let mut right = l2;
  let mut head = Some(Box::new(ListNode::new(0)));
  let mut current = head.as_mut();
  let mut remainder = false;

  while left.is_some() || right.is_some() {
    let mut sum = if remainder {1} else {0};
    if let Some(node) = left {
      sum += node.val;
      left = node.next;
    }
    if let Some(node) = right {
      sum += node.val;
      right = node.next;
    }

    remainder = sum >= 10;

    if let Some(node) = current {
      node.next = Some(Box::new(ListNode::new(sum % 10)));
      current = node.next.as_mut();
    }
  }

  if remainder {
    current.unwrap().next = Some(Box::new(ListNode::new(1)));
  }

  head.unwrap().next
}