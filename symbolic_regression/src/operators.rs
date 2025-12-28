use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;

use dynamic_expressions::operator_enum::builtin;
use dynamic_expressions::{HasOp, OpId, OperatorSet};
use fastrand::Rng;

use crate::random::usize_range;

#[derive(Clone, Debug)]
pub struct Operators<const D: usize> {
    pub ops_by_arity: [Vec<OpId>; D],
}

#[derive(Debug, Clone)]
pub enum OperatorSelectError {
    Lookup(dynamic_expressions::LookupError),
    ArityMismatch { token: String, expected: u8, found: u8 },
    ArityTooLarge { token: String, arity: u8, max_arity: usize },
    Duplicate(String),
    Empty,
}

impl fmt::Display for OperatorSelectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperatorSelectError::Lookup(e) => write!(f, "{e:?}"),
            OperatorSelectError::ArityMismatch { token, expected, found } => write!(
                f,
                "operator token {token:?} has arity={found} but was provided for arity={expected}"
            ),
            OperatorSelectError::ArityTooLarge {
                token,
                arity,
                max_arity,
            } => write!(
                f,
                "operator token {token:?} has arity={arity} which exceeds D={max_arity}"
            ),
            OperatorSelectError::Duplicate(tok) => write!(f, "duplicate operator token {tok:?}"),
            OperatorSelectError::Empty => write!(f, "no operators provided"),
        }
    }
}

impl std::error::Error for OperatorSelectError {}

impl<const D: usize> Operators<D> {
    pub fn new() -> Self {
        Self {
            ops_by_arity: std::array::from_fn(|_| Vec::new()),
        }
    }

    pub fn push(&mut self, op: OpId) {
        let arity = op.arity as usize;
        assert!((1..=D).contains(&arity));
        self.ops_by_arity[arity - 1].push(op);
    }

    pub fn nops(&self, arity: usize) -> usize {
        self.ops_by_arity[arity - 1].len()
    }

    pub fn total_ops_up_to(&self, max_arity: usize) -> usize {
        let max_arity = max_arity.min(D);
        (1..=max_arity).map(|a| self.nops(a)).sum()
    }

    pub fn sample_arity(&self, rng: &mut Rng, max_arity: usize) -> usize {
        let max_arity = max_arity.min(D);
        let total: usize = (1..=max_arity).map(|a| self.nops(a)).sum();
        assert!(total > 0, "no operators available up to arity={max_arity}");
        let mut r = usize_range(rng, 0..total);
        for arity in 1..=max_arity {
            let n = self.nops(arity);
            if r < n {
                return arity;
            }
            r -= n;
        }
        unreachable!()
    }

    pub fn sample_op(&self, rng: &mut Rng, arity: usize) -> OpId {
        let v = &self.ops_by_arity[arity - 1];
        let i = usize_range(rng, 0..v.len());
        v[i]
    }

    pub fn from_names<Ops: OperatorSet>(names: &[&str]) -> Result<Self, OperatorSelectError> {
        if names.is_empty() {
            return Err(OperatorSelectError::Empty);
        }
        let mut out = Operators::new();
        let mut seen: HashSet<(u8, u16)> = HashSet::new();

        for &tok in names {
            let op = Ops::lookup(tok).map_err(OperatorSelectError::Lookup)?;
            if (op.arity as usize) > D {
                return Err(OperatorSelectError::ArityTooLarge {
                    token: tok.to_string(),
                    arity: op.arity,
                    max_arity: D,
                });
            }
            let key = (op.arity, op.id);
            if !seen.insert(key) {
                return Err(OperatorSelectError::Duplicate(tok.to_string()));
            }
            out.push(op);
        }
        Ok(out)
    }

    pub fn from_names_by_arity<Ops: OperatorSet>(names_by_arity: [&[&str]; D]) -> Result<Self, OperatorSelectError> {
        if names_by_arity.iter().all(|toks| toks.is_empty()) {
            return Err(OperatorSelectError::Empty);
        }

        let mut out = Operators::new();
        let mut seen: HashSet<(u8, u16)> = HashSet::new();

        for (arity_minus_one, toks) in names_by_arity.iter().enumerate() {
            let expected: u8 = (arity_minus_one + 1).try_into().expect("arity fits in u8");
            for &tok in *toks {
                let op = Ops::lookup_with_arity(tok, expected).map_err(OperatorSelectError::Lookup)?;
                if op.arity != expected {
                    return Err(OperatorSelectError::ArityMismatch {
                        token: tok.to_string(),
                        expected,
                        found: op.arity,
                    });
                }
                if (op.arity as usize) > D {
                    return Err(OperatorSelectError::ArityTooLarge {
                        token: tok.to_string(),
                        arity: op.arity,
                        max_arity: D,
                    });
                }
                let key = (op.arity, op.id);
                if !seen.insert(key) {
                    return Err(OperatorSelectError::Duplicate(tok.to_string()));
                }
                out.push(op);
            }
        }

        Ok(out)
    }
}

