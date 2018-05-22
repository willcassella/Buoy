use std::rc::Rc;
use super::element::{Template, TemplateElement, TemplateHandler, TemplateHandlerElement};

enum ElementType {
    Template,
    TemplateHandler,
}

pub struct Context {
    context_children: Vec<TemplateElement>,
    context_next: Vec<TemplateElement>,

    build_stack: Vec<ElementType>,
    build_template_stack: Vec<TemplateElement>,
    build_template_handler: Option<Rc<TemplateHandlerElement>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            context_children: Vec::new(),
            context_next: Vec::new(),

            build_stack: Vec::new(),
            build_template_stack: Vec::new(),
            build_template_handler: None,
        }
    }

    pub fn pop(&mut self) {
        let elem_type = self.build_stack.pop().expect("Bad pop");

        match elem_type {
            ElementType::Template => {
                let elem = self.build_template_stack.pop().unwrap();

                if let Some(parent) = self.build_template_stack.last_mut() {
                    parent.children.push(elem);
                } else {
                    self.context_next.push(elem);
                }
            }
            ElementType::TemplateHandler => {
                let handler = self.build_template_handler.take().unwrap();
                self.build_template_handler = handler.parent.clone();
            }
        }
    }

    pub fn yield_children(&mut self) {
        if let Some(parent) = self.build_template_stack.last_mut() {
            parent.children.append(&mut self.context_children.clone());
        } else {
            self.context_next.append(&mut self.context_children.clone());
        }
    }

    pub fn push_template(&mut self, template: Box<Template>) {
        let elem = TemplateElement::new(template, self.build_template_handler.clone());
        self.build_template_stack.push(elem);
        self.build_stack.push(ElementType::Template);
    }

    pub fn push_template_handler(&mut self, handler: Box<TemplateHandler>) {
        let elem = TemplateHandlerElement::new(handler, self.build_template_handler.take());
        self.build_template_handler = Some(Rc::new(elem));

        self.build_stack.push(ElementType::TemplateHandler);
    }

    pub fn run(&mut self) {
        while let Some(template) = self.context_next.pop() {
            if !self.build_stack.is_empty() {
                panic!("Stack left unpopped");
            }

            // Setup the context based on this template
            self.context_children = template.children;

            if let Some(handler) = template.handler {
                self.build_template_handler = handler.parent.clone();
                handler.handler.run(self, template.template);
            } else {
                template.template.run(self);
            }
        }
    }
}
