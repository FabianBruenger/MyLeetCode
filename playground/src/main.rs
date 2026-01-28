mod leetcode_functions;

fn main() {
    let l1_list = [2,4,3];
    let l2_list = [5,6,4];
    let mut l1: Option<Box<leetcode_functions::ListNode>> = None;
    let mut l2: Option<Box<leetcode_functions::ListNode>> = None;

    for x in l1_list.iter().rev() {
        l1 = Some(Box::new(leetcode_functions::ListNode {
            val: *x as i32,
            next: l1,
        }));
    }
    
    for x in l2_list.iter().rev() {
        l2 = Some(Box::new(leetcode_functions::ListNode {
            val: *x as i32,
            next: l2,
        }));
        println!("{:?}", l2);
    }

    println!("{:?}", l1);
    println!("{:?}", l2);

    let res = leetcode_functions::add_two_numbers(l1,l2);
    println!("Res {:?}", res);
}
