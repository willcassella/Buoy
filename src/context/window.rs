use std::rc::Rc;
use std::mem::replace;
use layout::Area;
use tree::{Socket, Generator, Element, Filter};
use super::{WidgetInfo, WidgetId};
use super::context::{ContextId, FrameId, Context, TreeNode, NonTerminalNode, NonTerminalKind};

struct SocketContext {
    id: WidgetId,
    socket: Box<dyn Socket>,

    bounds: Area,
    siblings: Vec<TreeNode>,
    inner_filters: Vec<Rc<dyn Filter>>,
}

struct FilterContext {
    filter: Rc<dyn Filter>,
    siblings: Vec<TreeNode>,
}

#[derive(Default)]
pub struct Window {
    frame_id: FrameId,
    next_context_id: ContextId,

    bounds: Area,
    sockets: Vec<SocketContext>,
    filters: Vec<FilterContext>,
    roots: Vec<TreeNode>,
}

impl Window {
    pub fn run(&mut self, bounds: Area, root: Box<dyn Generator>) -> Option<Box<dyn Element>> {
        self.bounds = bounds;
        self.next_context_id = Default::default();

        let root = TreeNode::NonTerminal(NonTerminalNode {
            kind: NonTerminalKind::Generator(WidgetInfo::new_str_id(""), root),
            children: Vec::new(),
        });

        self.roots = vec![root];
        self.run_impl()
    }

    fn push_filter(&mut self, filter: Rc<dyn Filter>, siblings: Vec<TreeNode>) {
        let filter_ctx = FilterContext {
            filter,
            siblings,
        };
        self.filters.push(filter_ctx);
    }

    fn pop_filter(&mut self) -> Option<Rc<dyn Filter>> {
        let filter = match self.filters.pop() {
            Some(filter) => filter,
            None => return None,
        };

        // If there are roots, set this filter back up as a node
        if !self.roots.is_empty() {
            let filter_node = TreeNode::NonTerminal(NonTerminalNode {
                kind: NonTerminalKind::Filter(filter.filter.clone()),
                children: replace(&mut self.roots, filter.siblings),
            });
            self.roots.push(filter_node);
        } else {
            self.roots = filter.siblings;
        }

        return Some(filter.filter);
    }

    fn run_impl(&mut self) -> Option<Box<Element>> {
        loop {
            // If there are no roots, move up the tree
            if self.roots.is_empty() {
                // If there's a filter, continue with its siblings
                if let Some(filter) = self.filters.pop() {
                    self.roots = filter.siblings;
                    continue;
                }

                // Close the first socket. If there is no socket to close, we couldn't build a UI
                let socket = match self.sockets.pop() {
                    Some(socket) => socket,
                    None => return None,
                };

                // Create filters for each inner filter of the context
                // Don't have to worry about popping existing filters, because in this case there are none
                for filter in socket.inner_filters {
                    self.push_filter(filter, Vec::new());
                }

                // Construct context object from SocketContext
                let mut ctx = Context::new(self.frame_id, self.next_context_id, socket.id, socket.bounds);
                self.next_context_id.0 += 1;

                // From the perspective of the socket, current roots are children and siblings are current roots
                // ...but roots is empty, so no reason to add as children
                self.roots = socket.siblings;

                // Run the socket
                socket.socket.close(&mut ctx);

                self.roots.append(&mut ctx.roots);
                self.bounds = socket.bounds;
                continue;
            }

            // Get the first root
            let root = self.roots.pop().unwrap();
            match root {
                TreeNode::Terminal(bounds, element) => {
                    // Get the current socket
                    // If we have an element but no socket, then we're done building the UI
                    let socket = match self.sockets.pop() {
                        Some(socket) => socket,
                        None => return Some(element),
                    };

                    // Pop all existing filters
                    while self.pop_filter().is_some() {
                    }

                    // Set up filter contexts for each inner filter of the socket
                    for filter in socket.inner_filters {
                        self.push_filter(filter, Vec::new());
                    }

                    // Create a context for running the socket
                    let mut ctx = Context::new(self.frame_id, self.next_context_id, socket.id, socket.bounds);
                    self.next_context_id.0 += 1;

                    // From the perspective of the socket, current roots are children and siblings are current roots
                    ctx.children = replace(&mut self.roots, socket.siblings);

                    // Run the socket
                    socket.socket.child(&mut ctx, bounds, element);

                    self.roots.append(&mut ctx.roots);
                    self.bounds = socket.bounds;
                },
                TreeNode::NonTerminal(NonTerminalNode{ kind: NonTerminalKind::Filter(filter), children }) => {
                    // If this filter doesn't have any children, just discard it
                    if children.is_empty() {
                        continue;
                    }

                    let siblings = replace(&mut self.roots, children);
                    self.push_filter(filter, siblings);
                },
                TreeNode::NonTerminal(NonTerminalNode{ kind: NonTerminalKind::Generator(info, generator), children }) => {
                    // Pick the first filter from the filter stack
                    if let Some(filter) = self.pop_filter() {
                        // Create a context for running the filter
                        let mut ctx = Context::new(self.frame_id, self.next_context_id, info.id, self.bounds);
                        self.next_context_id.0 += 1;
                        ctx.children = children;

                        // Run the filter
                        filter.generator(&filter, &mut ctx, info, generator);

                        // Put the result into the root set
                        self.roots.append(&mut ctx.roots);
                    } else {
                        // Create a context for running the generator
                        let mut ctx = Context::new(self.frame_id, self.next_context_id, info.id, self.bounds);
                        self.next_context_id.0 += 1;
                        ctx.children = children;

                        // Run the generator
                        generator.run(&mut ctx);

                        // Put the result into the root set
                        self.roots.append(&mut ctx.roots);
                    }
                },
                TreeNode::NonTerminal(NonTerminalNode{ kind: NonTerminalKind::Socket(info, socket), children }) => {
                    // Run filter, if one exists
                    if let Some(filter) = self.pop_filter() {
                        // Create a context for running the filter
                        let mut ctx = Context::new(self.frame_id, self.next_context_id, info.id, self.bounds);
                        self.next_context_id.0 += 1;
                        ctx.children = children;

                        // Run the filter
                        filter.socket(&filter, &mut ctx, info, socket);

                        // Put the result into the root set
                        self.roots.append(&mut ctx.roots);
                    } else {
                        // Otherwise, set up a SocketContext for the new socket
                        let socket_ctx = SocketContext {
                            id: info.id,
                            socket,

                            bounds: self.bounds,
                            siblings: replace(&mut self.roots, children),
                            inner_filters: info.inner_filters,
                        };

                        self.bounds = socket_ctx.socket.get_child_max(self.bounds);
                        self.sockets.push(socket_ctx);
                    }
                },
            }
        }
    }
}