use std::rc::Rc;
use std::ops::Deref;

use crate::core::element::*;
use crate::util::linked_buffer::LBBox;

struct FilterNode {
    filter: Rc<dyn Filter>,
    parent: Option<Rc<FilterNode>>,
}

impl Deref for FilterNode {
    type Target = dyn Filter;

    fn deref(&self) -> &(dyn Filter + 'static) {
        &*self.filter
    }
}

#[derive(Clone, Default)]
pub struct FilterStack {
    head: Option<Rc<FilterNode>>,
}

impl FilterStack {
    pub fn append(&mut self, filter: Rc<dyn Filter>) {
        let head = FilterNode {
            filter,
            parent: self.head.take(),
        };
        self.head = Some(Rc::new(head));
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn pop(&mut self) -> Option<Rc<dyn Filter>> {
        match self.head.take() {
            Some(node) => {
                self.head = node.parent.clone();
                Some(node.filter.clone())
            },
            None => None,
        }
    }
}

pub struct FilterStackBuilder {
    head: Option<Rc<FilterNode>>,
    tail: *mut FilterNode,
}

impl FilterStackBuilder {
    pub fn append(&mut self, filter: Rc<dyn Filter>) {
        let head = Rc::new(FilterNode {
            filter,
            parent: self.head.take(),
        });

        if self.tail.is_null() {
            self.tail = (&*head as *const FilterNode) as *mut FilterNode;
        }

        self.head = Some(head);
    }

    pub fn append_to(self, stack: FilterStack) -> FilterStack {
        if !self.tail.is_null() {
            unsafe { std::ptr::write(&mut (*self.tail).parent, stack.head) }
            FilterStack { head: self.head }
        } else {
            stack
        }
    }

    pub fn into_stack(self) -> FilterStack {
        FilterStack { head: self.head }
    }
}

impl Default for FilterStackBuilder {
    fn default() -> Self {
        FilterStackBuilder {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }
}

pub enum PredicateResult {
    RunFilter,
    Pass,
    PassRecurse,
}

pub trait Filter {
    fn predicate(
        &self,
        _id: Id,
        _element: &dyn Element
    ) -> PredicateResult {
        PredicateResult::PassRecurse
    }

    fn element<'ctx, 'frm>(
        &self,
        mut ctx: Context<'ctx, 'frm>,
        id: Id,
        element: LBBox<'frm, dyn Element>,
    ) -> LayoutNode<'frm> {
        // Default implementation just uses the element as a sub-element (no-op)
        let mut sub = ctx.open_sub(ctx.max_area(), id, element);
        sub.connect_all_sockets();
        sub.close()
    }
}