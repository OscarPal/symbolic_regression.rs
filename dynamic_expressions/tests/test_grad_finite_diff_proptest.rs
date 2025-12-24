use dynamic_expressions::expression::{Metadata, PostfixExpr};
use dynamic_expressions::operator_enum::builtin;
use dynamic_expressions::operator_enum::presets::BuiltinOpsF64;
use dynamic_expressions::operator_enum::scalar::HasOp;
use dynamic_expressions::utils::ZipEq;
use dynamic_expressions::{EvalOptions, GradContext, eval_grad_tree_array, eval_tree_array, proptest_utils};
use ndarray::Array2;
use proptest::prelude::*;

const N_FEATURES: usize = 3;
const N_CONSTS: usize = 3;
const D_TEST: usize = 2;
const EPS: f64 = 1e-6;
const ATOL: f64 = 5e-5;
const RTOL: f64 = 5e-5;

fn safe_unary_ops() -> Vec<u16> {
    vec![
        <BuiltinOpsF64 as HasOp<builtin::Cos, 1>>::ID,
        <BuiltinOpsF64 as HasOp<builtin::Sin, 1>>::ID,
    ]
}

fn safe_binary_ops() -> Vec<u16> {
    vec![
        <BuiltinOpsF64 as HasOp<builtin::Add, 2>>::ID,
        <BuiltinOpsF64 as HasOp<builtin::Sub, 2>>::ID,
        <BuiltinOpsF64 as HasOp<builtin::Mul, 2>>::ID,
    ]
}

fn eval_with_x(expr: &PostfixExpr<f64, BuiltinOpsF64, D_TEST>, x_data: &[f64], n_rows: usize) -> Vec<f64> {
    let x = Array2::from_shape_vec((N_FEATURES, n_rows), x_data.to_vec()).unwrap();
    let opts = EvalOptions {
        check_finite: true,
        early_exit: true,
    };
    let (y, ok) = eval_tree_array::<f64, BuiltinOpsF64, D_TEST>(expr, x.view(), &opts);
    assert!(ok);
    y
}

fn finite_diff_feature(
    expr: &PostfixExpr<f64, BuiltinOpsF64, D_TEST>,
    x_data: &[f64],
    n_rows: usize,
    dir: usize,
) -> Vec<f64> {
    let mut plus = x_data.to_vec();
    let mut minus = x_data.to_vec();
    for row in 0..n_rows {
        plus[dir * n_rows + row] += EPS;
        minus[dir * n_rows + row] -= EPS;
    }
    let y_plus = eval_with_x(expr, &plus, n_rows);
    let y_minus = eval_with_x(expr, &minus, n_rows);
    y_plus
        .iter()
        .zip_eq(&y_minus)
        .map(|(a, b)| (a - b) / (2.0 * EPS))
        .collect()
}

fn finite_diff_const(
    expr: &PostfixExpr<f64, BuiltinOpsF64, D_TEST>,
    x_data: &[f64],
    n_rows: usize,
    const_idx: usize,
) -> Vec<f64> {
    let mut plus = expr.consts.clone();
    let mut minus = expr.consts.clone();
    plus[const_idx] += EPS;
    minus[const_idx] -= EPS;

    let mut expr_plus = expr.clone();
    expr_plus.consts = plus;
    let mut expr_minus = expr.clone();
    expr_minus.consts = minus;

    let y_plus = eval_with_x(&expr_plus, x_data, n_rows);
    let y_minus = eval_with_x(&expr_minus, x_data, n_rows);
    y_plus
        .iter()
        .zip_eq(&y_minus)
        .map(|(a, b)| (a - b) / (2.0 * EPS))
        .collect()
}

fn assert_close(a: &[f64], b: &[f64]) {
    for (&av, &bv) in a.iter().zip_eq(b) {
        let diff = (av - bv).abs();
        let tol = ATOL + RTOL * av.abs().max(bv.abs());
        assert!(diff <= tol, "diff {diff} > tol {tol} (a={av}, b={bv})");
    }
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 64, .. ProptestConfig::default() })]

    #[test]
    fn grad_matches_finite_diff_variables(
        nodes in proptest_utils::arb_postfix_nodes(
            N_FEATURES,
            N_CONSTS,
            safe_unary_ops(),
            safe_binary_ops(),
            Vec::new(),
            5,
            64,
            8,
        ),
        consts in prop::collection::vec(-1.0f64..=1.0, N_CONSTS),
        (n_rows, x_data) in (1usize..5).prop_flat_map(|n_rows| {
            prop::collection::vec(-1.0f64..=1.0, N_FEATURES * n_rows)
                .prop_map(move |x| (n_rows, x))
        }),
    ) {
        let expr = PostfixExpr::new(nodes, consts, Metadata::default());
        let x: Array2<f64> = Array2::from_shape_vec((N_FEATURES, n_rows), x_data.clone()).unwrap();
        let opts = EvalOptions { check_finite: true, early_exit: true };

        let mut gctx = GradContext::<f64, D_TEST>::new(n_rows);
        let (_eval, grad, ok) = eval_grad_tree_array::<f64, BuiltinOpsF64, D_TEST>(
            &expr,
            x.view(),
            true,
            &mut gctx,
            &opts,
        );
        prop_assert!(ok);
        prop_assert_eq!(grad.n_dir, N_FEATURES);
        prop_assert_eq!(grad.n_rows, n_rows);

        for dir in 0..N_FEATURES {
            let fd = finite_diff_feature(&expr, &x_data, n_rows, dir);
            let grad_dir = &grad.data[dir * n_rows..(dir + 1) * n_rows];
            assert_close(grad_dir, &fd);
        }
    }

    #[test]
    fn grad_matches_finite_diff_constants(
        nodes in proptest_utils::arb_postfix_nodes(
            N_FEATURES,
            N_CONSTS,
            safe_unary_ops(),
            safe_binary_ops(),
            Vec::new(),
            5,
            64,
            8,
        ),
        consts in prop::collection::vec(-1.0f64..=1.0, N_CONSTS),
        (n_rows, x_data) in (1usize..5).prop_flat_map(|n_rows| {
            prop::collection::vec(-1.0f64..=1.0, N_FEATURES * n_rows)
                .prop_map(move |x| (n_rows, x))
        }),
    ) {
        let expr = PostfixExpr::new(nodes, consts, Metadata::default());
        let x: Array2<f64> = Array2::from_shape_vec((N_FEATURES, n_rows), x_data.clone()).unwrap();
        let opts = EvalOptions { check_finite: true, early_exit: true };

        let mut gctx = GradContext::<f64, D_TEST>::new(n_rows);
        let (_eval, grad, ok) = eval_grad_tree_array::<f64, BuiltinOpsF64, D_TEST>(
            &expr,
            x.view(),
            false,
            &mut gctx,
            &opts,
        );
        prop_assert!(ok);
        prop_assert_eq!(grad.n_dir, N_CONSTS);
        prop_assert_eq!(grad.n_rows, n_rows);

        for cidx in 0..N_CONSTS {
            let fd = finite_diff_const(&expr, &x_data, n_rows, cidx);
            let grad_dir = &grad.data[cidx * n_rows..(cidx + 1) * n_rows];
            assert_close(grad_dir, &fd);
        }
    }
}
