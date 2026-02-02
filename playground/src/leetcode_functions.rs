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

    // Constrains
    let max_number_of_nodes = 100;
    let min_number_of_nodes = 1;

    let max_node_value = 9;
    let min_node_value = 0;

    // buffers L1
    let mut current_node_l1 = l1;
    let mut number_of_nodes_l1_cnt = 0;
    let mut l1_array = Vec::new();

    // buffers L2
    let mut current_node_l2 = l2;
    let mut number_of_nodes_l2_cnt = 0;
    let mut l2_array: Vec<i32> = Vec::new();
    
    // Getting arrays from linked lists
    while let Some(node) = current_node_l1 {
        // println!("{}", node.val);
        // Verify if node value is in range
        if node.val < min_node_value || node.val > max_node_value {
            panic!("Node value is out of bounds");
        }
        current_node_l1 = node.next;
        number_of_nodes_l1_cnt += 1;
        l1_array.push(node.val);
    }

    while let Some(node) = current_node_l2 {
        // println!("{}", node.val);
        // Verify if node value is in range
        if node.val < min_node_value || node.val > max_node_value {
            panic!("Node value is out of bounds");
        }
        current_node_l2 = node.next;
        number_of_nodes_l2_cnt += 1;
        l2_array.push(node.val);
    }

    // Debugging output
    println!("Total nodes in l1: {}", number_of_nodes_l1_cnt);
    println!("{:?}", l1_array);
    println!("Total nodes in l2: {}", number_of_nodes_l2_cnt);
    println!("{:?}", l2_array);

    // Checks before proceeding
    // Should be minimum length
    if number_of_nodes_l1_cnt < min_number_of_nodes || number_of_nodes_l1_cnt > max_number_of_nodes {
        panic!("The number of nodes in the linked list is out of bounds");
    }
    if number_of_nodes_l2_cnt < min_number_of_nodes || number_of_nodes_l2_cnt > max_number_of_nodes {
        panic!("The number of nodes in the linked list is out of bounds");
    }

    // Check if the last node is not zero (no leading zeros)
    if let Some(&last) = l1_array.last() {
        if last == 0 && number_of_nodes_l1_cnt > 1 {
            panic!("The list represents a number that has leading zeros");
        }
    }
    if let Some(&last) = l2_array.last() {
        if last == 0 && number_of_nodes_l2_cnt > 1 {
            panic!("The list represents a number that has leading zeros");
        }
    }

    // Processing: Result number
    let l1_number = l1_array.iter().rev().fold(0, |acc: usize, &digit| acc * 10 + digit as usize);
    let l2_number = l2_array.iter().rev().fold(0, |acc: usize, &digit| acc * 10 + digit as usize);
    let res = l1_number + l2_number;
    println!("l1 number: {}", l1_number);
    println!("l2 number: {}", l2_number);
    println!("res number: {}", res);
    
    // Edge case for 0 result
    if res == 0 {
        return Some(Box::new(ListNode { val: 0, next: None }));
    } 
    
    let mut arr  = Vec::new();
    let mut n = res;
    
    while n > 0 { 
      arr.push((n % 10) as i32);
      n /= 10;
    }

    println!("res array: {:?}", arr);

    // Final output linked list
    let mut res_final: Option<Box<ListNode>> = None;
    for x in arr.iter().rev() {
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