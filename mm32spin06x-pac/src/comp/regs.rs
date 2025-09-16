#[doc = "COMP1 Control State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp1Csr(pub u32);
impl Comp1Csr {
    #[doc = "Comparator 1 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 1 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 1 mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Comparator 1 inverting input selection"]
    #[must_use]
    #[inline(always)]
    pub const fn inm_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 1 inverting input selection"]
    #[inline(always)]
    pub const fn set_inm_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Comparator 1 normal phase input selection"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_sel(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 1 normal phase input selection"]
    #[inline(always)]
    pub const fn set_inp_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "Comparator 1 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn out_sel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "Comparator 1 output selection"]
    #[inline(always)]
    pub const fn set_out_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "Comparator 1 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 output polarity"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Comparator 1 hysteresis"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 1 hysteresis"]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Comparator output filter"]
    #[must_use]
    #[inline(always)]
    pub const fn oflt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator output filter"]
    #[inline(always)]
    pub const fn set_oflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Comparator 1 lock"]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 lock"]
    #[inline(always)]
    pub const fn set_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Comparator lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Comp1Csr {
    #[inline(always)]
    fn default() -> Comp1Csr {
        Comp1Csr(0)
    }
}
impl core::fmt::Debug for Comp1Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp1Csr")
            .field("en", &self.en())
            .field("mode", &self.mode())
            .field("inm_sel", &self.inm_sel())
            .field("inp_sel", &self.inp_sel())
            .field("out_sel", &self.out_sel())
            .field("pol", &self.pol())
            .field("hyst", &self.hyst())
            .field("oflt", &self.oflt())
            .field("out", &self.out())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp1Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp1Csr {{ en: {=bool:?}, mode: {=u8:?}, inm_sel: {=u8:?}, inp_sel: {=u8:?}, out_sel: {=u8:?}, pol: {=bool:?}, hyst: {=u8:?}, oflt: {=u8:?}, out: {=bool:?}, lock: {=bool:?} }}",
            self.en(),
            self.mode(),
            self.inm_sel(),
            self.inp_sel(),
            self.out_sel(),
            self.pol(),
            self.hyst(),
            self.oflt(),
            self.out(),
            self.lock()
        )
    }
}
#[doc = "COMP1 Polling Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp1Poll(pub u32);
impl Comp1Poll {
    #[doc = "Comparator 1 polling enable"]
    #[must_use]
    #[inline(always)]
    pub const fn poll_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 polling enable"]
    #[inline(always)]
    pub const fn set_poll_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 1 polling pollingchannel"]
    #[must_use]
    #[inline(always)]
    pub const fn poll_ch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 polling pollingchannel"]
    #[inline(always)]
    pub const fn set_poll_ch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator 1 Polling inverting input fix"]
    #[must_use]
    #[inline(always)]
    pub const fn fixn(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 Polling inverting input fix"]
    #[inline(always)]
    pub const fn set_fixn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Comparator 1 polling wait cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn period(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 1 polling wait cycle"]
    #[inline(always)]
    pub const fn set_period(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Comparator 1 Polling output"]
    #[must_use]
    #[inline(always)]
    pub const fn pout(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 1 Polling output"]
    #[inline(always)]
    pub const fn set_pout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Comp1Poll {
    #[inline(always)]
    fn default() -> Comp1Poll {
        Comp1Poll(0)
    }
}
impl core::fmt::Debug for Comp1Poll {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp1Poll")
            .field("poll_en", &self.poll_en())
            .field("poll_ch", &self.poll_ch())
            .field("fixn", &self.fixn())
            .field("period", &self.period())
            .field("pout", &self.pout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp1Poll {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp1Poll {{ poll_en: {=bool:?}, poll_ch: {=bool:?}, fixn: {=bool:?}, period: {=u8:?}, pout: {=u8:?} }}",
            self.poll_en(),
            self.poll_ch(),
            self.fixn(),
            self.period(),
            self.pout()
        )
    }
}
#[doc = "COMP2 Control State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp2Csr(pub u32);
impl Comp2Csr {
    #[doc = "Comparator 2 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 2 enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 2 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 2 mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Comparator 2 inverting input selection"]
    #[must_use]
    #[inline(always)]
    pub const fn inm_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 2 inverting input selection"]
    #[inline(always)]
    pub const fn set_inm_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Comparator 2 normal phase input selection"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_sel(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 2 normal phase input selection"]
    #[inline(always)]
    pub const fn set_inp_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "Comparator 2 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn out_sel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "Comparator 2 output selection"]
    #[inline(always)]
    pub const fn set_out_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "Comparator 2 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 2 output polarity"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Comparator 2 hysteresis"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 2 hysteresis"]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Comparator output filter"]
    #[must_use]
    #[inline(always)]
    pub const fn oflt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator output filter"]
    #[inline(always)]
    pub const fn set_oflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Comparator 2 lock"]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 2 lock"]
    #[inline(always)]
    pub const fn set_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Comparator lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Comp2Csr {
    #[inline(always)]
    fn default() -> Comp2Csr {
        Comp2Csr(0)
    }
}
impl core::fmt::Debug for Comp2Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp2Csr")
            .field("en", &self.en())
            .field("mode", &self.mode())
            .field("inm_sel", &self.inm_sel())
            .field("inp_sel", &self.inp_sel())
            .field("out_sel", &self.out_sel())
            .field("pol", &self.pol())
            .field("hyst", &self.hyst())
            .field("oflt", &self.oflt())
            .field("out", &self.out())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp2Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp2Csr {{ en: {=bool:?}, mode: {=u8:?}, inm_sel: {=u8:?}, inp_sel: {=u8:?}, out_sel: {=u8:?}, pol: {=bool:?}, hyst: {=u8:?}, oflt: {=u8:?}, out: {=bool:?}, lock: {=bool:?} }}",
            self.en(),
            self.mode(),
            self.inm_sel(),
            self.inp_sel(),
            self.out_sel(),
            self.pol(),
            self.hyst(),
            self.oflt(),
            self.out(),
            self.lock()
        )
    }
}
#[doc = "COMP2 Polling Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp2Poll(pub u32);
impl Comp2Poll {
    #[doc = "Comparator 2 polling enable"]
    #[must_use]
    #[inline(always)]
    pub const fn poll_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 2 polling enable"]
    #[inline(always)]
    pub const fn set_poll_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 2 polling channel"]
    #[must_use]
    #[inline(always)]
    pub const fn poll_ch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 2 polling channel"]
    #[inline(always)]
    pub const fn set_poll_ch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator 2 Polling inverting input fix"]
    #[must_use]
    #[inline(always)]
    pub const fn fixn(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 2 Polling inverting input fix"]
    #[inline(always)]
    pub const fn set_fixn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Comparator 2 polling wait cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn period(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 2 polling wait cycle"]
    #[inline(always)]
    pub const fn set_period(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Comparator 2 Polling output"]
    #[must_use]
    #[inline(always)]
    pub const fn pout(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 2 Polling output"]
    #[inline(always)]
    pub const fn set_pout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Comp2Poll {
    #[inline(always)]
    fn default() -> Comp2Poll {
        Comp2Poll(0)
    }
}
impl core::fmt::Debug for Comp2Poll {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp2Poll")
            .field("poll_en", &self.poll_en())
            .field("poll_ch", &self.poll_ch())
            .field("fixn", &self.fixn())
            .field("period", &self.period())
            .field("pout", &self.pout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp2Poll {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp2Poll {{ poll_en: {=bool:?}, poll_ch: {=bool:?}, fixn: {=bool:?}, period: {=u8:?}, pout: {=u8:?} }}",
            self.poll_en(),
            self.poll_ch(),
            self.fixn(),
            self.period(),
            self.pout()
        )
    }
}
#[doc = "COMP Extern Reference Voltage"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompCrv(pub u32);
impl CompCrv {
    #[doc = "Comparator external reference voltage select"]
    #[must_use]
    #[inline(always)]
    pub const fn crv_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Comparator external reference voltage select"]
    #[inline(always)]
    pub const fn set_crv_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Comparator external reference voltage enable"]
    #[must_use]
    #[inline(always)]
    pub const fn crv_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator external reference voltage enable"]
    #[inline(always)]
    pub const fn set_crv_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Comparator external reference voltage source select"]
    #[must_use]
    #[inline(always)]
    pub const fn crv_src(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator external reference voltage source select"]
    #[inline(always)]
    pub const fn set_crv_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for CompCrv {
    #[inline(always)]
    fn default() -> CompCrv {
        CompCrv(0)
    }
}
impl core::fmt::Debug for CompCrv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CompCrv")
            .field("crv_sel", &self.crv_sel())
            .field("crv_en", &self.crv_en())
            .field("crv_src", &self.crv_src())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CompCrv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CompCrv {{ crv_sel: {=u8:?}, crv_en: {=bool:?}, crv_src: {=bool:?} }}",
            self.crv_sel(),
            self.crv_en(),
            self.crv_src()
        )
    }
}
