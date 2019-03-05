use std::any::Any;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter, Error};
use crate::Context;

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

    pub fn begin<'a, 'ui, 'ctx>(
        self,
        ctx: &'a mut Context<'ui, 'ctx>,
    ) -> &'a mut Context<'ui, 'ctx> {
        ctx.begin_widget(self.upcast());
        ctx
    }
}

pub trait UIWidgetImpl: Sized + Clone + Any {
    fn run<'ui, 'ctx>(
        self,
        ctx: &mut Context<'ui, 'ctx>,
    );

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
}

pub trait DynUIWidgetImpl {
    fn box_clone(
        &self
    ) -> Box<dyn DynUIWidgetImpl>;

    fn box_run<'ui, 'ctx>(
        self: Box<Self>,
        ctx: &mut Context<'ui, 'ctx>,
    );

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

    fn box_run<'ui, 'ctx>(
        self: Box<Self>,
        ctx: &mut Context<'ui, 'ctx>,
    ) {
        self.run(ctx)
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

impl UIWidgetImpl for Box<dyn DynUIWidgetImpl> {
    fn run<'ui, 'ctx>(
        self,
        ctx: &mut Context<'ui, 'ctx>,
    ) {
        self.box_run(ctx);
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
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(fmt)
    }
}
