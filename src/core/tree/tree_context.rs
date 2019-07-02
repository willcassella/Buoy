use std::rc::Rc;

use crate::util::fill::Fill;
use crate::space::*;
use crate::input::*;
use crate::core::element::*;
use crate::core::tree::*;
use crate::core::filter::*;
use crate::core::common::*;

pub struct TreeContext<'window, 'ctx> {
    pub(crate) socket: &'ctx mut dyn Socket,
    pub(crate) max_area: Area,

    pub(crate) outer: &'ctx mut TreeContext<'window, 'ctx>,

    pub(crate) prev_input: &'window InputCache,
    pub(crate) global_data: &'window mut GlobalData,
}

impl<'window, 'ctx> TreeContext<'window, 'ctx> {
    // Returns the remaining number of spaces in the current socket
    pub fn remaining_capacity(&self) -> usize {
        self.socket.remaining_capacity()
    }

    // Evaluates an element at this point in the tree
    // id: Id of the element
    pub fn element(
        &mut self,
        id: Id,
        element: &dyn Element,
        sub_provider: &mut dyn TreeProvider,
    ) {
        let mut layout = None;

        // Create a new element context
        let ctx = Context {
            tree_provider: sub_provider,
            out_layout: &mut layout,
            element_id: id,
            max_area: self.max_area,
            prev_input: self.prev_input,
            global_data: self.global_data
        };

        element.run(ctx);

        if let Some(layout) = layout {
            self.socket.push(layout);
        }
    }

    // Allows this TreeProvider to delegate to the parent TreeProvider
    // Opens a socket for the external element
    // Problem: What if you don't want to open a super socket here?
    // Is that a possibility?
    // Otherwise, can you say you trust the evaluation of subcontexts completely?
    pub fn super_socket(
        &mut self,
        name: SocketName,
        socket: &mut dyn Socket,
        child_max_area: Area,
    ) {

    }
}
