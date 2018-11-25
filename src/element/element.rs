use std::any::Any;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::convert::Into;
use crate::Context;
use crate::render::UIRenderObj;
use crate::util::fill::Fill;
use crate::layout::Area;
use super::{Filter, FilterStack};

pub trait UIElement: Any + UIElementUpcast {
    fn open<'a>(&'a mut self, max_area: Area) -> (&'a mut Fill<UIRenderObj>, Area);

    fn close(self: Box<Self>, ctx: &mut Context);
}

pub trait UIElementUpcast {
    fn upcast(self: Box<Self>) -> Box<UIElement>;
}

impl<T: UIElement> UIElementUpcast for T {
    fn upcast(self: Box<Self>) -> Box<UIElement> {
        self
    }
}

pub trait IntoUIElement {
    type Target: UIElement;
}

impl<T: UIElement> IntoUIElement for T {
    type Target = Self;
}

pub trait IntoObj: IntoUIElement {
    fn into_obj(self, id: Id) -> UIElementObj<Self::Target>;
}

impl<T> IntoObj for T where
    T: IntoUIElement + Into<<T as IntoUIElement>::Target>
{
    fn into_obj(self, id: Id) -> UIElementObj<T::Target> {
        UIElementObj::new(id, Box::new(self.into()))
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct Id(u64);

impl Id {
    pub fn str(id: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);

        Id(hasher.finish())
    }

    pub fn num(id: u64) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);

        Id(hasher.finish())
    }

    pub fn append(self, id: Id) -> Self {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        id.hash(&mut hasher);

        Id(hasher.finish())
    }

    pub fn append_str(self, id: &str) -> Self {
        self.append(Id::str(id))
    }

    pub fn append_num(self, id: u64) -> Self {
        self.append(Id::num(id))
    }
}

pub struct UIElementObj<T: ?Sized + UIElement = dyn UIElement> {
    pub id: Id,
    pub element: Box<T>,
    pub filter_stack: FilterStack,
}

impl<T: UIElement> UIElementObj<T> {
    pub fn new(id: Id, element: Box<T>) -> Self {
        UIElementObj {
            id,
            element,
            filter_stack: FilterStack::new(),
        }
    }
}

impl<T: ?Sized + UIElement> UIElementObj<T> {
    pub fn attach_filter_pre(&mut self, filter: Rc<Filter>) {
        self.filter_stack.add_filter_pre(filter);
    }

    pub fn attach_filter_post(&mut self, filter: Rc<Filter>) {
        self.filter_stack.add_filter_post(filter);
    }

    pub fn upcast(self) -> UIElementObj<UIElement> {
        UIElementObj {
            id: self.id,
            element: self.element.upcast(),
            filter_stack: self.filter_stack,
        }
    }

    pub fn push(self, ctx: &mut Context) -> &mut Context {
        ctx.push(self.upcast());
        ctx
    }
}
