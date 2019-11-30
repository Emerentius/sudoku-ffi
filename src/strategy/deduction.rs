use ::sudoku::strategy::Deduction as RDeduction;
use ::sudoku::board::positions::HouseType as RHouseType;
use ::sudoku::board::Candidate as RCandidate;
use ::sudoku::bitset::Set;
use ::sudoku::board::Digit;
use ::sudoku::board::positions::{Cell, Position, House, Line};
use ::core::slice;
use ::libc::size_t;

use ::board::candidate::Candidate;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Deduction {
    tag: DeductionTag,
    data: DeductionData,
}

impl<'a> From<RDeduction<&'a [RCandidate]>> for Deduction {
    fn from(deduction: RDeduction<&'a [RCandidate]>) -> Self {
        let tag;
        let data;

        use self::RDeduction::*;
        match deduction {
            NakedSingles(candidate) => {
                tag = DeductionTag::NakedSingles;
                data = DeductionData { naked_singles: NakedSingle { candidate: candidate.into() } };
            }
            HiddenSingles(candidate, house_type) => {
                tag = DeductionTag::HiddenSingles;
                data = DeductionData {
                    hidden_singles: HiddenSingle {
                        candidate: candidate.into(),
                        house_type: house_type.into(),
                    }
                };
            }
            LockedCandidates { miniline, digit, is_pointing, conflicts } => {
                tag = DeductionTag::LockedCandidates;
                data = DeductionData {
                    locked_candidates: self::LockedCandidates {
                        miniline: miniline.get(),
                        digit: digit.get(),
                        is_pointing,
                        conflicts: conflicts.into(),
                    }
                };
            }
            Subsets { house, positions, digits, conflicts } => {
                tag = DeductionTag::Subsets;
                data = DeductionData {
                    subsets: self::Subsets {
                        house: house.get(),
                        digits: mask_of_digits(digits),
                        positions: mask_of_positions_house(positions),
                        conflicts: conflicts.into(),
                    }
                }
            }
            BasicFish { lines, positions, digit, conflicts } => {
                tag = DeductionTag::BasicFish;
                data = DeductionData {
                    basic_fish: self::BasicFish {
                        lines: mask_of_lines(lines),
                        positions: mask_of_positions_line(positions),
                        digit: digit.get(),
                        conflicts: conflicts.into(),
                    }
                }
            }
            Fish { digit, base, cover, conflicts } => {
                tag = DeductionTag::Fish;
                data = DeductionData {
                    fish: self::Fish {
                        digit: digit.get(),
                        base: base.bits(),
                        cover: cover.bits(),
                        conflicts: conflicts.into(),
                    }
                }
            }
            Wing { hinge, hinge_digits, pincers, conflicts } => {
                tag = DeductionTag::Wing;
                data = DeductionData {
                    wing: self::Wing {
                        hinge: hinge.get(),
                        hinge_digits: hinge_digits.bits(),
                        pincers: mask_of_cells(pincers),
                        conflicts: conflicts.into(),
                    }
                }
            }
            AvoidableRectangle { lines, conflicts } => {
                tag = DeductionTag::AvoidableRectangle;
                data = DeductionData {
                    avoidable_rectangle: self::AvoidableRectangle {
                        lines: lines.bits(),
                        conflicts: conflicts.into(),
                    }
                }
            }
            /*
            SinglesChain(conflicts) => {
                tag = DeductionTag::SinglesChain;
                data = DeductionData {
                    singles_chain: self::SinglesChain {
                        conflicts: conflicts.into(),
                    }
                }
            }
            */
            __NonExhaustive => unreachable!(),
        }

        Deduction { tag, data }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum DeductionTag {
    NakedSingles,
    HiddenSingles,
    LockedCandidates,
    Subsets,
    BasicFish,
    Fish,
    Wing,
    AvoidableRectangle,
    //SinglesChain,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union DeductionData {
    naked_singles: NakedSingle,
    hidden_singles: HiddenSingle,
    locked_candidates: LockedCandidates,
    subsets: Subsets,
    basic_fish: BasicFish,
    fish: Fish,
    wing: Wing,
    avoidable_rectangle: AvoidableRectangle,
    //singles_chain: SinglesChain,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NakedSingle {
    candidate: Candidate,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HiddenSingle {
    candidate: Candidate,
    house_type: HouseType,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum HouseType {
    Row,
    Col,
    Block,
}

impl From<RHouseType> for HouseType {
    fn from(house_type: RHouseType) -> Self {
        use self::RHouseType::*;
        match house_type {
            Row(_) => HouseType::Row,
            Col(_) => HouseType::Col,
            Block(_) => HouseType::Block,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LockedCandidates {
    miniline: u8,
    digit: u8,
    is_pointing: bool,
    conflicts: Conflicts,
}

// bitmask
type Mask16 = u16;
type Mask32 = u32;
type Mask128 = [u64; 2];

// TODO: make sure there are no endian-issues
//       and use .bits() instead of manual set conversion
//
// as_index() isn't on a trait, so these need two functions for now
fn mask_of_digits(set: Set<Digit>) -> Mask16 {
    let mut mask = 0;
    for digit in set {
        mask |= 1 << digit.as_index();
    }
    mask
}

fn mask_of_positions_house(set: Set<Position<House>>) -> Mask16 {
    let mut mask = 0;
    for position in set {
        mask |= 1 << position.as_index();
    }
    mask
}

fn mask_of_positions_line(set: Set<Position<Line>>) -> Mask16 {
    let mut mask = 0;
    for position in set {
        mask |= 1 << position.as_index();
    }
    mask
}

fn mask_of_lines(set: Set<Line>) -> Mask32 {
    let mut mask = 0;
    for line in set {
        mask |= 1 << line.as_index();
    }
    mask
}

fn mask_of_cells(set: Set<Cell>) -> Mask128 {
    let mut mask = [0; 2];
    for cell in set {
        let idx = cell.as_index();
        mask[idx / 64] |= 1 << idx % 64;
    }
    mask
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Subsets {
    house: u8,
    positions: Mask16,
    digits: Mask16,
    conflicts: Conflicts,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BasicFish {
    lines: Mask32,
    positions: Mask16,
    digit: u8,
    conflicts: Conflicts,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Fish {
    digit: u8,
    base: Mask32, // set of houses
    cover: Mask32, // set of houses
    conflicts: Conflicts,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Wing {
    hinge: u8, // cell
    hinge_digits: Mask16,
    pincers: Mask128, // mask of cells
    conflicts: Conflicts,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AvoidableRectangle {
    /// The 2 rows and 2 columns forming the avoidable rectangle.
    /// The cells where they overlap always occupy 2 blocks in one chute.
    lines: Mask32, // mask of lines
    conflicts: Conflicts,
}

/*
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SinglesChain {
    conflicts: Conflicts,
}
*/

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Conflicts {
    ptr: *const _RCandidate,
    len: size_t,
}

pub enum _RCandidate {}

impl<'a> From<&'a [RCandidate]> for Conflicts {
    fn from(slice: &[RCandidate]) -> Self {
        let ptr = slice.as_ptr() as *const _RCandidate;
        let len = slice.len();
        Conflicts {
            ptr, len
        }
    }
}

#[no_mangle]
pub extern "C" fn conflicts_len(conflicts: Conflicts) -> size_t {
    conflicts.len
}

#[no_mangle]
pub unsafe extern "C" fn conflicts_get(conflicts: Conflicts, idx: size_t) -> Candidate {
    let Conflicts { ptr, len } = conflicts;
    assert!(idx < len);
    let slice = slice::from_raw_parts(ptr as *const RCandidate, len);
    slice[idx].into()
}
