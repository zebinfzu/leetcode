#![allow(unused)]
#![allow(dead_code)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    fn stringify(&self) -> String {
        let ref next = self.next;
        match next {
            Some(node) => {
                format!("{}->{}", self.val, node.stringify())
            }
            None => {
                format!("{}->Nil", self.val)
            }
        }
    }
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    let mut head = head;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = pre;
        pre = Some(node);
    }
    pre
}

fn reverse_list_(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn reverse(head: Option<Box<ListNode>>, pre: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let tail = node.next.take();
            node.next = pre;
            return reverse(tail, Some(node));
        }
        pre
    }
    reverse(head, None)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
}
