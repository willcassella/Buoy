mod context;

mod tree;
pub use self::tree::{Filter, Generator, Socket, Element, NullElement};

pub mod widgets;
pub mod commands;
pub mod layout;
pub mod color;
pub mod ffi;

#[cfg(test)]
mod tests {
    use ffi;

    #[test]
    fn it_works() {
        let primitives = ffi::buoy_get_primitives(800_f32, 600_f32);
        ffi::buoy_free_primitives(primitives);
    }
}
