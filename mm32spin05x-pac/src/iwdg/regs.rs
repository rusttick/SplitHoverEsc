#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Interruput select"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interruput select"]
    #[inline(always)]
    pub const fn set_irq_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interruput clear"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_clr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interruput clear"]
    #[inline(always)]
    pub const fn set_irq_clr(&mut self, val: bool) {
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
            .field("irq_sel", &self.irq_sel())
            .field("irq_clr", &self.irq_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ irq_sel: {=bool:?}, irq_clr: {=bool:?} }}",
            self.irq_sel(),
            self.irq_clr()
        )
    }
}
#[doc = "Key register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kr(pub u32);
impl Kr {
    #[doc = "Key value"]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Key value"]
    #[inline(always)]
    pub const fn set_key(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Kr {
    #[inline(always)]
    fn default() -> Kr {
        Kr(0)
    }
}
impl core::fmt::Debug for Kr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kr").field("key", &self.key()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Kr {{ key: {=u16:?} }}", self.key())
    }
}
#[doc = "Prescaler register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pr(pub u32);
impl Pr {
    #[doc = "Prescaler divider"]
    #[must_use]
    #[inline(always)]
    pub const fn pr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Prescaler divider"]
    #[inline(always)]
    pub const fn set_pr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Pr {
    #[inline(always)]
    fn default() -> Pr {
        Pr(0)
    }
}
impl core::fmt::Debug for Pr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pr").field("pr", &self.pr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pr {{ pr: {=u8:?} }}", self.pr())
    }
}
#[doc = "Reload register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rlr(pub u32);
impl Rlr {
    #[doc = "Watchdog counter reload value"]
    #[must_use]
    #[inline(always)]
    pub const fn rl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Watchdog counter reload value"]
    #[inline(always)]
    pub const fn set_rl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Rlr {
    #[inline(always)]
    fn default() -> Rlr {
        Rlr(0)
    }
}
impl core::fmt::Debug for Rlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rlr").field("rl", &self.rl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rlr {{ rl: {=u16:?} }}", self.rl())
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Watchdog prescaler value update"]
    #[must_use]
    #[inline(always)]
    pub const fn pvu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog prescaler value update"]
    #[inline(always)]
    pub const fn set_pvu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Watchdog counter reload value update"]
    #[must_use]
    #[inline(always)]
    pub const fn rvu(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog counter reload value update"]
    #[inline(always)]
    pub const fn set_rvu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
        f.debug_struct("Sr")
            .field("pvu", &self.pvu())
            .field("rvu", &self.rvu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ pvu: {=bool:?}, rvu: {=bool:?} }}",
            self.pvu(),
            self.rvu()
        )
    }
}
