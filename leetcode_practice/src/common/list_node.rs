#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub(crate) fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl From<&[i32]> for ListNode {
    fn from(value: &[i32]) -> Self {
        let mut head = None;
        for i in value.iter().rev() {
            let temp = ListNode {
                val: *i,
                next: head,
            };
            head = Some(Box::new(temp));
        }

        head.map_or(ListNode::new(0), |h| *h)
    }
}

impl From<ListNode> for Vec<i32> {
    fn from(value: ListNode) -> Self {
        let mut result = vec![];
        let mut current = Some(Box::new(value));
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }
}