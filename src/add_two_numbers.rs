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
      val,
    }
  }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1c = l1.clone();
    let mut l2c = l2.clone();

    let mut rem: i32 = 0;
    let mut result = Vec::new();

    while l1c.is_some() || l2c.is_some() ||  rem > 0 {
        let val1 = number_or_zero(&l1c);
        let val2 = number_or_zero(&l2c);

        l1c = match l1c { None => None, Some(l) => l.next };
        l2c = match l2c { None => None, Some(l) => l.next };

        let res = val1 + val2 + rem;
        rem = res.div_euclid(10);

        result.push(res % 10);
    }

    result.reverse();
    return from_vec(result)
}

pub fn from_vec(l: Vec<i32>) -> Option<Box<ListNode>> {
    let mut node = None;
    for i in l {
       node = Some(Box::new(ListNode {
           val: i,
           next: node,
       }))
    }

    node
}

pub fn parse(l: Option<Box<ListNode>>) -> i32 {
    let mut node = l;
    let mut result: i32 = 0;
    let mut idx = 0;

    while node.is_some() {
        let n = node.as_ref().unwrap().val * 10_i32.pow(idx);
        result += n;
        idx += 1;
        node = node.unwrap().next;
    }

    return result
}

pub fn number_or_zero(l: &Option<Box<ListNode>>) -> i32 {
    match l {
        None => 0,
        Some(n) => n.val,
    }
}


#[cfg(test)]
mod tests {
    use crate::add_two_numbers::*;
    #[test]
    fn test_add_two_numbers_long() {
        let input1 = Some(Box::new(ListNode::new(9)));

        let input2 = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode::new(1)))
        }));

        assert_eq!(22, parse(add_two_numbers(input1, input2)))
    }

    #[test]
    fn test_add_two_numbers_none() {
        let input1 = None;
        let input2 = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode::new(1)))
        }));

        assert_eq!(13, parse(add_two_numbers(input1, input2)))
    }

    #[test]
    fn test_add_two_numbers() {
        let input1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(1)))
        }));

        let input2 = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode::new(1)))
        }));

        assert_eq!(25, parse(add_two_numbers(input1, input2)))
    }

    #[test]
    fn test_parse() {
        let input1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(1)))
        }));

        let out = 12;
        assert_eq!(out, parse(input1))
    }

    #[test]
    fn test_parse_2() {
        let input1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(1)))
            }))
        }));

        let out = 132;
        assert_eq!(out, parse(input1))
    }
}
