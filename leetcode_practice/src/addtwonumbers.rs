#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    zip_add(l1, l2)
}

// this is the actual solution, everything else is just scaffolding for testing because this is a fully owned datastructure
fn zip_add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = Some(Box::new(ListNode::new(0)));
    let mut carryover = 0;

    let mut l1_current = l1;
    let mut l2_current = l2;
    let mut result_current = result.as_mut();

    loop {
        let temp = l1_current.as_ref().map_or(0, |node| node.val)
            + l2_current.as_ref().map_or(0, |node| node.val)
            + carryover;
        result_current.as_mut().unwrap().val = temp % 10;
        carryover = temp / 10;

        l1_current = l1_current.and_then(|node| node.next);
        l2_current = l2_current.and_then(|node| node.next);
        if l1_current.is_none() && l2_current.is_none() && carryover == 0 {
            break;
        }

        result_current.as_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
        result_current = result_current.unwrap().next.as_mut();
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::addtwonumbers::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[2,4,3],&[5,6,4],&[7,0,8])]
    #[case(&[0],&[0],&[0])]
    #[case(&[9,9,9,9,9,9,9],&[9,9,9,9],&[8,9,9,9,0,0,0,1])]
    fn test_zip_add(#[case] l1: &[i32], #[case] l2: &[i32], #[case] expected: impl Into<Vec<i32>>) {
        let result: Vec<i32> =
            (*zip_add(Some(Box::new(l1.into())), Some(Box::new(l2.into()))).unwrap()).into();
        assert_eq!(result.as_slice(), expected.into().as_slice());
    }
}
