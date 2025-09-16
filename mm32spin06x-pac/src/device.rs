#[doc = "DEVICE"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Device {
    ptr: *mut u8,
}
unsafe impl Send for Device {}
unsafe impl Sync for Device {}
impl Device {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn uid1(self) -> crate::common::Reg<regs::Uid1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn uid2(self) -> crate::common::Reg<regs::Uid2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn uid3(self) -> crate::common::Reg<regs::Uid3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
