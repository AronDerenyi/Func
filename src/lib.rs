#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Func<C, P, R> {
    pub captured: C,
    pub function: fn(&C, P) -> R,
}

macro_rules! impl_func_call {
    ($($param_name:ident: $param_type:ident),*) => {
        impl<C $(,$param_type)*, R> Func<C, ($($param_type,)*), R> {
            pub fn call(&self $(,$param_name: $param_type)*) -> R {
                (self.function)(&self.captured, ($($param_name,)*))
            }

            pub fn to_fn(self) -> impl Fn($($param_type),*) -> R {
                let Self{captured, function} = self;
                move |$($param_name: $param_type),*| {
                    function(&captured, ($($param_name,)*))
                }
            }
        }
    };
}

impl_func_call!();
impl_func_call!(p1: P1);
impl_func_call!(p1: P1, p2: P2);
impl_func_call!(p1: P1, p2: P2, p3: P3);
impl_func_call!(p1: P1, p2: P2, p3: P3, p4: P4);
impl_func_call!(p1: P1, p2: P2, p3: P3, p4: P4, p5: P5);
impl_func_call!(p1: P1, p2: P2, p3: P3, p4: P4, p5: P5, p6: P6);
impl_func_call!(p1: P1, p2: P2, p3: P3, p4: P4, p5: P5, p6: P6, p7: P7);
impl_func_call!(p1: P1, p2: P2, p3: P3, p4: P4, p5: P5, p6: P6, p7: P7, p8: P8);

#[macro_export]
macro_rules! cap_internal {
    (@internal $cap_ident:ident) => {
        $cap_ident
    };
    (@internal $cap_ident:ident, $cap_expr:expr) => {
        $cap_expr
    };
}

#[macro_export]
macro_rules! param_internal {
    (@internal) => {
        _
    };
    (@internal $param_ty:ty) => {
        $param_ty
    };
}

#[macro_export]
macro_rules! func {
    (
        $([$($cap_ident:ident $(: $cap_expr:expr)?),*])?
        $(|$(mut)? $($param_ident:ident $(: $param_ty:ty)?),*|)?
        $(-> $r_type:ty)?
        $body:block
    ) => {
        $crate::Func {
            captured: (
                $($($crate::cap_internal!(@internal $cap_ident $(, $cap_expr)?)),*)?
            ),
            function: |
                ($($($cap_ident),*)?),
                ($($($param_ident,)*)?): (
                    $($($crate::param_internal!($(@internal $param_ty)?),)*)?
                )
            | $(-> $r_type)? { $body },
        }
    };
}
