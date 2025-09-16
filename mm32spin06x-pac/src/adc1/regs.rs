#[doc = "Arbitrary channel configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnyCfg(pub u32);
impl AnyCfg {
    #[doc = "channel number configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn chany_num(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "channel number configuration"]
    #[inline(always)]
    pub const fn set_chany_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for AnyCfg {
    #[inline(always)]
    fn default() -> AnyCfg {
        AnyCfg(0)
    }
}
impl core::fmt::Debug for AnyCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnyCfg")
            .field("chany_num", &self.chany_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnyCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AnyCfg {{ chany_num: {=u8:?} }}", self.chany_num())
    }
}
#[doc = "Arbitrary channel control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnyCr(pub u32);
impl AnyCr {
    #[doc = "Any channel configuration mode enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn chany_mden(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Any channel configuration mode enable bit"]
    #[inline(always)]
    pub const fn set_chany_mden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for AnyCr {
    #[inline(always)]
    fn default() -> AnyCr {
        AnyCr(0)
    }
}
impl core::fmt::Debug for AnyCr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnyCr")
            .field("chany_mden", &self.chany_mden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnyCr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AnyCr {{ chany_mden: {=bool:?} }}", self.chany_mden())
    }
}
#[doc = "Configure register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc = "ADC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aden(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC enable"]
    #[inline(always)]
    pub const fn set_aden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC window comparison enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adwen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC window comparison enable"]
    #[inline(always)]
    pub const fn set_adwen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Temperature sensor enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tsen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature sensor enable"]
    #[inline(always)]
    pub const fn set_tsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reference voltage enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vsen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reference voltage enable"]
    #[inline(always)]
    pub const fn set_vsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ADC high prescaler coefficient"]
    #[must_use]
    #[inline(always)]
    pub const fn adcpreh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "ADC high prescaler coefficient"]
    #[inline(always)]
    pub const fn set_adcpreh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn rsltctl(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "Resolution"]
    #[inline(always)]
    pub const fn set_rsltctl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "Channel x Sample time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn samctl(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel x Sample time selection"]
    #[inline(always)]
    pub const fn set_samctl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "ADC low prescaler coefficient"]
    #[must_use]
    #[inline(always)]
    pub const fn adcprel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ADC low prescaler coefficient"]
    #[inline(always)]
    pub const fn set_adcprel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
            .field("aden", &self.aden())
            .field("adwen", &self.adwen())
            .field("tsen", &self.tsen())
            .field("vsen", &self.vsen())
            .field("adcpreh", &self.adcpreh())
            .field("rsltctl", &self.rsltctl())
            .field("samctl", &self.samctl())
            .field("adcprel", &self.adcprel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfgr {{ aden: {=bool:?}, adwen: {=bool:?}, tsen: {=bool:?}, vsen: {=bool:?}, adcpreh: {=u8:?}, rsltctl: {=u8:?}, samctl: {=u8:?}, adcprel: {=bool:?} }}",
            self.aden(),
            self.adwen(),
            self.tsen(),
            self.vsen(),
            self.adcpreh(),
            self.rsltctl(),
            self.samctl(),
            self.adcprel()
        )
    }
}
#[doc = "Arbitrary channel channel selection register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chany0(pub u32);
impl Chany0 {
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Chany0 {
    #[inline(always)]
    fn default() -> Chany0 {
        Chany0(0)
    }
}
impl core::fmt::Debug for Chany0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chany0")
            .field("chany_sel0", &self.chany_sel0())
            .field("chany_sel1", &self.chany_sel1())
            .field("chany_sel2", &self.chany_sel2())
            .field("chany_sel3", &self.chany_sel3())
            .field("chany_sel4", &self.chany_sel4())
            .field("chany_sel5", &self.chany_sel5())
            .field("chany_sel6", &self.chany_sel6())
            .field("chany_sel7", &self.chany_sel7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chany0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Chany0 {{ chany_sel0: {=u8:?}, chany_sel1: {=u8:?}, chany_sel2: {=u8:?}, chany_sel3: {=u8:?}, chany_sel4: {=u8:?}, chany_sel5: {=u8:?}, chany_sel6: {=u8:?}, chany_sel7: {=u8:?} }}",
            self.chany_sel0(),
            self.chany_sel1(),
            self.chany_sel2(),
            self.chany_sel3(),
            self.chany_sel4(),
            self.chany_sel5(),
            self.chany_sel6(),
            self.chany_sel7()
        )
    }
}
#[doc = "Arbitrary channel channel selection register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chany1(pub u32);
impl Chany1 {
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[must_use]
    #[inline(always)]
    pub const fn chany_sel15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub const fn set_chany_sel15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Chany1 {
    #[inline(always)]
    fn default() -> Chany1 {
        Chany1(0)
    }
}
impl core::fmt::Debug for Chany1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chany1")
            .field("chany_sel8", &self.chany_sel8())
            .field("chany_sel9", &self.chany_sel9())
            .field("chany_sel14", &self.chany_sel14())
            .field("chany_sel15", &self.chany_sel15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chany1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Chany1 {{ chany_sel8: {=u8:?}, chany_sel9: {=u8:?}, chany_sel14: {=u8:?}, chany_sel15: {=u8:?} }}",
            self.chany_sel8(),
            self.chany_sel9(),
            self.chany_sel14(),
            self.chany_sel15()
        )
    }
}
#[doc = "Channel select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chsr(pub u32);
impl Chsr {
    #[doc = "Analog input channel 0 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chen0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 0 enable"]
    #[inline(always)]
    pub const fn set_chen0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Analog input channel 1 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chen1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 1 enable"]
    #[inline(always)]
    pub const fn set_chen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Analog input channel 2 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chen2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 2 enable"]
    #[inline(always)]
    pub const fn set_chen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Analog input channel 3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chen3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 3 enable"]
    #[inline(always)]
    pub const fn set_chen3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog input channel 4 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chen4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 4 enable"]
    #[inline(always)]
    pub const fn set_chen4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Analog input channel 5 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chen5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 5 enable"]
    #[inline(always)]
    pub const fn set_chen5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Analog input channel 6 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chen6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 6 enable"]
    #[inline(always)]
    pub const fn set_chen6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Analog input channel 7 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chen7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 7 enable"]
    #[inline(always)]
    pub const fn set_chen7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Analog input channel 8 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chen8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 8 enable"]
    #[inline(always)]
    pub const fn set_chen8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Analog input channel 9 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chen9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 9 enable"]
    #[inline(always)]
    pub const fn set_chen9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Temperature Sensor channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chents(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Sensor channel enable"]
    #[inline(always)]
    pub const fn set_chents(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Internal reference voltage channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chenvs(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Internal reference voltage channel enable"]
    #[inline(always)]
    pub const fn set_chenvs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Chsr {
    #[inline(always)]
    fn default() -> Chsr {
        Chsr(0)
    }
}
impl core::fmt::Debug for Chsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chsr")
            .field("chen0", &self.chen0())
            .field("chen1", &self.chen1())
            .field("chen2", &self.chen2())
            .field("chen3", &self.chen3())
            .field("chen4", &self.chen4())
            .field("chen5", &self.chen5())
            .field("chen6", &self.chen6())
            .field("chen7", &self.chen7())
            .field("chen8", &self.chen8())
            .field("chen9", &self.chen9())
            .field("chents", &self.chents())
            .field("chenvs", &self.chenvs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Chsr {{ chen0: {=bool:?}, chen1: {=bool:?}, chen2: {=bool:?}, chen3: {=bool:?}, chen4: {=bool:?}, chen5: {=bool:?}, chen6: {=bool:?}, chen7: {=bool:?}, chen8: {=bool:?}, chen9: {=bool:?}, chents: {=bool:?}, chenvs: {=bool:?} }}",
            self.chen0(),
            self.chen1(),
            self.chen2(),
            self.chen3(),
            self.chen4(),
            self.chen5(),
            self.chen6(),
            self.chen7(),
            self.chen8(),
            self.chen9(),
            self.chents(),
            self.chenvs()
        )
    }
}
#[doc = "Compare register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpr(pub u32);
impl Cmpr {
    #[doc = "Compare data low limit"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpldata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Compare data low limit"]
    #[inline(always)]
    pub const fn set_cmpldata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Compare data high limit"]
    #[must_use]
    #[inline(always)]
    pub const fn cmphdata(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Compare data high limit"]
    #[inline(always)]
    pub const fn set_cmphdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Cmpr {
    #[inline(always)]
    fn default() -> Cmpr {
        Cmpr(0)
    }
}
impl core::fmt::Debug for Cmpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpr")
            .field("cmpldata", &self.cmpldata())
            .field("cmphdata", &self.cmphdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpr {{ cmpldata: {=u16:?}, cmphdata: {=u16:?} }}",
            self.cmpldata(),
            self.cmphdata()
        )
    }
}
#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "ADC interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC interrupt enable"]
    #[inline(always)]
    pub const fn set_adie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC window comparator interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adwie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC window comparator interrupt enable"]
    #[inline(always)]
    pub const fn set_adwie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "External trigger enable"]
    #[must_use]
    #[inline(always)]
    pub const fn trgen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger enable"]
    #[inline(always)]
    pub const fn set_trgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Direct memory access enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Direct memory access enable"]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "External trigger selection low"]
    #[must_use]
    #[inline(always)]
    pub const fn trgsell(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger selection low"]
    #[inline(always)]
    pub const fn set_trgsell(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "ADC start"]
    #[must_use]
    #[inline(always)]
    pub const fn adst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ADC start"]
    #[inline(always)]
    pub const fn set_adst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ADC mode"]
    #[must_use]
    #[inline(always)]
    pub const fn admd(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "ADC mode"]
    #[inline(always)]
    pub const fn set_admd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Data alignment"]
    #[must_use]
    #[inline(always)]
    pub const fn align(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Data alignment"]
    #[inline(always)]
    pub const fn set_align(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Window comparison channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpch(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Window comparison channel selection"]
    #[inline(always)]
    pub const fn set_cmpch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "ADC scan direction"]
    #[must_use]
    #[inline(always)]
    pub const fn scandir(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ADC scan direction"]
    #[inline(always)]
    pub const fn set_scandir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "External trigger selection high"]
    #[must_use]
    #[inline(always)]
    pub const fn trgselh(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[doc = "External trigger selection high"]
    #[inline(always)]
    pub const fn set_trgselh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[doc = "External trigger shift sample"]
    #[must_use]
    #[inline(always)]
    pub const fn trgshift(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger shift sample"]
    #[inline(always)]
    pub const fn set_trgshift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "Trigger edge selection"]
    #[must_use]
    #[inline(always)]
    pub const fn trg_edge(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Trigger edge selection"]
    #[inline(always)]
    pub const fn set_trg_edge(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
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
            .field("adie", &self.adie())
            .field("adwie", &self.adwie())
            .field("trgen", &self.trgen())
            .field("dmaen", &self.dmaen())
            .field("trgsell", &self.trgsell())
            .field("adst", &self.adst())
            .field("admd", &self.admd())
            .field("align", &self.align())
            .field("cmpch", &self.cmpch())
            .field("scandir", &self.scandir())
            .field("trgselh", &self.trgselh())
            .field("trgshift", &self.trgshift())
            .field("trg_edge", &self.trg_edge())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ adie: {=bool:?}, adwie: {=bool:?}, trgen: {=bool:?}, dmaen: {=bool:?}, trgsell: {=u8:?}, adst: {=bool:?}, admd: {=u8:?}, align: {=bool:?}, cmpch: {=u8:?}, scandir: {=bool:?}, trgselh: {=u8:?}, trgshift: {=u8:?}, trg_edge: {=u8:?} }}",
            self.adie(),
            self.adwie(),
            self.trgen(),
            self.dmaen(),
            self.trgsell(),
            self.adst(),
            self.admd(),
            self.align(),
            self.cmpch(),
            self.scandir(),
            self.trgselh(),
            self.trgshift(),
            self.trg_edge()
        )
    }
}
#[doc = "Data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "ADC current channel convert data"]
    #[must_use]
    #[inline(always)]
    pub const fn ch(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "ADC current channel convert data"]
    #[inline(always)]
    pub const fn set_ch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Data {
    #[inline(always)]
    fn default() -> Data {
        Data(0)
    }
}
impl core::fmt::Debug for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data")
            .field("data", &self.data())
            .field("ch", &self.ch())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Data {{ data: {=u16:?}, ch: {=u8:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.ch(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 0 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr0(pub u32);
impl Dr0 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Dr0 {
    #[inline(always)]
    fn default() -> Dr0 {
        Dr0(0)
    }
}
impl core::fmt::Debug for Dr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr0")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr0 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 1 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr1(pub u32);
impl Dr1 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
        f.debug_struct("Dr1")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr1 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 14 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr14(pub u32);
impl Dr14 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
        f.debug_struct("Dr14")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr14 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 15 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr15(pub u32);
impl Dr15 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
        f.debug_struct("Dr15")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr15 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 2 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr2(pub u32);
impl Dr2 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
        f.debug_struct("Dr2")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr2 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 3 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr3(pub u32);
impl Dr3 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
        f.debug_struct("Dr3")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr3 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 4 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr4(pub u32);
impl Dr4 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
        f.debug_struct("Dr4")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr4 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 5 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr5(pub u32);
impl Dr5 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
        f.debug_struct("Dr5")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr5 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 6 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr6(pub u32);
impl Dr6 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
        f.debug_struct("Dr6")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr6 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 7 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr7(pub u32);
impl Dr7 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
        f.debug_struct("Dr7")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr7 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 8 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr8(pub u32);
impl Dr8 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
        f.debug_struct("Dr8")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr8 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 9 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr9(pub u32);
impl Dr9 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
        f.debug_struct("Dr9")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr9 {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "ADC interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn adif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC interrupt flag"]
    #[inline(always)]
    pub const fn set_adif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC window comparator interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn adwif(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC window comparator interrupt flag"]
    #[inline(always)]
    pub const fn set_adwif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Current conversion channel"]
    #[must_use]
    #[inline(always)]
    pub const fn ch(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Current conversion channel"]
    #[inline(always)]
    pub const fn set_ch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x03ff;
        val as u16
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 8usize)) | (((val as u32) & 0x03ff) << 8usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
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
            .field("adif", &self.adif())
            .field("adwif", &self.adwif())
            .field("busy", &self.busy())
            .field("ch", &self.ch())
            .field("valid", &self.valid())
            .field("overrun", &self.overrun())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ adif: {=bool:?}, adwif: {=bool:?}, busy: {=bool:?}, ch: {=u8:?}, valid: {=u16:?}, overrun: {=u16:?} }}",
            self.adif(),
            self.adwif(),
            self.busy(),
            self.ch(),
            self.valid(),
            self.overrun()
        )
    }
}
#[doc = "Extended status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StaExt(pub u32);
impl StaExt {
    #[doc = "Valid flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for StaExt {
    #[inline(always)]
    fn default() -> StaExt {
        StaExt(0)
    }
}
impl core::fmt::Debug for StaExt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StaExt")
            .field("valid", &self.valid())
            .field("overrun", &self.overrun())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StaExt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StaExt {{ valid: {=u8:?}, overrun: {=u8:?} }}",
            self.valid(),
            self.overrun()
        )
    }
}
