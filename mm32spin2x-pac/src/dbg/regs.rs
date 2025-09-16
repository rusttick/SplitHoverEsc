#[doc = "CR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Debug Sleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_sleep(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Sleep mode"]
    #[inline(always)]
    pub const fn set_dbg_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Debug Stop mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Stop mode"]
    #[inline(always)]
    pub const fn set_dbg_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Debug Standby mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_standby(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Standby mode"]
    #[inline(always)]
    pub const fn set_dbg_standby(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug independent watchdog stopped when core is stopped"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_iwdg_stop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Debug independent watchdog stopped when core is stopped"]
    #[inline(always)]
    pub const fn set_dbg_iwdg_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Debug window watchdog when core is halted"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_wwdg_stop(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Debug window watchdog when core is halted"]
    #[inline(always)]
    pub const fn set_dbg_wwdg_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "TIMx counter stopped when core is halted"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_timx_stop(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "TIMx counter stopped when core is halted"]
    #[inline(always)]
    pub const fn set_dbg_timx_stop(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
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
            .field("dbg_sleep", &self.dbg_sleep())
            .field("dbg_stop", &self.dbg_stop())
            .field("dbg_standby", &self.dbg_standby())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
            .field("dbg_timx_stop", &self.dbg_timx_stop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ dbg_sleep: {=bool:?}, dbg_stop: {=bool:?}, dbg_standby: {=bool:?}, dbg_iwdg_stop: {=bool:?}, dbg_wwdg_stop: {=bool:?}, dbg_timx_stop: {=u8:?} }}",
            self.dbg_sleep(),
            self.dbg_stop(),
            self.dbg_standby(),
            self.dbg_iwdg_stop(),
            self.dbg_wwdg_stop(),
            self.dbg_timx_stop()
        )
    }
}
#[doc = "IDCODE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idcode(pub u32);
impl Idcode {
    #[doc = "Device identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Device identifier"]
    #[inline(always)]
    pub const fn set_dev_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Idcode {
    #[inline(always)]
    fn default() -> Idcode {
        Idcode(0)
    }
}
impl core::fmt::Debug for Idcode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Idcode")
            .field("dev_id", &self.dev_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Idcode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Idcode {{ dev_id: {=u32:?} }}", self.dev_id())
    }
}
