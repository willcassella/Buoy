use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::convert::From;
use crate::util::fill::Fill;
use crate::Context;
use crate::element::UIElement;
use crate::render::UIRenderObj;
use crate::layout::Area;

pub trait Panel: Any {
    fn open(&self, max_area: Area) -> Area;

    fn close(self, ctx: &mut Context, children: Vec<UIRenderObj>);
}

pub struct PanelObj<T: Panel> {
    children: Vec<UIRenderObj>,
    panel: T,
}

impl<T: Panel> From<T> for PanelObj<T> {
    fn from(panel: T) -> Self {
        PanelObj {
            children: Vec::new(),
            panel,
        }
    }
}

impl<T: Panel> UIElement for PanelObj<T> {
    fn open<'a>(&'a mut self, max_area: Area) -> (&'a mut Fill<UIRenderObj>, Area) {
        (&mut self.children, self.panel.open(max_area))
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        let this = *self;
        this.panel.close(ctx, this.children);
    }
}

impl<T: Panel> Deref for PanelObj<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.panel
    }
}

impl<T: Panel> DerefMut for PanelObj<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.panel
    }
}

pub trait Widget: Any {
    fn open(&self, max_area: Area) -> Area {
        max_area
    }

    fn close_some(self, ctx: &mut Context, child: UIRenderObj);

    fn close_none(self, ctx: &mut Context);
}

pub struct WidgetObj<T: Widget> {
    child: Option<UIRenderObj>,
    widget: T,
}

impl<T: Widget> From<T> for WidgetObj<T> {
    fn from(widget: T) -> Self {
        WidgetObj {
            child: None,
            widget,
        }
    }
}

impl<T: Widget> UIElement for WidgetObj<T> {
    fn open<'a>(&'a mut self, max_area: Area) -> (&'a mut Fill<UIRenderObj>, Area) {
        (&mut self.child, self.widget.open(max_area))
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        // Moving out of box to work around issue with destructuring and boxes
        let this = *self;

        match this.child {
            Some(child) => this.widget.close_some(ctx, child),
            None => this.widget.close_none(ctx),
        }
    }
}

impl<T: Widget> Deref for WidgetObj<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.widget
    }
}

impl<T: Widget> DerefMut for WidgetObj<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.widget
    }
}

pub trait Stub: Any {
    fn generate(self, ctx: &mut Context);
}

pub struct StubObj<T: Stub>(pub T, ());

impl<T: Stub> From<T> for StubObj<T> {
    fn from(stub: T) -> Self {
        StubObj(stub, ())
    }
}

impl<T: Stub> UIElement for StubObj<T> {
    fn open<'a>(&'a mut self, max_area: Area) -> (&'a mut Fill<UIRenderObj>, Area) {
        (&mut self.1, max_area)
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        self.0.generate(ctx);
    }
}

impl<T: Stub> Deref for StubObj<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Stub> DerefMut for StubObj<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
