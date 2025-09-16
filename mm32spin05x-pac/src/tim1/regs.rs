#[doc = "auto-reload register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arr(pub u32);
impl Arr {
    #[doc = "Auto reload value"]
    #[must_use]
    #[inline(always)]
    pub const fn arr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Auto reload value"]
    #[inline(always)]
    pub const fn set_arr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Arr {
    #[inline(always)]
    fn default() -> Arr {
        Arr(0)
    }
}
impl core::fmt::Debug for Arr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Arr").field("arr", &self.arr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Arr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Arr {{ arr: {=u16:?} }}", self.arr())
    }
}
#[doc = "break and dead-time register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bdtr(pub u32);
impl Bdtr {
    #[doc = "Dead-time generator setup"]
    #[must_use]
    #[inline(always)]
    pub const fn dtg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Dead-time generator setup"]
    #[inline(always)]
    pub const fn set_dtg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Lock configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Lock configuration"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Off-state selection for Idle mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ossi(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Off-state selection for Idle mode"]
    #[inline(always)]
    pub const fn set_ossi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Off-state selection for Run mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ossr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Off-state selection for Run mode"]
    #[inline(always)]
    pub const fn set_ossr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Break enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bke(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Break enable"]
    #[inline(always)]
    pub const fn set_bke(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Break polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn bkp(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Break polarity"]
    #[inline(always)]
    pub const fn set_bkp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Automatic output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aoe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic output enable"]
    #[inline(always)]
    pub const fn set_aoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Main output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn moe(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Main output enable"]
    #[inline(always)]
    pub const fn set_moe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Direct output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn doe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Direct output enable"]
    #[inline(always)]
    pub const fn set_doe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Bdtr {
    #[inline(always)]
    fn default() -> Bdtr {
        Bdtr(0)
    }
}
impl core::fmt::Debug for Bdtr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bdtr")
            .field("dtg", &self.dtg())
            .field("lock", &self.lock())
            .field("ossi", &self.ossi())
            .field("ossr", &self.ossr())
            .field("bke", &self.bke())
            .field("bkp", &self.bkp())
            .field("aoe", &self.aoe())
            .field("moe", &self.moe())
            .field("doe", &self.doe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bdtr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bdtr {{ dtg: {=u8:?}, lock: {=u8:?}, ossi: {=bool:?}, ossr: {=bool:?}, bke: {=bool:?}, bkp: {=bool:?}, aoe: {=bool:?}, moe: {=bool:?}, doe: {=bool:?} }}",
            self.dtg(),
            self.lock(),
            self.ossi(),
            self.ossr(),
            self.bke(),
            self.bkp(),
            self.aoe(),
            self.moe(),
            self.doe()
        )
    }
}
#[doc = "capture/compare enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccer(pub u32);
impl Ccer {
    #[doc = "Capture/Compare 1 output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1e(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output enable"]
    #[inline(always)]
    pub const fn set_cc1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output polarity"]
    #[inline(always)]
    pub const fn set_cc1p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture/Compare 1 complementary output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1ne(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub const fn set_cc1ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture/Compare 1 complementary output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1np(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 complementary output polarity"]
    #[inline(always)]
    pub const fn set_cc1np(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture/Compare 2 output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc2e(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 output enable"]
    #[inline(always)]
    pub const fn set_cc2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Capture/Compare 2 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn cc2p(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 output polarity"]
    #[inline(always)]
    pub const fn set_cc2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Capture/Compare 2 complementary output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc2ne(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 complementary output enable"]
    #[inline(always)]
    pub const fn set_cc2ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Capture/Compare 2 complementary output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn cc2np(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 complementary output polarity"]
    #[inline(always)]
    pub const fn set_cc2np(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture/Compare 3 output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc3e(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 output enable"]
    #[inline(always)]
    pub const fn set_cc3e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture/Compare 3 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn cc3p(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 output polarity"]
    #[inline(always)]
    pub const fn set_cc3p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture/Compare 3 complementary output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc3ne(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 complementary output enable"]
    #[inline(always)]
    pub const fn set_cc3ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture/Compare 3 complementary output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn cc3np(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 complementary output polarity"]
    #[inline(always)]
    pub const fn set_cc3np(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Capture/Compare 4 output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc4e(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 output enable"]
    #[inline(always)]
    pub const fn set_cc4e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Capture/Compare 4 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn cc4p(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 output polarity"]
    #[inline(always)]
    pub const fn set_cc4p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Capture/Compare 5 output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc5e(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 5 output enable"]
    #[inline(always)]
    pub const fn set_cc5e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Capture/Compare 5 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn cc5p(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 5 output polarity"]
    #[inline(always)]
    pub const fn set_cc5p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Ccer {
    #[inline(always)]
    fn default() -> Ccer {
        Ccer(0)
    }
}
impl core::fmt::Debug for Ccer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccer")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1ne", &self.cc1ne())
            .field("cc1np", &self.cc1np())
            .field("cc2e", &self.cc2e())
            .field("cc2p", &self.cc2p())
            .field("cc2ne", &self.cc2ne())
            .field("cc2np", &self.cc2np())
            .field("cc3e", &self.cc3e())
            .field("cc3p", &self.cc3p())
            .field("cc3ne", &self.cc3ne())
            .field("cc3np", &self.cc3np())
            .field("cc4e", &self.cc4e())
            .field("cc4p", &self.cc4p())
            .field("cc5e", &self.cc5e())
            .field("cc5p", &self.cc5p())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccer {{ cc1e: {=bool:?}, cc1p: {=bool:?}, cc1ne: {=bool:?}, cc1np: {=bool:?}, cc2e: {=bool:?}, cc2p: {=bool:?}, cc2ne: {=bool:?}, cc2np: {=bool:?}, cc3e: {=bool:?}, cc3p: {=bool:?}, cc3ne: {=bool:?}, cc3np: {=bool:?}, cc4e: {=bool:?}, cc4p: {=bool:?}, cc5e: {=bool:?}, cc5p: {=bool:?} }}",
            self.cc1e(),
            self.cc1p(),
            self.cc1ne(),
            self.cc1np(),
            self.cc2e(),
            self.cc2p(),
            self.cc2ne(),
            self.cc2np(),
            self.cc3e(),
            self.cc3p(),
            self.cc3ne(),
            self.cc3np(),
            self.cc4e(),
            self.cc4p(),
            self.cc5e(),
            self.cc5p()
        )
    }
}
#[doc = "capture/compare mode register 1 (input mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr1Input(pub u32);
impl Ccmr1Input {
    #[doc = "Capture/compare 1 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/compare 1 selection"]
    #[inline(always)]
    pub const fn set_cc1s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Input capture 1 prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn ic1psc(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 1 prescaler"]
    #[inline(always)]
    pub const fn set_ic1psc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Input capture 1 filter"]
    #[must_use]
    #[inline(always)]
    pub const fn ic1f(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 1 filter"]
    #[inline(always)]
    pub const fn set_ic1f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Capture/Compare 2 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cc2s(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 2 selection"]
    #[inline(always)]
    pub const fn set_cc2s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Input capture 2 prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn ic2psc(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 2 prescaler"]
    #[inline(always)]
    pub const fn set_ic2psc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Input capture 2 filter"]
    #[must_use]
    #[inline(always)]
    pub const fn ic2f(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 2 filter"]
    #[inline(always)]
    pub const fn set_ic2f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Ccmr1Input {
    #[inline(always)]
    fn default() -> Ccmr1Input {
        Ccmr1Input(0)
    }
}
impl core::fmt::Debug for Ccmr1Input {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccmr1Input")
            .field("cc1s", &self.cc1s())
            .field("ic1psc", &self.ic1psc())
            .field("ic1f", &self.ic1f())
            .field("cc2s", &self.cc2s())
            .field("ic2psc", &self.ic2psc())
            .field("ic2f", &self.ic2f())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr1Input {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccmr1Input {{ cc1s: {=u8:?}, ic1psc: {=u8:?}, ic1f: {=u8:?}, cc2s: {=u8:?}, ic2psc: {=u8:?}, ic2f: {=u8:?} }}",
            self.cc1s(),
            self.ic1psc(),
            self.ic1f(),
            self.cc2s(),
            self.ic2psc(),
            self.ic2f()
        )
    }
}
#[doc = "capture/compare mode register 1 (output mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr1Output(pub u32);
impl Ccmr1Output {
    #[doc = "Capture/Compare 2 output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 2 output enable"]
    #[inline(always)]
    pub const fn set_cc1s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Output compare 1 fast enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc1fe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 1 fast enable"]
    #[inline(always)]
    pub const fn set_oc1fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Output compare 1 preload enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc1pe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 1 preload enable"]
    #[inline(always)]
    pub const fn set_oc1pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Output compare 1 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn oc1m(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Output compare 1 mode"]
    #[inline(always)]
    pub const fn set_oc1m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Output compare 1 clear enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc1ce(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 1 clear enable"]
    #[inline(always)]
    pub const fn set_oc1ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capure/Compare 2 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cc2s(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capure/Compare 2 selection"]
    #[inline(always)]
    pub const fn set_cc2s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Output compare 4 fast enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc2fe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 4 fast enable"]
    #[inline(always)]
    pub const fn set_oc2fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Output compare 2 preload enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc2pe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 2 preload enable"]
    #[inline(always)]
    pub const fn set_oc2pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Output compare 2 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn oc2m(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Output compare 2 mode"]
    #[inline(always)]
    pub const fn set_oc2m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Output compare 2 clear enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc2ce(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 2 clear enable"]
    #[inline(always)]
    pub const fn set_oc2ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ccmr1Output {
    #[inline(always)]
    fn default() -> Ccmr1Output {
        Ccmr1Output(0)
    }
}
impl core::fmt::Debug for Ccmr1Output {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccmr1Output")
            .field("cc1s", &self.cc1s())
            .field("oc1fe", &self.oc1fe())
            .field("oc1pe", &self.oc1pe())
            .field("oc1m", &self.oc1m())
            .field("oc1ce", &self.oc1ce())
            .field("cc2s", &self.cc2s())
            .field("oc2fe", &self.oc2fe())
            .field("oc2pe", &self.oc2pe())
            .field("oc2m", &self.oc2m())
            .field("oc2ce", &self.oc2ce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr1Output {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccmr1Output {{ cc1s: {=u8:?}, oc1fe: {=bool:?}, oc1pe: {=bool:?}, oc1m: {=u8:?}, oc1ce: {=bool:?}, cc2s: {=u8:?}, oc2fe: {=bool:?}, oc2pe: {=bool:?}, oc2m: {=u8:?}, oc2ce: {=bool:?} }}",
            self.cc1s(),
            self.oc1fe(),
            self.oc1pe(),
            self.oc1m(),
            self.oc1ce(),
            self.cc2s(),
            self.oc2fe(),
            self.oc2pe(),
            self.oc2m(),
            self.oc2ce()
        )
    }
}
#[doc = "capture/compare mode register 2 (input mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr2Input(pub u32);
impl Ccmr2Input {
    #[doc = "Capture/compare 3 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cc3s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/compare 3 selection"]
    #[inline(always)]
    pub const fn set_cc3s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Input capture 3 prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn ic3psc(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 3 prescaler"]
    #[inline(always)]
    pub const fn set_ic3psc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Input capture 3 filter"]
    #[must_use]
    #[inline(always)]
    pub const fn ic3f(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 3 filter"]
    #[inline(always)]
    pub const fn set_ic3f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Capture/Compare 4 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cc4s(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 4 selection"]
    #[inline(always)]
    pub const fn set_cc4s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Input capture 4 prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn ic4psc(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 4 prescaler"]
    #[inline(always)]
    pub const fn set_ic4psc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Input capture 4 filter"]
    #[must_use]
    #[inline(always)]
    pub const fn ic4f(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 4 filter"]
    #[inline(always)]
    pub const fn set_ic4f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Ccmr2Input {
    #[inline(always)]
    fn default() -> Ccmr2Input {
        Ccmr2Input(0)
    }
}
impl core::fmt::Debug for Ccmr2Input {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccmr2Input")
            .field("cc3s", &self.cc3s())
            .field("ic3psc", &self.ic3psc())
            .field("ic3f", &self.ic3f())
            .field("cc4s", &self.cc4s())
            .field("ic4psc", &self.ic4psc())
            .field("ic4f", &self.ic4f())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr2Input {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccmr2Input {{ cc3s: {=u8:?}, ic3psc: {=u8:?}, ic3f: {=u8:?}, cc4s: {=u8:?}, ic4psc: {=u8:?}, ic4f: {=u8:?} }}",
            self.cc3s(),
            self.ic3psc(),
            self.ic3f(),
            self.cc4s(),
            self.ic4psc(),
            self.ic4f()
        )
    }
}
#[doc = "capture/compare mode register 2(output mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr2Output(pub u32);
impl Ccmr2Output {
    #[doc = "Capture/Compare 3 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cc3s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 3 selection"]
    #[inline(always)]
    pub const fn set_cc3s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Output compare 3 fast enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc3fe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 3 fast enable"]
    #[inline(always)]
    pub const fn set_oc3fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Output compare 3 preload enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc3pe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 3 preload enable"]
    #[inline(always)]
    pub const fn set_oc3pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Output compare 3 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn oc3m(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Output compare 3 mode"]
    #[inline(always)]
    pub const fn set_oc3m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Output compare 3 clear enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc3ce(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 3 clear enable"]
    #[inline(always)]
    pub const fn set_oc3ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture/Compare 4 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cc4s(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 4 selection"]
    #[inline(always)]
    pub const fn set_cc4s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Output compare 4 fast enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc4fe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 4 fast enable"]
    #[inline(always)]
    pub const fn set_oc4fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Output compare 4 preload enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc4pe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 4 preload enable"]
    #[inline(always)]
    pub const fn set_oc4pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Output compare 4 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn oc4m(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Output compare 4 mode"]
    #[inline(always)]
    pub const fn set_oc4m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Output compare 4 clear enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc4ce(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 4 clear enable"]
    #[inline(always)]
    pub const fn set_oc4ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ccmr2Output {
    #[inline(always)]
    fn default() -> Ccmr2Output {
        Ccmr2Output(0)
    }
}
impl core::fmt::Debug for Ccmr2Output {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccmr2Output")
            .field("cc3s", &self.cc3s())
            .field("oc3fe", &self.oc3fe())
            .field("oc3pe", &self.oc3pe())
            .field("oc3m", &self.oc3m())
            .field("oc3ce", &self.oc3ce())
            .field("cc4s", &self.cc4s())
            .field("oc4fe", &self.oc4fe())
            .field("oc4pe", &self.oc4pe())
            .field("oc4m", &self.oc4m())
            .field("oc4ce", &self.oc4ce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr2Output {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccmr2Output {{ cc3s: {=u8:?}, oc3fe: {=bool:?}, oc3pe: {=bool:?}, oc3m: {=u8:?}, oc3ce: {=bool:?}, cc4s: {=u8:?}, oc4fe: {=bool:?}, oc4pe: {=bool:?}, oc4m: {=u8:?}, oc4ce: {=bool:?} }}",
            self.cc3s(),
            self.oc3fe(),
            self.oc3pe(),
            self.oc3m(),
            self.oc3ce(),
            self.cc4s(),
            self.oc4fe(),
            self.oc4pe(),
            self.oc4m(),
            self.oc4ce()
        )
    }
}
#[doc = "capture/compare mode register 3 (output mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr3Output(pub u32);
impl Ccmr3Output {
    #[doc = "Output compare 5 fast enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc5fe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 5 fast enable"]
    #[inline(always)]
    pub const fn set_oc5fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Output compare 5 preload enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc5pe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 5 preload enable"]
    #[inline(always)]
    pub const fn set_oc5pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Output compare 5 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn oc5m(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Output compare 5 mode"]
    #[inline(always)]
    pub const fn set_oc5m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Output compare 5 clear enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc5ce(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 5 clear enable"]
    #[inline(always)]
    pub const fn set_oc5ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ccmr3Output {
    #[inline(always)]
    fn default() -> Ccmr3Output {
        Ccmr3Output(0)
    }
}
impl core::fmt::Debug for Ccmr3Output {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccmr3Output")
            .field("oc5fe", &self.oc5fe())
            .field("oc5pe", &self.oc5pe())
            .field("oc5m", &self.oc5m())
            .field("oc5ce", &self.oc5ce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr3Output {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccmr3Output {{ oc5fe: {=bool:?}, oc5pe: {=bool:?}, oc5m: {=u8:?}, oc5ce: {=bool:?} }}",
            self.oc5fe(),
            self.oc5pe(),
            self.oc5m(),
            self.oc5ce()
        )
    }
}
#[doc = "capture/compare register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc = "Capture/Compare 1 value"]
    #[must_use]
    #[inline(always)]
    pub const fn ccr1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture/Compare 1 value"]
    #[inline(always)]
    pub const fn set_ccr1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ccr1 {
    #[inline(always)]
    fn default() -> Ccr1 {
        Ccr1(0)
    }
}
impl core::fmt::Debug for Ccr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr1").field("ccr1", &self.ccr1()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ccr1 {{ ccr1: {=u16:?} }}", self.ccr1())
    }
}
#[doc = "capture/compare register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2(pub u32);
impl Ccr2 {
    #[doc = "Capture/Compare 2 value"]
    #[must_use]
    #[inline(always)]
    pub const fn ccr2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture/Compare 2 value"]
    #[inline(always)]
    pub const fn set_ccr2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ccr2 {
    #[inline(always)]
    fn default() -> Ccr2 {
        Ccr2(0)
    }
}
impl core::fmt::Debug for Ccr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr2").field("ccr2", &self.ccr2()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ccr2 {{ ccr2: {=u16:?} }}", self.ccr2())
    }
}
#[doc = "capture/compare register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3(pub u32);
impl Ccr3 {
    #[doc = "Capture/Compare 3 value"]
    #[must_use]
    #[inline(always)]
    pub const fn ccr3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture/Compare 3 value"]
    #[inline(always)]
    pub const fn set_ccr3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ccr3 {
    #[inline(always)]
    fn default() -> Ccr3 {
        Ccr3(0)
    }
}
impl core::fmt::Debug for Ccr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr3").field("ccr3", &self.ccr3()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ccr3 {{ ccr3: {=u16:?} }}", self.ccr3())
    }
}
#[doc = "capture/compare register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4(pub u32);
impl Ccr4 {
    #[doc = "Capture/Compare 4 value"]
    #[must_use]
    #[inline(always)]
    pub const fn ccr4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture/Compare 4 value"]
    #[inline(always)]
    pub const fn set_ccr4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ccr4 {
    #[inline(always)]
    fn default() -> Ccr4 {
        Ccr4(0)
    }
}
impl core::fmt::Debug for Ccr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr4").field("ccr4", &self.ccr4()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ccr4 {{ ccr4: {=u16:?} }}", self.ccr4())
    }
}
#[doc = "capture/compare register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr5(pub u32);
impl Ccr5 {
    #[doc = "Capture/Compare 5 value"]
    #[must_use]
    #[inline(always)]
    pub const fn ccr5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture/Compare 5 value"]
    #[inline(always)]
    pub const fn set_ccr5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ccr5 {
    #[inline(always)]
    fn default() -> Ccr5 {
        Ccr5(0)
    }
}
impl core::fmt::Debug for Ccr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr5").field("ccr5", &self.ccr5()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ccr5 {{ ccr5: {=u16:?} }}", self.ccr5())
    }
}
#[doc = "counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "Counter value"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter value"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0)
    }
}
impl core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "Counter enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter enable"]
    #[inline(always)]
    pub const fn set_cen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Update disable"]
    #[must_use]
    #[inline(always)]
    pub const fn udis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Update disable"]
    #[inline(always)]
    pub const fn set_udis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Update request source"]
    #[must_use]
    #[inline(always)]
    pub const fn urs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Update request source"]
    #[inline(always)]
    pub const fn set_urs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "One pulse mode"]
    #[must_use]
    #[inline(always)]
    pub const fn opm(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "One pulse mode"]
    #[inline(always)]
    pub const fn set_opm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Center-aligned mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cms(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Center-aligned mode selection"]
    #[inline(always)]
    pub const fn set_cms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Auto-reload preload enable"]
    #[must_use]
    #[inline(always)]
    pub const fn arpe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-reload preload enable"]
    #[inline(always)]
    pub const fn set_arpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Clock division"]
    #[must_use]
    #[inline(always)]
    pub const fn ckd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Clock division"]
    #[inline(always)]
    pub const fn set_ckd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
impl core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr1")
            .field("cen", &self.cen())
            .field("udis", &self.udis())
            .field("urs", &self.urs())
            .field("opm", &self.opm())
            .field("dir", &self.dir())
            .field("cms", &self.cms())
            .field("arpe", &self.arpe())
            .field("ckd", &self.ckd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr1 {{ cen: {=bool:?}, udis: {=bool:?}, urs: {=bool:?}, opm: {=bool:?}, dir: {=bool:?}, cms: {=u8:?}, arpe: {=bool:?}, ckd: {=u8:?} }}",
            self.cen(),
            self.udis(),
            self.urs(),
            self.opm(),
            self.dir(),
            self.cms(),
            self.arpe(),
            self.ckd()
        )
    }
}
#[doc = "control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc = "Capture/compare preloaded control"]
    #[must_use]
    #[inline(always)]
    pub const fn ccpc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare preloaded control"]
    #[inline(always)]
    pub const fn set_ccpc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/compare control update selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ccus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare control update selection"]
    #[inline(always)]
    pub const fn set_ccus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture/compare DMA selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ccds(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare DMA selection"]
    #[inline(always)]
    pub const fn set_ccds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Master mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mms(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Master mode selection"]
    #[inline(always)]
    pub const fn set_mms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "TI1 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ti1s(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TI1 selection"]
    #[inline(always)]
    pub const fn set_ti1s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Output Idle state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ois1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 1"]
    #[inline(always)]
    pub const fn set_ois1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Output Idle state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ois1n(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 1"]
    #[inline(always)]
    pub const fn set_ois1n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OIS2"]
    #[must_use]
    #[inline(always)]
    pub const fn ois2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OIS2"]
    #[inline(always)]
    pub const fn set_ois2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OIS2N"]
    #[must_use]
    #[inline(always)]
    pub const fn ois2n(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OIS2N"]
    #[inline(always)]
    pub const fn set_ois2n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "OIS3"]
    #[must_use]
    #[inline(always)]
    pub const fn ois3(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "OIS3"]
    #[inline(always)]
    pub const fn set_ois3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "OIS3N"]
    #[must_use]
    #[inline(always)]
    pub const fn ois3n(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "OIS3N"]
    #[inline(always)]
    pub const fn set_ois3n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "OIS4"]
    #[must_use]
    #[inline(always)]
    pub const fn ois4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "OIS4"]
    #[inline(always)]
    pub const fn set_ois4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        Cr2(0)
    }
}
impl core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr2")
            .field("ccpc", &self.ccpc())
            .field("ccus", &self.ccus())
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .field("ois1", &self.ois1())
            .field("ois1n", &self.ois1n())
            .field("ois2", &self.ois2())
            .field("ois2n", &self.ois2n())
            .field("ois3", &self.ois3())
            .field("ois3n", &self.ois3n())
            .field("ois4", &self.ois4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr2 {{ ccpc: {=bool:?}, ccus: {=bool:?}, ccds: {=bool:?}, mms: {=u8:?}, ti1s: {=bool:?}, ois1: {=bool:?}, ois1n: {=bool:?}, ois2: {=bool:?}, ois2n: {=bool:?}, ois3: {=bool:?}, ois3n: {=bool:?}, ois4: {=bool:?} }}",
            self.ccpc(),
            self.ccus(),
            self.ccds(),
            self.mms(),
            self.ti1s(),
            self.ois1(),
            self.ois1n(),
            self.ois2(),
            self.ois2n(),
            self.ois3(),
            self.ois3n(),
            self.ois4()
        )
    }
}
#[doc = "DMA control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcr(pub u32);
impl Dcr {
    #[doc = "DMA base address"]
    #[must_use]
    #[inline(always)]
    pub const fn dba(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DMA base address"]
    #[inline(always)]
    pub const fn set_dba(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "DMA burst length"]
    #[must_use]
    #[inline(always)]
    pub const fn dbl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "DMA burst length"]
    #[inline(always)]
    pub const fn set_dbl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
}
impl Default for Dcr {
    #[inline(always)]
    fn default() -> Dcr {
        Dcr(0)
    }
}
impl core::fmt::Debug for Dcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcr")
            .field("dba", &self.dba())
            .field("dbl", &self.dbl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcr {{ dba: {=u8:?}, dbl: {=u8:?} }}",
            self.dba(),
            self.dbl()
        )
    }
}
#[doc = "DMA/Interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dier(pub u32);
impl Dier {
    #[doc = "Update interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update interrupt enable"]
    #[inline(always)]
    pub const fn set_uie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub const fn set_cc1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture/Compare 2 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc2ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub const fn set_cc2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture/Compare 3 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc3ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub const fn set_cc3ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture/Compare 4 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc4ie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    pub const fn set_cc4ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "COM interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn comie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "COM interrupt enable"]
    #[inline(always)]
    pub const fn set_comie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger interrupt enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Break interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Break interrupt enable"]
    #[inline(always)]
    pub const fn set_bie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Update DMA request enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ude(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Update DMA request enable"]
    #[inline(always)]
    pub const fn set_ude(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture/Compare 1 DMA request enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1de(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub const fn set_cc1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture/Compare 2 DMA request enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc2de(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub const fn set_cc2de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture/Compare 3 DMA request enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc3de(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub const fn set_cc3de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Capture/Compare 4 DMA request enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc4de(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    pub const fn set_cc4de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "COM DMA request enable"]
    #[must_use]
    #[inline(always)]
    pub const fn comde(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "COM DMA request enable"]
    #[inline(always)]
    pub const fn set_comde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Trigger DMA request enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tde(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA request enable"]
    #[inline(always)]
    pub const fn set_tde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Capture/Compare 5 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc5ie(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 5 interrupt enable"]
    #[inline(always)]
    pub const fn set_cc5ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Capture/Compare 5 DMA request enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc5de(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 5 DMA request enable"]
    #[inline(always)]
    pub const fn set_cc5de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Dier {
    #[inline(always)]
    fn default() -> Dier {
        Dier(0)
    }
}
impl core::fmt::Debug for Dier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dier")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .field("cc2ie", &self.cc2ie())
            .field("cc3ie", &self.cc3ie())
            .field("cc4ie", &self.cc4ie())
            .field("comie", &self.comie())
            .field("tie", &self.tie())
            .field("bie", &self.bie())
            .field("ude", &self.ude())
            .field("cc1de", &self.cc1de())
            .field("cc2de", &self.cc2de())
            .field("cc3de", &self.cc3de())
            .field("cc4de", &self.cc4de())
            .field("comde", &self.comde())
            .field("tde", &self.tde())
            .field("cc5ie", &self.cc5ie())
            .field("cc5de", &self.cc5de())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dier {{ uie: {=bool:?}, cc1ie: {=bool:?}, cc2ie: {=bool:?}, cc3ie: {=bool:?}, cc4ie: {=bool:?}, comie: {=bool:?}, tie: {=bool:?}, bie: {=bool:?}, ude: {=bool:?}, cc1de: {=bool:?}, cc2de: {=bool:?}, cc3de: {=bool:?}, cc4de: {=bool:?}, comde: {=bool:?}, tde: {=bool:?}, cc5ie: {=bool:?}, cc5de: {=bool:?} }}",
            self.uie(),
            self.cc1ie(),
            self.cc2ie(),
            self.cc3ie(),
            self.cc4ie(),
            self.comie(),
            self.tie(),
            self.bie(),
            self.ude(),
            self.cc1de(),
            self.cc2de(),
            self.cc3de(),
            self.cc4de(),
            self.comde(),
            self.tde(),
            self.cc5ie(),
            self.cc5de()
        )
    }
}
#[doc = "DMA address for full transfer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmar(pub u32);
impl Dmar {
    #[doc = "DMA register for burst accesses"]
    #[must_use]
    #[inline(always)]
    pub const fn dmab(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "DMA register for burst accesses"]
    #[inline(always)]
    pub const fn set_dmab(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dmar {
    #[inline(always)]
    fn default() -> Dmar {
        Dmar(0)
    }
}
impl core::fmt::Debug for Dmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmar").field("dmab", &self.dmab()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmar {{ dmab: {=u16:?} }}", self.dmab())
    }
}
#[doc = "event generation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Egr(pub u32);
impl Egr {
    #[doc = "Update generation"]
    #[must_use]
    #[inline(always)]
    pub const fn ug(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update generation"]
    #[inline(always)]
    pub const fn set_ug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 generation"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1g(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 generation"]
    #[inline(always)]
    pub const fn set_cc1g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture/Compare 2 generation"]
    #[must_use]
    #[inline(always)]
    pub const fn cc2g(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 generation"]
    #[inline(always)]
    pub const fn set_cc2g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture/Compare 3 generation"]
    #[must_use]
    #[inline(always)]
    pub const fn cc3g(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 generation"]
    #[inline(always)]
    pub const fn set_cc3g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture/Compare 4 generation"]
    #[must_use]
    #[inline(always)]
    pub const fn cc4g(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 generation"]
    #[inline(always)]
    pub const fn set_cc4g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Capture/Compare control update generation"]
    #[must_use]
    #[inline(always)]
    pub const fn comg(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare control update generation"]
    #[inline(always)]
    pub const fn set_comg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger generation"]
    #[must_use]
    #[inline(always)]
    pub const fn tg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger generation"]
    #[inline(always)]
    pub const fn set_tg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Break generation"]
    #[must_use]
    #[inline(always)]
    pub const fn bg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Break generation"]
    #[inline(always)]
    pub const fn set_bg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture/Compare 5 generation"]
    #[must_use]
    #[inline(always)]
    pub const fn cc5g(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 5 generation"]
    #[inline(always)]
    pub const fn set_cc5g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Egr {
    #[inline(always)]
    fn default() -> Egr {
        Egr(0)
    }
}
impl core::fmt::Debug for Egr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Egr")
            .field("ug", &self.ug())
            .field("cc1g", &self.cc1g())
            .field("cc2g", &self.cc2g())
            .field("cc3g", &self.cc3g())
            .field("cc4g", &self.cc4g())
            .field("comg", &self.comg())
            .field("tg", &self.tg())
            .field("bg", &self.bg())
            .field("cc5g", &self.cc5g())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Egr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Egr {{ ug: {=bool:?}, cc1g: {=bool:?}, cc2g: {=bool:?}, cc3g: {=bool:?}, cc4g: {=bool:?}, comg: {=bool:?}, tg: {=bool:?}, bg: {=bool:?}, cc5g: {=bool:?} }}",
            self.ug(),
            self.cc1g(),
            self.cc2g(),
            self.cc3g(),
            self.cc4g(),
            self.comg(),
            self.tg(),
            self.bg(),
            self.cc5g()
        )
    }
}
#[doc = "prescaler"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psc(pub u32);
impl Psc {
    #[doc = "Prescaler value"]
    #[must_use]
    #[inline(always)]
    pub const fn psc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub const fn set_psc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Psc {
    #[inline(always)]
    fn default() -> Psc {
        Psc(0)
    }
}
impl core::fmt::Debug for Psc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psc").field("psc", &self.psc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Psc {{ psc: {=u16:?} }}", self.psc())
    }
}
#[doc = "repetition counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc = "Repetition counter value"]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Repetition counter value"]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rcr {
    #[inline(always)]
    fn default() -> Rcr {
        Rcr(0)
    }
}
impl core::fmt::Debug for Rcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr").field("rep", &self.rep()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rcr {{ rep: {=u8:?} }}", self.rep())
    }
}
#[doc = "slave mode control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcr(pub u32);
impl Smcr {
    #[doc = "Slave mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sms(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Slave mode selection"]
    #[inline(always)]
    pub const fn set_sms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Output compare clear selection"]
    #[must_use]
    #[inline(always)]
    pub const fn occs(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare clear selection"]
    #[inline(always)]
    pub const fn set_occs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ts(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Trigger selection"]
    #[inline(always)]
    pub const fn set_ts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Master/slave mode"]
    #[must_use]
    #[inline(always)]
    pub const fn msm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Master/slave mode"]
    #[inline(always)]
    pub const fn set_msm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "External trigger filter"]
    #[must_use]
    #[inline(always)]
    pub const fn etf(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "External trigger filter"]
    #[inline(always)]
    pub const fn set_etf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "External trigger prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn etps(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "External trigger prescaler"]
    #[inline(always)]
    pub const fn set_etps(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "External clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ece(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External clock enable"]
    #[inline(always)]
    pub const fn set_ece(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "External trigger polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn etp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger polarity"]
    #[inline(always)]
    pub const fn set_etp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Smcr {
    #[inline(always)]
    fn default() -> Smcr {
        Smcr(0)
    }
}
impl core::fmt::Debug for Smcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcr")
            .field("sms", &self.sms())
            .field("occs", &self.occs())
            .field("ts", &self.ts())
            .field("msm", &self.msm())
            .field("etf", &self.etf())
            .field("etps", &self.etps())
            .field("ece", &self.ece())
            .field("etp", &self.etp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smcr {{ sms: {=u8:?}, occs: {=bool:?}, ts: {=u8:?}, msm: {=bool:?}, etf: {=u8:?}, etps: {=u8:?}, ece: {=bool:?}, etp: {=bool:?} }}",
            self.sms(),
            self.occs(),
            self.ts(),
            self.msm(),
            self.etf(),
            self.etps(),
            self.ece(),
            self.etp()
        )
    }
}
#[doc = "status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Update interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn uif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update interrupt flag"]
    #[inline(always)]
    pub const fn set_uif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1if(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 interrupt flag"]
    #[inline(always)]
    pub const fn set_cc1if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture/Compare 2 interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cc2if(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub const fn set_cc2if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture/Compare 3 interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cc3if(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub const fn set_cc3if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture/Compare 4 interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cc4if(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub const fn set_cc4if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "COM interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn comif(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "COM interrupt flag"]
    #[inline(always)]
    pub const fn set_comif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger interrupt flag"]
    #[inline(always)]
    pub const fn set_tif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Break interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bif(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Break interrupt flag"]
    #[inline(always)]
    pub const fn set_bif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture/Compare 1 overcapture flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1of(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub const fn set_cc1of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture/Compare 2 overcapture flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cc2of(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 overcapture flag"]
    #[inline(always)]
    pub const fn set_cc2of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture/Compare 3 overcapture flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cc3of(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub const fn set_cc3of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Capture/Compare 4 overcapture flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cc4of(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub const fn set_cc4of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Capture/Compare 5 interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cc5if(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 5 interrupt flag"]
    #[inline(always)]
    pub const fn set_cc5if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("cc2if", &self.cc2if())
            .field("cc3if", &self.cc3if())
            .field("cc4if", &self.cc4if())
            .field("comif", &self.comif())
            .field("tif", &self.tif())
            .field("bif", &self.bif())
            .field("cc1of", &self.cc1of())
            .field("cc2of", &self.cc2of())
            .field("cc3of", &self.cc3of())
            .field("cc4of", &self.cc4of())
            .field("cc5if", &self.cc5if())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ uif: {=bool:?}, cc1if: {=bool:?}, cc2if: {=bool:?}, cc3if: {=bool:?}, cc4if: {=bool:?}, comif: {=bool:?}, tif: {=bool:?}, bif: {=bool:?}, cc1of: {=bool:?}, cc2of: {=bool:?}, cc3of: {=bool:?}, cc4of: {=bool:?}, cc5if: {=bool:?} }}",
            self.uif(),
            self.cc1if(),
            self.cc2if(),
            self.cc3if(),
            self.cc4if(),
            self.comif(),
            self.tif(),
            self.bif(),
            self.cc1of(),
            self.cc2of(),
            self.cc3of(),
            self.cc4of(),
            self.cc5if()
        )
    }
}
