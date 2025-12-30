//! # Linear programming solver
//!
//! Implements the [simplex method](<https://en.wikipedia.org/wiki/Simplex_algorithm>) to solve a linear programming problem.
#![allow(unused)]

use anyhow::Result;
use approx::{abs_diff_eq, relative_eq};
use colored::Colorize;
use itertools::Itertools;
use nalgebra::{ComplexField, Const, DMatrix, DVector, Dyn, OMatrix, U1, Vector1, stack};
use num::rational::Rational64;
use num::{Signed, Zero};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LPOps {
    Eq,
    Gte,
    Lte,
}

fn to_rational(x: i64) -> Rational64 {
    Rational64::from_integer(x)
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct LPBuilder {
    pub(crate) objective: Vec<i64>,
    pub(crate) constraints: Vec<Vec<i64>>,
    pub(crate) ans: Vec<i64>,
    pub(crate) ops: Vec<LPOps>,
}

impl LPBuilder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add_objective(&mut self, objective: Vec<i64>) -> &mut Self {
        self.objective = objective;
        self
    }

    pub(crate) fn add_constraint(
        &mut self,
        mut constraint: Vec<i64>,
        mut op: LPOps,
        mut ans: i64,
    ) -> &mut Self {
        // Need to ensure that RHS of constraint is positive
        if ans < 0 {
            ans *= -1;
            for term in constraint.iter_mut() {
                *term *= -1;
            }
            op = match op {
                LPOps::Eq => LPOps::Eq,
                LPOps::Gte => LPOps::Lte,
                LPOps::Lte => LPOps::Gte,
            }
        }

        let constraint_exists = self
            .constraints
            .iter()
            .enumerate()
            .filter(|&(ind, c)| c == &constraint)
            .map(|(ind, _)| ind)
            .any(|ind| (self.ops[ind] == op && self.ans[ind] == ans));

        if !constraint_exists {
            self.constraints.push(constraint);
            self.ans.push(ans);
            self.ops.push(op);
        }
        self
    }

    pub(crate) fn build(&self) -> LinearProgrammingProblem {
        let m = self.constraints.len();
        let n_x = self.constraints.first().map(|v| v.len()).unwrap_or(0);

        let n_slack = self
            .ops
            .iter()
            .filter(|&op| matches!(op, LPOps::Lte | LPOps::Gte))
            .count();

        let n_art = self
            .ops
            .iter()
            .filter(|&op| matches!(op, LPOps::Gte | LPOps::Eq))
            .count();

        let slack_start = n_x;
        let art_start = slack_start + n_slack;
        let z_col = art_start + n_art;
        let w_col = z_col + 1;
        let rhs_col = w_col + 1;

        let total_cols = rhs_col + 1;
        let total_rows = m + 2;

        let p2_row = m;
        let p1_row = m + 1;

        let mut t = vec![vec![Rational64::zero(); total_cols]; total_rows];
        let mut active = vec![usize::MAX; total_rows];

        let mut slack_j = slack_start;
        let mut art_j = art_start;

        // Constraints
        for i in 0..m {
            for (j, tableau_cell) in t[i].iter_mut().enumerate().take(n_x) {
                *tableau_cell = Rational64::from_integer(self.constraints[i][j]);
            }

            match self.ops[i] {
                LPOps::Lte => {
                    t[i][slack_j] = Rational64::ONE;
                    active[i] = slack_j;
                    slack_j += 1;
                }
                LPOps::Gte => {
                    // surplus -1 and artificial +1, basic is artificial
                    t[i][slack_j] = -Rational64::ONE;
                    slack_j += 1;

                    t[i][art_j] = Rational64::ONE;
                    active[i] = art_j;
                    art_j += 1;
                }
                LPOps::Eq => {
                    t[i][art_j] = Rational64::ONE;
                    active[i] = art_j;
                    art_j += 1;
                }
            }

            t[i][rhs_col] = Rational64::from_integer(self.ans[i])
        }

        // Phase 2 objective: -c^T x + z = 0
        for (j, tableau_cell) in t[p2_row].iter_mut().enumerate().take(n_x) {
            *tableau_cell = Rational64::from_integer(-self.objective[j]);
        }
        t[p2_row][z_col] = Rational64::ONE;
        active[p2_row] = z_col;

        // Phase 1 objective: (sum artificials) + w = 0  => w = -sum a
        // Initialise the coefficients on artificials to +1, and w to +1.
        for tableau_cell in t[p1_row].iter_mut().take(z_col).skip(art_start) {
            *tableau_cell = Rational64::ONE;
        }
        t[p1_row][w_col] = Rational64::ONE;
        active[p1_row] = w_col;

        let mut lp = LinearProgrammingProblem {
            tableau: t,
            n_constraints: m,
            slack_var_start: slack_start,
            artificial_var_start: art_start,
            z_col,
            rhs_col,
            active,
        };

        // "Price out" phase 1 objective with respect to the initial basis:
        // For each constraint row where an artificial is basic, eliminate it from the phase 1 row.
        for i in 0..m {
            let bc = lp.active[i];
            if bc >= lp.artificial_var_start && bc < lp.z_col {
                // Phase1 has +1 at this artificial; subtract the row to make it 0.
                lp.row_add_scaled(p1_row, i, -lp.tableau[p1_row][bc]);
            }
        }

        lp
    }
}

