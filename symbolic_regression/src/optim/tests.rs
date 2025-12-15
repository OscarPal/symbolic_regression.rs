use super::bfgs::{bfgs_minimize, newton_1d_minimize};
use super::line_search::{backtracking_linesearch, LineSearchInput};
use super::{BackTracking, EvalBudget, Objective, OptimOptions};

struct Quad2D;

impl Objective for Quad2D {
    fn f_only(&mut self, x: &[f64], budget: &mut EvalBudget) -> Option<f64> {
        budget.f_calls += 1;
        let t0 = x[0] - 1.0;
        let t1 = x[1] + 2.0;
        Some(t0 * t0 + 10.0 * t1 * t1)
    }

    fn fg(&mut self, x: &[f64], g_out: &mut [f64], budget: &mut EvalBudget) -> Option<f64> {
        budget.f_calls += 1;
        let t0 = x[0] - 1.0;
        let t1 = x[1] + 2.0;
        g_out[0] = 2.0 * t0;
        g_out[1] = 20.0 * t1;
        Some(t0 * t0 + 10.0 * t1 * t1)
    }
}

struct Quad1D;

impl Objective for Quad1D {
    fn f_only(&mut self, x: &[f64], budget: &mut EvalBudget) -> Option<f64> {
        budget.f_calls += 1;
        let t = x[0] - 3.0;
        Some(t * t)
    }

    fn fg(&mut self, x: &[f64], g_out: &mut [f64], budget: &mut EvalBudget) -> Option<f64> {
        budget.f_calls += 1;
        let t = x[0] - 3.0;
        let f = t * t;
        g_out[0] = 2.0 * t;
        Some(f)
    }
}

#[test]
fn backtracking_satisfies_armijo_for_quadratic() {
    let ls = BackTracking::default();
    let x = [0.0];
    let s = [6.0];
    let phi0 = 9.0;
    let dphi0 = -36.0;
    let mut x_new = [0.0];
    let mut phi_at = |xv: &[f64]| {
        let t = xv[0] - 3.0;
        Some(t * t)
    };
    let (alpha, phi) = backtracking_linesearch(
        &ls,
        LineSearchInput {
            x: &x,
            s: &s,
            alpha0: 1.0,
            phi0,
            dphi0,
        },
        &mut x_new,
        &mut phi_at,
    )
    .expect("line search failed");
    assert!(alpha > 0.0 && alpha < 1.0);
    assert!(phi <= phi0 + ls.c1 * alpha * dphi0);
}

#[test]
fn bfgs_minimizes_simple_quadratic() {
    let opts = OptimOptions {
        iterations: 50,
        f_calls_limit: 0,
        g_abstol: 1e-10,
    };
    let ls = BackTracking::default();
    let mut obj = Quad2D;
    let res = bfgs_minimize(&[0.0, 0.0], &mut obj, opts, ls).unwrap();
    assert!((res.minimizer[0] - 1.0).abs() < 1e-6);
    assert!((res.minimizer[1] + 2.0).abs() < 1e-6);
}

#[test]
fn newton_1d_minimizes_quadratic() {
    let opts = OptimOptions {
        iterations: 25,
        f_calls_limit: 0,
        g_abstol: 1e-10,
    };
    let ls = BackTracking::default();
    let mut obj = Quad1D;
    let res = newton_1d_minimize(0.0, &mut obj, opts, ls).unwrap();
    assert!((res.minimizer[0] - 3.0).abs() < 1e-6);
}

struct Quad3DOffDiag;

impl Objective for Quad3DOffDiag {
    fn f_only(&mut self, x: &[f64], budget: &mut EvalBudget) -> Option<f64> {
        budget.f_calls += 1;
        // f(x) = 0.5 x^T A x - b^T x
        // A = [[2,1,0],[1,2,0],[0,0,3]] (SPD), b = [1,0,2]
        let a00 = 2.0;
        let a01 = 1.0;
        let a11 = 2.0;
        let a22 = 3.0;
        let b0 = 1.0;
        let b1 = 0.0;
        let b2 = 2.0;

        let x0 = x[0];
        let x1 = x[1];
        let x2 = x[2];

        let ax0 = a00 * x0 + a01 * x1;
        let ax1 = a01 * x0 + a11 * x1;
        let ax2 = a22 * x2;

        let xtax = x0 * ax0 + x1 * ax1 + x2 * ax2;
        Some(0.5 * xtax - (b0 * x0 + b1 * x1 + b2 * x2))
    }

    fn fg(&mut self, x: &[f64], g_out: &mut [f64], budget: &mut EvalBudget) -> Option<f64> {
        budget.f_calls += 1;
        let a00 = 2.0;
        let a01 = 1.0;
        let a11 = 2.0;
        let a22 = 3.0;
        let b0 = 1.0;
        let b1 = 0.0;
        let b2 = 2.0;

        let x0 = x[0];
        let x1 = x[1];
        let x2 = x[2];

        let ax0 = a00 * x0 + a01 * x1;
        let ax1 = a01 * x0 + a11 * x1;
        let ax2 = a22 * x2;

        g_out[0] = ax0 - b0;
        g_out[1] = ax1 - b1;
        g_out[2] = ax2 - b2;

        let xtax = x0 * ax0 + x1 * ax1 + x2 * ax2;
        Some(0.5 * xtax - (b0 * x0 + b1 * x1 + b2 * x2))
    }
}

#[test]
fn bfgs_minimizes_spd_quadratic_with_off_diagonal() {
    let opts = OptimOptions {
        iterations: 100,
        f_calls_limit: 0,
        g_abstol: 1e-10,
    };
    let ls = BackTracking::default();
    let mut obj = Quad3DOffDiag;
    let res = bfgs_minimize(&[0.5, -0.5, 0.0], &mut obj, opts, ls).unwrap();
    // Solve A x = b:
    // [2 1] [x0] = [1]  => x0=2/3, x1=-1/3 ; x2 = 2/3
    assert!((res.minimizer[0] - (2.0 / 3.0)).abs() < 1e-6);
    assert!((res.minimizer[1] - (-1.0 / 3.0)).abs() < 1e-6);
    assert!((res.minimizer[2] - (2.0 / 3.0)).abs() < 1e-6);
    assert!(res.minimum.is_finite());
}
