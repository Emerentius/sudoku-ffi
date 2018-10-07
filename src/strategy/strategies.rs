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
}

impl From<RStrategy> for Strategy {
    fn from(strat: RStrategy) -> Self {
        use self::Strategy::*;
        match strat {
            RStrategy::NakedSingles => NakedSingles,
            RStrategy::HiddenSingles => HiddenSingles,
            RStrategy::LockedCandidates => LockedCandidates,
            RStrategy::NakedPairs => NakedPairs,
            RStrategy::NakedTriples => NakedTriples,
            RStrategy::NakedQuads => NakedQuads,
            RStrategy::HiddenPairs => HiddenPairs,
            RStrategy::HiddenTriples => HiddenTriples,
            RStrategy::HiddenQuads => HiddenQuads,
            RStrategy::XWing => XWing,
            RStrategy::Swordfish => Swordfish,
            RStrategy::Jellyfish => Jellyfish,
            RStrategy::SinglesChain => SinglesChain,
            RStrategy::__NonExhaustive => unreachable!(),
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
            LockedCandidates => RStrategy::LockedCandidates,
            NakedPairs => RStrategy::NakedPairs,
            NakedTriples => RStrategy::NakedTriples,
            NakedQuads => RStrategy::NakedQuads,
            HiddenPairs => RStrategy::HiddenPairs,
            HiddenTriples => RStrategy::HiddenTriples,
            HiddenQuads => RStrategy::HiddenQuads,
            XWing => RStrategy::XWing,
            Swordfish => RStrategy::Swordfish,
            Jellyfish => RStrategy::Jellyfish,
            SinglesChain => RStrategy::SinglesChain,
        }
    }
}
