#[doc = "Register `TAR` reader"]
pub type R = crate::R<TarSpec>;
#[doc = "Register `TAR` writer"]
pub type W = crate::W<TarSpec>;
#[doc = "Field `ADDR` reader - This is the target address for any master transaction"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - This is the target address for any master transaction"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `GC` reader - If bit 11(SPECIAL)is set to 1"]
pub type GcR = crate::BitReader;
#[doc = "Field `GC` writer - If bit 11(SPECIAL)is set to 1"]
pub type GcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPECIAL` reader - This bit indicates whether software performs a General Call or START BYTE conmmend"]
pub type SpecialR = crate::BitReader;
#[doc = "Field `SPECIAL` writer - This bit indicates whether software performs a General Call or START BYTE conmmend"]
pub type SpecialW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - This is the target address for any master transaction"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - If bit 11(SPECIAL)is set to 1"]
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit indicates whether software performs a General Call or START BYTE conmmend"]
    #[inline(always)]
    pub fn special(&self) -> SpecialR {
        SpecialR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This is the target address for any master transaction"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, TarSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 10 - If bit 11(SPECIAL)is set to 1"]
    #[inline(always)]
    pub fn gc(&mut self) -> GcW<'_, TarSpec> {
        GcW::new(self, 10)
    }
    #[doc = "Bit 11 - This bit indicates whether software performs a General Call or START BYTE conmmend"]
    #[inline(always)]
    pub fn special(&mut self) -> SpecialW<'_, TarSpec> {
        SpecialW::new(self, 11)
    }
}
#[doc = "Target Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TarSpec;
impl crate::RegisterSpec for TarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar::R`](R) reader structure"]
impl crate::Readable for TarSpec {}
#[doc = "`write(|w| ..)` method takes [`tar::W`](W) writer structure"]
impl crate::Writable for TarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAR to value 0x55"]
impl crate::Resettable for TarSpec {
    const RESET_VALUE: u32 = 0x55;
}
