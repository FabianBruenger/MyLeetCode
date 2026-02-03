use core::num;

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
    println!("Add two numbers called");
    // The number of nodes in each linked list is in the range [1, 100].
    // 0 <= Node.val <= 9
    // It is guaranteed that the list represents a number that does not have leading zeros.

    // Iterate through both lists and add both values with transfer
    let mut l1_current = l1;
    let mut l2_current = l2;
    let mut transfer = 0;
    let mut res_final: Option<Box<ListNode>> = None;
    let mut res_final_arra: Vec<i32> = Vec::new();

    while l1_current.is_some() || l2_current.is_some() || transfer != 0 {
      println!("In while.. l1 = {:?}, l2 = {:?}, transfer = {}\n", l1_current, l2_current, transfer);

      // get values and add them
      // Note: The map_or function I needed to look up!
      let val1 = l1_current.as_ref().map_or(0, |node| node.val);
      let val2 = l2_current.as_ref().map_or(0, |node| node.val);

      let sum = val1 + val2 + transfer;
      transfer = sum / 10;
      let digit = sum % 10;

      println!("Calculated: sum = val1 + val2 + transfer. {} = {} + {} + {} \n", sum, val1, val2, transfer);

      // Push the digits to the result array, later transform it.
      res_final_arra.push(digit);

      // Note: The end_then function I needed to look up!
      l1_current = l1_current.and_then(|node| node.next);
      l2_current = l2_current.and_then(|node| node.next);
    }

    // Transform the result array into the linked list
    // Note: This could maybe be smarter. 
    for x in res_final_arra.iter().rev() {
        res_final = Some(Box::new(ListNode {
            val: *x as i32,
            next: res_final,
        }));
    }

    res_final
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(digits: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &digit in digits.iter().rev() {
            head = Some(Box::new(ListNode {
                val: digit,
                next: head,
            }));
        }
        head
    }

    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        result
    }

    #[test]
    fn test_add_two_numbers_342_plus_465() {
        let l1 = create_list(vec![2, 4, 3]);
        let l2 = create_list(vec![5, 6, 4]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![7, 0, 8]);
    }

    #[test]
    fn test_add_two_numbers_zero_plus_zero() {
        let l1 = create_list(vec![0]);
        let l2 = create_list(vec![0]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![0]);
    }

    #[test]
    fn test_add_two_numbers_large_numbers() {
        let l1 = create_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = create_list(vec![9, 9, 9, 9]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn test_add_two_numbers_different_lengths() {
        let l1 = create_list(vec![9]);
        let l2 = create_list(vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn test_add_two_numbers_very_large() {
        // 10000000000000000000000000000001 + 465
        let l1 = create_list(vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        let l2 = create_list(vec![5, 6, 4]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![6, 6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    }
}