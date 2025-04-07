use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    where
        T: Ord + Clone,
    {
        let mut merged_list = LinkedList::new();
        let mut ptr_a = list_a.start;
        let mut ptr_b = list_b.start;

        // Traverse both lists and merge them
        while let (Some(a), Some(b)) = (ptr_a, ptr_b) {
            if unsafe { a.as_ref() }.val <= unsafe { b.as_ref() }.val {
                merged_list.add(unsafe { a.as_ref() }.val.clone());
                ptr_a = unsafe { a.as_ref() }.next;
            } else {
                merged_list.add(unsafe { b.as_ref() }.val.clone());
                ptr_b = unsafe { b.as_ref() }.next;
            }
        }

        // If list_a still has remaining nodes, append them
        while let Some(a) = ptr_a {
            merged_list.add(unsafe { a.as_ref() }.val.clone());
            ptr_a = unsafe { a.as_ref() }.next;
        }

        // If list_b still has remaining nodes, append them
        while let Some(b) = ptr_b {
            merged_list.add(unsafe { b.as_ref() }.val.clone());
            ptr_b = unsafe { b.as_ref() }.next;
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_merge_linked_list() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &i in &vec_a {
            list_a.add(i);
        }
        for &i in &vec_b {
            list_b.add(i);
        }

        let mut merged_list = LinkedList::<i32>::merge(list_a, list_b);

        for (i, &target) in target_vec.iter().enumerate() {
            assert_eq!(target, *merged_list.get(i as i32).unwrap());
        }
    }
}
