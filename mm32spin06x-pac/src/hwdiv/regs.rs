#[doc = "control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "unsigned enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usign(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "unsigned enable"]
    #[inline(always)]
    pub const fn set_usign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Overflow interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovfe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow interrupt enable"]
    #[inline(always)]
    pub const fn set_ovfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("usign", &self.usign())
            .field("ovfe", &self.ovfe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ usign: {=bool:?}, ovfe: {=bool:?} }}",
            self.usign(),
            self.ovfe()
        )
    }
}
#[doc = "Dividend register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvdr(pub u32);
impl Dvdr {
    #[doc = "Dividend data"]
    #[must_use]
    #[inline(always)]
    pub const fn dividend(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Dividend data"]
    #[inline(always)]
    pub const fn set_dividend(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dvdr {
    #[inline(always)]
    fn default() -> Dvdr {
        Dvdr(0)
    }
}
impl core::fmt::Debug for Dvdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dvdr")
            .field("dividend", &self.dividend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dvdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dvdr {{ dividend: {=u32:?} }}", self.dividend())
    }
}
#[doc = "Divisor register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvsr(pub u32);
impl Dvsr {
    #[doc = "Divisor data"]
    #[must_use]
    #[inline(always)]
    pub const fn divisor(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Divisor data"]
    #[inline(always)]
    pub const fn set_divisor(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dvsr {
    #[inline(always)]
    fn default() -> Dvsr {
        Dvsr(0)
    }
}
impl core::fmt::Debug for Dvsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dvsr")
            .field("divisor", &self.divisor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dvsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dvsr {{ divisor: {=u32:?} }}", self.divisor())
    }
}
#[doc = "Quotient register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Quotr(pub u32);
impl Quotr {
    #[doc = "Quotient data"]
    #[must_use]
    #[inline(always)]
    pub const fn quotient(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Quotient data"]
    #[inline(always)]
    pub const fn set_quotient(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Quotr {
    #[inline(always)]
    fn default() -> Quotr {
        Quotr(0)
    }
}
impl core::fmt::Debug for Quotr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Quotr")
            .field("quotient", &self.quotient())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Quotr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Quotr {{ quotient: {=u32:?} }}", self.quotient())
    }
}
#[doc = "Remainder register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmdr(pub u32);
impl Rmdr {
    #[doc = "Remainder data"]
    #[must_use]
    #[inline(always)]
    pub const fn remainder(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Remainder data"]
    #[inline(always)]
    pub const fn set_remainder(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rmdr {
    #[inline(always)]
    fn default() -> Rmdr {
        Rmdr(0)
    }
}
impl core::fmt::Debug for Rmdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rmdr")
            .field("remainder", &self.remainder())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rmdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rmdr {{ remainder: {=u32:?} }}", self.remainder())
    }
}
#[doc = "status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Overflow flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ovf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow flag"]
    #[inline(always)]
    pub const fn set_ovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr").field("ovf", &self.ovf()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sr {{ ovf: {=bool:?} }}", self.ovf())
    }
}
