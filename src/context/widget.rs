use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::Context;
use crate::layout::{Area, Region};
use crate::commands::CommandList;

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

    pub fn new_str_id(id: &str, widget: Box<T>) -> Self {
        WidgetObj {
            id: WidgetId::str(id),
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
}

pub trait Widget {
    fn child_layout(&self, self_max: Area) -> (usize, Area);

    fn child_elements(self: Box<Self>, ctx: &mut Context, children: Vec<ElementObj>);
}

pub trait Wrapper: Widget {
    fn child_layout(&self, self_max: Area) -> Area {
        self_max
    }

    fn child_element(self: Box<Self>, ctx: &mut Context, child: ElementObj);

    fn close(self: Box<Self>, ctx: &mut Context);
}

impl<T: Wrapper> Widget for T {
    fn child_layout(&self, self_max: Area) -> (usize, Area) {
        (1, Wrapper::child_layout(self, self_max))
    }

    fn child_elements(self: Box<Self>, ctx: &mut Context, children: Vec<ElementObj>) {
        match children.into_iter().next() {
            Some(child) => Wrapper::child_element(self, ctx, child),
            None => Wrapper::close(self, ctx),
        }
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

pub trait Element {
    fn render(&self, region: Region, cmds: &mut CommandList);
}

pub struct ElementObj {
    pub min_area: Area,
    pub element: Box<Element>,
}

impl<T> Element for T where
    T: Fn(Region, &mut CommandList)
{
    fn render(&self, region: Region, cmds: &mut CommandList) {
        self(region, cmds);
    }
}

#[derive(Clone, Copy)]
pub struct NullElement;

impl Element for NullElement {
    fn render(&self, _region: Region, _cmds: &mut CommandList) {
        // Null elements only take up space
    }
}