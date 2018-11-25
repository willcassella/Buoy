use crate::util::fill::Fill;
use crate::layout::Area;
use crate::element::{UIElementObj, FilterStack};
use crate::render::UIRenderObj;
use super::context::{Context, UINode, UINodeKind, UIElementNode, FrameId};

#[derive(Default)]
pub struct Window {
    frame_id: FrameId,
}

impl Window {
    pub fn run(&mut self, max_area: Area, root: UIElementObj) -> Option<UIRenderObj> {
        // Increment frame id
        self.frame_id.0 += 1;

        // Create storage for resulting render
        let mut out: Option<UIRenderObj> = None;

        // Insert root as the initial root
        let mut roots = vec![UINode::from_element(UIElementNode{ obj: root, children: Vec::new() })];

        // Fill the element
        self.fill_element(&mut out, max_area, FilterStack::new(), &mut roots);

        return out;
    }

    fn fill_element(
        &mut self,
        fill: &mut Fill<UIRenderObj>,
        max_area: Area,
        filters: FilterStack,
        roots: &mut Vec<UINode>,
    ) {
        while fill.remaining_capacity() != 0 {
            let root = match roots.pop() {
                Some(x) => x,
                None => return,
            };

            match root.kind {
                UINodeKind::Render(render_obj) => fill.push(render_obj),
                UINodeKind::Element(mut element_node) => {
                    // If we still have filters to run on this node
                    if root.filter_index < filters.0.len() {
                        // Create a context for running the filter
                        let mut ctx = Context::new(self.frame_id, element_node.obj.id, max_area);
                        ctx.children = element_node.children;

                        // Run the filter
                        let filter = &filters.0[root.filter_index];
                        filter.filter(filter, &mut ctx, element_node.obj);

                        // Increment the root filter index
                        for new_root in &mut ctx.roots {
                            new_root.filter_index = root.filter_index + 1;
                        }

                        // Put the results into the root set
                        roots.append(&mut ctx.roots);
                    } else {
                        // Lay out the children of the element
                        {
                            let (child_fill, child_max_area) = element_node.obj.element.open(max_area);
                            self.fill_element(child_fill, child_max_area, element_node.obj.filter_stack, &mut element_node.children);
                        }

                        // Create a context for closing the element
                        let mut ctx = Context::new(self.frame_id, element_node.obj.id, max_area);
                        ctx.children = element_node.children;

                        // Run the element
                        element_node.obj.element.close(&mut ctx);

                        // Put the results into the root set
                        roots.append(&mut ctx.roots);
                    }
                }
            }
        }
    }
}