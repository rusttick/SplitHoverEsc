#[doc = "Register `EP4_CTRL` reader"]
pub type R = crate::R<Ep4CtrlSpec>;
#[doc = "Register `EP4_CTRL` writer"]
pub type W = crate::W<Ep4CtrlSpec>;
#[doc = "Field `TRANCOUNT` reader - EP4 transfer counter"]
pub type TrancountR = crate::FieldReader;
#[doc = "Field `TRANCOUNT` writer - EP4 transfer counter"]
pub type TrancountW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TRANEN` reader - EP4 transfer enable"]
pub type TranenR = crate::BitReader;
#[doc = "Field `TRANEN` writer - EP4 transfer enable"]
pub type TranenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - EP4 transfer counter"]
    #[inline(always)]
    pub fn trancount(&self) -> TrancountR {
        TrancountR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - EP4 transfer enable"]
    #[inline(always)]
    pub fn tranen(&self) -> TranenR {
        TranenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - EP4 transfer counter"]
    #[inline(always)]
    pub fn trancount(&mut self) -> TrancountW<'_, Ep4CtrlSpec> {
        TrancountW::new(self, 0)
    }
    #[doc = "Bit 7 - EP4 transfer enable"]
    #[inline(always)]
    pub fn tranen(&mut self) -> TranenW<'_, Ep4CtrlSpec> {
        TranenW::new(self, 7)
    }
}
#[doc = "EP4 CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep4_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep4CtrlSpec;
impl crate::RegisterSpec for Ep4CtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep4_ctrl::R`](R) reader structure"]
impl crate::Readable for Ep4CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ep4_ctrl::W`](W) writer structure"]
impl crate::Writable for Ep4CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP4_CTRL to value 0"]
impl crate::Resettable for Ep4CtrlSpec {}
