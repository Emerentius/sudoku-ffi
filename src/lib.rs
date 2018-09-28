extern crate core;
extern crate sudoku;
extern crate libc;
use sudoku::Sudoku as RSudoku;
use sudoku::strategy::{
    Strategy as RStrategy,
    StrategySolver as RStrategySolver,
    Deduction as RDeduction,
    DeductionResult as RDeductionResult,
    Deductions as RDeductions,
    Entry as REntry,
    CellState,
    //DeductionsIter
};

use libc::size_t;

#[repr(C)]
#[derive(Clone, Copy)]
/// The central structure of the library. Represents a classical 9x9 sudoku.
/// All instances of this MUST be valid sudoku grids, but not necessarily solvable or
/// uniquely solvable.
pub struct Sudoku([u8; 81]);

pub enum _StrategySolver {}
pub enum _Deductions {}
pub enum _Deduction {}
pub enum _DeductionResult {}
pub enum _Entry {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Entry {
    cell: u8,
    num: u8,
}

impl core::convert::From<REntry> for Entry {
    fn from(entry: REntry) -> Self {
        let num = entry.num();
        let cell = entry.cell() as u8;
        Entry { cell, num }
    }
}

impl core::convert::From<Entry> for REntry {
    fn from(Entry { cell, num }: Entry) -> Self {
        REntry::new(cell, num)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StrategySolver(*mut _StrategySolver);

#[repr(C)]
pub struct StrategySolvingResult {
    pub is_solved: bool,
    pub sudoku: Sudoku,
    pub deductions: Deductions,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Deductions(*mut _Deductions);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Deduction(*const _Deduction);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DeductionResult {
    is_forced: bool,
    entry: Entry, // if is_forced
    eliminated_ptr: *const _Entry,
    len: size_t,
}

#[repr(C, u8)]
#[derive(Clone, Copy)]
pub enum Strategy {
    NakedSingles,
    HiddenSingles,
    LockedCandidates,
	NakedPairs,
	NakedTriples,
	NakedQuads,
	HiddenPairs,
	HiddenTriples,
	HiddenQuads,
    XWing,
    Swordfish,
    Jellyfish,
    SinglesChain,
    // chosen because it's the maximum for an 8bit number
    // the memory layout for this enum may change in the future
    UnknownStrategy = 255,
}

impl Sudoku {
    fn from_rust_sudoku(sudoku: sudoku::Sudoku) -> Sudoku {
        Sudoku(sudoku.to_bytes())
    }

    fn to_rust_sudoku(self) -> RSudoku {
        RSudoku::from_bytes(self.0).unwrap()
    }
}

impl Strategy {
    /*
    fn from_rstrategy(strat: RStrategy) -> Self {
        use Strategy::*;
        match strat {
            RStrategy::NakedSingles => NakedSingles,
            RStrategy::HiddenSingles => HiddenSingles,
            RStrategy::LockedCandidates => NakedSingles,
            RStrategy::NakedPairs => NakedSingles,
            RStrategy::NakedTriples => NakedSingles,
            RStrategy::NakedQuads => NakedSingles,
            RStrategy::HiddenPairs => NakedSingles,
            RStrategy::HiddenTriples => NakedSingles,
            RStrategy::HiddenQuads => NakedSingles,
            RStrategy::XWing => NakedSingles,
            RStrategy::Swordfish => NakedSingles,
            RStrategy::Jellyfish => NakedSingles,
            RStrategy::SinglesChain => NakedSingles,
            RStrategy::__NonExhaustive => UnknownStrategy,
        }
    }
    */

    // TODO: How should UnknownStrategy be handled?
    fn to_rstrategy(self) -> RStrategy {
        use Strategy::*;
        match self {
            NakedSingles => RStrategy::NakedSingles,
            HiddenSingles => RStrategy::HiddenSingles,
            LockedCandidates => RStrategy::NakedSingles,
            NakedPairs => RStrategy::NakedSingles,
            NakedTriples => RStrategy::NakedSingles,
            NakedQuads => RStrategy::NakedSingles,
            HiddenPairs => RStrategy::NakedSingles,
            HiddenTriples => RStrategy::NakedSingles,
            HiddenQuads => RStrategy::NakedSingles,
            XWing => RStrategy::NakedSingles,
            Swordfish => RStrategy::NakedSingles,
            Jellyfish => RStrategy::NakedSingles,
            SinglesChain => RStrategy::NakedSingles,
            UnknownStrategy => RStrategy::__NonExhaustive,
        }
    }
}

impl StrategySolver {
    fn as_rsolver(self) -> *mut RStrategySolver {
        self.0 as *mut RStrategySolver
    }
}

impl Deductions {
    fn as_rdeductions(self) -> *mut RDeductions {
        self.0 as *mut RDeductions
    }
}

impl Deduction {
    fn as_rdeduction<'a>(self) -> *mut RDeduction<'a> {
        self.0 as *mut RDeduction
    }
}

/// Creates a sudoku from an array of 81 bytes. All numbers must be below 10.
/// Empty cells are denoted by 0, clues by the numbers 1-9.
/// If any cell contains invalid entries, an empty sudoku will be returned.
#[no_mangle]
pub unsafe extern "C" fn sudoku_from_bytes(bytes: *const u8) -> Sudoku {
    let slice = core::slice::from_raw_parts(bytes, 81);
    let mut bytes = [0; 81];
    bytes.copy_from_slice(slice);

    match RSudoku::from_bytes(bytes) {
        Ok(sudoku) => Sudoku::from_rust_sudoku(sudoku),
        Err(_) => Sudoku([0; 81]),
    }
}

/// Returns a pointer to the bytes of the sudoku.
/// Empty cells are denoted by 0, clues by the numbers 1-9
#[no_mangle]
pub extern "C" fn sudoku_as_ptr(sudoku: *const Sudoku) -> *const u8 {
    sudoku as *const _
}

/// Generates a random, valid, solved `Sudoku`.
#[no_mangle]
pub extern "C" fn sudoku_generate_filled() -> Sudoku {
    Sudoku::from_rust_sudoku( RSudoku::generate_filled() )
}

/// Generates a random, uniquely solvable, minimal `Sudoku`. Most sudokus generated this way are very easy.
#[no_mangle]
pub extern "C" fn sudoku_generate_unique() -> Sudoku {
    Sudoku::from_rust_sudoku( RSudoku::generate_unique() )
}

/// Checks that all cells contain values from 0-9 (inclusive)
/// (Unique) solvability is not tested.
#[no_mangle]
pub extern "C" fn sudoku_is_valid_grid(sudoku: Sudoku) -> bool {
    RSudoku::from_bytes(sudoku.0).is_ok()
}

/// Counts sudoku solutions up to `limit`
#[no_mangle]
pub unsafe extern "C" fn sudoku_count_at_most(sudoku: Sudoku, limit: size_t) -> size_t {
    let sudoku = sudoku.to_rust_sudoku();
    sudoku.count_at_most(limit)
}

/// Finds and counts up to `limit` solutions and writes them into `solutions_buf` up to its capacity of `len_buf`.
/// Any additional solutions `> len_buf` but `<= limit` will be counted but not saved.
/// The number of found solutions is stored in `n_found`.
///
/// Immediately returns `false` if `solutions_buf` or `n_found` is null, otherwise `true`.
#[no_mangle]
pub unsafe extern "C" fn sudoku_solve_at_most(solutions_buf: *mut Sudoku, n_found: *mut size_t, len_buf: size_t, sudoku: Sudoku, limit: size_t) -> bool {
    if solutions_buf.is_null() || n_found.is_null() {
        return false;
    }
    let sudoku = sudoku.to_rust_sudoku();
    let target = core::slice::from_raw_parts_mut(solutions_buf as *mut [u8; 81], len_buf);
    *n_found = sudoku.solve_at_most_buffer(target, limit);
    true
}

/// Checks whether the sudoku is solved, i.e. completely filled in a valid way.
#[no_mangle]
pub extern "C" fn sudoku_is_solved(sudoku: Sudoku) -> bool {
    sudoku.to_rust_sudoku().is_solved()
}

/// Performs symmetry transformations that result in a different sudoku
/// with the same solution count and difficulty.
#[no_mangle]
pub unsafe extern "C" fn shuffle(sudoku: *mut Sudoku) {
    let ref_mut = match sudoku.as_mut() {
        Some(ref_mut) => ref_mut,
        None => return,
    };
    let mut sudoku = ref_mut.to_rust_sudoku();
    sudoku.shuffle();
    *ref_mut = Sudoku::from_rust_sudoku(sudoku);
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

    let strategies = unsafe { core::slice::from_raw_parts(strategies, len) };
    let strategies = strategies.iter().cloned().map(Strategy::to_rstrategy).collect::<Vec<_>>();

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
pub extern "C" fn strategy_solver_insert_entry(solver: StrategySolver, entry: Entry) -> bool {
    let solver = solver.as_rsolver();
    let solver = unsafe { &mut *solver };

    let result = solver.insert_entry(entry.into());
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

    match solver.cell_state(cell) {
        CellState::Number(_) => unimplemented!(),
        CellState::Candidates(mask) => mask.0,
    }
}


#[no_mangle]
pub extern "C" fn deductions_len(deductions: Deductions) -> size_t {
    unsafe {
        (&*deductions.as_rdeductions()).len()
    }
}

/// Indexing past bounds is Undefined Behaviour.
#[no_mangle]
pub unsafe extern "C" fn deductions_get(deductions: Deductions, idx: size_t) -> Deduction {
    let deductions = &*deductions.as_rdeductions();
    let deduction = deductions.get(idx).unwrap();

    let ptr = Box::into_raw(Box::new(deduction));
    Deduction(ptr as *const _)
}

#[no_mangle]
pub extern "C" fn deduction_results(deduction: Deduction) -> DeductionResult {
    let result = unsafe { (&*deduction.as_rdeduction()).results() };
    match result {
        RDeductionResult::Forced(entry) => {
            DeductionResult {
                is_forced: true,
                entry: entry.into(),
                eliminated_ptr: std::ptr::null_mut(),
                len: 0,
            }
        }
        RDeductionResult::Eliminated(slice) => {
            DeductionResult {
                is_forced: false,
                entry: Entry { cell: 0, num: 0 }, // invalid
                eliminated_ptr: slice.as_ptr() as *const _Entry,
                len: slice.len(),
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn deduction_result_len(results: DeductionResult) -> libc::size_t {
    if results.is_forced { 1 } else { results.len }
}

#[no_mangle]
pub unsafe extern "C" fn deduction_result_get_forced_entry(results: DeductionResult) -> Entry {
    results.entry
}

#[no_mangle]
pub unsafe extern "C" fn deduction_result_get_eliminated_entry(results: DeductionResult, idx: libc::size_t) -> Entry {
    let eliminated = results.eliminated_ptr as *mut REntry;
    let slice = core::slice::from_raw_parts(eliminated, results.len);
    slice[idx].into()
}
