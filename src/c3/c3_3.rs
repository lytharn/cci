pub struct SetOfStacks<T> {
    elements_in_stack: usize,
    data: Vec<Vec<T>>,
    stack: Option<usize>,
}

#[allow(dead_code)]
impl<T> SetOfStacks<T> {
    pub fn new(elements_in_stack: usize) -> Self {
        Self {
            elements_in_stack,
            data: vec![],
            stack: None,
        }
    }

    pub fn push(&mut self, value: T) {
        match self.stack {
            Some(ref mut s) if self.data[*s].len() == self.elements_in_stack => {
                self.data.push(vec![value]);
                *s += 1;
            }
            Some(s) => self.data[s].push(value),
            None => {
                self.data.push(vec![value]);
                self.stack = Some(0);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let stack = self.data.get_mut(self.stack?)?;
        if stack.len() == 1 {
            if self.stack == Some(0) {
                self.stack = None;
            } else {
                self.stack.as_mut().map(|x| *x -= 1);
            }
            self.data.pop()?.pop()
        } else {
            stack.pop()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_work_as_a_stack() {
        let mut stack = SetOfStacks::new(2);
        assert_eq!(stack.pop(), None);
        stack.push(1);
        assert_eq!(stack.pop(), Some(1));
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        stack.push(3);
        stack.push(4);
        assert_eq!(stack.pop(), Some(4));
        stack.push(5);
        stack.push(6);
        stack.push(7);
        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.pop(), Some(6));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), None);
    }
}
