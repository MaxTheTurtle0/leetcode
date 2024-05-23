// Definition for singly-linked list.
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


pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode{val: 0, next: head}));     
    let mut fast = dummy.as_ref();
    let mut slow = dummy.as_ref();


    for _ in 0..n + 1 {
        fast = Some(fast.unwrap().next.unwrap().as_ref());
    }


    while let Some(node) = fast {
        fast = Some(fast.unwrap().next.unwrap());
        slow = Some(slow.unwrap().next.unwrap());
    }

    slow.unwrap().next = slow.unwrap().next.take();

    return dummy.unwrap().next;
}
