use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter, Error};
use crate::Context;
use crate::util::cast::{IntoAny, Downcast};

pub struct UIElement<I: ?Sized + UIElementImpl = dyn UIElementImpl> {
    pub id: Id,
    pub imp: Box<I>,
}

impl<I: UIElementImpl> UIElement<I> {
    pub fn new(id: Id, imp: Box<I>) -> Self {
        UIElement {
            id,
            imp,
        }
    }
}

impl<I: ?Sized + UIElementImpl> UIElement<I> {
    pub fn downcast<D: Sized + UIElementImpl>(
        self,
    ) -> Result<UIElement<D>, UIElement<I>> {
        match Downcast::<D>::downcast(self.imp) {
            Ok(d) => Ok(UIElement{ id: self.id, imp: d }),
            Err(i) => Err(UIElement{ id: self.id, imp: i }),
        }
    }

    pub fn upcast(
        self
    ) -> UIElement<dyn UIElementImpl> {
        UIElement {
            id: self.id,
            imp: self.imp.upcast(),
        }
    }

    pub fn begin<'ui, 'ctx, 'a>(
        self,
        ctx: &'a mut Context<'ui, 'ctx>
    ) -> &'a mut Context<'ui, 'ctx> {
        ctx.element_begin(self.upcast());
        ctx
    }
}

pub trait UIElementImpl: UIElementUtil + IntoAny {
    fn run<'ui, 'ctx>(
        self: Box<Self>,
        ctx: &mut Context<'ui, 'ctx>, // TODO: Investigate if this can be extended to arbitrary depths
    );
}

pub trait UIElementUtil {
    fn box_clone(&self) -> Box<dyn UIElementImpl>;

    fn upcast(self: Box<Self>) -> Box<dyn UIElementImpl>;
}

impl<T: UIElementImpl + Clone> UIElementUtil for T {
    fn box_clone(&self) -> Box<dyn UIElementImpl> {
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
