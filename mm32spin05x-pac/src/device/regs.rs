#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uid1(pub u16);
impl Uid1 {
    #[doc = "15:0 unique ID bits"]
    #[must_use]
    #[inline(always)]
    pub const fn u_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0 unique ID bits"]
    #[inline(always)]
    pub const fn set_u_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Uid1 {
    #[inline(always)]
    fn default() -> Uid1 {
        Uid1(0)
    }
}
impl core::fmt::Debug for Uid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uid1").field("u_id", &self.u_id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uid1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uid1 {{ u_id: {=u16:?} }}", self.u_id())
    }
}
#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uid2(pub u16);
impl Uid2 {
    #[doc = "31:16 unique ID bits"]
    #[must_use]
    #[inline(always)]
    pub const fn u_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16 unique ID bits"]
    #[inline(always)]
    pub const fn set_u_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Uid2 {
    #[inline(always)]
    fn default() -> Uid2 {
        Uid2(0)
    }
}
impl core::fmt::Debug for Uid2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uid2").field("u_id", &self.u_id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uid2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uid2 {{ u_id: {=u16:?} }}", self.u_id())
    }
}
#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uid3(pub u32);
impl Uid3 {
    #[doc = "63:32 unique ID bits"]
    #[must_use]
    #[inline(always)]
    pub const fn u_id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "63:32 unique ID bits"]
    #[inline(always)]
    pub const fn set_u_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Uid3 {
    #[inline(always)]
    fn default() -> Uid3 {
        Uid3(0)
    }
}
impl core::fmt::Debug for Uid3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uid3").field("u_id", &self.u_id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uid3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uid3 {{ u_id: {=u32:?} }}", self.u_id())
    }
}
#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uid4(pub u32);
impl Uid4 {
    #[doc = "95:64 unique ID bits"]
    #[must_use]
    #[inline(always)]
    pub const fn u_id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "95:64 unique ID bits"]
    #[inline(always)]
    pub const fn set_u_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Uid4 {
    #[inline(always)]
    fn default() -> Uid4 {
        Uid4(0)
    }
}
impl core::fmt::Debug for Uid4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uid4").field("u_id", &self.u_id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uid4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uid4 {{ u_id: {=u32:?} }}", self.u_id())
    }
}
