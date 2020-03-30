use std::any::Any;
use std::rc::Rc;

use crate::core::element::*;
use crate::core::id::Id;

pub trait Filter {
    fn predicate(&self, _element: &dyn Any, _id: Id) -> PredicateResult {
        PredicateResult::PassRecurse
    }

    fn run<'ctx, 'frm>(&self, elem: Elem<'frm>, ctx: Context<'ctx, 'frm>) -> Output<'frm>;
}

pub trait TypedFilter {
    type Element: Element;

    fn predicate(&self, _element: &Self::Element, _id: Id) -> PredicateResult {
        PredicateResult::RunFilter
    }

    fn run<'ctx, 'frm>(
        &self,
        elem: Elem<'frm, Self::Element>,
        ctx: Context<'ctx, 'frm>,
    ) -> Output<'frm>;
}

impl<T: TypedFilter> Filter for T {
    fn predicate(&self, element: &dyn Any, id: Id) -> PredicateResult {
        match element.downcast_ref::<T::Element>() {
            Some(element) => <Self as TypedFilter>::predicate(self, element, id),
            None => PredicateResult::PassRecurse,
        }
    }

    fn run<'ctx, 'frm>(&self, elem: Elem<'frm>, ctx: Context<'ctx, 'frm>) -> Output<'frm> {
        let elem = Elem {
            id: elem.id,
            data: elem.data.downcast::<T::Element>().ok().unwrap(),
        };

        <Self as TypedFilter>::run(self, elem, ctx)
    }
}

pub struct Output<'frm> {
    pub layout: LayoutNode<'frm>,
    pub next: Option<Rc<dyn Filter>>,
}

pub enum PredicateResult {
    RunFilter,
    Pass,
    PassRecurse,
}

struct FilterNode {
    filter: Rc<dyn Filter>,
    parent: Option<Rc<FilterNode>>,
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
            }
            None => None,
        }
    }

    pub fn run<'ctx, 'frm>(
        &mut self,
        elem: Elem<'frm>,
        _ctx: Context<'ctx, 'frm>,
    ) -> Option<Output<'frm>> {
        let mut inner_stack = FilterStackMut::default();

        while let Some(filter) = self.head.take() {
            self.head = filter.parent.clone();

            match filter.filter.predicate((*elem.data).upcast(), elem.id) {
                PredicateResult::Pass => continue,
                PredicateResult::PassRecurse => inner_stack.append(filter.filter.clone()),
                PredicateResult::RunFilter => unimplemented!(),
            }
        }

        unimplemented!()
    }
}

pub struct FilterStackMut {
    head: Option<Rc<FilterNode>>,
    tail: *mut FilterNode,
}

impl FilterStackMut {
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

    pub fn append_stack(self, stack: FilterStack) -> FilterStack {
        if !self.tail.is_null() {
            unsafe {
                std::ptr::write(&mut (*self.tail).parent, stack.head);
            }
            FilterStack { head: self.head }
        } else {
            stack
        }
    }

    pub fn append_stack_mut(&mut self, stack: FilterStackMut) {
        if !self.tail.is_null() {
            if !stack.tail.is_null() {
                unsafe {
                    std::ptr::write(&mut (*self.tail).parent, stack.head);
                }
                self.tail = stack.tail;
            }
        } else {
            *self = stack;
        }
    }

    pub fn share(self) -> FilterStack {
        FilterStack { head: self.head }
    }
}

impl Default for FilterStackMut {
    fn default() -> Self {
        FilterStackMut {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }
}
