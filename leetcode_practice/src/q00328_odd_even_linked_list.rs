use crate::common::list_node::ListNode;

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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[1,2,3,4,5], &[1,3,5,2,4])]
    #[case(&[2,1,3,5,6,4,7], &[2,3,6,7,1,5,4])]
    fn test_odd_even_list(#[case] input: &[i32], #[case] expected: impl Into<Vec<i32>>) {
        let result: Vec<i32> = (*odd_even_list(Some(Box::new(input.into()))).unwrap()).into();
        assert_eq!(result, expected.into());
    }
}
