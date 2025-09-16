#[doc = "comparator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp {
    ptr: *mut u8,
}
unsafe impl Send for Comp {}
unsafe impl Sync for Comp {}
impl Comp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "COMP1_CSR"]
    #[inline(always)]
    pub const fn comp1_csr(self) -> crate::common::Reg<regs::Comp1Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "COMP_CRV"]
    #[inline(always)]
    pub const fn comp_crv(self) -> crate::common::Reg<regs::CompCrv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "COMP1_POLL"]
    #[inline(always)]
    pub const fn comp1_poll(self) -> crate::common::Reg<regs::Comp1Poll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs;
