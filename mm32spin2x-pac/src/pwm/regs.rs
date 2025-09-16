#[doc = "Auto phase mask dalay register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apmdlr(pub u32);
impl Apmdlr {
    #[doc = "*D0"]
    #[must_use]
    #[inline(always)]
    pub const fn apmdlr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "*D0"]
    #[inline(always)]
    pub const fn set_apmdlr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
    }
}
impl Default for Apmdlr {
    #[inline(always)]
    fn default() -> Apmdlr {
        Apmdlr(0)
    }
}
impl core::fmt::Debug for Apmdlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apmdlr")
            .field("apmdlr", &self.apmdlr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apmdlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Apmdlr {{ apmdlr: {=u32:?} }}", self.apmdlr())
    }
}
#[doc = "Auto phase mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apmskr(pub u32);
impl Apmskr {
    #[doc = "PWM Mask Data"]
    #[must_use]
    #[inline(always)]
    pub const fn mskdat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "PWM Mask Data"]
    #[inline(always)]
    pub const fn set_mskdat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "PWM Mask Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn msken(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "PWM Mask Function Enable"]
    #[inline(always)]
    pub const fn set_msken(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Auto Phase Mask Tigger Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn apm_trigsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Auto Phase Mask Tigger Selection"]
    #[inline(always)]
    pub const fn set_apm_trigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Auto Phase Mask Software Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn apm_strg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Phase Mask Software Trigger"]
    #[inline(always)]
    pub const fn set_apm_strg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Expect Next Trigger Input"]
    #[must_use]
    #[inline(always)]
    pub const fn entrgi(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Expect Next Trigger Input"]
    #[inline(always)]
    pub const fn set_entrgi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Current Trigger Input"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrgi(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x07;
        val as u8
    }
    #[doc = "Current Trigger Input"]
    #[inline(always)]
    pub const fn set_ctrgi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
    }
}
impl Default for Apmskr {
    #[inline(always)]
    fn default() -> Apmskr {
        Apmskr(0)
    }
}
impl core::fmt::Debug for Apmskr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apmskr")
            .field("mskdat", &self.mskdat())
            .field("msken", &self.msken())
            .field("apm_trigsel", &self.apm_trigsel())
            .field("apm_strg", &self.apm_strg())
            .field("entrgi", &self.entrgi())
            .field("ctrgi", &self.ctrgi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apmskr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apmskr {{ mskdat: {=u8:?}, msken: {=u8:?}, apm_trigsel: {=u8:?}, apm_strg: {=bool:?}, entrgi: {=u8:?}, ctrgi: {=u8:?} }}",
            self.mskdat(),
            self.msken(),
            self.apm_trigsel(),
            self.apm_strg(),
            self.entrgi(),
            self.ctrgi()
        )
    }
}
#[doc = "Control PWM output status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Current Compensation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cce(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current Compensation Enable"]
    #[inline(always)]
    pub const fn set_cce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Current Protection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cpe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Current Protection Enable"]
    #[inline(always)]
    pub const fn set_cpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto Phase Mask Trigger Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn apmtie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Phase Mask Trigger Interrupt Enable"]
    #[inline(always)]
    pub const fn set_apmtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Trigger Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn terrie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_terrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Current Protection Trigger Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cc_trgsel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Current Protection Trigger Selection"]
    #[inline(always)]
    pub const fn set_cc_trgsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Current Compensation Software Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn cc_strg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Current Compensation Software Trigger"]
    #[inline(always)]
    pub const fn set_cc_strg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Current Protection Trigger Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cp_trgsel(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "Current Protection Trigger Selection"]
    #[inline(always)]
    pub const fn set_cp_trgsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "Current Protection Mode Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cp_mdsel(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Current Protection Mode Selection"]
    #[inline(always)]
    pub const fn set_cp_mdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Auto Phase Mask Trigger Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn apmtif(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Phase Mask Trigger Flag"]
    #[inline(always)]
    pub const fn set_apmtif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Trigger Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn terrif(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Error Flag"]
    #[inline(always)]
    pub const fn set_terrif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO Input Filter"]
    #[must_use]
    #[inline(always)]
    pub const fn ioflt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "GPIO Input Filter"]
    #[inline(always)]
    pub const fn set_ioflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Hall Sensor Trigger 3-channel select"]
    #[must_use]
    #[inline(always)]
    pub const fn hall_trgsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Hall Sensor Trigger 3-channel select"]
    #[inline(always)]
    pub const fn set_hall_trgsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Enable Current Input Status Value"]
    #[must_use]
    #[inline(always)]
    pub const fn curen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Current Input Status Value"]
    #[inline(always)]
    pub const fn set_curen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Immediate Output of The Port when PWM is Masked"]
    #[must_use]
    #[inline(always)]
    pub const fn mskdat(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x3f;
        val as u8
    }
    #[doc = "Immediate Output of The Port when PWM is Masked"]
    #[inline(always)]
    pub const fn set_mskdat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 19usize)) | (((val as u32) & 0x3f) << 19usize);
    }
    #[doc = "PWM Output Mask Immediately Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn msken_curr(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x3f;
        val as u8
    }
    #[doc = "PWM Output Mask Immediately Enable"]
    #[inline(always)]
    pub const fn set_msken_curr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 25usize)) | (((val as u32) & 0x3f) << 25usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr")
            .field("cce", &self.cce())
            .field("cpe", &self.cpe())
            .field("apmtie", &self.apmtie())
            .field("terrie", &self.terrie())
            .field("cc_trgsel", &self.cc_trgsel())
            .field("cc_strg", &self.cc_strg())
            .field("cp_trgsel", &self.cp_trgsel())
            .field("cp_mdsel", &self.cp_mdsel())
            .field("apmtif", &self.apmtif())
            .field("terrif", &self.terrif())
            .field("ioflt", &self.ioflt())
            .field("hall_trgsel", &self.hall_trgsel())
            .field("curen", &self.curen())
            .field("mskdat", &self.mskdat())
            .field("msken_curr", &self.msken_curr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csr {{ cce: {=bool:?}, cpe: {=bool:?}, apmtie: {=bool:?}, terrie: {=bool:?}, cc_trgsel: {=u8:?}, cc_strg: {=bool:?}, cp_trgsel: {=u8:?}, cp_mdsel: {=bool:?}, apmtif: {=bool:?}, terrif: {=bool:?}, ioflt: {=u8:?}, hall_trgsel: {=u8:?}, curen: {=bool:?}, mskdat: {=u8:?}, msken_curr: {=u8:?} }}",
            self.cce(),
            self.cpe(),
            self.apmtie(),
            self.terrie(),
            self.cc_trgsel(),
            self.cc_strg(),
            self.cp_trgsel(),
            self.cp_mdsel(),
            self.apmtif(),
            self.terrif(),
            self.ioflt(),
            self.hall_trgsel(),
            self.curen(),
            self.mskdat(),
            self.msken_curr()
        )
    }
}
