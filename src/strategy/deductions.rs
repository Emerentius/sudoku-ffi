use ::sudoku::strategy::deduction::Deductions as RDeductions;
use ::libc::size_t;

pub enum _Deductions {}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Deductions(pub *mut _Deductions);

impl Deductions {
    fn as_rdeductions(self) -> *mut RDeductions {
        self.0 as *mut RDeductions
    }
}

#[no_mangle]
pub extern "C" fn deductions_len(deductions: Deductions) -> size_t {
    unsafe {
        (&*deductions.as_rdeductions()).len()
    }
}
