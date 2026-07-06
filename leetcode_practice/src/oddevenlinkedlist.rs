#[derive(PartialEq, Eq, Debug)]
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

// must be O(1) space and O(n) time
pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none()
        || head.as_ref().unwrap().next.is_none()
        || head.as_ref().unwrap().next.as_ref().unwrap().next.is_none()
    {
        return head;
    }

    let mut odds = head.unwrap();
    let mut evens = odds.next.take().unwrap();
    let mut cursor = evens.next.take().unwrap();
    let mut odds_tail = &mut odds;
    let mut evens_tail = &mut evens;
    let mut cursor_is_odd = true;

    loop {
        if let Some(next) = cursor.next.take() {
            if cursor_is_odd {
                odds_tail.next = Some(cursor);
                odds_tail = odds_tail.next.as_mut().unwrap();
                cursor = next;
            } else {
                evens_tail.next = Some(cursor);
                evens_tail = evens_tail.next.as_mut().unwrap();
                cursor = next;
            }
            cursor_is_odd = !cursor_is_odd;
        } else {
            if cursor_is_odd {
                odds_tail.next = Some(cursor);
                odds_tail = odds_tail.next.as_mut().unwrap();
            } else {
                evens_tail.next = Some(cursor);
            }
            odds_tail.next = Some(evens);
            break;
        }
    }

    Some(odds)
}
