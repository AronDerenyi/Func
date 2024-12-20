#[derive(Debug)]
pub struct Func<C, P, R> {
    pub captured: C,
    pub function: fn(&C, P) -> R,
}

#[derive(Debug)]
pub struct FuncMut<C, P, R> {
    pub captured: C,
    pub function: fn(&mut C, P) -> R,
}

#[derive(Debug)]
pub struct FuncOnce<C, P, R> {
    pub captured: C,
    pub function: fn(C, P) -> R,
}

macro_rules! impl_func {
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

        impl<C $(,$param_type)*, R> FuncMut<C, ($($param_type,)*), R> {
            pub fn call(&mut self $(,$param_name: $param_type)*) -> R {
                (self.function)(&mut self.captured, ($($param_name,)*))
            }

            pub fn to_fn_mut(self) -> impl FnMut($($param_type),*) -> R {
                let Self{mut captured, function} = self;
                move |$($param_name: $param_type),*| {
                    function(&mut captured, ($($param_name,)*))
                }
            }
        }

        impl<C $(,$param_type)*, R> FuncOnce<C, ($($param_type,)*), R> {
            pub fn call(self $(,$param_name: $param_type)*) -> R {
                (self.function)(self.captured, ($($param_name,)*))
            }

            pub fn to_fn_once(self) -> impl FnOnce($($param_type),*) -> R {
                let Self{captured, function} = self;
                move |$($param_name: $param_type),*| {
                    function(captured, ($($param_name,)*))
                }
            }
        }
    };
}

impl<C: Clone, P, R> Clone for Func<C, P, R> {
    fn clone(&self) -> Self {
        Self {
            captured: self.captured.clone(),
            function: self.function.clone(),
        }
    }
}

impl<C: Clone, P, R> Clone for FuncMut<C, P, R> {
    fn clone(&self) -> Self {
        Self {
            captured: self.captured.clone(),
            function: self.function.clone(),
        }
    }
}

impl<C: Clone, P, R> Clone for FuncOnce<C, P, R> {
    fn clone(&self) -> Self {
        Self {
            captured: self.captured.clone(),
            function: self.function.clone(),
        }
    }
}

impl<C: Copy, P, R> Copy for Func<C, P, R> {}
impl<C: Copy, P, R> Copy for FuncMut<C, P, R> {}
impl<C: Copy, P, R> Copy for FuncOnce<C, P, R> {}

impl<C: PartialEq, P, R> PartialEq for Func<C, P, R> {
    fn eq(&self, other: &Self) -> bool {
        self.captured == other.captured && self.function == other.function
    }
}

impl<C: PartialEq, P, R> PartialEq for FuncMut<C, P, R> {
    fn eq(&self, other: &Self) -> bool {
        self.captured == other.captured && self.function == other.function
    }
}

impl<C: PartialEq, P, R> PartialEq for FuncOnce<C, P, R> {
    fn eq(&self, other: &Self) -> bool {
        self.captured == other.captured && self.function == other.function
    }
}

impl<C: Eq, P, R> Eq for Func<C, P, R> {}
impl<C: Eq, P, R> Eq for FuncMut<C, P, R> {}
impl<C: Eq, P, R> Eq for FuncOnce<C, P, R> {}

#[macro_export]
macro_rules! func {
    ($($body:tt)*) => {
        $crate::func_internal! {
            @internal
            Func
            $($body)*
        }
    };
}

#[macro_export]
macro_rules! func_mut {
    ($($body:tt)*) => {
        $crate::func_internal! {
            @internal
            FuncMut
            $($body)*
        }
    };
}

#[macro_export]
macro_rules! func_once {
    ($($body:tt)*) => {
        $crate::func_internal! {
            @internal
            FuncOnce
            $($body)*
        }
    };
}

#[macro_export]
macro_rules! func_internal {
    (
        @internal
        $type:ident
        $([$($cap_ident:ident $(: $cap_expr:expr)?),*])?
        $(|$($param_ident:ident $(: $param_ty:ty)?),*|)?
        $(-> $r_type:ty)?
        $body:block
    ) => {
        $crate::$type {
            captured: (
                $($($crate::cap_internal!(@internal $cap_ident $(, $cap_expr)?),)*)?
            ),
            function: |
                ($($($cap_ident,)*)?),
                ($($($param_ident,)*)?): (
                    $($($crate::param_internal!(@internal $($param_ty)?),)*)?
                )
            | $(-> $r_type)? { $body },
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
macro_rules! param_internal {
    (@internal) => {
        _
    };
    (@internal $param_ty:ty) => {
        $param_ty
    };
}

impl_func!();
impl_func!(p1: P1);
impl_func!(p1: P1, p2: P2);
impl_func!(p1: P1, p2: P2, p3: P3);
impl_func!(p1: P1, p2: P2, p3: P3, p4: P4);
impl_func!(p1: P1, p2: P2, p3: P3, p4: P4, p5: P5);
impl_func!(p1: P1, p2: P2, p3: P3, p4: P4, p5: P5, p6: P6);
impl_func!(p1: P1, p2: P2, p3: P3, p4: P4, p5: P5, p6: P6, p7: P7);
impl_func!(p1: P1, p2: P2, p3: P3, p4: P4, p5: P5, p6: P6, p7: P7, p8: P8);

mod test {

    #[test]
    fn addition_func_with_type_annotations() {
        let add = func!(|a: i32, b: i32| -> i32 { a + b });
        assert_eq!(add.call(1, 2), 3);
    }

    #[test]
    fn addition_func_without_parameter_type_annotations() {
        let add = func!(|a, b| -> i32 { a + b });
        assert_eq!(add.call(1, 2), 3);
    }

    #[test]
    fn addition_func_without_type_annotations() {
        let add = func!(|a, b| { a + b });
        assert_eq!(add.call(1, 2), 3);
    }

    #[test]
    fn labeled_capturing_addition_func() {
        let coeff = 3;
        let add = func!([c: coeff] |a: i32, b: i32| -> i32 { c * a + c * b });
        assert_eq!(add.call(1, 2), 9);
    }

    #[test]
    fn unlabeled_capturing_addition_func() {
        let c = 3;
        let add = func!([c] |a: i32, b: i32| -> i32 { c * a + c * b });
        assert_eq!(add.call(1, 2), 9);
    }
}
