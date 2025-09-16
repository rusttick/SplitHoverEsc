#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "CRC reset"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CRC reset"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("reset", &self.reset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctrl {{ reset: {=bool:?} }}", self.reset())
    }
}
#[doc = "Data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "Data register bits"]
    #[must_use]
    #[inline(always)]
    pub const fn dr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data register bits"]
    #[inline(always)]
    pub const fn set_dr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
impl core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr").field("dr", &self.dr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr {{ dr: {=u32:?} }}", self.dr())
    }
}
#[doc = "Independent data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idr(pub u32);
impl Idr {
    #[doc = "General-purpose 8-bit data register bits"]
    #[must_use]
    #[inline(always)]
    pub const fn idr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "General-purpose 8-bit data register bits"]
    #[inline(always)]
    pub const fn set_idr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Idr {
    #[inline(always)]
    fn default() -> Idr {
        Idr(0)
    }
}
impl core::fmt::Debug for Idr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Idr").field("idr", &self.idr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Idr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Idr {{ idr: {=u8:?} }}", self.idr())
    }
}
#[doc = "Reversed data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reverse(pub u32);
impl Reverse {
    #[doc = "Data reverse register bits"]
    #[must_use]
    #[inline(always)]
    pub const fn reverse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Data reverse register bits"]
    #[inline(always)]
    pub const fn set_reverse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Reverse {
    #[inline(always)]
    fn default() -> Reverse {
        Reverse(0)
    }
}
impl core::fmt::Debug for Reverse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reverse")
            .field("reverse", &self.reverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reverse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reverse {{ reverse: {=bool:?} }}", self.reverse())
    }
}
