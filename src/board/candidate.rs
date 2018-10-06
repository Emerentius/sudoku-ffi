use ::sudoku::board::Candidate as RCandidate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Candidate {
    cell: u8,
    num: u8,
}

impl From<RCandidate> for Candidate {
    fn from(entry: RCandidate) -> Self {
        let num = entry.digit.get();
        let cell = entry.cell.get();
        Candidate { cell, num }
    }
}

impl From<Candidate> for RCandidate {
    fn from(Candidate { cell, num }: Candidate) -> Self {
        RCandidate::new(cell, num)
    }
}
