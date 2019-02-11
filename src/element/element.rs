use std::any::Any;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::Context;
use crate::layout::Area;
use super::{Filter, FilterStack, UISocket};

pub struct UIElement<I: ?Sized + UIElementImpl = dyn UIElementImpl> {
    pub id: Id,
    pub filter_stack: FilterStack,
    pub imp: Box<I>,
}

impl<I: UIElementImpl> UIElement<I> {
    pub fn new(id: Id, imp: Box<I>) -> Self {
        UIElement {
            id,
            filter_stack: FilterStack::new(),
            imp,
        }
    }
}

impl<I: ?Sized + UIElementImpl> UIElement<I> {
    pub fn attach_filter_pre(&mut self, filter: Rc<Filter>) {
        self.filter_stack.add_filter_pre(filter);
    }

    pub fn attach_filter_post(&mut self, filter: Rc<Filter>) {
        self.filter_stack.add_filter_post(filter);
    }

    pub fn downcast<D: Sized + UIElementImpl>(self) -> Result<UIElement<D>, UIElement<I>> {
        unimplemented!()
    }

    pub fn upcast(self) -> UIElement<dyn UIElementImpl> {
        UIElement {
            id: self.id,
            filter_stack: self.filter_stack,
            imp: self.imp.upcast(),
        }
    }

    pub fn push<'a, 'ui>(self, ctx: &'a mut Context<'ui>) -> &'a mut Context<'ui> {
        ctx.push(self.upcast());
        ctx
    }
}

pub trait UIElementImpl: UIElementUtil + Any {
    fn open(
        self: Box<Self>,
        max_area: Area
    ) -> UISocket;
}

pub trait UIElementUtil {
    fn box_cone(&self) -> Box<dyn UIElementImpl>;

    fn upcast(self: Box<Self>) -> Box<dyn UIElementImpl>;
}

impl<T: UIElementImpl + Clone> UIElementUtil for T {
    fn box_cone(&self) -> Box<dyn UIElementImpl> {
        Box::new(self.clone())
    }

    fn upcast(self: Box<Self>) -> Box<dyn UIElementImpl> {
        self
    }
}

pub trait IntoUIElement {
    type Target: UIElementImpl;
}

impl<T: UIElementImpl> IntoUIElement for T {
    type Target = Self;
}

pub trait IntoObj: IntoUIElement {
    fn into_obj(self, id: Id) -> UIElement<Self::Target>;
}

impl<T> IntoObj for T where
    T: IntoUIElement + Into<<T as IntoUIElement>::Target>
{
    fn into_obj(self, id: Id) -> UIElement<T::Target> {
        UIElement::new(id, Box::new(self.into()))
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