fn _subscript_variable(variable: char, ind: usize) -> String {
    let ind_str = ind
        .to_string()
        .chars()
        .map(|c| match c {
            '0' => '\u{2080}',
            '1' => '\u{2081}',
            '2' => '\u{2082}',
            '3' => '\u{2083}',
            '4' => '\u{2084}',
            '5' => '\u{2085}',
            '6' => '\u{2086}',
            '7' => '\u{2087}',
            '8' => '\u{2088}',
            '9' => '\u{2089}',
            _ => c,
        })
        .collect::<String>();
    format!("{}{}", variable, ind_str)
}

fn _pretty_print_variable(variable: char, ind: usize, term: f64) -> String {
    let variable = _subscript_variable(variable, ind);

    if term == 1_f64 {
        variable.to_string()
    } else if term == -1_f64 {
        format!("-{}", variable)
    } else {
        format!("{}{}", term, variable)
    }
}

/// Solve LP problem using normal simplex method.
pub struct LinearProgrammingProblem {
    /// The simplex tableau.
    tableau: Vec<Vec<Rational64>>,
    /// Number of constraints
    n_constraints: usize,
    /// Index where slack variables start.
    slack_var_start: usize,
    /// Index where artificial variables start.
    artificial_var_start: usize,
    z_col: usize,
    rhs_col: usize,
    active: Vec<usize>,
}

pub enum SimplexResult {
    Optimal,
    Unbounded,
}

impl LinearProgrammingProblem {
    fn is_basic_in_constraints(&self, col: usize) -> bool {
        self.active
            .iter()
            .take(self.n_constraints)
            .any(|&bc| bc == col)
    }

    fn rhs(&self, row: usize) -> Rational64 {
        self.tableau[row][self.rhs_col]
    }

    fn row_add_scaled(&mut self, dst: usize, src: usize, scale: Rational64) {
        if scale.is_zero() {
            return;
        }
        for j in 0..self.tableau[dst].len() {
            let v = scale * self.tableau[src][j];
            self.tableau[dst][j] += v;
        }
    }

    /// Choose entering variable as the most negative coefficient in the objective row
    /// among allowed columns, excluding `z`, `w`, `rhs`, and columns currently basic.
    fn pivot_col(&self, obj_row: usize) -> Option<usize> {
        self.tableau[obj_row][0..self.artificial_var_start]
            .iter()
            .enumerate()
            .filter(|&(col, coeff)| !self.is_basic_in_constraints(col) && coeff.is_negative())
            .min_by_key(|&(_, coeff)| *coeff)
            .map(|(col, _)| col)
    }

    /// Choose leaving row by minimum ratio test among constraint rows with positive pivot column coefficient.
    fn pivot_row(&self, enter_col: usize) -> Option<usize> {
        (0..self.n_constraints)
            .map(|i| (i, self.tableau[i][enter_col]))
            .filter(|(_, a)| a.is_positive())
            .min_by_key(|&(i, a)| self.rhs(i) / a)
            .map(|(i, _)| i)
    }

