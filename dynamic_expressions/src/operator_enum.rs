// -------------------------------------------------------------------------------------------------
// Operator definition macro
// -------------------------------------------------------------------------------------------------

/// Define an operator marker type implementing [`crate::traits::Operator`] + [`crate::traits::OpTag`].
///
/// This is the same DSL used for builtins in `operator_enum::builtin`, but exported so downstream
/// crates can define their own operators with identical syntax.
#[macro_export]
macro_rules! op {
    (@count) => { 0usize };
    (@count $head:tt $(, $tail:tt)*) => { 1usize + $crate::op!(@count $($tail),*) };

    (@maybe $x:expr) => { Some($x) };
    (@maybe) => { None };

    (@name $Op:ident, $name:literal) => { $name };
    (@name $Op:ident) => { $crate::paste::paste! { stringify!([<$Op:snake>]) } };

    (@infix $v:expr) => { Some($v) };
    (@infix) => { None };
    (@commutative $v:expr) => { $v };
    (@commutative) => { false };
    (@associative $v:expr) => { $v };
    (@associative) => { false };
    (@complexity $v:expr) => { $v };
    (@complexity) => { 1u16 };

    (
        $(#[$meta:meta])*
        $Op:ident for<$T:ident> { $($body:tt)* }
    ) => {
        $crate::op!(
            $(#[$meta])*
            pub $Op for<$T> { $($body)* }
        );
    };

    (
        $(#[$meta:meta])*
        $vis:vis $Op:ident for<$T:ident> {
            $(name: $name:literal,)?
            $(display: $display:expr,)?
            $(infix: $infix:expr,)?
            $(commutative: $commutative:expr,)?
            $(associative: $associative:expr,)?
            $(complexity: $complexity:expr,)?
            eval: |[$($eval_arg:pat_param),+ $(,)?]| $eval_body:block,
            partial: |[$($partial_arg:pat_param),+ $(,)?], $idx:pat_param| $partial_body:block $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $Op;

        impl $crate::traits::OpTag for $Op {
            const ARITY: u8 = $crate::op!(@count $($eval_arg),+) as u8;
        }

        impl<$T: $crate::num_traits::Float> $crate::traits::Operator<$T, { $crate::op!(@count $($eval_arg),+) }> for $Op {
            const NAME: &'static str = $crate::op!(@name $Op $(, $name)?);
            const INFIX: Option<&'static str> = $crate::op!(@infix $($infix)?);
            const DISPLAY: &'static str = match ($crate::op!(@maybe $( $display )?), $crate::op!(@infix $($infix)?)) {
                (Some(d), _) => d,
                (None, Some(i)) => i,
                (None, None) => $crate::op!(@name $Op $(, $name)?),
            };
            const COMMUTATIVE: bool = $crate::op!(@commutative $($commutative)?);
            const ASSOCIATIVE: bool = $crate::op!(@associative $($associative)?);
            const COMPLEXITY: u16 = $crate::op!(@complexity $($complexity)?);

            #[inline]
            fn eval(args: &[$T; { $crate::op!(@count $($eval_arg),+) }]) -> $T {
                let [$($eval_arg),+] = *args;
                $eval_body
            }

            #[inline]
            fn partial(args: &[$T; { $crate::op!(@count $($partial_arg),+) }], idx: usize) -> $T {
                let [$($partial_arg),+] = *args;
                let $idx = idx;
                $partial_body
            }
        }
    };

    (
        $(#[$meta:meta])*
        $Op:ident for $t:ty { $($body:tt)* }
    ) => {
        $crate::op!(
            $(#[$meta])*
            pub $Op for $t { $($body)* }
        );
    };

    (
        $(#[$meta:meta])*
        $vis:vis $Op:ident for $t:ty {
            $(name: $name:literal,)?
            $(display: $display:expr,)?
            $(infix: $infix:expr,)?
            $(commutative: $commutative:expr,)?
            $(associative: $associative:expr,)?
            $(complexity: $complexity:expr,)?
            eval: |[$($eval_arg:pat_param),+ $(,)?]| $eval_body:block,
            partial: |[$($partial_arg:pat_param),+ $(,)?], $idx:pat_param| $partial_body:block $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $Op;

        impl $crate::traits::OpTag for $Op {
            const ARITY: u8 = $crate::op!(@count $($eval_arg),+) as u8;
        }

        impl $crate::traits::Operator<$t, { $crate::op!(@count $($eval_arg),+) }> for $Op {
            const NAME: &'static str = $crate::op!(@name $Op $(, $name)?);
            const INFIX: Option<&'static str> = $crate::op!(@infix $($infix)?);
            const DISPLAY: &'static str = match ($crate::op!(@maybe $( $display )?), $crate::op!(@infix $($infix)?)) {
                (Some(d), _) => d,
                (None, Some(i)) => i,
                (None, None) => $crate::op!(@name $Op $(, $name)?),
            };
            const COMMUTATIVE: bool = $crate::op!(@commutative $($commutative)?);
            const ASSOCIATIVE: bool = $crate::op!(@associative $($associative)?);
            const COMPLEXITY: u16 = $crate::op!(@complexity $($complexity)?);

            #[inline]
            fn eval(args: &[$t; { $crate::op!(@count $($eval_arg),+) }]) -> $t {
                let [$($eval_arg),+] = *args;
                $eval_body
            }

            #[inline]
            fn partial(args: &[$t; { $crate::op!(@count $($partial_arg),+) }], idx: usize) -> $t {
                let [$($partial_arg),+] = *args;
                let $idx = idx;
                $partial_body
            }
        }
    };
}

pub mod builtin {
    use num_traits::Float;

    fn two<T: Float>() -> T {
        T::one() + T::one()
    }

    fn three<T: Float>() -> T {
        T::one() + T::one() + T::one()
    }

    fn half<T: Float>() -> T {
        T::from(0.5).expect("Float can represent 0.5")
    }

    crate::op!(Cos for<T> {
        eval: |[x]| { x.cos() },
        partial: |[x], _idx| { -x.sin() },
    });

    crate::op!(Sin for<T> {
        eval: |[x]| { x.sin() },
        partial: |[x], _idx| { x.cos() },
    });

    crate::op!(Tan for<T> {
        eval: |[x]| { x.tan() },
        partial: |[x], _idx| {
            let c = x.cos();
            T::one() / (c * c)
        },
    });

    crate::op!(Asin for<T> {
        eval: |[x]| { x.asin() },
        partial: |[x], _idx| { T::one() / (T::one() - x * x).sqrt() },
    });

    crate::op!(Acos for<T> {
        eval: |[x]| { x.acos() },
        partial: |[x], _idx| { -T::one() / (T::one() - x * x).sqrt() },
    });

    crate::op!(Atan for<T> {
        eval: |[x]| { x.atan() },
        partial: |[x], _idx| { T::one() / (T::one() + x * x) },
    });

    crate::op!(Sinh for<T> {
        eval: |[x]| { x.sinh() },
        partial: |[x], _idx| { x.cosh() },
    });

    crate::op!(Cosh for<T> {
        eval: |[x]| { x.cosh() },
        partial: |[x], _idx| { x.sinh() },
    });

    crate::op!(Tanh for<T> {
        eval: |[x]| { x.tanh() },
        partial: |[x], _idx| {
            let c = x.cosh();
            T::one() / (c * c)
        },
    });

    crate::op!(Asinh for<T> {
        eval: |[x]| { x.asinh() },
        partial: |[x], _idx| { T::one() / (x * x + T::one()).sqrt() },
    });

    crate::op!(Acosh for<T> {
        eval: |[x]| { x.acosh() },
        partial: |[x], _idx| { T::one() / ((x - T::one()).sqrt() * (x + T::one()).sqrt()) },
    });

    crate::op!(Atanh for<T> {
        eval: |[x]| { x.atanh() },
        partial: |[x], _idx| { T::one() / (T::one() - x * x) },
    });

    crate::op!(Sec for<T> {
        eval: |[x]| { T::one() / x.cos() },
        partial: |[x], _idx| { (T::one() / x.cos()) * x.tan() },
    });

    crate::op!(Csc for<T> {
        eval: |[x]| { T::one() / x.sin() },
        partial: |[x], _idx| {
            let csc = T::one() / x.sin();
            let cot = T::one() / x.tan();
            -csc * cot
        },
    });

    crate::op!(Cot for<T> {
        eval: |[x]| { T::one() / x.tan() },
        partial: |[x], _idx| {
            let s = x.sin();
            -T::one() / (s * s)
        },
    });

    crate::op!(Exp for<T> {
        eval: |[x]| { x.exp() },
        partial: |[x], _idx| { x.exp() },
    });

    crate::op!(Log for<T> {
        eval: |[x]| { x.ln() },
        partial: |[x], _idx| { T::one() / x },
    });

    crate::op!(Log2 for<T> {
        eval: |[x]| { x.log2() },
        partial: |[x], _idx| { T::one() / (x * two::<T>().ln()) },
    });

    crate::op!(Log10 for<T> {
        eval: |[x]| { x.log10() },
        partial: |[x], _idx| {
            let ten = T::from(10.0).expect("Float can represent 10.0");
            T::one() / (x * ten.ln())
        },
    });

    crate::op!(Log1p for<T> {
        eval: |[x]| { x.ln_1p() },
        partial: |[x], _idx| { T::one() / (T::one() + x) },
    });

    crate::op!(Exp2 for<T> {
        eval: |[x]| { x.exp2() },
        partial: |[x], _idx| { x.exp2() * two::<T>().ln() },
    });

    crate::op!(Expm1 for<T> {
        eval: |[x]| { x.exp_m1() },
        partial: |[x], _idx| { x.exp() },
    });

    crate::op!(Sqrt for<T> {
        eval: |[x]| { x.sqrt() },
        partial: |[x], _idx| { T::one() / (two::<T>() * x.sqrt()) },
    });

    crate::op!(Cbrt for<T> {
        eval: |[x]| { x.cbrt() },
        partial: |[x], _idx| {
            T::one() / (three::<T>() * x.cbrt().powi(2))
        },
    });

    crate::op!(Abs for<T> {
        eval: |[x]| { x.abs() },
        partial: |[x], _idx| {
            if x > T::zero() { T::one() } else if x < T::zero() { -T::one() } else { T::zero() }
        },
    });

    crate::op!(Abs2 for<T> {
        eval: |[x]| { x * x },
        partial: |[x], _idx| { two::<T>() * x },
    });

    crate::op!(Inv for<T> {
        eval: |[x]| { x.recip() },
        partial: |[x], _idx| { -T::one() / (x * x) },
    });

    crate::op!(Sign for<T> {
        eval: |[x]| { x.signum() },
        partial: |[_x], _idx| { T::zero() },
    });

    crate::op!(Identity for<T> {
        eval: |[x]| { x },
        partial: |[_x], _idx| { T::one() },
    });

    crate::op!(Neg for<T> {
        infix: "-",
        eval: |[x]| { -x },
        partial: |[_x], _idx| { -T::one() },
    });

    crate::op!(Add for<T> {
        infix: "+",
        commutative: true,
        associative: true,
        eval: |[x, y]| { x + y },
        partial: |[_x, _y], _idx| { T::one() },
    });

    crate::op!(Sub for<T> {
        infix: "-",
        eval: |[x, y]| { x - y },
        partial: |[_x, _y], idx| { if idx == 0 { T::one() } else { -T::one() } },
    });

    crate::op!(Mul for<T> {
        infix: "*",
        commutative: true,
        associative: true,
        eval: |[x, y]| { x * y },
        partial: |[x, y], idx| { if idx == 0 { y } else { x } },
    });

    crate::op!(Div for<T> {
        infix: "/",
        eval: |[x, y]| { x / y },
        partial: |[x, y], idx| { if idx == 0 { T::one() / y } else { -x / (y * y) } },
    });

    crate::op!(Pow for<T> {
        eval: |[x, y]| { x.powf(y) },
        partial: |[x, y], idx| {
            if idx == 0 { y * x.powf(y - T::one()) } else { x.powf(y) * x.ln() }
        },
    });

    crate::op!(Atan2 for<T> {
        eval: |[y, x]| { y.atan2(x) },
        partial: |[y, x], idx| {
            let denom = x * x + y * y;
            if idx == 0 { x / denom } else { -y / denom }
        },
    });

    crate::op!(Min for<T> {
        commutative: true,
        associative: true,
        eval: |[x, y]| { x.min(y) },
        partial: |[x, y], idx| {
            if x.is_nan() && y.is_nan() {
                half::<T>()
            } else if x.is_nan() {
                if idx == 0 { T::zero() } else { T::one() }
            } else if y.is_nan() {
                if idx == 0 { T::one() } else { T::zero() }
            } else if idx == 0 {
                if x < y { T::one() } else if x > y { T::zero() } else { half::<T>() }
            } else {
                if y < x { T::one() } else if y > x { T::zero() } else { half::<T>() }
            }
        },
    });

    crate::op!(Max for<T> {
        commutative: true,
        associative: true,
        eval: |[x, y]| { x.max(y) },
        partial: |[x, y], idx| {
            if x.is_nan() && y.is_nan() {
                half::<T>()
            } else if x.is_nan() {
                if idx == 0 { T::zero() } else { T::one() }
            } else if y.is_nan() {
                if idx == 0 { T::one() } else { T::zero() }
            } else if idx == 0 {
                if x > y { T::one() } else if x < y { T::zero() } else { half::<T>() }
            } else {
                if y > x { T::one() } else if y < x { T::zero() } else { half::<T>() }
            }
        },
    });

    crate::op!(Fma for<T> {
        eval: |[x, y, z]| { x.mul_add(y, z) },
        partial: |[x, y, _z], idx| { if idx == 0 { y } else if idx == 1 { x } else { T::one() } },
    });

    crate::op!(Clamp for<T> {
        eval: |[x, lo, hi]| {
            // `Float::clamp` may panic if `lo > hi`, but SR might violate this,
            // so we handle it with a NaN.
            if lo <= hi {
                x.clamp(lo, hi)
            } else {
                T::nan()
            }
        },
        partial: |[x, lo, hi], idx| {
            if idx == 0 {
                if x < lo || x > hi { T::zero() } else { T::one() }
            } else if idx == 1 {
                if x < lo { T::one() } else { T::zero() }
            } else {
                if x > hi { T::one() } else { T::zero() }
            }
        },
    });
}

pub mod presets {
    use crate::operator_enum::builtin;
    use crate::opset;

    // A convenient, batteries-included opset so downstream crates (like `symbolic_regression`)
    // don't need to define their own `Ops` type for common scalar use cases.

    opset! {
        pub struct BuiltinOpsF32<f32> {
            1 => {
                Abs = builtin::Abs, Abs2 = builtin::Abs2, Acos = builtin::Acos, Acosh = builtin::Acosh,
                Asin = builtin::Asin, Asinh = builtin::Asinh, Atan = builtin::Atan, Atanh = builtin::Atanh,
                Cbrt = builtin::Cbrt, Cos = builtin::Cos, Cosh = builtin::Cosh, Cot = builtin::Cot,
                Csc = builtin::Csc, Exp = builtin::Exp, Exp2 = builtin::Exp2, Expm1 = builtin::Expm1,
                Identity = builtin::Identity, Inv = builtin::Inv, Log = builtin::Log, Log1p = builtin::Log1p,
                Log2 = builtin::Log2, Log10 = builtin::Log10, Neg = builtin::Neg, Sec = builtin::Sec,
                Sign = builtin::Sign, Sin = builtin::Sin, Sinh = builtin::Sinh, Sqrt = builtin::Sqrt,
                Tan = builtin::Tan, Tanh = builtin::Tanh,
            }
            2 => {
                Add = builtin::Add, Atan2 = builtin::Atan2, Div = builtin::Div, Max = builtin::Max,
                Min = builtin::Min, Mul = builtin::Mul, Pow = builtin::Pow, Sub = builtin::Sub,
            }
            3 => { Clamp = builtin::Clamp, Fma = builtin::Fma, }
        }
    }

    opset! {
        pub struct BuiltinOpsF64<f64> {
            1 => {
                Abs = builtin::Abs, Abs2 = builtin::Abs2, Acos = builtin::Acos, Acosh = builtin::Acosh,
                Asin = builtin::Asin, Asinh = builtin::Asinh, Atan = builtin::Atan, Atanh = builtin::Atanh,
                Cbrt = builtin::Cbrt, Cos = builtin::Cos, Cosh = builtin::Cosh, Cot = builtin::Cot,
                Csc = builtin::Csc, Exp = builtin::Exp, Exp2 = builtin::Exp2, Expm1 = builtin::Expm1,
                Identity = builtin::Identity, Inv = builtin::Inv, Log = builtin::Log, Log1p = builtin::Log1p,
                Log2 = builtin::Log2, Log10 = builtin::Log10, Neg = builtin::Neg, Sec = builtin::Sec,
                Sign = builtin::Sign, Sin = builtin::Sin, Sinh = builtin::Sinh, Sqrt = builtin::Sqrt,
                Tan = builtin::Tan, Tanh = builtin::Tanh,
            }
            2 => {
                Add = builtin::Add, Atan2 = builtin::Atan2, Div = builtin::Div, Max = builtin::Max,
                Min = builtin::Min, Mul = builtin::Mul, Pow = builtin::Pow, Sub = builtin::Sub,
            }
            3 => { Clamp = builtin::Clamp, Fma = builtin::Fma, }
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Operator set macros
// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! opset {
    // Compute the maximum arity.
    (@max_arity $a:literal) => { $a as u8 };
    (@max_arity $a:literal, $($rest:literal),+ $(,)?) => {
        if $a as u8 > $crate::opset!(@max_arity $($rest),+) {
            $a as u8
        } else {
            $crate::opset!(@max_arity $($rest),+)
        }
    };

    // Operator type resolution:
    // - Default: the type is `OpName` (must be in scope)
    // - Override per op: `OpName = some::path::Type`
    (@op_ty $op_name:ident) => { $op_name };
    (@op_ty $op_name:ident = $op_path:path) => { $op_path };

    (
        $(#[$meta:meta])* $vis:vis struct $Ops:ident<$t:ty> {
            $(
                $arity:literal => { $($op_name:ident $(= $op_path:path)?),* $(,)? }
            )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug, Default)]
        $vis struct $Ops;

        $crate::paste::paste! {
            $(
                #[repr(u16)]
                #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
                #[allow(non_camel_case_types)]
                enum [<__ $Ops _op_id_ $arity>] { $($op_name,)* }
            )*
        }

        impl $crate::traits::OperatorSet for $Ops {
            type T = $t;
            const MAX_ARITY: u8 = $crate::opset!(@max_arity $($arity),*);

            fn ops_with_arity(arity: u8) -> &'static [u16] {
                match arity {
                    $(
                        $arity => &[$( $crate::paste::paste! { [<__ $Ops _op_id_ $arity>]::$op_name as u16 }, )*],
                    )*
                    _ => &[],
                }
            }

            fn meta(op: $crate::traits::OpId) -> Option<&'static $crate::traits::OpMeta> {
                match op.arity {
                    $(
                        $arity => {
                            const META: &[$crate::traits::OpMeta] = &[
                                $(
                                    $crate::traits::OpMeta {
                                        arity: $arity as u8,
                                        id: $crate::paste::paste! { [<__ $Ops _op_id_ $arity>]::$op_name as u16 },
                                        name: <$crate::opset!(@op_ty $op_name $(= $op_path)?) as $crate::traits::Operator<$t, $arity>>::NAME,
                                        display: <$crate::opset!(@op_ty $op_name $(= $op_path)?) as $crate::traits::Operator<$t, $arity>>::DISPLAY,
                                        infix: <$crate::opset!(@op_ty $op_name $(= $op_path)?) as $crate::traits::Operator<$t, $arity>>::INFIX,
                                        commutative: <$crate::opset!(@op_ty $op_name $(= $op_path)?) as $crate::traits::Operator<$t, $arity>>::COMMUTATIVE,
                                        associative: <$crate::opset!(@op_ty $op_name $(= $op_path)?) as $crate::traits::Operator<$t, $arity>>::ASSOCIATIVE,
                                        complexity: <$crate::opset!(@op_ty $op_name $(= $op_path)?) as $crate::traits::Operator<$t, $arity>>::COMPLEXITY,
                                    },
                                )*
                            ];
                            META.get(op.id as usize)
                        }
                    )*
                    _ => None,
                }
            }

            fn eval(op: $crate::traits::OpId, ctx: $crate::dispatch::EvalKernelCtx<'_, '_, $t>) -> bool {
                match op.arity {
                    $(
                        $arity => match op.id {
                            $(
                                x if x == ($crate::paste::paste! { [<__ $Ops _op_id_ $arity>]::$op_name as u16 }) =>
                                    $crate::evaluate::kernels::eval_nary::<$arity, $t, $crate::opset!(@op_ty $op_name $(= $op_path)?)>(
                                        ctx.out,
                                        ctx.args,
                                        ctx.opts,
                                    ),
                            )*
                            _ => panic!("unknown op id {} for arity {}", op.id, op.arity),
                        },
                    )*
                    _ => panic!("unsupported arity {}", op.arity),
                }
            }

            fn diff(op: $crate::traits::OpId, ctx: $crate::dispatch::DiffKernelCtx<'_, '_, $t>) -> bool {
                match op.arity {
                    $(
                        $arity => match op.id {
                            $(
                                x if x == ($crate::paste::paste! { [<__ $Ops _op_id_ $arity>]::$op_name as u16 }) =>
                                    $crate::evaluate::kernels::diff_nary::<$arity, $t, $crate::opset!(@op_ty $op_name $(= $op_path)?)>(
                                        ctx.out_val,
                                        ctx.out_der,
                                        ctx.args,
                                        ctx.dargs,
                                        ctx.opts,
                                    ),
                            )*
                            _ => panic!("unknown op id {} for arity {}", op.id, op.arity),
                        },
                    )*
                    _ => panic!("unsupported arity {}", op.arity),
                }
            }

            fn grad(op: $crate::traits::OpId, ctx: $crate::dispatch::GradKernelCtx<'_, '_, $t>) -> bool {
                match op.arity {
                    $(
                        $arity => match op.id {
                            $(
                                x if x == ($crate::paste::paste! { [<__ $Ops _op_id_ $arity>]::$op_name as u16 }) =>
                                    $crate::evaluate::kernels::grad_nary::<$arity, $t, $crate::opset!(@op_ty $op_name $(= $op_path)?)>(ctx),
                            )*
                            _ => panic!("unknown op id {} for arity {}", op.id, op.arity),
                        },
                    )*
                    _ => panic!("unsupported arity {}", op.arity),
                }
            }
        }

        $(
            $(
                impl $crate::traits::HasOp<$crate::opset!(@op_ty $op_name $(= $op_path)?)> for $Ops {
                    const ID: u16 = $crate::paste::paste! { [<__ $Ops _op_id_ $arity>]::$op_name as u16 };
                }
            )*
        )*
    };
}
