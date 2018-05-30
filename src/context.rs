use super::widget::{Bounds, Widget, WidgetHandler};
use super::layout::{Layout, LayoutHandler};

pub trait WidgetContext {
    // Moves the context upward to the parent of the current element
    // This will panic if the parent is not in scope for this context!
    fn pop(&mut self);

    // Sets the bounds for this point in the stack
    fn push_bounds(&mut self, bounds: Bounds);

    // Gets the bounds for this point in the stack
    fn get_bounds(&self) -> Bounds;

    // Pushes a widget as a child of whatever the current top of the stack is.
    fn push_widget(&mut self, template: Box<Widget>);

    fn push_layout(&mut self, layout: Box<Layout>);

    fn push_layout_handler(&mut self, layout_handler: Box<LayoutHandler>);

    fn anchor_child(&mut self, handler: Option<Box<WidgetHandler>>);

    fn anchor_children(&mut self, handler: Option<Box<WidgetHandler>>);
}

// struct LayoutElement {
//     pub layout: Box<Layout>,
//     pub children: Arc<Mutex<Vec<Box<Layout>>>>,
// }

// impl LayoutContext for LayoutElement {
//     fn set_area(&mut self, area: Area) {
//         self.children.as_ref().
//     }
// }

// pub struct LayoutContext {
//     pub(crate) area: Area,
//     pub(crate) layout: Box<Layout>,
//     pub(crate) children: Vec<LayoutElement>,
// }

// impl LayoutContext {
//     pub fn set_area(&mut self, area: Area) {
//         // Write something....
//     }
// }

// pub struct BoundsElement {
//     pub(crate) parent: Rc<BoundsContext>,
//     pub(crate) child_bounds: Vec<Bounds>,
// }

// pub struct BoundsContext {
//     pub(crate) parent: Rc<BoundsContext>,
//     pub(crate) child_bounds: Vec<Bounds>,

// }

// enum ElementType {
//     Widget,
// }

// pub struct WidgetElement {
//     pub widget: Box<Widget>,
//     pub children: Vec<WidgetElement>,
//     pub handler: Option<Rc<WidgetHandlerElement>>,
// }

// impl WidgetElement {
//     pub fn new(template: Box<Template>, handler: Option<Rc<TemplateHandlerElement>>) -> Self {
//         Self {
//             template,
//             children: Vec::new(),
//             handler,
//         }
//     }
// }

// pub struct WidgetHandlerElement {
//     pub handler: Box<WidgetHandler>,
//     pub parent: Option<Rc<WidgetHandlerElement>>,
// }

// impl TemplateHandlerElement {
//     pub fn new(handler: Box<TemplateHandler>, parent: Option<Rc<TemplateHandlerElement>>) -> Self {
//         Self {
//             handler,
//             parent
//         }
//     }
// }

// pub struct WidgetContext {
//     context_child_handler: // This is either a WidgetHandler or a SyncWidgetHandler
//     context_children: Vec<TemplateElement>,

//     build_stack: Vec<ElementType>,
//     build_widget_stack: Vec<WidgetElement>,
//     build_widget_handler: Option<Rc<WidgetHandlerElement>>,
// }

// impl WidgetContex {
//     pub fn yield_child(&mut self, handler: Option<Box<SyncWidgetHandler>>) {
//         // Need to add first child as a child of the current parent
//         if let Some(parent) = self.build_widget_stack.last_mut() {
            
//         }
//     }
// }

// impl Context {
//     pub fn new() -> Self {
//         Self {
//             context_children: Vec::new(),
//             context_next: Vec::new(),

//             build_stack: Vec::new(),
//             build_template_stack: Vec::new(),
//             build_template_handler: None,
//         }
//     }

//     pub fn pop(&mut self) {
//         let elem_type = self.build_stack.pop().expect("Bad pop");

//         match elem_type {
//             ElementType::Template => {
//                 let elem = self.build_template_stack.pop().unwrap();

//                 if let Some(parent) = self.build_template_stack.last_mut() {
//                     parent.children.push(elem);
//                 } else {
//                     self.context_next.push(elem);
//                 }
//             }
//             ElementType::TemplateHandler => {
//                 let handler = self.build_template_handler.take().unwrap();
//                 self.build_template_handler = handler.parent.clone();
//             }
//         }
//     }

//     pub fn yield_children(&mut self) {
//         if let Some(parent) = self.build_template_stack.last_mut() {
//             parent.children.append(&mut self.context_children.clone());
//         } else {
//             self.context_next.append(&mut self.context_children.clone());
//         }
//     }

//     pub fn push_template(&mut self, template: Box<Template>) {
//         let elem = TemplateElement::new(template, self.build_template_handler.clone());
//         self.build_template_stack.push(elem);
//         self.build_stack.push(ElementType::Template);
//     }

//     pub fn push_template_handler(&mut self, handler: Box<TemplateHandler>) {
//         let elem = TemplateHandlerElement::new(handler, self.build_template_handler.take());
//         self.build_template_handler = Some(Rc::new(elem));

//         self.build_stack.push(ElementType::TemplateHandler);
//     }

//     pub fn run(&mut self) {
//         while let Some(template) = self.context_next.pop() {
//             if !self.build_stack.is_empty() {
//                 panic!("Stack left unpopped");
//             }

//             // Setup the context based on this template
//             self.context_children = template.children;

//             if let Some(handler) = template.handler {
//                 self.build_template_handler = handler.parent.clone();
//                 handler.handler.run(self, template.template);
//             } else {
//                 template.template.run(self);
//             }
//         }
//     }
// }
