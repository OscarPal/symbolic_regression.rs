use dynamic_expressions::operator_enum::builtin::*;
use dynamic_expressions::opset;

pub type T = f64;
pub const D: usize = 2;

opset! {
    pub TestOps for f64 { Sin, Cos, Exp, Log, Neg, Add, Sub, Mul, Div }
}
