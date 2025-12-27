//! # Linear programming solver
//!
//! Implements the [revised simplex method](<https://en.wikipedia.org/wiki/Revised_simplex_method>) to solve a linear programming problem.
#![allow(unused)]

use anyhow::Result;
use approx::{abs_diff_eq, relative_eq};
use itertools::Itertools;
use nalgebra::{ComplexField, Const, DMatrix, DVector, Dyn, OMatrix, U1, Vector1, stack};
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

    #[allow(clippy::toplevel_ref_arg)]
    fn build(self) -> Result<LPSolver> {
        // Get the number of constraints and the number of variables
        let m = self.constraints.len();
        let num_variables = self.constraints.first().map(|v| v.len()).unwrap_or(0);

        // Add slack and artificial variables
        let artificial_vars_matrix = DMatrix::<f64>::from_diagonal_element(m, m, 1_f64);
        let num_slack_vars = self
            .ops
            .iter()
            .filter(|&op| matches!(op, LPOps::Gte | LPOps::Lte))
            .count();
        let mut slack_vars_matrix = DMatrix::<f64>::zeros(m, num_slack_vars);

        let mut curr_slack_var = 0;
        for (i, op) in self.ops.iter().enumerate() {
            match op {
                LPOps::Eq => continue,
                LPOps::Gte => slack_vars_matrix[(i, curr_slack_var)] = -1_f64,
                LPOps::Lte => slack_vars_matrix[(i, curr_slack_var)] = 1_f64,
            }
            curr_slack_var += 1;
        }

        let initial_objective_function = DVector::<f64>::from_iterator(
            self.objective.len(),
            self.objective.iter().map(|&v| v as f64),
        );
        let constraints_flat = self.constraints.iter().flatten().map(|&v| v as f64);
        let constraints_matrix = DMatrix::from_row_iterator(
            self.constraints.len(),
            self.constraints[0].len(),
            constraints_flat,
        );

        // Creating necessary block vector and matrices
        let b = DVector::<f64>::from_iterator(self.ans.len(), self.ans.iter().map(|&v| v as f64));
        let c = stack![initial_objective_function; DVector::<f64>::zeros(num_slack_vars + m)];
        let p1_objective = stack![DVector::<f64>::zeros(num_slack_vars + num_variables); DVector::<f64>::from_element(m, 1_f64)];
        let constraints = stack![
            constraints_matrix,
            slack_vars_matrix,
            artificial_vars_matrix
        ];

        Ok(LPSolver {
            p1_objective,
            c,
            b,
            constraints,
            slack_var_start: num_variables,
            artificial_var_start: num_variables + num_slack_vars,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Solution {
    pub(crate) minima: f64,
    solution: OMatrix<f64, Dyn, U1>,
    basis: Vec<usize>,
}

impl Solution {
    fn is_integer_solution(&self) -> bool {
        self.solution.iter().all(|&v| v.round() == v)
    }

    /// Generates the new fractional constraints to be added to the LP problem
    fn get_fractional_constraints(&self) -> HashMap<usize, (i64, i64)> {
        self.solution
            .iter()
            .enumerate()
            .filter(|&(ind, v)| v.fract() != 0_f64)
            .map(|(ind, v)| (ind, (v.floor() as i64, v.ceil() as i64)))
            .collect()
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Minima: {}", self.minima)?;
        writeln!(
            f,
            "Solution: {{{}}}",
            self.solution
                .row_iter()
                .enumerate()
                .map(|(i, value)| format!("{}: {}", _subscript_variable('x', i), value[(0, 0)]))
                .join(", ")
        )?;
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct LPSolver {
    p1_objective: OMatrix<f64, Dyn, U1>,
    c: OMatrix<f64, Dyn, U1>,
    b: OMatrix<f64, Dyn, U1>,
    constraints: OMatrix<f64, Dyn, Dyn>,
    slack_var_start: usize,
    artificial_var_start: usize,
}

impl fmt::Display for LPSolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let objective = self
            .c
            .iter()
            .take(self.slack_var_start)
            .enumerate()
            .map(|(ind, term)| _pretty_print_variable('x', ind, *term))
            .collect::<Vec<_>>()
            .join(" + ");
        write!(f, "Minimise: Z = {}", objective)?;
        writeln!(f)?;
        writeln!(f, "Subject to:")?;
        for (i, row) in self.constraints.row_iter().enumerate() {
            let constraint = row
                .iter()
                .enumerate()
                .filter(|&(_, term)| term != &0_f64)
                .map(|(ind, term)| {
                    if ind < self.slack_var_start {
                        _pretty_print_variable('x', ind, *term)
                    } else if ind < self.artificial_var_start {
                        _pretty_print_variable('s', ind - self.slack_var_start, *term)
                    } else {
                        _pretty_print_variable('a', ind - self.artificial_var_start, *term)
                    }
                })
                .collect::<Vec<_>>()
                .join(" + ");
            writeln!(f, "  {} = {}", constraint, self.b[i])?;
        }
        Ok(())
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

#[derive(Debug, Error)]
pub(crate) enum LPSolverError {
    #[error("Problem is unbounded.")]
    UnboundedProblem,
    #[error("Problem has no solution.")]
    NoSolution,
    #[error("Max iterations reached.")]
    MaxIterationsReached,
}

type RevisedSimplexOutput = (OMatrix<f64, Dyn, U1>, Vec<usize>);

impl LPSolver {
    fn _get_basic_feasible_solution(&self) -> Result<Vec<usize>, LPSolverError> {
        let b_columns: Vec<usize> = (self.artificial_var_start..self.constraints.ncols()).collect();

        let (solution, final_basis) =
            self._revised_simplex_method(b_columns, self.p1_objective.clone(), true)?;

        if &self.p1_objective.transpose() * &solution != Vector1::zeros() {
            return Err(LPSolverError::NoSolution);
        }
        Ok(final_basis)
    }

    fn solve(&self) -> Result<Solution, LPSolverError> {
        // Phase 1 generate the basic feasible solution
        let bfs_basis = self._get_basic_feasible_solution()?;

        // Phase 2 now solve the original LP task using the basic feasible solution
        let (solution, final_basis) =
            self._revised_simplex_method(bfs_basis, self.c.clone(), false)?;

        Ok(Solution {
            minima: (&self.c.transpose() * &solution)[(0, 0)],
            solution: solution.select_rows(&(0..self.slack_var_start).collect::<Vec<_>>()),
            basis: final_basis,
        })
    }

    fn _revised_simplex_method(
        &self,
        mut b_columns: Vec<usize>,
        objective_function: OMatrix<f64, Dyn, U1>,
        use_artificial_vars: bool,
    ) -> Result<RevisedSimplexOutput, LPSolverError> {
        let mut final_index = self.constraints.ncols();
        if !use_artificial_vars {
            final_index = self.artificial_var_start;
        }
        let mut n_columns = (0..final_index)
            .filter(|x| !b_columns.contains(x))
            .collect::<Vec<_>>();
        let mut iterations = 0;
        loop {
            let basic_vars_matrix = self.constraints.select_columns(&b_columns);
            let non_basic_matrix = self.constraints.select_columns(&n_columns);
            let basis_matrix_inv = basic_vars_matrix.clone().try_inverse().unwrap();
            let non_basic_coeffs = objective_function.select_rows(&n_columns);
            let basic_var_coeffs = objective_function.select_rows(&b_columns);

            let simplex_multipliers =
                basic_vars_matrix.transpose().try_inverse().unwrap() * basic_var_coeffs;
            let reduced_costs =
                non_basic_coeffs - non_basic_matrix.transpose() * simplex_multipliers;

            // Deals with some issues where tolerance causes issues with the algorithm failing to stop
            if reduced_costs >= DVector::<f64>::zeros(reduced_costs.nrows())
                || abs_diff_eq!(
                    reduced_costs,
                    DVector::<f64>::zeros(reduced_costs.nrows()),
                    epsilon = 1e-15_f64
                )
            {
                let mut solution = DVector::<f64>::zeros(self.constraints.ncols());
                let basis_solution = basis_matrix_inv * &self.b;

                for (i, &col) in b_columns.iter().enumerate() {
                    solution.set_row(col, &basis_solution.row(i));
                }
                return Ok((solution, b_columns));
            }
            let entering_index = n_columns[reduced_costs.argmin().0];
            let pivot_column_vector = &basis_matrix_inv * self.constraints.column(entering_index);

            if pivot_column_vector <= DVector::<f64>::zeros(pivot_column_vector.nrows()) {
                return Err(LPSolverError::UnboundedProblem);
            }

            // leaving_index = argmin{xᵢ/dᵢ; dᵢ > 0}
            let leaving_col = (&basis_matrix_inv * &self.b)
                .row_iter()
                .zip(pivot_column_vector.row_iter())
                .enumerate()
                .filter(|&(_, (_, pivot_val))| pivot_val.x > 0_f64)
                .map(|(i, (x, pivot_val))| (i, x.x / pivot_val.x))
                .min_by(|&(_, a), &(i2, b)| a.total_cmp(&b))
                .unwrap()
                .0;

            b_columns[leaving_col] = entering_index;
            n_columns[reduced_costs.argmin().0] = b_columns[leaving_col];
            iterations += 1;
            if iterations == 10000 {
                let mut solution = DVector::<f64>::zeros(self.constraints.ncols());
                let basis_solution = basis_matrix_inv * &self.b;

                for (i, &col) in b_columns.iter().enumerate() {
                    solution.set_row(col, &basis_solution.row(i));
                }
                return Err(LPSolverError::MaxIterationsReached);
            }
        }
    }
}

pub(crate) fn branch_and_bound(initial_lp: &LPBuilder) -> Option<Solution> {
    let mut queue = VecDeque::from(vec![initial_lp.clone()]);
    let mut best_solution: Option<Solution> = None;
    let mut best_minima = f64::MAX;

    while let Some(lp) = queue.pop_front() {
        let solver = lp.clone().build().unwrap();
        let solution = solver.solve();
        match solution {
            Ok(solution) => {
                if best_minima <= solution.minima {
                    continue;
                }
                if solution.is_integer_solution() {
                    best_minima = solution.minima;
                    best_solution = Some(solution);
                    continue;
                }

                // Performing strong branching by exploring each fractional constraint
                for (variable, (lower_bound, upper_bound)) in
                    solution.get_fractional_constraints().iter()
                {
                    let mut constraint_eq = vec![0; solution.solution.nrows()];
                    constraint_eq[*variable] = 1;

                    // Generate upper bound sub-problem
                    let mut branched_problem_upper = lp.clone();
                    branched_problem_upper.add_constraint(
                        constraint_eq.clone(),
                        LPOps::Gte,
                        *upper_bound,
                    );
                    if branched_problem_upper != lp {
                        queue.push_back(branched_problem_upper);
                    }

                    // Generate lower bound sub-problem
                    let mut branched_problem_lower = lp.clone();
                    branched_problem_lower.add_constraint(
                        constraint_eq.clone(),
                        LPOps::Lte,
                        *lower_bound,
                    );
                    if branched_problem_lower != lp {
                        queue.push_back(branched_problem_lower);
                    }
                }
            }
            Err(_) => continue,
        }
    }
    best_solution
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn lp_builder() -> LPBuilder {
        let mut builder = LPBuilder::new();
        builder.add_objective(vec![-2, -3, -4]);
        builder.add_constraint(vec![3, 2, 1], LPOps::Lte, 10);
        builder.add_constraint(vec![2, 5, 3], LPOps::Lte, 15);
        builder
    }

    fn lp_solver() -> LPSolver {
        lp_builder().build().unwrap()
    }

    #[test]
    fn test_build() {
        let b1 = lp_builder();
        let b2 = LPBuilder::default()
            .add_objective(vec![-2, -3, -4])
            .add_constraint(vec![3, 2, 1], LPOps::Lte, 10)
            .add_constraint(vec![2, 5, 3], LPOps::Lte, 15)
            .clone();
        assert_eq!(b1, b2);
    }

    #[test]
    fn test_lp_solver_display() {
        let solver = lp_solver();

        let expected = "Minimise: Z = -2x₀ + -3x₁ + -4x₂
Subject to:
  3x₀ + 2x₁ + x₂ + s₀ + a₀ = 10
  2x₀ + 5x₁ + 3x₂ + s₁ + a₁ = 15\n";
        assert_eq!(expected, solver.to_string());
    }

    #[test]
    fn test_solve() {
        let solver = lp_solver();

        let solution = solver.solve().unwrap();
        let expected = Solution {
            minima: -20.0,
            solution: DVector::from_column_slice(&[0.0, 0.0, 5.0]),
            basis: vec![3, 2],
        };
        assert_eq!(solution, expected);
    }

    #[test]
    fn test_solve_tolerance_edge_case() {
        let mut builder = LPBuilder::new();
        builder.add_objective(vec![-2, -3, -4]);
        builder.add_constraint(vec![3, 2, 1], LPOps::Eq, 10);
        builder.add_constraint(vec![2, 5, 3], LPOps::Eq, 15);
        let solver = builder.build().unwrap();
        let solution = solver.solve().unwrap();
        let expected = Solution {
            minima: -18.571428571428573,
            solution: DVector::from_column_slice(&[15_f64 / 7_f64, 0.0, 25_f64 / 7_f64]),
            basis: vec![0, 1, 2],
        };
        assert!(abs_diff_eq!(
            solution.minima,
            expected.minima,
            epsilon = 1e-14_f64
        ));
        assert!(abs_diff_eq!(
            solution.solution,
            expected.solution,
            epsilon = 1e-14_f64
        ));
    }

    #[test]
    fn test_pretty_print_variable() {
        assert_eq!("x\u{2080}", _pretty_print_variable('x', 0, 1_f64));
    }

    #[test]
    fn test_is_integer_solution() {
        let s1 = Solution {
            minima: -20.0,
            solution: DVector::from_column_slice(&[0.0, 0.0, 5.0]),
            basis: vec![0, 1, 2],
        };
        let s2 = Solution {
            minima: -20.0,
            solution: DVector::from_column_slice(&[0.0, 0.0, 5.1]),
            basis: vec![0, 1, 2],
        };
        assert!(s1.is_integer_solution());
        assert!(!s2.is_integer_solution());
    }

    #[test]
    fn test_display_solution() {
        let solution = Solution {
            minima: -20.0,
            solution: DVector::from_column_slice(&[0.0, 0.0, 5.0]),
            basis: vec![0, 1, 2],
        };
        let expected = "Minima: -20\nSolution: {x₀: 0, x₁: 0, x₂: 5}\n";
        assert_eq!(expected, solution.to_string());
    }

    #[test]
    fn test_get_fractional_constraints() {
        let s1 = Solution {
            minima: -20.0,
            solution: DVector::from_column_slice(&[1.6, 0.2, 5.5]),
            basis: vec![0, 1, 2],
        };
        assert!(!s1.is_integer_solution());
        let fractional_constraints = s1.get_fractional_constraints();
        assert_eq!(fractional_constraints.get(&2), Some(&(5, 6)));
        assert_eq!(fractional_constraints.get(&0), Some(&(1, 2)));
        assert_eq!(fractional_constraints.get(&1), Some(&(0, 1)));
    }

    #[test]
    fn test_branch_and_bound() {
        let mut builder = LPBuilder::new();
        builder.add_objective(vec![0, -1]);
        builder.add_constraint(vec![-1, 1], LPOps::Lte, 1);
        builder.add_constraint(vec![3, 2], LPOps::Lte, 12);
        builder.add_constraint(vec![2, 3], LPOps::Lte, 12);
        let solution = branch_and_bound(&builder);
        assert_eq!(solution.clone().unwrap().minima, -2.0);
        assert_eq!(solution.clone().unwrap().solution.row(1).x, 2.0);
    }
}
