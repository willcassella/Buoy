use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter, Error};
use crate::Context;
use crate::util::cast::{IntoAny, Downcast};

pub struct UIWidget<I: ?Sized + UIWidgetImpl = dyn UIWidgetImpl> {
    pub id: Id,
    pub imp: Box<I>,
}

impl<I: UIWidgetImpl> UIWidget<I> {
    pub fn new(id: Id, imp: Box<I>) -> Self {
        UIWidget {
            id,
            imp,
        }
    }
}

impl<I: ?Sized + UIWidgetImpl> UIWidget<I> {
    pub fn downcast<D: Sized + UIWidgetImpl>(
        self,
    ) -> Result<UIWidget<D>, UIWidget<I>> {
        match Downcast::<D>::downcast(self.imp) {
            Ok(d) => Ok(UIWidget{ id: self.id, imp: d }),
            Err(i) => Err(UIWidget{ id: self.id, imp: i }),
        }
    }

    pub fn upcast(
        self
    ) -> UIWidget<dyn UIWidgetImpl> {
        UIWidget {
            id: self.id,
            imp: self.imp.upcast(),
        }
    }

    pub fn begin<'ui, 'ctx, 'a>(
        self,
        ctx: &'a mut Context<'ui, 'ctx>
    ) -> &'a mut Context<'ui, 'ctx> {
        ctx.widget_begin(self.upcast());
        ctx
    }
}

pub trait UIWidgetImpl: UIWidgetUtil + IntoAny {
    fn run<'ui, 'ctx>(
        self: Box<Self>,
        ctx: &mut Context<'ui, 'ctx>,
    );
}

pub trait UIWidgetUtil {
    fn box_clone(&self) -> Box<dyn UIWidgetImpl>;

    fn upcast(self: Box<Self>) -> Box<dyn UIWidgetImpl>;
}

impl<T: UIWidgetImpl + Clone> UIWidgetUtil for T {
    fn box_clone(&self) -> Box<dyn UIWidgetImpl> {
        Box::new(self.clone())
    }

    fn upcast(self: Box<Self>) -> Box<dyn UIWidgetImpl> {
        self
    }
}

pub trait IntoUIWidget {
    type Target: UIWidgetImpl;
}

impl<T: UIWidgetImpl> IntoUIWidget for T {
    type Target = Self;
}

pub trait IntoObj: IntoUIWidget {
    fn into_obj(self, id: Id) -> UIWidget<Self::Target>;
}

impl<T> IntoObj for T where
    T: IntoUIWidget + Into<<T as IntoUIWidget>::Target>
{
    fn into_obj(self, id: Id) -> UIWidget<T::Target> {
        UIWidget::new(id, Box::new(self.into()))
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
