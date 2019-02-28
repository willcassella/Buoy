pub trait Append<T> {
    type Result;

    fn append(self, value: T) -> Self::Result;
}

macro_rules! impl_append {
    ( $( $t:ident:$i:tt ),* ) => {

        impl<T, $($t,)* > Append<T> for ( $($t,)* ) {
            type Result = ( $($t,)* T, );

            fn append(self, value: T) -> Self::Result {
                ( $(self.$i,)* value, )
            }
        }

    };
}

impl_append!();
impl_append!(T0:0);
impl_append!(T0:0, T1:1);
impl_append!(T0:0, T1:1, T2:2);
impl_append!(T0:0, T1:1, T2:2, T3:3);
impl_append!(T0:0, T1:1, T2:2, T3:3, T4:4);
impl_append!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5);
impl_append!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6);
impl_append!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7);
impl_append!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8);
