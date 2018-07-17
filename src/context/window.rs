use std::mem::replace;
use layout::Area;
use tree::{Socket, Generator, Element, Filter};
use super::WidgetId;
use super::context::{ContextId, FrameId, Context, TreeNode, NonTerminalNode, NonTerminalType};

struct SocketContext {
    previous_filter_depth: u32,
    socket: Box<Socket>,
    widget_id: WidgetId,
    bounds: Area,
    siblings: Vec<TreeNode>,
}

struct FilterContext {
    filter: Box<Filter>,
    siblings: Vec<TreeNode>,
}

pub struct Window {
    frame_id: FrameId,
    next_context_id: ContextId,

    bounds: Area,
    filter_depth: u32,
    sockets: Vec<SocketContext>,
    filters: Vec<FilterContext>,
}

impl Default for Window {
    fn default() -> Self {
        Window {
            frame_id: 0_u16,
            next_context_id: 0_u32,
            bounds: Area::zero(),
            filter_depth: 0_u32,
            sockets: Vec::new(),
            filters: Vec::new(),
        }
    }
}

fn insert_front<T>(dest: &mut Vec<T>, source: Vec<T>) {
    dest.reserve(source.len());
    for e in source {
        dest.insert(0, e);
    }
}

impl Window {
    pub fn run(&mut self, bounds: Area, root: Box<Generator>) -> Option<Box<Element>> {
        self.bounds = bounds;

        let root_node = NonTerminalNode {
            element: NonTerminalType::Generator(WidgetId::str(""), root),
            children: Vec::new(),
        };

        let roots = vec![TreeNode::NonTerminal(root_node)];
        self.run_impl(roots)
    }

    fn run_impl(&mut self, mut roots: Vec<TreeNode>) -> Option<Box<Element>> {
        loop {
            // If there are no roots, close the first socket
            if roots.is_empty() {
                // Pop all filters below the socket
                for _ in 0..self.filter_depth {
                    self.filters.pop().expect("Corrupt window stack");
                }

                // Close the first socket
                // If there is no socket to close, we couldn't build a UI
                let socket = match self.sockets.pop() {
                    Some(socket) => socket,
                    None => return None,
                };

                // Restore filter depth from before socket was pushed
                self.filter_depth = socket.previous_filter_depth;

                // Construct context object from SocketContext
                let mut ctx = Context::new(self.frame_id, self.next_context_id, socket.widget_id, socket.bounds);
                self.next_context_id += 1;

                // From the perspective of the socket, current roots are children and siblings are current roots
                // ...but roots is empty, so no reason to add as children
                roots = socket.siblings;

                // Run the socket
                socket.socket.close(&mut ctx);

                // Socket's roots replace the socket
                insert_front(&mut roots, ctx.roots);
                self.bounds = socket.bounds;
                continue;
            }

            // Get the first root (queue)
            match roots.remove(0) {
                TreeNode::Terminal(bounds, element) => {
                    // Pop all filters below the socket
                    for _ in 0..self.filter_depth {
                        self.filters.pop().expect("Corrupt filter stack");
                    }

                    // Get the current socket
                    // If we have an element but no socket, then we're done building the UI
                    let socket = match self.sockets.pop() {
                        Some(socket) => socket,
                        None => return Some(element),
                    };

                    // Restore filter depth from before socket was pushed
                    self.filter_depth = socket.previous_filter_depth;

                    let mut ctx = Context::new(self.frame_id, self.next_context_id, socket.widget_id, socket.bounds);
                    self.next_context_id += 1;

                    // From the perspective of the socket, current roots are children and siblings are current roots
                    ctx.children = replace(&mut roots, socket.siblings);

                    // Run the socket
                    socket.socket.child(&mut ctx, bounds, element);

                    // Socket's roots replace the socket (if socket called 'children()', previous roots will still exist)
                    insert_front(&mut roots, ctx.roots);
                    self.bounds = socket.bounds;
                },
                TreeNode::NonTerminal(NonTerminalNode{ element: NonTerminalType::Filter(filter), children }) => {
                    // Push the filter onto the filter stack
                    let filter_context = FilterContext {
                        filter,
                        siblings: replace(&mut roots, children),
                    };

                    self.filters.push(filter_context);
                    self.filter_depth += 1;
                },
                TreeNode::NonTerminal(NonTerminalNode{ element: NonTerminalType::Generator(id, generator), children }) => {
                    let mut ctx = Context::new(self.frame_id, self.next_context_id, id, self.bounds);
                    self.next_context_id += 1;
                    ctx.children = children;

                    // Run the generator
                    generator.run(&mut ctx);

                    // Add all of the context roots as roots
                    insert_front(&mut roots, ctx.roots);
                },
                TreeNode::NonTerminal(NonTerminalNode{ element: NonTerminalType::Socket(id, socket), children }) => {
                    // Create the StackContext
                    let socket = SocketContext {
                        previous_filter_depth: replace(&mut self.filter_depth, 0),
                        socket,
                        widget_id: id,
                        bounds: self.bounds,
                        siblings: replace(&mut roots, children),
                    };

                    self.bounds = socket.socket.get_child_max(self.bounds);
                    self.sockets.push(socket);
                },
            }
        }
    }
}
