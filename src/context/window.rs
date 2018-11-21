use std::mem::replace;
use crate::layout::Area;
use super::widget::{WidgetId, Widget, WidgetObj, ElementObj, FilterStack};
use super::context::{Context, ContextId, UINode, UINodeKind, WidgetNode, FrameId};

struct WidgetContext {
    widget_id: WidgetId,
    widget: Box<Widget>,

    parent_filter_stack: FilterStack,
    parent_max_children: usize,
    parent_child_elements: Vec<ElementObj>,

    self_bounds: Area,
    siblings: Vec<UINode>,
}

pub struct Window {
    frame_id: FrameId,
    next_context_id: ContextId,

    filter_stack: FilterStack,
    max_children: usize,
    child_elements: Vec<ElementObj>,

    bounds: Area,
    widget_stack: Vec<WidgetContext>,
    roots: Vec<UINode>,
}

impl Window {
    pub fn run(&mut self, bounds: Area, root: WidgetObj) -> Option<ElementObj> {
        // Increment frame id
        self.frame_id.0 += 1;

        // Reset everything else
        self.next_context_id = ContextId::default();
        self.filter_stack = FilterStack::new();
        self.max_children = 1;
        self.child_elements = Vec::new();
        self.bounds = bounds;
        self.widget_stack = Vec::new();

        // Insert root as the initial root
        let root = UINode::widget(WidgetNode{ obj: root, children: Vec::new() });
        self.roots = vec![root];

        self.run_impl()
    }

    fn push_widget(&mut self, widget_node: WidgetNode) {
        // Calculate the number of children this widget supports and the bounds for each child, given its own bounds
        let (max_children, child_bounds) = widget_node.obj.widget.child_layout(self.bounds);

        let widget_context = WidgetContext {
            widget_id: widget_node.obj.id,
            widget: widget_node.obj.widget,

            // Back up the parent's filter stack, and replace it with this widget's filter stack
            parent_filter_stack: replace(&mut self.filter_stack, widget_node.obj.filter_stack),

            // Back up the parent's max number of children, and replace it with this widget's max number of children
            parent_max_children: replace(&mut self.max_children, max_children),

            // Back up the parent's child elements, and create a new array for this widget
            parent_child_elements: replace(&mut self.child_elements, Vec::new()),

            // Back up the parent's bounds for each child, and replace it with this widget's bounds for its children
            self_bounds: replace(&mut self.bounds, child_bounds),

            // Back up the roots of the current widget (siblings of this widget), and initialize an array of un-filtered nodes for this widget
            siblings: replace(&mut self.roots, widget_node.children),
        };

        self.widget_stack.push(widget_context);
    }

    fn pop_widget(&mut self) {
        let widget_context = self.widget_stack.pop().unwrap();

        // Restore parent's filter stack
        self.filter_stack = widget_context.parent_filter_stack;

        // Restore parent's max children
        self.max_children = widget_context.parent_max_children;

        // restore parent's child elements
        let widget_child_elements = replace(&mut self.child_elements, widget_context.parent_child_elements);

        // Restore the parent's bounds
        self.bounds = widget_context.self_bounds;

        // Construct context object from WidgetContext
        let mut ctx = Context::new(self.frame_id, self.next_context_id, widget_context.widget_id, self.bounds);
        self.next_context_id.0 += 1;

        // From the perspective of the widget, current roots are children and siblings are current roots
        ctx.children = replace(&mut self.roots, widget_context.siblings);

        // Run the widget
        widget_context.widget.child_elements(&mut ctx, widget_child_elements);

        // Roots of the context are now roots
        self.roots.append(&mut ctx.roots);
    }

    fn run_impl(&mut self) -> Option<ElementObj> {
        loop {
            // If there are no roots, move up the tree
            if self.roots.is_empty() {
                // If there are no widgets, we couldn't build a UI
                if self.widget_stack.is_empty() {
                    return None;
                }

                // Run the first widget with children so far
                self.pop_widget();
                continue;
            }

            // Get the first root
            let root = self.roots.pop().unwrap();
            match root.kind {
                UINodeKind::Element(element_obj) => {
                    // If there are no widgets, we're done
                    if self.widget_stack.is_empty() {
                        return Some(element_obj);
                    }

                    // Add this widget to the array for the parent element
                    self.child_elements.push(element_obj);

                    // If we've met the parent element's supported number of children, pop it
                    if self.child_elements.len() == self.max_children {
                        self.pop_widget();
                    }
                },
                UINodeKind::Widget(widget_node) => {
                    // If we still have filters to run on this node
                    if root.filter_index < self.filter_stack.0.len() {
                        // Create a context for running the filter
                        let mut ctx = Context::new(self.frame_id, self.next_context_id, widget_node.obj.id, self.bounds);
                        self.next_context_id.0 += 1;
                        ctx.children = widget_node.children;

                        // Run the filter
                        let filter = &self.filter_stack.0[root.filter_index];
                        filter.filter(filter, &mut ctx, widget_node.obj);

                        // Increment the root filter index
                        for new_root in &mut ctx.roots {
                            new_root.filter_index = root.filter_index + 1;
                        }

                        // Put the results into the root set
                        self.roots.append(&mut ctx.roots);
                    } else {
                        // Otherwise, push this widget onto the widget stack
                        self.push_widget(widget_node);
                    }
                },
            }
        }
    }
}