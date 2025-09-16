#[doc = "OPAMP_CSR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OpampCsr(pub u32);
impl OpampCsr {
    #[doc = "operational amplifier 1 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "operational amplifier 1 enable"]
    #[inline(always)]
    pub const fn set_opamp1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "operational amplifier 2 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "operational amplifier 2 enable"]
    #[inline(always)]
    pub const fn set_opamp2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "operational amplifier 3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp3_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "operational amplifier 3 enable"]
    #[inline(always)]
    pub const fn set_opamp3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "operational amplifier 4 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp4_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "operational amplifier 4 enable"]
    #[inline(always)]
    pub const fn set_opamp4_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for OpampCsr {
    #[inline(always)]
    fn default() -> OpampCsr {
        OpampCsr(0)
    }
}
impl core::fmt::Debug for OpampCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OpampCsr")
            .field("opamp1_en", &self.opamp1_en())
            .field("opamp2_en", &self.opamp2_en())
            .field("opamp3_en", &self.opamp3_en())
            .field("opamp4_en", &self.opamp4_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OpampCsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OpampCsr {{ opamp1_en: {=bool:?}, opamp2_en: {=bool:?}, opamp3_en: {=bool:?}, opamp4_en: {=bool:?} }}",
            self.opamp1_en(),
            self.opamp2_en(),
            self.opamp3_en(),
            self.opamp4_en()
        )
    }
}
