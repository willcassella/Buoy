use std::mem;
use layout::{Bounds};
use tree::{Filter, Generator, Element, Socket};

pub struct Context {
    bounds: Bounds,

    children: Vec<TreeElement>,
    stack: Vec<StackElement>,
    roots: Vec<TreeElement>,
}

impl Context {
    fn new(bounds: Bounds) -> Self {
        Context {
            bounds,
            children: Vec::new(),
            stack: Vec::new(),
            roots: Vec::new(),
        }
    }
}

impl Context {
    // Moves the context upward to the parent of the current element
    // This will panic if the parent is not in scope for this context!
    pub fn pop(&mut self) {
        let element = self.stack.pop().expect("Bad pop");
        let element = TreeElement::StackElement(element);

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(element);
        } else {
            self.roots.push(element);
        }
    }

    pub fn push_generator(&mut self, generator: Box<Generator>) {
        let element = StackElement {
            element: Builder::Generator(generator),
            children: Vec::new(),
        };

        self.stack.push(element);
    }

    pub fn push_filter(&mut self, filter: Box<Filter>) {
        let element = StackElement {
            element: Builder::Filter(filter),
            children: Vec::new(),
        };

        self.stack.push(element);
    }

    pub fn push_socket(&mut self, socket: Box<Socket>) {
        // Need to add socket as a child of whatever the top of the stack is (socket or generator)
        let element = StackElement {
            element: Builder::Socket(socket),
            children: Vec::new(),
        };

        self.stack.push(element);
    }

    pub fn self_max(&self) -> Bounds {
        self.bounds
    }

    // Pushes a function that gets called with a final area
    pub fn element(&mut self, bounds: Bounds, element: Box<Element>) {
        let element = TreeElement::Terminal(bounds, element);

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(element);
        } else {
            self.roots.push(element);
        }
    }

    pub fn children(&mut self) {
        if let Some(parent) = self.stack.last_mut() {
            parent.children.append(&mut self.children);
        } else {
            self.roots.append(&mut self.children);
        }
    }
}

enum TreeElement {
    Terminal(Bounds, Box<Element>),
    StackElement(StackElement),
}

enum Builder {
    Filter(Box<Filter>),
    Generator(Box<Generator>),
    Socket(Box<Socket>),
}

struct StackElement {
    element: Builder,
    children: Vec<TreeElement>,
}

pub struct GlobalContext {
    bounds: Bounds,
    filters: Vec<FilterContext>,
    sockets: Vec<SocketContext>,
    elements: Vec<GlobalContextElement>,
}

impl Default for GlobalContext {
    fn default() -> Self {
        GlobalContext {
            bounds: Bounds::zero(),
            filters: Vec::new(),
            sockets: Vec::new(),
            elements: Vec::new(),
        }
    }
}

fn insert_front<T>(dest: &mut Vec<T>, source: Vec<T>) {
    dest.reserve(source.len());
    for e in source {
        dest.insert(0, e);
    }
}

impl GlobalContext {
    pub fn run(&mut self, bounds: Bounds, root: Box<Generator>) -> Option<Box<Element>> {
        self.bounds = bounds;

        let root_element = StackElement {
            element: Builder::Generator(root),
            children: Vec::new(),
        };

        let roots = vec![TreeElement::StackElement(root_element)];
        self.run_impl(roots)
    }

    fn run_impl(&mut self, mut roots: Vec<TreeElement>) -> Option<Box<Element>> {
        while !roots.is_empty() {
            // Get the first root (queue)
            match roots.remove(0) {
                TreeElement::Terminal(bounds, element) => {
                    // Get the current socket
                    // If we have an element but no socket, then we're done building the UI
                    let socket = match self.sockets.pop() {
                        Some(socket) => socket,
                        None => return Some(element),
                    };

                    // From the perspective of the socket, current roots are it's children
                    let mut ctx = Context::new(socket.bounds);
                    ctx.children = mem::replace(&mut roots, Vec::new());

                    // Run the socket
                    socket.socket.child(&mut ctx, bounds, element);

                    // Socket's roots replace the socket (if socket called 'children()', previous roots will still exist
                    roots = ctx.roots;
                    self.bounds = socket.bounds;
                },
                TreeElement::StackElement(StackElement{ element: Builder::Filter(_filter), children }) => {
                },
                TreeElement::StackElement(StackElement{ element: Builder::Generator(generator), children }) => {
                    let mut ctx = Context::new(self.bounds);
                    ctx.children = children;

                    // Run the generator
                    generator.run(&mut ctx);

                    // Add all of the context roots as roots
                    insert_front(&mut roots, ctx.roots);
                },
                TreeElement::StackElement(StackElement{ element: Builder::Socket(socket), children }) => {
                    // Create the StackContext
                    let socket = SocketContext {
                        socket,
                        bounds: self.bounds,
                        siblings: mem::replace(&mut roots, children),
                    };

                    self.bounds = socket.socket.get_child_max(self.bounds);
                    self.sockets.push(socket);
                },
            }
        }

        None
    }
}

struct FilterContext {
    filter: Box<Filter>,
}

struct SocketContext {
    socket: Box<Socket>,
    bounds: Bounds,
    siblings: Vec<TreeElement>,
}

enum GlobalContextElement {
    Filter,
    Socket,
}