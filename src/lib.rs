macro_rules! decl_func {
    ($func_name:ident $(,$param_name:ident: $param_type:ident)*) => {
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        pub struct $func_name<C $(,$param_type)*, R> {
            pub function: fn(&C $(,$param_type)*) -> R,
            pub captured: C,
        }

        impl<C $(,$param_type)*, R> $func_name<C $(,$param_type)*, R> {
            pub fn call(&self $(,$param_name: $param_type)*) -> R {
                (self.function)(&self.captured $(,$param_name)*)
            }
        }
    };
}

decl_func!(Func0);
decl_func!(Func1, p1: P1);
decl_func!(Func2, p1: P1, p2: P2);
decl_func!(Func3, p1: P1, p2: P2, p3: P3);
decl_func!(Func4, p1: P1, p2: P2, p3: P3, p4: P4);
decl_func!(Func5, p1: P1, p2: P2, p3: P3, p4: P4, p5: P5);

#[macro_export]
macro_rules! func_internal {
    (@internal $(())?, $($tail:tt)*) => {
        $crate::Func0 {
            $($tail)*
        }
    };
    (@internal ($p1i:ident $(:$p1t:ty)?), $($tail:tt)*) => {
        $crate::Func1 {
            $($tail)*
        }
    };
    (@internal ($p1i:ident $(:$p1t:ty)?, $p2i:ident $(:$p2t:ty)?), $($tail:tt)*) => {
        $crate::Func2 {
            $($tail)*
        }
    };
    (@internal ($p1i:ident $(:$p1t:ty)?, $p2i:ident $(:$p2t:ty)?, $p3i:ident $(:$p3t:ty)?), $($tail:tt)*) => {
        $crate::Func3 {
            $($tail)*
        }
    };
    (@internal ($p1i:ident $(:$p1t:ty)?, $p2i:ident $(:$p2t:ty)?, $p3i:ident $(:$p3t:ty)?, $p4i:ident $(:$p4t:ty)?), $($tail:tt)*) => {
        $crate::Func4 {
            $($tail)*
        }
    };
    (@internal ($p1i:ident $(:$p1t:ty)?, $p2i:ident $(:$p2t:ty)?, $p3i:ident $(:$p3t:ty)?, $p4i:ident $(:$p4t:ty)?, $p5i:ident $(:$p5t:ty)?), $($tail:tt)*) => {
        $crate::Func5 {
            $($tail)*
        }
    };
}

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
macro_rules! func {
    (
        $([$($cap_ident:ident $(: $cap_expr:expr)?),*])?
        $(|$($param_ident:ident $(: $param_ty:ty)?),*|)?
        $(-> $r_type:ty)?
        $body:block
    ) => {
        $crate::func_internal! {
            @internal
            $(($($param_ident $(: $param_ty)?),*))?,
            #[allow(unused_parens)]
            function: |($($($cap_ident),*)?) $($(,$param_ident $(: $param_ty)?)*)?| $(-> $r_type)? { $body },
            captured: ($($($crate::cap_internal!(@internal $cap_ident $(, $cap_expr)?)),*)?),
        }
    };
}
