use ::sudoku::strategy::Strategy as RStrategy;

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

impl From<RStrategy> for Strategy {
    fn from(strat: RStrategy) -> Self {
        use self::Strategy::*;
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
}

impl From<Strategy> for RStrategy {
    // TODO: How should UnknownStrategy be handled?
    fn from(strat: Strategy) -> RStrategy {
        use self::Strategy::*;
        match strat {
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
