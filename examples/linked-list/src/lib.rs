#![allow(dead_code)]

use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Rc::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
       self.head.as_ref().map(|node| {
            let node = Rc::try_unwrap(node).ok().unwrap();
            self.head = node.next;
            node.elem.take()
        })

//        match self.head.take() {
//            None => None,
//            Some(node) => {
//                self.head = Rc::try_unwrap(node.next.unwrap()).ok().unwrap().into_inner().elem;
//                Some(node.elem)
//            }
//        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn new() {
        let l = LinkedList::<i32>::new();
        assert!(l.head.is_none());
    }

    #[test]
    fn push_single() {
        let mut l = LinkedList::new();
        l.push(1);

        let head_node = *l.head.unwrap();
        assert_eq!(head_node.elem, 1);
        assert!(head_node.next.is_none());
    }

    #[test]
    fn push_multiple() {
        let mut l = LinkedList::new();
        l.push(1);
        l.push(2);

        let head_node = *l.head.unwrap();
        assert_eq!(head_node.elem, 2);
        assert!(head_node.next.is_some());

        let next_node = head_node.next.unwrap();
        assert_eq!(next_node.elem, 1);
        assert!(next_node.next.is_none());
    }

    #[test]
    fn pop_single() {
        let mut l = LinkedList::new();
        l.push(1);

        let head_elem = l.pop();
        assert!(head_elem.is_some());
        assert_eq!(head_elem.unwrap(), 1);

        assert!(l.head.is_none());
    }

    #[test]
    fn pop_multiple() {
        let mut l = LinkedList::new();
        l.push(1);
        l.push(2);

        let head_elem = l.pop();
        assert!(head_elem.is_some());
        assert_eq!(head_elem.unwrap(), 2);

        let new_head_node = *l.head.unwrap();
        assert!(new_head_node.next.is_none());
        assert_eq!(new_head_node.elem, 1);

        let head_elem = l.pop();
        assert!(head_elem.is_some());
        assert_eq!(head_elem.unwrap(), 1);

        assert!(l.head.is_none());
    }

    #[test]
    fn full_api() {}
}
