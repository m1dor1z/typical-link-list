#[derive(Debug)]
struct LinkList {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}


#[derive(Debug)]
struct Node {
    payload: i32,
    next: Link
}


impl LinkList {
    pub fn new() -> Self {
        LinkList { head: Link::Empty }
    }

    pub fn push(&mut self, payload: i32) {
        let node = Node {
            payload,
            next: std::mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.payload)
            }
        }
    }
}


impl Drop for LinkList {
    fn drop(&mut self) {
        let mut curr = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = curr {
            curr = std::mem::replace(&mut self.head, Link::Empty);
        }
    }
}


// #[cfg(test)]
// mod test {
//     use super::LinkList;

//     #[test]
//     fn basics() {
//         let mut list = LinkList::new();

//         // Check empty list behaves right
//         assert_eq!(list.pop(), None);

//         // Populate list
//         list.push(1);
//         list.push(2);
//         list.push(3);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(3));
//         assert_eq!(list.pop(), Some(2));

//         // Push some more just to make sure nothing's corrupted
//         list.push(4);
//         list.push(5);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(5));
//         assert_eq!(list.pop(), Some(4));

//         // Check exhaustion
//         assert_eq!(list.pop(), Some(1));
//         assert_eq!(list.pop(), None);
//     }
// }
