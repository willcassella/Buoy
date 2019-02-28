use crate::element::{
    UIRender,
    UISocketImpl,
    UIFilter,
};

pub trait Output {
    fn render<'a>(
        &'a mut self,
        render: UIRender,
    ) -> &'a mut dyn Output;

    fn filter_pre<'a>(
        &'a mut self,
        filter: UIFilter,
    ) -> &'a mut dyn Output;

    fn filter_post<'a>(
        &'a mut self,
        filter: UIFilter,
    ) -> &'a mut dyn Output;
}

pub struct SocketOutput<'prev, 'socket> {
    socket: &'socket mut dyn UISocketImpl,
    prev: &'prev mut dyn Output,
}

// impl<'prev, 'socket> Output for SocketOutput<'prev, 'socket> {
//     fn render(
//         &mut self,
//         render: UIRender,
//     ) -> &mut dyn Output {
//         self.socket.push(render);
//         self
//     }
// }
