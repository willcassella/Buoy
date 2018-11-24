use crate::util::fill::Fill;
use crate::layout::Area;
use super::widget::{WidgetObj, FilterStack};
use super::element::ElementObj;
use super::context::{Context, ContextId, UINode, UINodeKind, WidgetNode, FrameId};

#[derive(Default)]
pub struct Window {
    frame_id: FrameId,
    next_context_id: ContextId,
}

impl Window {
    pub fn run(&mut self, bounds: Area, root: WidgetObj) -> Option<ElementObj> {
        // Increment frame id
        self.frame_id.0 += 1;

        // Create fill for resulting widget
        let mut fill: Option<ElementObj> = None;

        // Insert root as the initial root
        let mut roots = vec![UINode::widget(WidgetNode{ obj: root, children: Vec::new() })];

        // Fill the widget
        self.fill_widget(&mut fill, bounds, FilterStack::new(), &mut roots);

        return fill;
    }

    fn fill_widget(
        &mut self,
        fill: &mut Fill<ElementObj>,
        bounds: Area,
        filters: FilterStack,
        roots: &mut Vec<UINode>,
    ) {
        while fill.remaining_capacity() != 0 {
            let root = match roots.pop() {
                Some(x) => x,
                None => return,
            };

            match root.kind {
                UINodeKind::Element(element_obj) => fill.push(element_obj),
                UINodeKind::Widget(mut widget_node) => {
                    // If we still have filters to run on this node
                    if root.filter_index < filters.0.len() {
                        // Create a context for running the filter
                        let mut ctx = Context::new(self.frame_id, self.next_context_id, widget_node.obj.id, bounds);
                        self.next_context_id.0 += 1;
                        ctx.children = widget_node.children;

                        // Run the filter
                        let filter = &filters.0[root.filter_index];
                        filter.filter(filter, &mut ctx, widget_node.obj);

                        // Increment the root filter index
                        for new_root in &mut ctx.roots {
                            new_root.filter_index = root.filter_index + 1;
                        }

                        // Put the results into the root set
                        roots.append(&mut ctx.roots);
                    } else {
                        let (child_fill, child_bounds) = widget_node.obj.widget.open(bounds);
                        self.fill_widget(child_fill, child_bounds, widget_node.obj.filter_stack, &mut widget_node.children);

                        // Create a context for closing the widget
                        let mut ctx = Context::new(self.frame_id, self.next_context_id, widget_node.obj.id, bounds);
                        self.next_context_id.0 += 1;
                        ctx.children = widget_node.children;
                    }
                }
            }
        }
    }
}