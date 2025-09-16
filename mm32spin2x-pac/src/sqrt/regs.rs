#[doc = "RESULT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Result(pub u32);
impl Result {
    #[doc = "Result data"]
    #[must_use]
    #[inline(always)]
    pub const fn result(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Result data"]
    #[inline(always)]
    pub const fn set_result(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Result {
    #[inline(always)]
    fn default() -> Result {
        Result(0)
    }
}
impl core::fmt::Debug for Result {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Result")
            .field("result", &self.result())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Result {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Result {{ result: {=u16:?} }}", self.result())
    }
}
#[doc = "SQR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqr(pub u32);
impl Sqr {
    #[doc = "Square data"]
    #[must_use]
    #[inline(always)]
    pub const fn square(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Square data"]
    #[inline(always)]
    pub const fn set_square(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sqr {
    #[inline(always)]
    fn default() -> Sqr {
        Sqr(0)
    }
}
impl core::fmt::Debug for Sqr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sqr")
            .field("square", &self.square())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sqr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sqr {{ square: {=u32:?} }}", self.square())
    }
}
