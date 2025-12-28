use dynamic_expressions::operator_enum::builtin::*;
use dynamic_expressions::opset;

pub type T = f64;
pub const D: usize = 2;

opset! {
    pub struct TestOps<f64> {
        1 => { Sin, Cos, Exp, Log, Neg }
        2 => { Add, Sub, Mul, Div }
    }
}
