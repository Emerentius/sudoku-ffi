extern crate core;
extern crate sudoku;
extern crate libc;
use sudoku::Sudoku as RSudoku;

use libc::size_t;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sudoku(pub [u8; 81]);

impl Sudoku {
    fn from_rust_sudoku(sudoku: sudoku::Sudoku) -> Sudoku {
        Sudoku(sudoku.to_bytes())
    }

    fn to_rust_sudoku(self) -> Option<RSudoku> {
        RSudoku::from_bytes(self.0).ok()
    }
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

/// Counts sudoku solutions up to `limit` and writes how many were found into `n_found`.
///
/// Returns false if `sudoku` is invalid or `n_found` is null.
#[no_mangle]
pub unsafe extern "C" fn sudoku_count_at_most(n_found: *mut size_t, sudoku: Sudoku, limit: size_t) -> bool {
    if n_found.is_null() {
        return false;
    }
    let sudoku = match sudoku.to_rust_sudoku() {
        Some(s) => s,
        None => return false,
    };
    *n_found = sudoku.count_at_most(limit);
    true
}

/// Finds and counts up to `limit` solutions and writes them into `solutions_buf` up to its capacity of `len_buf`.
/// Any additional solutions `> len_buf` but `<= limit` will be counted but not saved.
/// The number of found solutions is stored in `n_found`.
///
/// Immediately returns false if `solutions_buf` or `n_found` is null or the sudoku is invalid, otherwise `true`.
#[no_mangle]
pub unsafe extern "C" fn sudoku_solve_at_most(solutions_buf: *mut Sudoku, n_found: *mut size_t, len_buf: size_t, sudoku: Sudoku, limit: size_t) -> bool {
    if solutions_buf.is_null() || n_found.is_null() {
        return false;
    }
    let sudoku = match sudoku.to_rust_sudoku() {
        Some(s) => s,
        None => return false,
    };
    let target = core::slice::from_raw_parts_mut(solutions_buf as *mut [u8; 81], len_buf);
    *n_found = sudoku.solve_at_most_buffer(target, limit);
    true
}

/// Checks whether the sudoku is solved, i.e. completely filled in a valid way.
///
/// Returns false if the sudoku contains invalid numbers above 9. Use `sudoku_is_valid_grid` if correct
/// value range is not guaranteed.
#[no_mangle]
pub extern "C" fn sudoku_is_solved(sudoku: Sudoku) -> bool {
    sudoku.to_rust_sudoku()
        .map_or(false, |sudoku| sudoku.is_solved())
}

/// Performs symmetry transformations that result in a different sudoku
/// with the same solution count and difficulty.
///
/// Returns `false` if a null pointer or a pointer to an invalid sudoku is passed, otherwise `true`.
#[no_mangle]
pub unsafe extern "C" fn shuffle(sudoku: *mut Sudoku) -> bool {
    let ref_mut = match sudoku.as_mut() {
        Some(ref_mut) => ref_mut,
        None => return false,
    };
    let mut sudoku = match ref_mut.to_rust_sudoku() {
        Some(s) => s,
        None => return false,
    };
    sudoku.shuffle();
    *ref_mut = Sudoku::from_rust_sudoku(sudoku);
    true
}
