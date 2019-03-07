use std::any::Any;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{self, Display, Debug, Formatter};
use crate::Context;
use crate::element::UISocket;
use crate::builder::BuilderContext;

#[derive(Clone)]
pub struct UIWidget<I: UIWidgetImpl = Box<dyn DynUIWidgetImpl>> {
    pub id: Id,
    pub imp: I,
}

impl<I: UIWidgetImpl> UIWidget<I> {
    pub fn new(id: Id, imp: I) -> Self {
        UIWidget {
            id,
            imp,
        }
    }

    pub fn upcast(
        self,
    ) -> UIWidget<Box<dyn DynUIWidgetImpl>> {
        UIWidget {
            id: self.id,
            imp: self.imp.upcast(),
        }
    }

    pub fn downcast<D: UIWidgetImpl>(
        self,
    ) -> Result<UIWidget<D>, UIWidget<I>> {
        match self.imp.downcast::<D>() {
            Ok(d) => Ok(UIWidget{ id: self.id, imp: d }),
            Err(i) => Err(UIWidget{ id: self.id, imp: i }),
        }
    }

    pub fn begin<'a, 'b, 'ctx>(
        self,
        builder: &'a mut BuilderContext<'b, 'ctx>,
    ) -> &'a mut BuilderContext<'b, 'ctx> {
        builder.element_begin(self);
        builder
    }
}

pub trait UIWidgetImpl: Sized + Clone + Any {
    type Next: UIWidgetImpl;

    fn run(
        self,
        ctx: &mut Context,
        socket: &mut dyn UISocket,
    ) -> Option<Self::Next>;

    fn upcast(
        self,
    ) -> Box<dyn DynUIWidgetImpl> {
        Box::new(self)
    }

    fn downcast<D: UIWidgetImpl>(
        self,
    ) -> Result<D, Self> {
        Err(self) // TODO: This should handle when Self == D
    }

    // TODO: This doesn't belong here (should be an extension trait or something)
    fn into_obj(
        self,
        id: Id,
    ) -> UIWidget<Self> {
        UIWidget::new(id, self)
    }
}

impl UIWidgetImpl for () {
    type Next = ();

    fn run(
        self,
        _ctx: &mut Context,
        _socket: &mut dyn UISocket,
    ) -> Option<Self::Next> {
        None
    }
}

pub trait DynUIWidgetImpl {
    fn box_clone(
        &self
    ) -> Box<dyn DynUIWidgetImpl>;

    fn box_run(
        self: Box<Self>,
        ctx: &mut Context,
        socket: &mut dyn UISocket,
    ) -> Option<Box<dyn DynUIWidgetImpl>>;

    fn into_any_mut(
        &mut self,
    ) -> &mut Any;
}

impl<T: UIWidgetImpl> DynUIWidgetImpl for T {
    fn box_clone(
        &self
    ) -> Box<dyn DynUIWidgetImpl> {
        Box::new(self.clone())
    }

    fn box_run(
        self: Box<Self>,
        ctx: &mut Context,
        socket: &mut dyn UISocket,
    ) -> Option<Box<dyn DynUIWidgetImpl>> {
        let next = self.run(ctx, socket);
        next.map(|x| Box::new(x) as Box<dyn DynUIWidgetImpl>)
    }

    fn into_any_mut(
        &mut self,
    ) -> &mut Any {
        self
    }
}

impl Clone for Box<dyn DynUIWidgetImpl> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Debug for Box<dyn DynUIWidgetImpl> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        Debug::fmt(&*self, fmt)
    }
}

impl UIWidgetImpl for Box<dyn DynUIWidgetImpl> {
    type Next = Self;

    fn run(
        self,
        ctx: &mut Context,
        socket: &mut dyn UISocket,
    ) -> Option<Self> {
        self.box_run(ctx, socket)
    }

    fn upcast(
        self,
    ) -> Box<dyn DynUIWidgetImpl> {
        self
    }

    fn downcast<D: UIWidgetImpl>(
        self,
    ) -> Result<D, Self> {
        let raw = Box::into_raw(self);

        unsafe {
            match (*raw).into_any_mut().downcast_mut::<D>() {
                Some(v) => Ok(*Box::from_raw(v)),
                None => Err(Box::from_raw(raw))
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
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

impl Display for Id {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, fmt)
    }
}
