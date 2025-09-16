#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc = "7-bit window value"]
    #[must_use]
    #[inline(always)]
    pub const fn window(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "7-bit window value"]
    #[inline(always)]
    pub const fn set_window(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Timer base"]
    #[must_use]
    #[inline(always)]
    pub const fn wdgtb(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Timer base"]
    #[inline(always)]
    pub const fn set_wdgtb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "Early wakeup interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ewi(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Early wakeup interrupt"]
    #[inline(always)]
    pub const fn set_ewi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Cfgr {
    #[inline(always)]
    fn default() -> Cfgr {
        Cfgr(0)
    }
}
impl core::fmt::Debug for Cfgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgr")
            .field("window", &self.window())
            .field("wdgtb", &self.wdgtb())
            .field("ewi", &self.ewi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfgr {{ window: {=u8:?}, wdgtb: {=u8:?}, ewi: {=bool:?} }}",
            self.window(),
            self.wdgtb(),
            self.ewi()
        )
    }
}
#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "7-bit counter"]
    #[must_use]
    #[inline(always)]
    pub const fn t(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "7-bit counter"]
    #[inline(always)]
    pub const fn set_t(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Activation bit"]
    #[must_use]
    #[inline(always)]
    pub const fn wdga(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Activation bit"]
    #[inline(always)]
    pub const fn set_wdga(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("t", &self.t())
            .field("wdga", &self.wdga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ t: {=u8:?}, wdga: {=bool:?} }}",
            self.t(),
            self.wdga()
        )
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Early wakeup interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ewif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Early wakeup interrupt flag"]
    #[inline(always)]
    pub const fn set_ewif(&mut self, val: bool) {
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
        f.debug_struct("Sr").field("ewif", &self.ewif()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sr {{ ewif: {=bool:?} }}", self.ewif())
    }
}
