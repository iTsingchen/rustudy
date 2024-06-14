#[derive(Debug)]
pub struct IntoIter<T>(Stack<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Debug)]
pub struct Iter<'a, T: 'a>(Vec<&'a T>);
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T: 'a>(Vec<&'a mut T>);
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        let index = self.size() - 1;
        self.data.get(index)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }
        let index = self.size() - 1;
        self.data.get_mut(index)
    }

    pub fn info_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.data.iter().rev().collect::<Vec<&T>>())
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.data.iter_mut().rev().collect::<Vec<&mut T>>())
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::Stack;

    #[test]
    fn basic() {
        let mut s = Stack::new();
        assert_eq!(s.size(), 0);

        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.size(), 3);

        let item = s.pop();
        assert_eq!(item.unwrap(), 3);
        assert_eq!(s.size(), 2);

        s.push(4);
        s.push(5);
        assert_eq!(s.size(), 4);

        s.clear();
        assert_eq!(s.size(), 0);
    }

    #[test]
    fn peek() {
        let mut s = Stack::new();

        s.push(1);
        s.push(2);
        s.push(3);

        let item = s.peek();
        assert_eq!(item.unwrap(), &3);
        assert_eq!(s.size(), 3);

        {
            let item = s.peek_mut();
            if let Some(top) = item {
                *top = 4;
            }
        }

        let item = s.peek();
        assert_eq!(item.unwrap(), &4);
        assert_eq!(s.size(), 3);
    }

    #[test]
    fn iter() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);

        let mut iter = s.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));

        let sum1: i32 = s.iter().sum();
        assert_eq!(sum1, 6);

        for item in s.iter_mut() {
            *item += 1;
        }

        let sum2: i32 = s.iter().sum();
        assert_eq!(sum2, 9);

        assert_eq!(s.info_iter().sum::<i32>(), 9);
    }
}
