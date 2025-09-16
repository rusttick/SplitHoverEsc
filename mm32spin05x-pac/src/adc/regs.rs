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
    pub const fn preh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "ADC high prescaler coefficient"]
    #[inline(always)]
    pub const fn set_preh(&mut self, val: u8) {
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
    pub const fn prel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ADC low prescaler coefficient"]
    #[inline(always)]
    pub const fn set_prel(&mut self, val: bool) {
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
            .field("preh", &self.preh())
            .field("rsltctl", &self.rsltctl())
            .field("samctl", &self.samctl())
            .field("prel", &self.prel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfgr {{ aden: {=bool:?}, adwen: {=bool:?}, tsen: {=bool:?}, vsen: {=bool:?}, preh: {=u8:?}, rsltctl: {=u8:?}, samctl: {=u8:?}, prel: {=bool:?} }}",
            self.aden(),
            self.adwen(),
            self.tsen(),
            self.vsen(),
            self.preh(),
            self.rsltctl(),
            self.samctl(),
            self.prel()
        )
    }
}
#[doc = "Channel 0 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0dr(pub u32);
impl Ch0dr {
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
impl Default for Ch0dr {
    #[inline(always)]
    fn default() -> Ch0dr {
        Ch0dr(0)
    }
}
impl core::fmt::Debug for Ch0dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch0dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 10 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch10dr(pub u32);
impl Ch10dr {
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
impl Default for Ch10dr {
    #[inline(always)]
    fn default() -> Ch10dr {
        Ch10dr(0)
    }
}
impl core::fmt::Debug for Ch10dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch10dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch10dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch10dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 11 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch11dr(pub u32);
impl Ch11dr {
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
impl Default for Ch11dr {
    #[inline(always)]
    fn default() -> Ch11dr {
        Ch11dr(0)
    }
}
impl core::fmt::Debug for Ch11dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch11dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch11dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch11dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 12 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch12dr(pub u32);
impl Ch12dr {
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
impl Default for Ch12dr {
    #[inline(always)]
    fn default() -> Ch12dr {
        Ch12dr(0)
    }
}
impl core::fmt::Debug for Ch12dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch12dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch12dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch12dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 14 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch14dr(pub u32);
impl Ch14dr {
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
impl Default for Ch14dr {
    #[inline(always)]
    fn default() -> Ch14dr {
        Ch14dr(0)
    }
}
impl core::fmt::Debug for Ch14dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch14dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch14dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch14dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 15 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch15dr(pub u32);
impl Ch15dr {
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
impl Default for Ch15dr {
    #[inline(always)]
    fn default() -> Ch15dr {
        Ch15dr(0)
    }
}
impl core::fmt::Debug for Ch15dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch15dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch15dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch15dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 1 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1dr(pub u32);
impl Ch1dr {
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
impl Default for Ch1dr {
    #[inline(always)]
    fn default() -> Ch1dr {
        Ch1dr(0)
    }
}
impl core::fmt::Debug for Ch1dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch1dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch1dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 2 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2dr(pub u32);
impl Ch2dr {
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
impl Default for Ch2dr {
    #[inline(always)]
    fn default() -> Ch2dr {
        Ch2dr(0)
    }
}
impl core::fmt::Debug for Ch2dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch2dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch2dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch2dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 3 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3dr(pub u32);
impl Ch3dr {
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
impl Default for Ch3dr {
    #[inline(always)]
    fn default() -> Ch3dr {
        Ch3dr(0)
    }
}
impl core::fmt::Debug for Ch3dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch3dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch3dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch3dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 4 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4dr(pub u32);
impl Ch4dr {
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
impl Default for Ch4dr {
    #[inline(always)]
    fn default() -> Ch4dr {
        Ch4dr(0)
    }
}
impl core::fmt::Debug for Ch4dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch4dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch4dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch4dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 5 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5dr(pub u32);
impl Ch5dr {
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
impl Default for Ch5dr {
    #[inline(always)]
    fn default() -> Ch5dr {
        Ch5dr(0)
    }
}
impl core::fmt::Debug for Ch5dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch5dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch5dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch5dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 6 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6dr(pub u32);
impl Ch6dr {
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
impl Default for Ch6dr {
    #[inline(always)]
    fn default() -> Ch6dr {
        Ch6dr(0)
    }
}
impl core::fmt::Debug for Ch6dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch6dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch6dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch6dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 7 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7dr(pub u32);
impl Ch7dr {
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
impl Default for Ch7dr {
    #[inline(always)]
    fn default() -> Ch7dr {
        Ch7dr(0)
    }
}
impl core::fmt::Debug for Ch7dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch7dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch7dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch7dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 8 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch8dr(pub u32);
impl Ch8dr {
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
impl Default for Ch8dr {
    #[inline(always)]
    fn default() -> Ch8dr {
        Ch8dr(0)
    }
}
impl core::fmt::Debug for Ch8dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch8dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch8dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch8dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
        )
    }
}
#[doc = "Channel 9 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch9dr(pub u32);
impl Ch9dr {
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
impl Default for Ch9dr {
    #[inline(always)]
    fn default() -> Ch9dr {
        Ch9dr(0)
    }
}
impl core::fmt::Debug for Ch9dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch9dr")
            .field("data", &self.data())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch9dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch9dr {{ data: {=u16:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.overrun(),
            self.valid()
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
    pub const fn ch0en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 0 enable"]
    #[inline(always)]
    pub const fn set_ch0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Analog input channel 1 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch1en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 1 enable"]
    #[inline(always)]
    pub const fn set_ch1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Analog input channel 2 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch2en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 2 enable"]
    #[inline(always)]
    pub const fn set_ch2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Analog input channel 3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch3en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 3 enable"]
    #[inline(always)]
    pub const fn set_ch3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog input channel 4 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch4en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 4 enable"]
    #[inline(always)]
    pub const fn set_ch4en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Analog input channel 5 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch5en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 5 enable"]
    #[inline(always)]
    pub const fn set_ch5en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Analog input channel 6 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch6en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 6 enable"]
    #[inline(always)]
    pub const fn set_ch6en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Analog input channel 7 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch7en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 7 enable"]
    #[inline(always)]
    pub const fn set_ch7en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Analog input channel 8 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch8en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 8 enable"]
    #[inline(always)]
    pub const fn set_ch8en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Analog input channel 9 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch9en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 9 enable"]
    #[inline(always)]
    pub const fn set_ch9en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog input channel 10 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch10en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 10 enable"]
    #[inline(always)]
    pub const fn set_ch10en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Analog input channel 11 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch11en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Analog input channel 11 enable"]
    #[inline(always)]
    pub const fn set_ch11en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable ADC internal self-calibration channel"]
    #[must_use]
    #[inline(always)]
    pub const fn chcalib(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ADC internal self-calibration channel"]
    #[inline(always)]
    pub const fn set_chcalib(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Temperature Sensor channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chtsen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Sensor channel enable"]
    #[inline(always)]
    pub const fn set_chtsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Internal reference voltage channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chvsen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Internal reference voltage channel enable"]
    #[inline(always)]
    pub const fn set_chvsen(&mut self, val: bool) {
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
            .field("ch0en", &self.ch0en())
            .field("ch1en", &self.ch1en())
            .field("ch2en", &self.ch2en())
            .field("ch3en", &self.ch3en())
            .field("ch4en", &self.ch4en())
            .field("ch5en", &self.ch5en())
            .field("ch6en", &self.ch6en())
            .field("ch7en", &self.ch7en())
            .field("ch8en", &self.ch8en())
            .field("ch9en", &self.ch9en())
            .field("ch10en", &self.ch10en())
            .field("ch11en", &self.ch11en())
            .field("chcalib", &self.chcalib())
            .field("chtsen", &self.chtsen())
            .field("chvsen", &self.chvsen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Chsr {{ ch0en: {=bool:?}, ch1en: {=bool:?}, ch2en: {=bool:?}, ch3en: {=bool:?}, ch4en: {=bool:?}, ch5en: {=bool:?}, ch6en: {=bool:?}, ch7en: {=bool:?}, ch8en: {=bool:?}, ch9en: {=bool:?}, ch10en: {=bool:?}, ch11en: {=bool:?}, chcalib: {=bool:?}, chtsen: {=bool:?}, chvsen: {=bool:?} }}",
            self.ch0en(),
            self.ch1en(),
            self.ch2en(),
            self.ch3en(),
            self.ch4en(),
            self.ch5en(),
            self.ch6en(),
            self.ch7en(),
            self.ch8en(),
            self.ch9en(),
            self.ch10en(),
            self.ch11en(),
            self.chcalib(),
            self.chtsen(),
            self.chvsen()
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
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "ADC mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
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
    #[doc = "Self-calibration enable"]
    #[must_use]
    #[inline(always)]
    pub const fn caliben(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Self-calibration enable"]
    #[inline(always)]
    pub const fn set_caliben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Self-calibration voltage selection"]
    #[must_use]
    #[inline(always)]
    pub const fn calibsel(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Self-calibration voltage selection"]
    #[inline(always)]
    pub const fn set_calibsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
            .field("mode", &self.mode())
            .field("align", &self.align())
            .field("cmpch", &self.cmpch())
            .field("scandir", &self.scandir())
            .field("trgselh", &self.trgselh())
            .field("trgshift", &self.trgshift())
            .field("caliben", &self.caliben())
            .field("calibsel", &self.calibsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ adie: {=bool:?}, adwie: {=bool:?}, trgen: {=bool:?}, dmaen: {=bool:?}, trgsell: {=u8:?}, adst: {=bool:?}, mode: {=u8:?}, align: {=bool:?}, cmpch: {=u8:?}, scandir: {=bool:?}, trgselh: {=u8:?}, trgshift: {=u8:?}, caliben: {=bool:?}, calibsel: {=bool:?} }}",
            self.adie(),
            self.adwie(),
            self.trgen(),
            self.dmaen(),
            self.trgsell(),
            self.adst(),
            self.mode(),
            self.align(),
            self.cmpch(),
            self.scandir(),
            self.trgselh(),
            self.trgshift(),
            self.caliben(),
            self.calibsel()
        )
    }
}
#[doc = "Data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
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
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
impl core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr")
            .field("data", &self.data())
            .field("ch", &self.ch())
            .field("overrun", &self.overrun())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr {{ data: {=u16:?}, ch: {=u8:?}, overrun: {=bool:?}, valid: {=bool:?} }}",
            self.data(),
            self.ch(),
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
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
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
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Valid flag"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrun(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn set_overrun(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
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
