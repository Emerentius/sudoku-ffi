use ::board::candidate::Candidate;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Deduction {
    tag: DeductionTag,
    data: DeductionData,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum DeductionTag {
    NakedSingle,
    HiddenSingle,
    LockedCandidates,
    NakedSubset,
    HiddenSubset,
    BasicFish,
    SinglesChain,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union DeductionData {
    given: Given,
    naked_single: NakedSingle,
    hidden_single: HiddenSingle,
    locked_candidates: LockedCandidates,
    naked_subsets: NakedSubsets,
    hidden_subsets: HiddenSubsets,
    basic_fish: BasicFish,
    singles_chain: SinglesChain,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Given {
    candidate: Candidate,
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LockedCandidates {
    miniline: u8,
    digits: Mask16,
}

// bitmask
type Mask16 = u16;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NakedSubsets {
    house: u8,
    positions: Mask16,
    digits: Mask16,
    conflicts: *const Candidate,
    len_conflicts: ::libc::size_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HiddenSubsets {
    house: u8,
    digits: Mask16,
    positions: Mask16,
    conflicts: *const Candidate,
    len_conflicts: ::libc::size_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BasicFish {
    lines: Mask16,
    positions: Mask16,
    digit: u8,
    conflicts: *const Candidate,
    len_conflicts: ::libc::size_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SinglesChain {
    conflicts: *const Candidate,
    len_conflicts: ::libc::size_t,
}
