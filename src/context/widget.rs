use std::any::Any;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::convert::Into;
use crate::{Context, ElementObj};
use crate::util::fill::Fill;
use crate::layout::Area;

pub trait Widget: Any + WidgetUpcast {
    fn open<'a>(&'a mut self, self_bounds: Area) -> (&'a mut Fill<ElementObj>, Area);

    fn close(self: Box<Self>, ctx: &mut Context);
}

pub trait WidgetUpcast {
    fn upcast(self: Box<Self>) -> Box<Widget>;
}

impl<T: Widget> WidgetUpcast for T {
    fn upcast(self: Box<Self>) -> Box<Widget> {
        self
    }
}

pub trait Filter {
    fn filter(&self, alias: &Rc<Filter>, ctx: &mut Context, mut widget_obj: WidgetObj) {
        widget_obj.attach_filter_post(alias.clone());
        ctx.push_widget(widget_obj);
            ctx.children();
        ctx.pop();
    }
}

pub trait WidgetType {
    type Target: Widget;
}

impl<T: Widget> WidgetType for T {
    type Target = Self;
}

pub trait IntoObj: WidgetType {
    fn into_obj(self, id: WidgetId) -> WidgetObj<Self::Target>;
}

impl<T> IntoObj for T where T: WidgetType + Into<<T as WidgetType>::Target> {
    fn into_obj(self, id: WidgetId) -> WidgetObj<T::Target> {
        WidgetObj::new(id, Box::new(self.into()))
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct WidgetId(u64);

impl WidgetId {
    pub fn str(id: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);

        WidgetId(hasher.finish())
    }

    pub fn num(id: u64) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);

        WidgetId(hasher.finish())
    }

    pub fn suffix(prefix: WidgetId, id: WidgetId) -> Self {
        let mut hasher = DefaultHasher::new();
        prefix.hash(&mut hasher);
        id.hash(&mut hasher);

        WidgetId(hasher.finish())
    }

    pub fn suffix_str(prefix: WidgetId, id: &str) -> Self {
        WidgetId::suffix(prefix, WidgetId::str(id))
    }

    pub fn suffix_num(prefix: WidgetId, id: u64) -> Self {
        WidgetId::suffix(prefix, WidgetId::num(id))
    }
}

#[derive(Clone, Default)]
pub struct FilterStack(pub Vec<Rc<Filter>>);

impl FilterStack {
    pub fn new() -> Self {
        FilterStack(Vec::new())
    }

    pub fn add_filter_pre(&mut self, filter: Rc<Filter>) {
        self.0.insert(0, filter);
    }

    pub fn add_filter_post(&mut self, filter: Rc<Filter>) {
        self.0.push(filter);
    }
}

pub struct WidgetObj<T: ?Sized + Widget = dyn Widget> {
    pub id: WidgetId,
    pub widget: Box<T>,
    pub filter_stack: FilterStack,
}

impl<T: Widget> WidgetObj<T> {
    pub fn new(id: WidgetId, widget: Box<T>) -> Self {
        WidgetObj {
            id,
            widget,
            filter_stack: FilterStack::new(),
        }
    }
}

impl<T: ?Sized + Widget> WidgetObj<T> {
    pub fn attach_filter_pre(&mut self, filter: Rc<Filter>) {
        self.filter_stack.add_filter_pre(filter);
    }

    pub fn attach_filter_post(&mut self, filter: Rc<Filter>) {
        self.filter_stack.add_filter_post(filter);
    }

    pub fn erase(self) -> WidgetObj<Widget> {
        WidgetObj {
            id: self.id,
            widget: self.widget.upcast(),
            filter_stack: self.filter_stack,
        }
    }

    pub fn push(self, ctx: &mut Context) {
        ctx.push_widget(self.erase());
    }
}
