// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Constrains

    // The number of nodes in each linked list is in the range [1, 100].
    // 0 <= Node.val <= 9
    // It is guaranteed that the list represents a number that does not have leading zeros.
    

    let mut current = l1;
    let mut cnt_l1 = 0;
    while let Some(node) = current {
        println!("{}", node.val);
        // println!("{}", cnt_l1);
        current = node.next;
        cnt_l1 += 1;
    }

    // Check when cnt is 1, 

    
    
    
    None
}