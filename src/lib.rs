use std::collections::{LinkedList, VecDeque};

struct LinkedVec<T> {
    vec_size: usize,
    list: LinkedList<VecDeque<T>>,
}

impl<T> LinkedVec<T> {
    pub fn new(vec_size: usize) -> Self {
        let mut list = LinkedList::new();
        list.push_front(VecDeque::with_capacity(vec_size));

        Self { vec_size, list }
    }

    pub fn len(&self) -> usize {
        let front = self.front();

        if self.list.len() == 1 {
            return front.len();
        }
        let back = self.back();

        (self.list.len() - 2) * self.vec_size + front.len() + back.len()
    }

    pub fn capacity(&self) -> usize {
        self.list.len() * self.vec_size
    }

    pub fn push_front(&mut self, value: T) {
        let mut front = self.front_mut();

        if front.len() == front.capacity() {
            let mut v = VecDeque::with_capacity(self.vec_size);
            v.push_front(value);
            self.list.push_front(v);
        } else {
            front.push_front(value)
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.front().len() == 1 && self.list.len() > 1 {
            self.list.pop_front()?.pop_front()
        } else {
            self.front_mut().pop_front()
        }
    }

    pub fn push_back(&mut self, value: T) {
        let mut back = self.back_mut();

        if back.len() == back.capacity() {
            let mut v = VecDeque::with_capacity(self.vec_size);
            v.push_front(value);
            self.list.push_back(v);
        } else {
            back.push_back(value);
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.back().len() == 1 && self.list.len() > 1 {
            self.list.pop_back()?.pop_back()
        } else {
            self.back_mut().pop_back()
        }
    }

    fn front(&self) -> &VecDeque<T> {
        match self.list.front() {
            None => unreachable!(),
            Some(v) => v,
        }
    }

    fn front_mut(&mut self) -> &mut VecDeque<T> {
        match self.list.front_mut() {
            None => unreachable!(),
            Some(v) => v,
        }
    }

    fn back(&self) -> &VecDeque<T> {
        match self.list.back() {
            None => unreachable!(),
            Some(v) => v,
        }
    }

    fn back_mut(&mut self) -> &mut VecDeque<T> {
        match self.list.back_mut() {
            None => unreachable!(),
            Some(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedVec;

    #[test]
    fn test_push_front_pop_front() {
        let mut lv = LinkedVec::new(5);

        lv.push_front(1);
        lv.push_front(2);
        lv.push_front(3);
        lv.push_front(4);
        lv.push_front(5);
        lv.push_front(6);

        assert_eq!(lv.len(), 6);
        assert_eq!(lv.capacity(), 10);

        assert_eq!(lv.pop_front(), Some(6));
        assert_eq!(lv.pop_front(), Some(5));
        assert_eq!(lv.pop_front(), Some(4));
        assert_eq!(lv.pop_front(), Some(3));
        assert_eq!(lv.pop_front(), Some(2));
        assert_eq!(lv.pop_front(), Some(1));
        assert_eq!(lv.pop_front(), None);

        assert_eq!(lv.len(), 0);
        assert_eq!(lv.capacity(), 5);
    }

    #[test]
    fn test_push_back_pop_front() {
        let mut lv = LinkedVec::new(5);

        lv.push_back(1);
        lv.push_back(2);
        lv.push_back(3);
        lv.push_back(4);
        lv.push_back(5);
        lv.push_back(6);

        assert_eq!(lv.len(), 6);
        assert_eq!(lv.capacity(), 10);

        assert_eq!(lv.pop_front(), Some(1));
        assert_eq!(lv.pop_front(), Some(2));
        assert_eq!(lv.pop_front(), Some(3));
        assert_eq!(lv.pop_front(), Some(4));
        assert_eq!(lv.pop_front(), Some(5));
        assert_eq!(lv.pop_front(), Some(6));
        assert_eq!(lv.pop_front(), None);

        assert_eq!(lv.len(), 0);
        assert_eq!(lv.capacity(), 5);
    }

    #[test]
    fn test_push_front_pop_back() {
        let mut lv = LinkedVec::new(5);

        lv.push_front(1);
        lv.push_front(2);
        lv.push_front(3);
        lv.push_front(4);
        lv.push_front(5);
        lv.push_front(6);

        assert_eq!(lv.len(), 6);
        assert_eq!(lv.capacity(), 10);

        assert_eq!(lv.pop_back(), Some(1));
        assert_eq!(lv.pop_back(), Some(2));
        assert_eq!(lv.pop_back(), Some(3));
        assert_eq!(lv.pop_back(), Some(4));
        assert_eq!(lv.pop_back(), Some(5));
        assert_eq!(lv.pop_back(), Some(6));
        assert_eq!(lv.pop_front(), None);

        assert_eq!(lv.len(), 0);
        assert_eq!(lv.capacity(), 5);
    }

    #[test]
    fn test_push_back_pop_back() {
        let mut lv = LinkedVec::new(5);

        lv.push_back(1);
        lv.push_back(2);
        lv.push_back(3);
        lv.push_back(4);
        lv.push_back(5);
        lv.push_back(6);

        assert_eq!(lv.len(), 6);
        assert_eq!(lv.capacity(), 10);

        assert_eq!(lv.pop_back(), Some(6));
        assert_eq!(lv.pop_back(), Some(5));
        assert_eq!(lv.pop_back(), Some(4));
        assert_eq!(lv.pop_back(), Some(3));
        assert_eq!(lv.pop_back(), Some(2));
        assert_eq!(lv.pop_back(), Some(1));
        assert_eq!(lv.pop_front(), None);

        assert_eq!(lv.len(), 0);
        assert_eq!(lv.capacity(), 5);
    }

    #[test]
    fn test_empty() {
        let mut lv: LinkedVec<i64> = LinkedVec::new(5);

        assert_eq!(lv.len(), 0);
        assert_eq!(lv.capacity(), 5);
        assert_eq!(lv.pop_back(), None);
        assert_eq!(lv.pop_front(), None);
    }

    #[test]
    fn test_partially_filled_back() {
        let mut lv: LinkedVec<i64> = LinkedVec::new(3);

        lv.push_back(1);
        lv.push_back(2);
        lv.push_back(3);
        lv.push_back(4);

        assert_eq!(lv.len(), 4);
        assert_eq!(lv.capacity(), 6);

        lv.push_front(5);
        assert_eq!(lv.len(), 5);
        assert_eq!(lv.capacity(), 9);

        assert_eq!(lv.pop_back(), Some(4));
        assert_eq!(lv.len(), 4);
        assert_eq!(lv.capacity(), 6);

        assert_eq!(lv.pop_front(), Some(5));
        assert_eq!(lv.len(), 3);
        assert_eq!(lv.capacity(), 3);
    }
}
