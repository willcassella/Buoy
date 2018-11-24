use std::usize;

pub trait Fill<T> {
    fn remaining_capacity(&self) -> usize;

    fn push(&mut self, item: T);
}

impl<T> Fill<T> for Vec<T> {
    fn remaining_capacity(&self) -> usize {
        usize::MAX
    }

    fn push(&mut self, item: T) {
        self.push(item);
    }
}

impl<T> Fill<T> for Option<T> {
    fn remaining_capacity(&self) -> usize {
        if self.is_some() {
            0_usize
        } else {
            1_usize
        }
    }

    fn push(&mut self, item: T) {
        if self.is_some() {
            panic!("push called beyond capacity");
        }

        *self = Some(item);
    }
}

impl<T> Fill<T> for () {
    fn remaining_capacity(&self) -> usize {
        0
    }

    fn push(&mut self, _item: T) {
        panic!("Push called beyond capacity");
    }
}

pub struct Limit<F> {
    fill: F,
    limit: usize,
}

pub fn limit<F>(fill: F, limit: usize) -> Limit<F> {
    Limit {
        fill,
        limit,
    }
}

impl<T, F: Fill<T>> Fill<T> for Limit<F> {
    fn remaining_capacity(&self) -> usize {
        self.limit.min(self.fill.remaining_capacity())
    }

    fn push(&mut self, item: T) {
        if self.limit == 0 {
            panic!("Push called beyond capacity");
        }

        self.limit -= 1;
        self.fill.push(item);
    }
}