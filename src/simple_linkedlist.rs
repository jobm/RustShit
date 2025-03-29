use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct SimpleLinkedList<T> {
    pub val: Option<T>,
    pub next: Option<Box<SimpleLinkedList<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            val: None,
            next: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.next.is_none()
    }

    pub fn len(&self) -> usize {
        let mut size: usize = 0;
        let mut curr_node = self.next.as_ref();
        while curr_node.is_some() {
            curr_node = curr_node.unwrap().next.as_ref();
            size += 1;
        }
        size
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Some(Box::new(SimpleLinkedList {
            val: Some(_element),
            next: self.next.take(),
        }));
        self.next = new_node
    }

    pub fn pop(&mut self) -> Option<T> {
        self.next.take().map(|node| {
            self.next = node.next;
            node.val
        })?
    }

    pub fn peek(&self) -> Option<&T> {
        Some(self.next.as_ref()?.val.as_ref()?)
    }

    #[must_use]
    pub fn rev(&self) -> SimpleLinkedList<T>
    where
        T: Clone + Debug,
    {
        let mut list = SimpleLinkedList::new();
        let mut curr_node = self.next.as_ref();
        while curr_node.is_some() {
            list.push(curr_node.cloned().unwrap().val.unwrap());
            curr_node = curr_node.as_ref().unwrap().next.as_ref();
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T>
where
    T: Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list: SimpleLinkedList<T> = SimpleLinkedList::new();
        for it in _iter {
            list.push(it)
        }
        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T>
where
    T: Clone + Debug,
{
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec: Vec<T> = vec![];
        let mut curr_node = _linked_list.next.as_ref();
        while curr_node.is_some() {
            vec.push(curr_node.cloned().unwrap().val.unwrap());
            curr_node = curr_node.as_ref().unwrap().next.as_ref();
        }
        vec.reverse();
        vec
    }
}
