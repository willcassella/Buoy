use crate::core::common::*;
use crate::core::element::*;
use crate::core::tree::*;

pub trait TreeProvider {
    // Provides the given socket with elements that appear at the given point in the tree
    fn socket<'window, 'ctx>(
        &mut self,
        ctx: TreeContext<'window, 'ctx>,
        name: SocketName,
    ) -> bool;
}

impl TreeProvider for () {
    fn socket<'window, 'ctx>(
        &mut self,
        _ctx: TreeContext<'window, 'ctx>,
        _name: SocketName,
    ) -> bool {
        // Do nothing
        false
    }
}