    /// Correct Gauss-Jordan simplex pivot:
    /// - Normalise the pivot row so the pivot element becomes 1.
    /// - Eliminate the entering column from all other rows.
    fn pivot(&mut self, pr: usize, pc: usize) {
        let pivot = self.tableau[pr][pc];
        assert!(!pivot.is_zero(), "pivot element must be non-zero");

        // Normalize pivot row
        let n_cols = self.tableau[pr].len();
        for j in 0..n_cols {
            self.tableau[pr][j] /= pivot;
        }

        // Eliminate pivot column in all other rows
        let pivot_row = self.tableau[pr].clone();
        for i in 0..self.tableau.len() {
            if i == pr {
                continue;
            }
            let factor = self.tableau[i][pc];
            if factor.is_zero() {
                continue;
            }
            for (j, pivot) in pivot_row.iter().enumerate().take(n_cols) {
                self.tableau[i][j] -= factor * pivot;
            }
        }

        self.active[pr] = pc;
    }

    fn remove_degenerate_artificials_from_basis(&mut self) {
        for i in 0..self.n_constraints {
            let basic = self.active[i];

            // Artificial basic?
            if !(basic >= self.artificial_var_start && basic < self.z_col) {
                continue;
            }
            // If this artificial is non-degenerate (rhs != 0), that
            // indicates phase 1 didn't actually drive artificials to
            // 0 (infeasible or not fully optimized).
            if !self.rhs(i).is_zero() {
                continue;
            }

            // Find a non-artificial, nonbasic column with a nonzero coefficient in this row.
            let entering = (0..self.artificial_var_start)
                .find(|&col| !self.tableau[i][col].is_zero() && !self.is_basic_in_constraints(col));

            if let Some(col) = entering {
                self.pivot(i, col);
            }
        }
    }

    fn simplex(&mut self, obj_row: usize) -> SimplexResult {
        loop {
            let Some(enter) = self.pivot_col(obj_row) else {
                return SimplexResult::Optimal;
            };
            let Some(leave) = self.pivot_row(enter) else {
                return SimplexResult::Unbounded;
            };
            self.pivot(leave, enter);
        }
    }

    pub fn minimize(&mut self) -> Option<Rational64> {
        let p2 = self.n_constraints;
        for v in self.tableau[p2][0..self.slack_var_start].iter_mut() {
            *v = -*v;
        }
        self.maximize().map(|n| -n)
    }

    pub fn maximize(&mut self) -> Option<Rational64> {
        let p2 = self.n_constraints;
        let p1 = self.n_constraints + 1;

        match self.simplex(p1) {
            SimplexResult::Optimal => {}
            SimplexResult::Unbounded => return None,
        }

        // Feasibility: w = RHS in phase1 row (since w is the objective variable with coefficient 1).
        if !self.rhs(p1).is_zero() {
            return None; // infeasible
        }
        self.remove_degenerate_artificials_from_basis();

        match self.simplex(p2) {
            SimplexResult::Optimal => Some(self.rhs(p2)),
            SimplexResult::Unbounded => None,
        }
    }

    pub fn solution_x(&self) -> Vec<Rational64> {
        let mut x = vec![Rational64::ZERO; self.slack_var_start];

        for row in 0..self.n_constraints {
            let col = self.active[row];
            if col < self.slack_var_start {
                x[col] = self.tableau[row][self.rhs_col];
            }
        }

        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn lp_builder() -> LPBuilder {
        let mut builder = LPBuilder::new();
        builder.add_objective(vec![2, 3, 4]);
        builder.add_constraint(vec![3, 2, 1], LPOps::Lte, 10);
        builder.add_constraint(vec![2, 5, 3], LPOps::Lte, 15);
        builder
    }

    fn lp_solver() -> LinearProgrammingProblem {
        lp_builder().build()
    }

    #[test]
    fn test_build() {
        let b1 = lp_builder();
        let b2 = LPBuilder::default()
            .add_objective(vec![2, 3, 4])
            .add_constraint(vec![3, 2, 1], LPOps::Lte, 10)
            .add_constraint(vec![2, 5, 3], LPOps::Lte, 15)
            .clone();
        assert_eq!(b1, b2);
    }

    #[test]
    fn test_solve() {
        let mut solver = lp_solver();
        assert_eq!(solver.maximize(), Some(Rational64::from_integer(20)));
    }
}
