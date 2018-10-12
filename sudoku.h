#ifndef sudoku_ffi_h
#define sudoku_ffi_h

#include <cstdint>
#include <cstdlib>

enum class DeductionTag {
  Given,
  NakedSingle,
  HiddenSingle,
  LockedCandidates,
  NakedSubset,
  HiddenSubset,
  BasicFish,
  SinglesChain,
};

enum class HouseType {
  Row,
  Col,
  Block,
};

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
};

struct _Deductions;

struct _RCandidate;

struct _StrategySolver;

struct Candidate {
  uint8_t cell;
  uint8_t num;
};

struct Conflicts {
  const _RCandidate *ptr;
  size_t len;
};

struct Given {
  Candidate candidate;
};

struct NakedSingle {
  Candidate candidate;
};

struct HiddenSingle {
  Candidate candidate;
  HouseType house_type;
};

struct LockedCandidates {
  uint8_t miniline;
  uint8_t digit;
  bool is_pointing;
  Conflicts conflicts;
};

using Mask16 = uint16_t;

struct NakedSubsets {
  uint8_t house;
  Mask16 positions;
  Mask16 digits;
  Conflicts conflicts;
};

struct HiddenSubsets {
  uint8_t house;
  Mask16 digits;
  Mask16 positions;
  Conflicts conflicts;
};

using Mask32 = uint32_t;

struct BasicFish {
  Mask32 lines;
  Mask16 positions;
  uint8_t digit;
  Conflicts conflicts;
};

struct SinglesChain {
  Conflicts conflicts;
};

union DeductionData {
  Given given;
  NakedSingle naked_single;
  HiddenSingle hidden_single;
  LockedCandidates locked_candidates;
  NakedSubsets naked_subsets;
  HiddenSubsets hidden_subsets;
  BasicFish basic_fish;
  SinglesChain singles_chain;
};

struct Deduction {
  DeductionTag tag;
  DeductionData data;
};

struct Deductions {
  _Deductions *_0;
};

struct StrategySolver {
  _StrategySolver *_0;
};

struct CellState {
  enum class Tag {
    Digit,
    Candidates,
  };

  struct Digit_Body {
    uint8_t _0;
  };

  struct Candidates_Body {
    Mask16 _0;
  };

  Tag tag;
  union {
    Digit_Body digit;
    Candidates_Body candidates;
  };
};

struct GridState {
  CellState grid[81];
};

// The central structure of the library. Represents a classical 9x9 sudoku.
// All instances of this MUST be valid sudoku grids, but not necessarily solvable or
// uniquely solvable.
struct Sudoku {
  uint8_t _0[81];
};

struct StrategySolvingResult {
  bool is_solved;
  Sudoku sudoku;
  Deductions deductions;
};

extern "C" {

Candidate conflicts_get(Conflicts conflicts, size_t idx);

size_t conflicts_len(Conflicts conflicts);

Deduction deductions_get(Deductions deductions, size_t idx);

size_t deductions_len(Deductions deductions);

// Returns the remaining possible candidates in `cell` as a 9-bit mask. The nth bit stands for the nth digit,
// counting from lowest to most significant bit.
//
// It's undefined behaviour to call this on an already filled cell or with `cell > 80`.
Mask16 strategy_solver_cell_candidates(StrategySolver solver,
                                       uint8_t cell);

StrategySolver strategy_solver_from_grid_state(GridState grid_state);

// Try to insert `entry`.
//
// Returns `false` if the cell is already filled, `true` otherwise.
bool strategy_solver_insert_entry(StrategySolver solver, Candidate entry);

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

// Performs symmetry transformations that result in a different sudoku
// with the same solution count and difficulty.
void sudoku_shuffle(Sudoku *sudoku);

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

#endif // sudoku_ffi_h