/// Convenience selection for any [`dynamic_expressions::OperatorSet`].
///
/// This is intentionally opinionated: `D` is inferred from the length of the passed array.
pub trait OperatorSetSelect: OperatorSet {
    fn from_names_by_arity<const D: usize>(names_by_arity: [&[&str]; D]) -> Result<Operators<D>, OperatorSelectError> {
        Operators::<D>::from_names_by_arity::<Self>(names_by_arity)
    }
}

impl<T: OperatorSet> OperatorSetSelect for T {}

impl<const D: usize> Default for Operators<D> {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! sr_ops {
    ($Ops:ty, D = $D:literal; $($arity:literal => ( $($op:path),* $(,)? ) ),* $(,)?) => {{
        let mut b = $crate::operators::Operators::<$D>::builder::<$Ops>();
        $(
            $(
                b = b.nary::<$arity, $op>();
            )*
        )*
        b.build()
    }};
}

#[derive(Clone, Debug)]
pub struct OperatorsBuilder<Ops, const D: usize> {
    operators: Operators<D>,
    _ops: PhantomData<Ops>,
}

impl<const D: usize> Operators<D> {
    pub fn builder<Ops>() -> OperatorsBuilder<Ops, D> {
        OperatorsBuilder {
            operators: Operators::new(),
            _ops: PhantomData,
        }
    }
}

impl<Ops, const D: usize> OperatorsBuilder<Ops, D> {
    pub fn build(self) -> Operators<D> {
        self.operators
    }
}

impl<Ops, const D: usize> OperatorsBuilder<Ops, D> {
    pub fn sr_default_binary(self) -> Self
    where
        Ops: HasOp<builtin::Add> + HasOp<builtin::Sub> + HasOp<builtin::Mul> + HasOp<builtin::Div> + OperatorSet,
    {
        self.binary::<builtin::Add>()
            .binary::<builtin::Sub>()
            .binary::<builtin::Mul>()
            .binary::<builtin::Div>()
    }

    pub fn unary<Op>(self) -> Self
    where
        Ops: HasOp<Op> + OperatorSet,
        Op: dynamic_expressions::OpTag,
    {
        let op = <Ops as HasOp<Op>>::op_id();
        assert_eq!(op.arity, 1, "unary() requires arity=1 (got {})", op.arity);
        self.push_op::<Op>()
    }

    pub fn binary<Op>(self) -> Self
    where
        Ops: HasOp<Op> + OperatorSet,
        Op: dynamic_expressions::OpTag,
    {
        let op = <Ops as HasOp<Op>>::op_id();
        assert_eq!(op.arity, 2, "binary() requires arity=2 (got {})", op.arity);
        self.push_op::<Op>()
    }

    fn push_op<Op>(mut self) -> Self
    where
        Ops: HasOp<Op> + OperatorSet,
        Op: dynamic_expressions::OpTag,
    {
        let op = <Ops as HasOp<Op>>::op_id();
        let arity = op.arity as usize;
        assert!(
            arity >= 1 && arity <= D,
            "operator arity {arity} not supported for D={D}"
        );
        self.operators.push(op);
        self
    }
}

#[cfg(test)]
mod tests {
    use dynamic_expressions::operator_enum::builtin;
    use dynamic_expressions::operator_enum::presets::BuiltinOpsF64;

    use super::*;

    dynamic_expressions::op!(Square for f64 {
        eval: |[x]| { x * x },
        partial: |[x], _idx| { 2.0 * x },
    });

    dynamic_expressions::opset! {
        struct V2Ops<f64> {
            1 => { Square }
        }
    }

    #[test]
    fn from_names_by_arity_resolves_dash_by_arity() {
        let empty: [&str; 0] = [];

        let unary = ["-"];
        let ops = Operators::<3>::from_names_by_arity::<BuiltinOpsF64>([&unary, &empty, &empty]).unwrap();
        assert_eq!(ops.nops(1), 1);
        assert_eq!(ops.ops_by_arity[0][0], <BuiltinOpsF64 as HasOp<builtin::Neg>>::op_id());

        let binary = ["-"];
        let ops = Operators::<3>::from_names_by_arity::<BuiltinOpsF64>([&empty, &binary, &empty]).unwrap();
        assert_eq!(ops.nops(2), 1);
        assert_eq!(ops.ops_by_arity[1][0], <BuiltinOpsF64 as HasOp<builtin::Sub>>::op_id());
    }

    #[test]
    fn v2_opset_dsl_builds_operator_set() {
        let ops = Operators::<3>::from_names::<V2Ops>(&["square"]).unwrap();
        assert_eq!(ops.nops(1), 1);
    }
}
