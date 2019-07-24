use std::collections::HashMap;

use crate::core::element::*;

use crate::util::linked_buffer::LBBox;
use crate::util::linked_queue::{Queue, QNode};

use super::context::ElementNode;

pub(crate) type ChildQNode<'frm> = LBBox<'frm, QNode<'frm, ElementNode<'frm>>>;
pub(crate) type ChildQueue<'frm> = Queue<'frm, ElementNode<'frm>>;

#[derive(Default)]
pub(crate) struct Children<'frm> {
    default_socket: ChildQueue<'frm>,
    other_sockets: Option<HashMap<SocketName, ChildQueue<'frm>>>,
}

impl<'frm> Children<'frm> {
    pub fn get_or_create(&mut self, socket: SocketName) -> &mut ChildQueue<'frm> {
        if socket.is_default() {
            &mut self.default_socket
        } else {
            self.other_sockets
                .get_or_insert_with(Default::default)
                .entry(socket)
                .or_default()
        }
    }

    pub fn get(&mut self, socket: SocketName) -> Option<&mut ChildQueue<'frm>> {
        if socket.is_default() {
            Some(&mut self.default_socket)
        } else {
            self.other_sockets.as_mut().and_then(|sockets| sockets.get_mut(&socket))
        }
    }

    pub fn remove(&mut self, socket: SocketName) -> Option<ChildQueue<'frm>> {
        if socket.is_default() {
            Some(std::mem::replace(&mut self.default_socket, Queue::default()))
        } else {
            self.other_sockets.as_mut().and_then(|sockets| sockets.remove(&socket))
        }
    }

    pub fn take(&mut self) -> Children<'frm> {
        Children {
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
            },
            None => self.other_sockets = Some(other_sockets),
        }
    }
}
