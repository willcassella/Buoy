#[allow(clippy::len_without_is_empty)]
pub trait Array {
    type Item;

    fn len(&self) -> usize;
    unsafe fn set_len(&mut self, len: usize);
    unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut Self::Item;
}

impl<T> Array for Vec<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    unsafe fn set_len(&mut self, len: usize) {
        self.set_len(len);
    }

    unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut Self::Item {
        self.get_unchecked_mut(index)
    }
}

pub struct Iter<'a, A: Array, F> {
    array: &'a mut A,
    read_index: usize,
    write_index: usize,
    len: usize,
    filter: F,
}

impl<'a, A: Array, F> Drop for Iter<'a, A, F> {
    fn drop(&mut self) {
        unsafe {
            // Iterator may not have run to completion, so shift remaining elements forwards in array\
            std::ptr::copy(
                self.array.get_mut_unchecked(self.read_index),
                self.array.get_mut_unchecked(self.write_index),
                self.len - self.read_index,
            );
            let new_len = self.len - (self.read_index - self.write_index);
            self.array.set_len(new_len);
        }
    }
}

impl<'a, A, F> Iterator for Iter<'a, A, F>
where
    A: Array,
    F: FnMut(&mut A::Item) -> bool,
{
    type Item = A::Item;
    fn next(&mut self) -> Option<A::Item> {
        while self.read_index < self.len {
            unsafe {
                let item = self.array.get_mut_unchecked(self.read_index);
                if (self.filter)(item) {
                    self.read_index += 1;
                    return Some(std::ptr::read(item));
                } else {
                    // TODO(perf): Check if it's faster to use std::ptr::copy_nonoverlapping inside if
                    std::ptr::copy(item, self.array.get_mut_unchecked(self.write_index), 1);
                    self.read_index += 1;
                    self.write_index += 1;
                }
            }
        }

        None
    }
}

pub trait DrainFilter: Sized + Array {
    fn buoy_drain_filter<F: FnMut(&mut Self::Item) -> bool>(&mut self, filter: F) -> Iter<Self, F>;
}

impl<T: Array> DrainFilter for T {
    fn buoy_drain_filter<F: FnMut(&mut Self::Item) -> bool>(&mut self, filter: F) -> Iter<Self, F> {
        let len = self.len();
        unsafe {
            self.set_len(0);
        }
        Iter {
            array: self,
            read_index: 0,
            write_index: 0,
            len,
            filter,
        }
    }
}
