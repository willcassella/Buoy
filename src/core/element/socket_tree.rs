use crate::core::element::*;
use crate::core::filter::*;
use crate::util::arena::ABox;
use crate::util::queue::{QNode, Queue};
use std::collections::HashMap;

pub type ElementQNode<'frm> = ABox<'frm, QNode<'frm, ElementNode<'frm>>>;
pub type ElementQueue<'frm> = Queue<'frm, ElementNode<'frm>>;

pub struct ElementNode<'frm> {
    pub elem: Elem<'frm>,
    pub filter_stack: FilterStack,
    pub children: SocketTree<'frm>,
}

impl<'frm> ElementNode<'frm> {
    pub fn new(elem: Elem<'frm>, filter_stack: FilterStack) -> Self {
        ElementNode {
            elem,
            filter_stack,
            children: SocketTree::default(),
        }
    }
}

#[derive(Default)]
pub struct SocketTree<'frm> {
    default_socket: ElementQueue<'frm>,
    other_sockets: Option<HashMap<SocketName, ElementQueue<'frm>>>,
}

impl<'frm> SocketTree<'frm> {
    pub fn get_or_create(&mut self, socket: SocketName) -> &mut ElementQueue<'frm> {
        if socket.is_default() {
            &mut self.default_socket
        } else {
            self.other_sockets
                .get_or_insert_with(Default::default)
                .entry(socket)
                .or_default()
        }
    }

    pub fn get(&mut self, socket: SocketName) -> Option<&mut ElementQueue<'frm>> {
        if socket.is_default() {
            Some(&mut self.default_socket)
        } else {
            self.other_sockets
                .as_mut()
                .and_then(|sockets| sockets.get_mut(&socket))
        }
    }

    pub fn remove(&mut self, socket: SocketName) -> Option<ElementQueue<'frm>> {
        if socket.is_default() {
            Some(std::mem::take(&mut self.default_socket))
        } else {
            self.other_sockets
                .as_mut()
                .and_then(|sockets| sockets.remove(&socket))
        }
    }

    pub fn take(&mut self) -> SocketTree<'frm> {
        SocketTree {
            default_socket: self.default_socket.take(),
            other_sockets: self.other_sockets.take(),
        }
    }

    pub fn append(&mut self, other: Self) {
        self.default_socket.append(other.default_socket);

        let other_sockets = match other.other_sockets {
            Some(other_sockets) => other_sockets,
            None => return,
        };

        match self.other_sockets {
            Some(ref mut sockets) => {
                for (socket, children) in other_sockets {
                    sockets.entry(socket).or_default().append(children);
                }
            }
            None => self.other_sockets = Some(other_sockets),
        }
    }
}
