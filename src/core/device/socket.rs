use crate::space::{Space, Region};
use std::rc::Rc;
use std::cell::Cell;

pub struct SocketLayout<S: Space> {
    region: Cell<Option<Region<S>>>,
    min_size: Cell<S::Size>,
    receivers: Cell<Vec<Box<dyn LayoutReceiver<S>>>>,
}

// Sockets are what you can push things into.
// They are use-once by default, but you can create shared sockets which may be used multiple times.
pub struct Socket<S: Space> {
    layout: Rc<SocketLayout<S>>,
    receiver: Rc<Cell<Option<
}

// SocketSource is what is given to a device so that it can perform layout.
pub struct SocketSource<S: Space> {
    space: Rc<Cell<Option<Region<S>>>>,
}
