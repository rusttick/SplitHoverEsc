#[doc = "Port Multiplexing Function High Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afrh(pub u32);
impl Afrh {
    #[doc = "Multiplexing function selection for bit 8 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 8 of portx"]
    #[inline(always)]
    pub const fn set_afr8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Multiplexing function selection for bit 9 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 9 of portx"]
    #[inline(always)]
    pub const fn set_afr9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Multiplexing function selection for bit 10 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 10 of portx"]
    #[inline(always)]
    pub const fn set_afr10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Multiplexing function selection for bit 11 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 11 of portx"]
    #[inline(always)]
    pub const fn set_afr11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Multiplexing function selection for bit 12 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 12 of portx"]
    #[inline(always)]
    pub const fn set_afr12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Multiplexing function selection for bit 13 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 13 of portx"]
    #[inline(always)]
    pub const fn set_afr13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Multiplexing function selection for bit 14 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 14 of portx"]
    #[inline(always)]
    pub const fn set_afr14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Multiplexing function selection for bit 15 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 15 of portx"]
    #[inline(always)]
    pub const fn set_afr15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Afrh {
    #[inline(always)]
    fn default() -> Afrh {
        Afrh(0)
    }
}
impl core::fmt::Debug for Afrh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Afrh")
            .field("afr8", &self.afr8())
            .field("afr9", &self.afr9())
            .field("afr10", &self.afr10())
            .field("afr11", &self.afr11())
            .field("afr12", &self.afr12())
            .field("afr13", &self.afr13())
            .field("afr14", &self.afr14())
            .field("afr15", &self.afr15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Afrh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Afrh {{ afr8: {=u8:?}, afr9: {=u8:?}, afr10: {=u8:?}, afr11: {=u8:?}, afr12: {=u8:?}, afr13: {=u8:?}, afr14: {=u8:?}, afr15: {=u8:?} }}",
            self.afr8(),
            self.afr9(),
            self.afr10(),
            self.afr11(),
            self.afr12(),
            self.afr13(),
            self.afr14(),
            self.afr15()
        )
    }
}
#[doc = "Port Multiplexing Function Low Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afrl(pub u32);
impl Afrl {
    #[doc = "Multiplexing function selection for bit 0 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 0 of portx"]
    #[inline(always)]
    pub const fn set_afr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Multiplexing function selection for bit 1 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 1 of portx"]
    #[inline(always)]
    pub const fn set_afr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Multiplexing function selection for bit 2 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 2 of portx"]
    #[inline(always)]
    pub const fn set_afr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Multiplexing function selection for bit 3 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 3 of portx"]
    #[inline(always)]
    pub const fn set_afr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Multiplexing function selection for bit 4 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 4 of portx"]
    #[inline(always)]
    pub const fn set_afr4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Multiplexing function selection for bit 5 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 5 of portx"]
    #[inline(always)]
    pub const fn set_afr5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Multiplexing function selection for bit 6 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 6 of portx"]
    #[inline(always)]
    pub const fn set_afr6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Multiplexing function selection for bit 7 of portx"]
    #[must_use]
    #[inline(always)]
    pub const fn afr7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Multiplexing function selection for bit 7 of portx"]
    #[inline(always)]
    pub const fn set_afr7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Afrl {
    #[inline(always)]
    fn default() -> Afrl {
        Afrl(0)
    }
}
impl core::fmt::Debug for Afrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Afrl")
            .field("afr0", &self.afr0())
            .field("afr1", &self.afr1())
            .field("afr2", &self.afr2())
            .field("afr3", &self.afr3())
            .field("afr4", &self.afr4())
            .field("afr5", &self.afr5())
            .field("afr6", &self.afr6())
            .field("afr7", &self.afr7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Afrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Afrl {{ afr0: {=u8:?}, afr1: {=u8:?}, afr2: {=u8:?}, afr3: {=u8:?}, afr4: {=u8:?}, afr5: {=u8:?}, afr6: {=u8:?}, afr7: {=u8:?} }}",
            self.afr0(),
            self.afr1(),
            self.afr2(),
            self.afr3(),
            self.afr4(),
            self.afr5(),
            self.afr6(),
            self.afr7()
        )
    }
}
#[doc = "bit reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brr(pub u32);
impl Brr {
    #[doc = "Port x Reset bit 0"]
    #[must_use]
    #[inline(always)]
    pub const fn br0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 0"]
    #[inline(always)]
    pub const fn set_br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x Reset bit 1"]
    #[must_use]
    #[inline(always)]
    pub const fn br1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 1"]
    #[inline(always)]
    pub const fn set_br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x Reset bit 2"]
    #[must_use]
    #[inline(always)]
    pub const fn br2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 2"]
    #[inline(always)]
    pub const fn set_br2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x Reset bit 3"]
    #[must_use]
    #[inline(always)]
    pub const fn br3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 3"]
    #[inline(always)]
    pub const fn set_br3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x Reset bit 4"]
    #[must_use]
    #[inline(always)]
    pub const fn br4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 4"]
    #[inline(always)]
    pub const fn set_br4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x Reset bit 5"]
    #[must_use]
    #[inline(always)]
    pub const fn br5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 5"]
    #[inline(always)]
    pub const fn set_br5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x Reset bit 6"]
    #[must_use]
    #[inline(always)]
    pub const fn br6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 6"]
    #[inline(always)]
    pub const fn set_br6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x Reset bit 7"]
    #[must_use]
    #[inline(always)]
    pub const fn br7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 7"]
    #[inline(always)]
    pub const fn set_br7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x Reset bit 8"]
    #[must_use]
    #[inline(always)]
    pub const fn br8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 8"]
    #[inline(always)]
    pub const fn set_br8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x Reset bit 9"]
    #[must_use]
    #[inline(always)]
    pub const fn br9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 9"]
    #[inline(always)]
    pub const fn set_br9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x Reset bit 10"]
    #[must_use]
    #[inline(always)]
    pub const fn br10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 10"]
    #[inline(always)]
    pub const fn set_br10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x Reset bit 11"]
    #[must_use]
    #[inline(always)]
    pub const fn br11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 11"]
    #[inline(always)]
    pub const fn set_br11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x Reset bit 12"]
    #[must_use]
    #[inline(always)]
    pub const fn br12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 12"]
    #[inline(always)]
    pub const fn set_br12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x Reset bit 13"]
    #[must_use]
    #[inline(always)]
    pub const fn br13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 13"]
    #[inline(always)]
    pub const fn set_br13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x Reset bit 14"]
    #[must_use]
    #[inline(always)]
    pub const fn br14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 14"]
    #[inline(always)]
    pub const fn set_br14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x Reset bit 15"]
    #[must_use]
    #[inline(always)]
    pub const fn br15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 15"]
    #[inline(always)]
    pub const fn set_br15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Brr {
    #[inline(always)]
    fn default() -> Brr {
        Brr(0)
    }
}
impl core::fmt::Debug for Brr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Brr")
            .field("br0", &self.br0())
            .field("br1", &self.br1())
            .field("br2", &self.br2())
            .field("br3", &self.br3())
            .field("br4", &self.br4())
            .field("br5", &self.br5())
            .field("br6", &self.br6())
            .field("br7", &self.br7())
            .field("br8", &self.br8())
            .field("br9", &self.br9())
            .field("br10", &self.br10())
            .field("br11", &self.br11())
            .field("br12", &self.br12())
            .field("br13", &self.br13())
            .field("br14", &self.br14())
            .field("br15", &self.br15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Brr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Brr {{ br0: {=bool:?}, br1: {=bool:?}, br2: {=bool:?}, br3: {=bool:?}, br4: {=bool:?}, br5: {=bool:?}, br6: {=bool:?}, br7: {=bool:?}, br8: {=bool:?}, br9: {=bool:?}, br10: {=bool:?}, br11: {=bool:?}, br12: {=bool:?}, br13: {=bool:?}, br14: {=bool:?}, br15: {=bool:?} }}",
            self.br0(),
            self.br1(),
            self.br2(),
            self.br3(),
            self.br4(),
            self.br5(),
            self.br6(),
            self.br7(),
            self.br8(),
            self.br9(),
            self.br10(),
            self.br11(),
            self.br12(),
            self.br13(),
            self.br14(),
            self.br15()
        )
    }
}
#[doc = "bit set/reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsrr(pub u32);
impl Bsrr {
    #[doc = "Port x Set bit 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bs0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 0"]
    #[inline(always)]
    pub const fn set_bs0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x Set bit 1"]
    #[must_use]
    #[inline(always)]
    pub const fn bs1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 1"]
    #[inline(always)]
    pub const fn set_bs1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x Set bit 2"]
    #[must_use]
    #[inline(always)]
    pub const fn bs2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 2"]
    #[inline(always)]
    pub const fn set_bs2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x Set bit 3"]
    #[must_use]
    #[inline(always)]
    pub const fn bs3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 3"]
    #[inline(always)]
    pub const fn set_bs3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x Set bit 4"]
    #[must_use]
    #[inline(always)]
    pub const fn bs4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 4"]
    #[inline(always)]
    pub const fn set_bs4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x Set bit 5"]
    #[must_use]
    #[inline(always)]
    pub const fn bs5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 5"]
    #[inline(always)]
    pub const fn set_bs5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x Set bit 6"]
    #[must_use]
    #[inline(always)]
    pub const fn bs6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 6"]
    #[inline(always)]
    pub const fn set_bs6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x Set bit 7"]
    #[must_use]
    #[inline(always)]
    pub const fn bs7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 7"]
    #[inline(always)]
    pub const fn set_bs7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x Set bit 8"]
    #[must_use]
    #[inline(always)]
    pub const fn bs8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 8"]
    #[inline(always)]
    pub const fn set_bs8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x Set bit 9"]
    #[must_use]
    #[inline(always)]
    pub const fn bs9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 9"]
    #[inline(always)]
    pub const fn set_bs9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x Set bit 10"]
    #[must_use]
    #[inline(always)]
    pub const fn bs10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 10"]
    #[inline(always)]
    pub const fn set_bs10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x Set bit 11"]
    #[must_use]
    #[inline(always)]
    pub const fn bs11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 11"]
    #[inline(always)]
    pub const fn set_bs11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x Set bit 12"]
    #[must_use]
    #[inline(always)]
    pub const fn bs12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 12"]
    #[inline(always)]
    pub const fn set_bs12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x Set bit 13"]
    #[must_use]
    #[inline(always)]
    pub const fn bs13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 13"]
    #[inline(always)]
    pub const fn set_bs13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x Set bit 14"]
    #[must_use]
    #[inline(always)]
    pub const fn bs14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 14"]
    #[inline(always)]
    pub const fn set_bs14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x Set bit 15"]
    #[must_use]
    #[inline(always)]
    pub const fn bs15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Set bit 15"]
    #[inline(always)]
    pub const fn set_bs15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Port x Reset bit 0"]
    #[must_use]
    #[inline(always)]
    pub const fn br0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 0"]
    #[inline(always)]
    pub const fn set_br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Port x Reset bit 1"]
    #[must_use]
    #[inline(always)]
    pub const fn br1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 1"]
    #[inline(always)]
    pub const fn set_br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Port x Reset bit 2"]
    #[must_use]
    #[inline(always)]
    pub const fn br2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 2"]
    #[inline(always)]
    pub const fn set_br2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Port x Reset bit 3"]
    #[must_use]
    #[inline(always)]
    pub const fn br3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 3"]
    #[inline(always)]
    pub const fn set_br3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Port x Reset bit 4"]
    #[must_use]
    #[inline(always)]
    pub const fn br4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 4"]
    #[inline(always)]
    pub const fn set_br4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Port x Reset bit 5"]
    #[must_use]
    #[inline(always)]
    pub const fn br5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 5"]
    #[inline(always)]
    pub const fn set_br5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Port x Reset bit 6"]
    #[must_use]
    #[inline(always)]
    pub const fn br6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 6"]
    #[inline(always)]
    pub const fn set_br6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Port x Reset bit 7"]
    #[must_use]
    #[inline(always)]
    pub const fn br7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 7"]
    #[inline(always)]
    pub const fn set_br7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Port x Reset bit 8"]
    #[must_use]
    #[inline(always)]
    pub const fn br8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 8"]
    #[inline(always)]
    pub const fn set_br8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Port x Reset bit 9"]
    #[must_use]
    #[inline(always)]
    pub const fn br9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 9"]
    #[inline(always)]
    pub const fn set_br9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Port x Reset bit 10"]
    #[must_use]
    #[inline(always)]
    pub const fn br10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 10"]
    #[inline(always)]
    pub const fn set_br10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Port x Reset bit 11"]
    #[must_use]
    #[inline(always)]
    pub const fn br11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 11"]
    #[inline(always)]
    pub const fn set_br11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Port x Reset bit 12"]
    #[must_use]
    #[inline(always)]
    pub const fn br12(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 12"]
    #[inline(always)]
    pub const fn set_br12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Port x Reset bit 13"]
    #[must_use]
    #[inline(always)]
    pub const fn br13(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 13"]
    #[inline(always)]
    pub const fn set_br13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Port x Reset bit 14"]
    #[must_use]
    #[inline(always)]
    pub const fn br14(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 14"]
    #[inline(always)]
    pub const fn set_br14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Port x Reset bit 15"]
    #[must_use]
    #[inline(always)]
    pub const fn br15(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Port x Reset bit 15"]
    #[inline(always)]
    pub const fn set_br15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Bsrr {
    #[inline(always)]
    fn default() -> Bsrr {
        Bsrr(0)
    }
}
impl core::fmt::Debug for Bsrr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bsrr")
            .field("bs0", &self.bs0())
            .field("bs1", &self.bs1())
            .field("bs2", &self.bs2())
            .field("bs3", &self.bs3())
            .field("bs4", &self.bs4())
            .field("bs5", &self.bs5())
            .field("bs6", &self.bs6())
            .field("bs7", &self.bs7())
            .field("bs8", &self.bs8())
            .field("bs9", &self.bs9())
            .field("bs10", &self.bs10())
            .field("bs11", &self.bs11())
            .field("bs12", &self.bs12())
            .field("bs13", &self.bs13())
            .field("bs14", &self.bs14())
            .field("bs15", &self.bs15())
            .field("br0", &self.br0())
            .field("br1", &self.br1())
            .field("br2", &self.br2())
            .field("br3", &self.br3())
            .field("br4", &self.br4())
            .field("br5", &self.br5())
            .field("br6", &self.br6())
            .field("br7", &self.br7())
            .field("br8", &self.br8())
            .field("br9", &self.br9())
            .field("br10", &self.br10())
            .field("br11", &self.br11())
            .field("br12", &self.br12())
            .field("br13", &self.br13())
            .field("br14", &self.br14())
            .field("br15", &self.br15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bsrr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bsrr {{ bs0: {=bool:?}, bs1: {=bool:?}, bs2: {=bool:?}, bs3: {=bool:?}, bs4: {=bool:?}, bs5: {=bool:?}, bs6: {=bool:?}, bs7: {=bool:?}, bs8: {=bool:?}, bs9: {=bool:?}, bs10: {=bool:?}, bs11: {=bool:?}, bs12: {=bool:?}, bs13: {=bool:?}, bs14: {=bool:?}, bs15: {=bool:?}, br0: {=bool:?}, br1: {=bool:?}, br2: {=bool:?}, br3: {=bool:?}, br4: {=bool:?}, br5: {=bool:?}, br6: {=bool:?}, br7: {=bool:?}, br8: {=bool:?}, br9: {=bool:?}, br10: {=bool:?}, br11: {=bool:?}, br12: {=bool:?}, br13: {=bool:?}, br14: {=bool:?}, br15: {=bool:?} }}",
            self.bs0(),
            self.bs1(),
            self.bs2(),
            self.bs3(),
            self.bs4(),
            self.bs5(),
            self.bs6(),
            self.bs7(),
            self.bs8(),
            self.bs9(),
            self.bs10(),
            self.bs11(),
            self.bs12(),
            self.bs13(),
            self.bs14(),
            self.bs15(),
            self.br0(),
            self.br1(),
            self.br2(),
            self.br3(),
            self.br4(),
            self.br5(),
            self.br6(),
            self.br7(),
            self.br8(),
            self.br9(),
            self.br10(),
            self.br11(),
            self.br12(),
            self.br13(),
            self.br14(),
            self.br15()
        )
    }
}
#[doc = "configuration high register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crh(pub u32);
impl Crh {
    #[doc = "Port 8 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Port 8 mode bits"]
    #[inline(always)]
    pub const fn set_mode8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Port 8 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf8(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Port 8 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Port 9 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Port 9 mode bits"]
    #[inline(always)]
    pub const fn set_mode9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Port 9 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf9(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Port 9 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Port 10 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Port 10 mode bits"]
    #[inline(always)]
    pub const fn set_mode10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Port 10 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf10(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Port 10 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Port 11 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Port 11 mode bits"]
    #[inline(always)]
    pub const fn set_mode11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Port 11 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf11(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Port 11 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Port 12 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Port 12 mode bits"]
    #[inline(always)]
    pub const fn set_mode12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Port 12 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf12(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Port 12 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Port 13 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Port 13 mode bits"]
    #[inline(always)]
    pub const fn set_mode13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Port 13 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf13(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Port 13 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Port 14 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Port 14 mode bits"]
    #[inline(always)]
    pub const fn set_mode14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Port 14 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf14(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Port 14 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Port 15 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Port 15 mode bits"]
    #[inline(always)]
    pub const fn set_mode15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Port 15 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Port 15 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Crh {
    #[inline(always)]
    fn default() -> Crh {
        Crh(0)
    }
}
impl core::fmt::Debug for Crh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crh")
            .field("mode8", &self.mode8())
            .field("cnf8", &self.cnf8())
            .field("mode9", &self.mode9())
            .field("cnf9", &self.cnf9())
            .field("mode10", &self.mode10())
            .field("cnf10", &self.cnf10())
            .field("mode11", &self.mode11())
            .field("cnf11", &self.cnf11())
            .field("mode12", &self.mode12())
            .field("cnf12", &self.cnf12())
            .field("mode13", &self.mode13())
            .field("cnf13", &self.cnf13())
            .field("mode14", &self.mode14())
            .field("cnf14", &self.cnf14())
            .field("mode15", &self.mode15())
            .field("cnf15", &self.cnf15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Crh {{ mode8: {=u8:?}, cnf8: {=u8:?}, mode9: {=u8:?}, cnf9: {=u8:?}, mode10: {=u8:?}, cnf10: {=u8:?}, mode11: {=u8:?}, cnf11: {=u8:?}, mode12: {=u8:?}, cnf12: {=u8:?}, mode13: {=u8:?}, cnf13: {=u8:?}, mode14: {=u8:?}, cnf14: {=u8:?}, mode15: {=u8:?}, cnf15: {=u8:?} }}",
            self.mode8(),
            self.cnf8(),
            self.mode9(),
            self.cnf9(),
            self.mode10(),
            self.cnf10(),
            self.mode11(),
            self.cnf11(),
            self.mode12(),
            self.cnf12(),
            self.mode13(),
            self.cnf13(),
            self.mode14(),
            self.cnf14(),
            self.mode15(),
            self.cnf15()
        )
    }
}
#[doc = "configuration low register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crl(pub u32);
impl Crl {
    #[doc = "Port 0 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Port 0 mode bits"]
    #[inline(always)]
    pub const fn set_mode0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Port 0 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf0(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Port 0 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Port 1 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Port 1 mode bits"]
    #[inline(always)]
    pub const fn set_mode1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Port 1 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf1(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Port 1 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Port 2 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Port 2 mode bits"]
    #[inline(always)]
    pub const fn set_mode2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Port 2 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf2(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Port 2 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Port 3 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Port 3 mode bits"]
    #[inline(always)]
    pub const fn set_mode3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Port 3 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf3(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Port 3 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Port 4 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Port 4 mode bits"]
    #[inline(always)]
    pub const fn set_mode4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Port 4 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf4(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Port 4 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Port 5 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Port 5 mode bits"]
    #[inline(always)]
    pub const fn set_mode5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Port 5 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf5(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Port 5 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Port 6 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Port 6 mode bits"]
    #[inline(always)]
    pub const fn set_mode6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Port 6 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf6(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Port 6 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Port 7 mode bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mode7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Port 7 mode bits"]
    #[inline(always)]
    pub const fn set_mode7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Port 7 configuration bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf7(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Port 7 configuration bits"]
    #[inline(always)]
    pub const fn set_cnf7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Crl {
    #[inline(always)]
    fn default() -> Crl {
        Crl(0)
    }
}
impl core::fmt::Debug for Crl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crl")
            .field("mode0", &self.mode0())
            .field("cnf0", &self.cnf0())
            .field("mode1", &self.mode1())
            .field("cnf1", &self.cnf1())
            .field("mode2", &self.mode2())
            .field("cnf2", &self.cnf2())
            .field("mode3", &self.mode3())
            .field("cnf3", &self.cnf3())
            .field("mode4", &self.mode4())
            .field("cnf4", &self.cnf4())
            .field("mode5", &self.mode5())
            .field("cnf5", &self.cnf5())
            .field("mode6", &self.mode6())
            .field("cnf6", &self.cnf6())
            .field("mode7", &self.mode7())
            .field("cnf7", &self.cnf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Crl {{ mode0: {=u8:?}, cnf0: {=u8:?}, mode1: {=u8:?}, cnf1: {=u8:?}, mode2: {=u8:?}, cnf2: {=u8:?}, mode3: {=u8:?}, cnf3: {=u8:?}, mode4: {=u8:?}, cnf4: {=u8:?}, mode5: {=u8:?}, cnf5: {=u8:?}, mode6: {=u8:?}, cnf6: {=u8:?}, mode7: {=u8:?}, cnf7: {=u8:?} }}",
            self.mode0(),
            self.cnf0(),
            self.mode1(),
            self.cnf1(),
            self.mode2(),
            self.cnf2(),
            self.mode3(),
            self.cnf3(),
            self.mode4(),
            self.cnf4(),
            self.mode5(),
            self.cnf5(),
            self.mode6(),
            self.cnf6(),
            self.mode7(),
            self.cnf7()
        )
    }
}
#[doc = "Port output open drain control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcr(pub u32);
impl Dcr {
    #[doc = "PX0"]
    #[must_use]
    #[inline(always)]
    pub const fn px0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "PX0"]
    #[inline(always)]
    pub const fn set_px0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "PX1"]
    #[must_use]
    #[inline(always)]
    pub const fn px1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PX1"]
    #[inline(always)]
    pub const fn set_px1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "PX2"]
    #[must_use]
    #[inline(always)]
    pub const fn px2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "PX2"]
    #[inline(always)]
    pub const fn set_px2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "PX3"]
    #[must_use]
    #[inline(always)]
    pub const fn px3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "PX3"]
    #[inline(always)]
    pub const fn set_px3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "PX4"]
    #[must_use]
    #[inline(always)]
    pub const fn px4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "PX4"]
    #[inline(always)]
    pub const fn set_px4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "PX5"]
    #[must_use]
    #[inline(always)]
    pub const fn px5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "PX5"]
    #[inline(always)]
    pub const fn set_px5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "PX6"]
    #[must_use]
    #[inline(always)]
    pub const fn px6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "PX6"]
    #[inline(always)]
    pub const fn set_px6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "PX7"]
    #[must_use]
    #[inline(always)]
    pub const fn px7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "PX7"]
    #[inline(always)]
    pub const fn set_px7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "PX8"]
    #[must_use]
    #[inline(always)]
    pub const fn px8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "PX8"]
    #[inline(always)]
    pub const fn set_px8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "PX9"]
    #[must_use]
    #[inline(always)]
    pub const fn px9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "PX9"]
    #[inline(always)]
    pub const fn set_px9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "PX10"]
    #[must_use]
    #[inline(always)]
    pub const fn px10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "PX10"]
    #[inline(always)]
    pub const fn set_px10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "PX11"]
    #[must_use]
    #[inline(always)]
    pub const fn px11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "PX11"]
    #[inline(always)]
    pub const fn set_px11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "PX12"]
    #[must_use]
    #[inline(always)]
    pub const fn px12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "PX12"]
    #[inline(always)]
    pub const fn set_px12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "PX13"]
    #[must_use]
    #[inline(always)]
    pub const fn px13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "PX13"]
    #[inline(always)]
    pub const fn set_px13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "PX14"]
    #[must_use]
    #[inline(always)]
    pub const fn px14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "PX14"]
    #[inline(always)]
    pub const fn set_px14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "PX15"]
    #[must_use]
    #[inline(always)]
    pub const fn px15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "PX15"]
    #[inline(always)]
    pub const fn set_px15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
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
            .field("px0", &self.px0())
            .field("px1", &self.px1())
            .field("px2", &self.px2())
            .field("px3", &self.px3())
            .field("px4", &self.px4())
            .field("px5", &self.px5())
            .field("px6", &self.px6())
            .field("px7", &self.px7())
            .field("px8", &self.px8())
            .field("px9", &self.px9())
            .field("px10", &self.px10())
            .field("px11", &self.px11())
            .field("px12", &self.px12())
            .field("px13", &self.px13())
            .field("px14", &self.px14())
            .field("px15", &self.px15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcr {{ px0: {=u8:?}, px1: {=u8:?}, px2: {=u8:?}, px3: {=u8:?}, px4: {=u8:?}, px5: {=u8:?}, px6: {=u8:?}, px7: {=u8:?}, px8: {=u8:?}, px9: {=u8:?}, px10: {=u8:?}, px11: {=u8:?}, px12: {=u8:?}, px13: {=u8:?}, px14: {=u8:?}, px15: {=u8:?} }}",
            self.px0(),
            self.px1(),
            self.px2(),
            self.px3(),
            self.px4(),
            self.px5(),
            self.px6(),
            self.px7(),
            self.px8(),
            self.px9(),
            self.px10(),
            self.px11(),
            self.px12(),
            self.px13(),
            self.px14(),
            self.px15()
        )
    }
}
#[doc = "input data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idr(pub u32);
impl Idr {
    #[doc = "Port input data"]
    #[must_use]
    #[inline(always)]
    pub const fn idr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Port input data"]
    #[inline(always)]
    pub const fn set_idr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Idr {
    #[inline(always)]
    fn default() -> Idr {
        Idr(0)
    }
}
impl core::fmt::Debug for Idr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Idr").field("idr", &self.idr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Idr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Idr {{ idr: {=u16:?} }}", self.idr())
    }
}
#[doc = "Port configuration lock register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lckr(pub u32);
impl Lckr {
    #[doc = "Port x Lock bit y"]
    #[must_use]
    #[inline(always)]
    pub const fn lck(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Port x Lock bit y"]
    #[inline(always)]
    pub const fn set_lck(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Lock key"]
    #[must_use]
    #[inline(always)]
    pub const fn lckk(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Lock key"]
    #[inline(always)]
    pub const fn set_lckk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Lckr {
    #[inline(always)]
    fn default() -> Lckr {
        Lckr(0)
    }
}
impl core::fmt::Debug for Lckr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lckr")
            .field("lck", &self.lck())
            .field("lckk", &self.lckk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lckr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lckr {{ lck: {=u16:?}, lckk: {=bool:?} }}",
            self.lck(),
            self.lckk()
        )
    }
}
#[doc = "output data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Odr(pub u32);
impl Odr {
    #[doc = "Port output data"]
    #[must_use]
    #[inline(always)]
    pub const fn odr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Port output data"]
    #[inline(always)]
    pub const fn set_odr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Odr {
    #[inline(always)]
    fn default() -> Odr {
        Odr(0)
    }
}
impl core::fmt::Debug for Odr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Odr").field("odr", &self.odr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Odr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Odr {{ odr: {=u16:?} }}", self.odr())
    }
}
