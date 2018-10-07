use ::core::slice;
use ::sudoku;
use ::libc::size_t;
use sudoku::Sudoku as RSudoku;

#[repr(C)]
#[derive(Clone, Copy)]
/// The central structure of the library. Represents a classical 9x9 sudoku.
/// All instances of this MUST be valid sudoku grids, but not necessarily solvable or
/// uniquely solvable.
pub struct Sudoku([u8; 81]);

impl Sudoku {
    pub fn from_rust_sudoku(sudoku: sudoku::Sudoku) -> Sudoku {
        Sudoku(sudoku.to_bytes())
    }

    pub fn to_rust_sudoku(self) -> RSudoku {
        RSudoku::from_bytes(self.0).unwrap()
    }
}


/// Creates a sudoku from an array of 81 bytes. All numbers must be below 10.
/// Empty cells are denoted by 0, clues by the numbers 1-9.
/// If any cell contains invalid entries, an empty sudoku will be returned.
#[no_mangle]
pub unsafe extern "C" fn sudoku_from_bytes(bytes: *const u8) -> Sudoku {
    let slice = slice::from_raw_parts(bytes, 81);
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
    let target = slice::from_raw_parts_mut(solutions_buf as *mut [u8; 81], len_buf);
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
pub unsafe extern "C" fn sudoku_shuffle(sudoku: *mut Sudoku) {
    let ref_mut = match sudoku.as_mut() {
        Some(ref_mut) => ref_mut,
        None => return,
    };
    let mut sudoku = ref_mut.to_rust_sudoku();
    sudoku.shuffle();
    *ref_mut = Sudoku::from_rust_sudoku(sudoku);
}
