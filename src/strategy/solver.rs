use ::sudoku::{
    strategy::{
        StrategySolver as RStrategySolver,
        Strategy as RStrategy,
    },
    board::{
        Cell,
        CellState,
    }
};
use ::libc::size_t;
use ::core::slice;
use board::{
    sudoku::Sudoku,
    candidate::Candidate,
    //cell_state::CellState,
};
use strategy::{
    deductions::Deductions,
    strategies::Strategy,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StrategySolver(*mut _StrategySolver);

pub enum _StrategySolver {}

#[repr(C)]
pub struct StrategySolvingResult {
    pub is_solved: bool,
    pub sudoku: Sudoku,
    pub deductions: Deductions,
}


impl StrategySolver {
    fn as_rsolver(self) -> *mut RStrategySolver {
        self.0 as *mut RStrategySolver
    }
}


#[no_mangle]
pub extern "C" fn strategy_solver_new(sudoku: Sudoku) -> StrategySolver {
    let sudoku = sudoku.to_rust_sudoku();
    let ss = RStrategySolver::from_sudoku(sudoku);
    let ptr = Box::into_raw(Box::new(ss)) as *mut _StrategySolver;
    StrategySolver(ptr)
}

/// This function is not threadsafe
#[no_mangle]
pub unsafe extern "C" fn strategy_solver_to_sudoku(solver: StrategySolver) -> Sudoku {
    Sudoku::from_rust_sudoku(
        (*solver.as_rsolver()).to_sudoku()
    )
}

/// This consumes the solver
#[no_mangle]
pub extern "C" fn strategy_solver_solve(solver: StrategySolver, strategies: *const Strategy, len: size_t) -> StrategySolvingResult {
    let solver = solver.as_rsolver();
    let solver = unsafe { Box::from_raw(solver) };

    let strategies = unsafe { slice::from_raw_parts(strategies, len) };
    let strategies = strategies.iter().cloned().map(RStrategy::from).collect::<Vec<_>>();

    let result = solver.solve(&strategies);

    let is_solved = result.is_ok();
    let (sudoku, deductions) = result.unwrap_or_else(|x| x);
    let ptr = Box::into_raw(Box::new(deductions)) as *mut _;

    StrategySolvingResult {
        is_solved,
        sudoku: Sudoku::from_rust_sudoku(sudoku),
        deductions: Deductions(ptr)
    }
}

/// Try to insert `entry`.
///
/// Returns `false` if the cell is already filled, `true` otherwise.
#[no_mangle]
pub extern "C" fn strategy_solver_insert_entry(solver: StrategySolver, entry: Candidate) -> bool {
    let solver = solver.as_rsolver();
    let solver = unsafe { &mut *solver };

    let result = solver.insert_candidate(entry.into());
    result.is_ok()
}

/// Returns the remaining possible candidates in `cell` as a 9-bit mask. The nth bit stands for the nth digit,
/// counting from lowest to most significant bit.
///
/// It's undefined behaviour to call this on an already filled cell or with `cell > 80`.
#[no_mangle]
pub extern "C" fn strategy_solver_cell_candidates(solver: StrategySolver, cell: u8) -> u16 {
    let solver = solver.as_rsolver();
    let solver = unsafe { &mut *solver };

    match solver.cell_state(Cell::new(cell)) {
        CellState::Digit(_) => unimplemented!(),
        CellState::Candidates(candidates) => {
            let mut mask = 0;
            for digit in candidates {
                mask |= 1 << digit.get() - 1;
            }
            mask
        },
    }
}
