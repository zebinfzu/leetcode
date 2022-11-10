#![allow(dead_code)]

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

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
      let mut dummy = Some(Box::new(ListNode {val:-1, next: head}));
      let mut slow_p = &mut dummy;
      // clone方法，复制了一份链表
      let mut fast_p = &slow_p.clone();
      for _ in 0..=n {
        if let Some(fast_node) = fast_p {
            fast_p = &fast_node.next;
        } else {
          return None;
        }
      }

      while fast_p.is_some() {
        fast_p = &fast_p.as_ref().unwrap().next;
        slow_p = &mut slow_p.as_mut().unwrap().next;
      }

      let remove_p = &mut slow_p.as_mut().unwrap().next;
        slow_p.as_mut().unwrap().next = remove_p.as_mut().unwrap().next.take();

        dummy.unwrap().next
    }
}
fn main() {
}
