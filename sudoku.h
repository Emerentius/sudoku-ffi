#include <cstdint>
#include <cstdlib>

enum class Strategy : uint8_t {
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
  UnknownStrategy = 255,
};

struct _Deduction;

struct _Deductions;

struct _StrategySolver;

struct Deduction {
  const _Deduction *_0;
};

struct Deductions {
  _Deductions *_0;
};

// The central structure of the library. Represents a classical 9x9 sudoku.
// All instances of this MUST be valid sudoku grids, but not necessarily solvable or
// uniquely solvable.
struct Sudoku {
  uint8_t _0[81];
};

struct StrategySolver {
  _StrategySolver *_0;
};

struct Entry {
  uint8_t cell;
  uint8_t num;
};

struct StrategySolvingResult {
  bool is_solved;
  Sudoku sudoku;
  Deductions deductions;
};

extern "C" {

// Indexing past bounds is Undefined Behaviour.
Deduction deductions_get(Deductions deductions, size_t idx);

size_t deductions_len(Deductions deductions);

// Performs symmetry transformations that result in a different sudoku
// with the same solution count and difficulty.
void shuffle(Sudoku *sudoku);

// Returns the remaining possible candidates in `cell` as a 9-bit mask. The nth bit stands for the nth digit,
// counting from lowest to most significant bit.
//
// It's undefined behaviour to call this on an already filled cell or with `cell > 80`.
uint16_t strategy_solver_cell_candidates(StrategySolver solver,
                                         uint8_t cell);

// Try to insert `entry`.
//
// Returns `false` if the cell is already filled, `true` otherwise.
bool strategy_solver_insert_entry(StrategySolver solver, Entry entry);

StrategySolver strategy_solver_new(Sudoku sudoku);

// This consumes the solver
StrategySolvingResult strategy_solver_solve(StrategySolver solver,
                                            const Strategy *strategies,
                                            size_t len);

// This function is not threadsafe
Sudoku strategy_solver_to_sudoku(StrategySolver solver);

// Returns a pointer to the bytes of the sudoku.
// Empty cells are denoted by 0, clues by the numbers 1-9
const uint8_t *sudoku_as_ptr(const Sudoku *sudoku);

// Counts sudoku solutions up to `limit`
size_t sudoku_count_at_most(Sudoku sudoku, size_t limit);

// Creates a sudoku from an array of 81 bytes. All numbers must be below 10.
// Empty cells are denoted by 0, clues by the numbers 1-9.
// If any cell contains invalid entries, an empty sudoku will be returned.
Sudoku sudoku_from_bytes(const uint8_t *bytes);

// Generates a random, valid, solved `Sudoku`.
Sudoku sudoku_generate_filled();

// Generates a random, uniquely solvable, minimal `Sudoku`. Most sudokus generated this way are very easy.
Sudoku sudoku_generate_unique();

// Checks whether the sudoku is solved, i.e. completely filled in a valid way.
bool sudoku_is_solved(Sudoku sudoku);

// Checks that all cells contain values from 0-9 (inclusive)
// (Unique) solvability is not tested.
bool sudoku_is_valid_grid(Sudoku sudoku);

// Finds and counts up to `limit` solutions and writes them into `solutions_buf` up to its capacity of `len_buf`.
// Any additional solutions `> len_buf` but `<= limit` will be counted but not saved.
// The number of found solutions is stored in `n_found`.
//
// Immediately returns `false` if `solutions_buf` or `n_found` is null, otherwise `true`.
bool sudoku_solve_at_most(Sudoku *solutions_buf,
                          size_t *n_found,
                          size_t len_buf,
                          Sudoku sudoku,
                          size_t limit);

} // extern "C"
