pub struct MinStack<T> {
    data: Vec<T>,
    min_indeces: Vec<usize>,
}

#[allow(dead_code)]
impl<T: PartialOrd> MinStack<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            min_indeces: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        let min_index = match self.min_indeces.last() {
            Some(&i) if value < self.data[i] => self.data.len(),
            Some(&i) => i,
            None => 0,
        };
        self.data.push(value);
        self.min_indeces.push(min_index);
    }

    pub fn pop(&mut self) {
        self.data.pop();
        self.min_indeces.pop();
    }

    pub fn min(&self) -> Option<&T> {
        self.min_indeces.last().map(|&i| self.data.get(i)).flatten()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_get_minimum_value() {
        let mut stack = MinStack::new();
        assert_eq!(stack.min(), None);
        stack.push(5);
        assert_eq!(stack.min(), Some(&5));
        stack.push(4);
        assert_eq!(stack.min(), Some(&4));
        stack.push(6);
        assert_eq!(stack.min(), Some(&4));
        stack.pop();
        assert_eq!(stack.min(), Some(&4));
        stack.pop();
        assert_eq!(stack.min(), Some(&5));
        stack.pop();
        assert_eq!(stack.min(), None);
    }
}
