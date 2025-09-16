#[doc = "Register `EP3_CTRL` reader"]
pub type R = crate::R<Ep3CtrlSpec>;
#[doc = "Register `EP3_CTRL` writer"]
pub type W = crate::W<Ep3CtrlSpec>;
#[doc = "Field `TRANCOUNT` reader - EP3 transfer counter"]
pub type TrancountR = crate::FieldReader;
#[doc = "Field `TRANCOUNT` writer - EP3 transfer counter"]
pub type TrancountW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TRANEN` reader - EP3 transfer enable"]
pub type TranenR = crate::BitReader;
#[doc = "Field `TRANEN` writer - EP3 transfer enable"]
pub type TranenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - EP3 transfer counter"]
    #[inline(always)]
    pub fn trancount(&self) -> TrancountR {
        TrancountR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - EP3 transfer enable"]
    #[inline(always)]
    pub fn tranen(&self) -> TranenR {
        TranenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - EP3 transfer counter"]
    #[inline(always)]
    pub fn trancount(&mut self) -> TrancountW<'_, Ep3CtrlSpec> {
        TrancountW::new(self, 0)
    }
    #[doc = "Bit 7 - EP3 transfer enable"]
    #[inline(always)]
    pub fn tranen(&mut self) -> TranenW<'_, Ep3CtrlSpec> {
        TranenW::new(self, 7)
    }
}
#[doc = "EP3 CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep3CtrlSpec;
impl crate::RegisterSpec for Ep3CtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep3_ctrl::R`](R) reader structure"]
impl crate::Readable for Ep3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ep3_ctrl::W`](W) writer structure"]
impl crate::Writable for Ep3CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP3_CTRL to value 0"]
impl crate::Resettable for Ep3CtrlSpec {}
