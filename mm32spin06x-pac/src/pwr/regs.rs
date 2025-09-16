#[doc = "CR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Low power deepsleep"]
    #[must_use]
    #[inline(always)]
    pub const fn lpds(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Low power deepsleep"]
    #[inline(always)]
    pub const fn set_lpds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power down deepsleep"]
    #[must_use]
    #[inline(always)]
    pub const fn pdds(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power down deepsleep"]
    #[inline(always)]
    pub const fn set_pdds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear wakeup flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cwuf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear wakeup flag"]
    #[inline(always)]
    pub const fn set_cwuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear standby flag"]
    #[must_use]
    #[inline(always)]
    pub const fn csbf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear standby flag"]
    #[inline(always)]
    pub const fn set_csbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Power voltage detector enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pvde(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Power voltage detector enable"]
    #[inline(always)]
    pub const fn set_pvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dbp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "domain write protection"]
    #[inline(always)]
    pub const fn set_dbp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PVD level selection"]
    #[must_use]
    #[inline(always)]
    pub const fn pls(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "PVD level selection"]
    #[inline(always)]
    pub const fn set_pls(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[doc = "Quickly wake-up standby mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn stdby_fs_wk(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Quickly wake-up standby mode selection"]
    #[inline(always)]
    pub const fn set_stdby_fs_wk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
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
            .field("lpds", &self.lpds())
            .field("pdds", &self.pdds())
            .field("cwuf", &self.cwuf())
            .field("csbf", &self.csbf())
            .field("pvde", &self.pvde())
            .field("dbp", &self.dbp())
            .field("pls", &self.pls())
            .field("stdby_fs_wk", &self.stdby_fs_wk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ lpds: {=bool:?}, pdds: {=bool:?}, cwuf: {=bool:?}, csbf: {=bool:?}, pvde: {=bool:?}, dbp: {=bool:?}, pls: {=u8:?}, stdby_fs_wk: {=u8:?} }}",
            self.lpds(),
            self.pdds(),
            self.cwuf(),
            self.csbf(),
            self.pvde(),
            self.dbp(),
            self.pls(),
            self.stdby_fs_wk()
        )
    }
}
#[doc = "CSR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Wakeup flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wuf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup flag"]
    #[inline(always)]
    pub const fn set_wuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Standby flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sbf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Standby flag"]
    #[inline(always)]
    pub const fn set_sbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PVD output"]
    #[must_use]
    #[inline(always)]
    pub const fn pvdo(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PVD output"]
    #[inline(always)]
    pub const fn set_pvdo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable WKUP pin"]
    #[must_use]
    #[inline(always)]
    pub const fn ewup(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable WKUP pin"]
    #[inline(always)]
    pub const fn set_ewup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
            .field("wuf", &self.wuf())
            .field("sbf", &self.sbf())
            .field("pvdo", &self.pvdo())
            .field("ewup", &self.ewup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csr {{ wuf: {=bool:?}, sbf: {=bool:?}, pvdo: {=bool:?}, ewup: {=bool:?} }}",
            self.wuf(),
            self.sbf(),
            self.pvdo(),
            self.ewup()
        )
    }
}
