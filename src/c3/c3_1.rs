pub struct ThreeStacks<T> {
    data: Vec<T>,
    next: [usize; 3],
    next_end: [usize; 3],
}

#[allow(dead_code)]
impl<T: Default> ThreeStacks<T> {
    pub fn new(size: usize) -> Self {
        let mut data: Vec<T> = Vec::new();
        data.resize_with(size, || T::default());
        let next1_end = (size + 2) / 3;
        let next2_end = next1_end + (size + 1) / 3;
        let next3_end = next2_end + size / 3;
        let next1 = 0;
        let next2 = next1_end;
        let next3 = next2_end;
        Self {
            data,
            next: [next1, next2, next3],
            next_end: [next1_end, next2_end, next3_end],
        }
    }

    pub fn push1(&mut self, value: T) {
        self.push(0, value);
    }

    pub fn push2(&mut self, value: T) {
        self.push(1, value);
    }

    pub fn push3(&mut self, value: T) {
        self.push(2, value);
    }

    fn push(&mut self, stack: usize, value: T) {
        if self.next[stack] == self.next_end[stack] {
            panic!("Cannot push full stack")
        }
        self.data[self.next[stack]] = value;
        self.next[stack] += 1;
    }

    pub fn pop1(&mut self) {
        self.pop(0);
    }

    pub fn pop2(&mut self) {
        self.pop(1);
    }

    pub fn pop3(&mut self) {
        self.pop(2);
    }

    fn pop(&mut self, stack: usize) {
        if self.is_empty(stack) {
            panic!("Cannot pop empty stack")
        }
        self.next[stack] -= 1;
    }

    pub fn peek1(&self) -> &T {
        self.peek(0)
    }

    pub fn peek2(&self) -> &T {
        self.peek(1)
    }

    pub fn peek3(&self) -> &T {
        self.peek(2)
    }

    fn peek(&self, stack: usize) -> &T {
        if self.is_empty(stack) {
            panic!("Cannot peek empty stack")
        }
        &self.data[self.next[stack] - 1]
    }

    pub fn is_empty1(&self) -> bool {
        self.is_empty(0)
    }

    pub fn is_empty2(&self) -> bool {
        self.is_empty(1)
    }

    pub fn is_empty3(&self) -> bool {
        self.is_empty(2)
    }

    fn is_empty(&self, stack: usize) -> bool {
        let start = if stack == 0 {
            0
        } else {
            self.next_end[stack - 1]
        };

        self.next[stack] == start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_push_pop_peek_and_check_empty() {
        let mut stack = ThreeStacks::new(9);
        assert!(stack.is_empty1());
        stack.push1(1);
        assert_eq!(stack.peek1(), &1);
        assert!(!stack.is_empty1());
        stack.push1(2);
        assert_eq!(stack.peek1(), &2);
        stack.push1(3);
        assert_eq!(stack.peek1(), &3);

        assert!(stack.is_empty2());
        stack.push2(3);
        assert_eq!(stack.peek2(), &3);
        assert!(!stack.is_empty2());
        stack.push2(4);
        assert_eq!(stack.peek2(), &4);
        stack.push2(5);
        assert_eq!(stack.peek2(), &5);

        assert!(stack.is_empty3());
        stack.push3(7);
        assert_eq!(stack.peek3(), &7);
        assert!(!stack.is_empty3());
        stack.push3(8);
        assert_eq!(stack.peek3(), &8);
        stack.push3(9);
        assert_eq!(stack.peek3(), &9);

        stack.pop1();
        assert_eq!(stack.peek1(), &2);
        stack.pop1();
        assert_eq!(stack.peek1(), &1);
        stack.pop1();
        assert!(stack.is_empty1());

        stack.pop2();
        assert_eq!(stack.peek2(), &4);
        stack.pop2();
        assert_eq!(stack.peek2(), &3);
        stack.pop2();
        assert!(stack.is_empty2());

        stack.pop3();
        assert_eq!(stack.peek3(), &8);
        stack.pop3();
        assert_eq!(stack.peek3(), &7);
        stack.pop3();
        assert!(stack.is_empty3());
    }

    #[test]
    #[should_panic]
    fn should_panic_if_try_to_push_to_a_full_stack1() {
        let mut stack = ThreeStacks::new(1);
        stack.push1(1);
        stack.push1(2);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_try_to_push_to_a_full_stack2() {
        let mut stack = ThreeStacks::new(2);
        stack.push2(1);
        stack.push2(2);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_try_to_push_to_a_full_stack3() {
        let mut stack = ThreeStacks::new(3);
        stack.push3(1);
        stack.push3(2);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_try_to_pop_empty_stack1() {
        let mut stack = ThreeStacks::new(1);
        stack.push1(1);
        stack.pop1();
        stack.pop1();
    }

    #[test]
    #[should_panic]
    fn should_panic_if_try_to_pop_empty_stack2() {
        let mut stack = ThreeStacks::new(2);
        stack.push2(1);
        stack.pop2();
        stack.pop2();
    }

    #[test]
    #[should_panic]
    fn should_panic_if_try_to_pop_empty_stack3() {
        let mut stack = ThreeStacks::new(3);
        stack.push3(1);
        stack.pop3();
        stack.pop3();
    }

    #[test]
    #[should_panic]
    fn should_panic_if_try_to_peek_empty_stack1() {
        let mut stack = ThreeStacks::new(1);
        stack.push1(1);
        stack.pop1();
        stack.peek1();
    }

    #[test]
    #[should_panic]
    fn should_panic_if_try_to_peek_empty_stack2() {
        let mut stack = ThreeStacks::new(2);
        stack.push2(1);
        stack.pop2();
        stack.peek2();
    }

    #[test]
    #[should_panic]
    fn should_panic_if_try_to_peek_empty_stack3() {
        let mut stack = ThreeStacks::new(3);
        stack.push3(1);
        stack.pop3();
        stack.peek3();
    }
}
