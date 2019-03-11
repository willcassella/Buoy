use crate::core::element::*;
use crate::core::tree::*;

pub trait TreeProvider {
    // TODO: This should return a more descriptive enum instead of true/false
    fn socket(
        &mut self,
        ctx: &mut TreeContext,
        name: SocketName,
    ) -> bool;
}

impl TreeProvider for Box<dyn TreeProvider> {
    fn socket(
        &mut self,
        ctx: &mut TreeContext,
        name: SocketName,
    ) -> bool {
        self.as_mut().socket(ctx, name)
    }
}

impl TreeProvider for () {
    fn socket(
        &mut self,
        _ctx: &mut TreeContext,
        _name: SocketName,
    ) -> bool {
        // Do nothing
        false
    }
}
