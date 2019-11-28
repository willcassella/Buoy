use std::any::Any;

pub trait IntoAny: Any {
    fn into_any(&self) -> &dyn Any;

    fn into_any_mut(&mut self) -> &mut dyn Any;
}

impl IntoAny for dyn Any {
    fn into_any(&self) -> &dyn Any {
        self
    }

    fn into_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<T: Any> IntoAny for T {
    fn into_any(&self) -> &dyn Any {
        self
    }

    fn into_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}