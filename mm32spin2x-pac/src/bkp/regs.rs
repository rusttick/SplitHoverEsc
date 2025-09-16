#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr1(pub u32);
impl Dr1 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr1 {
    #[inline(always)]
    fn default() -> Dr1 {
        Dr1(0)
    }
}
impl core::fmt::Debug for Dr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr1").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr1 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr10(pub u32);
impl Dr10 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr10 {
    #[inline(always)]
    fn default() -> Dr10 {
        Dr10(0)
    }
}
impl core::fmt::Debug for Dr10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr10").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr10 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr11(pub u32);
impl Dr11 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr11 {
    #[inline(always)]
    fn default() -> Dr11 {
        Dr11(0)
    }
}
impl core::fmt::Debug for Dr11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr11").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr11 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr12(pub u32);
impl Dr12 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr12 {
    #[inline(always)]
    fn default() -> Dr12 {
        Dr12(0)
    }
}
impl core::fmt::Debug for Dr12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr12").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr12 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr13(pub u32);
impl Dr13 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr13 {
    #[inline(always)]
    fn default() -> Dr13 {
        Dr13(0)
    }
}
impl core::fmt::Debug for Dr13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr13").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr13 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr14(pub u32);
impl Dr14 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr14 {
    #[inline(always)]
    fn default() -> Dr14 {
        Dr14(0)
    }
}
impl core::fmt::Debug for Dr14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr14").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr14 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr15(pub u32);
impl Dr15 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr15 {
    #[inline(always)]
    fn default() -> Dr15 {
        Dr15(0)
    }
}
impl core::fmt::Debug for Dr15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr15").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr15 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr16(pub u32);
impl Dr16 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr16 {
    #[inline(always)]
    fn default() -> Dr16 {
        Dr16(0)
    }
}
impl core::fmt::Debug for Dr16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr16").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr16 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr17(pub u32);
impl Dr17 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr17 {
    #[inline(always)]
    fn default() -> Dr17 {
        Dr17(0)
    }
}
impl core::fmt::Debug for Dr17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr17").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr17 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr18(pub u32);
impl Dr18 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr18 {
    #[inline(always)]
    fn default() -> Dr18 {
        Dr18(0)
    }
}
impl core::fmt::Debug for Dr18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr18").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr18 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr19(pub u32);
impl Dr19 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr19 {
    #[inline(always)]
    fn default() -> Dr19 {
        Dr19(0)
    }
}
impl core::fmt::Debug for Dr19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr19").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr19 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr2(pub u32);
impl Dr2 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr2 {
    #[inline(always)]
    fn default() -> Dr2 {
        Dr2(0)
    }
}
impl core::fmt::Debug for Dr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr2").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr2 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr20(pub u32);
impl Dr20 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr20 {
    #[inline(always)]
    fn default() -> Dr20 {
        Dr20(0)
    }
}
impl core::fmt::Debug for Dr20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr20").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr20 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr3(pub u32);
impl Dr3 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr3 {
    #[inline(always)]
    fn default() -> Dr3 {
        Dr3(0)
    }
}
impl core::fmt::Debug for Dr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr3").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr3 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr4(pub u32);
impl Dr4 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr4 {
    #[inline(always)]
    fn default() -> Dr4 {
        Dr4(0)
    }
}
impl core::fmt::Debug for Dr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr4").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr4 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr5(pub u32);
impl Dr5 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr5 {
    #[inline(always)]
    fn default() -> Dr5 {
        Dr5(0)
    }
}
impl core::fmt::Debug for Dr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr5").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr5 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr6(pub u32);
impl Dr6 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr6 {
    #[inline(always)]
    fn default() -> Dr6 {
        Dr6(0)
    }
}
impl core::fmt::Debug for Dr6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr6").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr6 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr7(pub u32);
impl Dr7 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr7 {
    #[inline(always)]
    fn default() -> Dr7 {
        Dr7(0)
    }
}
impl core::fmt::Debug for Dr7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr7").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr7 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr8(pub u32);
impl Dr8 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr8 {
    #[inline(always)]
    fn default() -> Dr8 {
        Dr8(0)
    }
}
impl core::fmt::Debug for Dr8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr8").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr8 {{ d: {=u16:?} }}", self.d())
    }
}
#[doc = "Backup data register(BKP_DR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr9(pub u32);
impl Dr9 {
    #[doc = "Backup data"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Backup data"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr9 {
    #[inline(always)]
    fn default() -> Dr9 {
        Dr9(0)
    }
}
impl core::fmt::Debug for Dr9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr9").field("d", &self.d()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr9 {{ d: {=u16:?} }}", self.d())
    }
}
