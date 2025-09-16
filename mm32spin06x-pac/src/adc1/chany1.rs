#[doc = "Register `CHANY1` reader"]
pub type R = crate::R<Chany1Spec>;
#[doc = "Register `CHANY1` writer"]
pub type W = crate::W<Chany1Spec>;
#[doc = "Field `CHANY_SEL8` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel8R = crate::FieldReader;
#[doc = "Field `CHANY_SEL8` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL9` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel9R = crate::FieldReader;
#[doc = "Field `CHANY_SEL9` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL14` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel14R = crate::FieldReader;
#[doc = "Field `CHANY_SEL14` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL15` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel15R = crate::FieldReader;
#[doc = "Field `CHANY_SEL15` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel8(&self) -> ChanySel8R {
        ChanySel8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel9(&self) -> ChanySel9R {
        ChanySel9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel14(&self) -> ChanySel14R {
        ChanySel14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel15(&self) -> ChanySel15R {
        ChanySel15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel8(&mut self) -> ChanySel8W<'_, Chany1Spec> {
        ChanySel8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel9(&mut self) -> ChanySel9W<'_, Chany1Spec> {
        ChanySel9W::new(self, 4)
    }
    #[doc = "Bits 24:27 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel14(&mut self) -> ChanySel14W<'_, Chany1Spec> {
        ChanySel14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel15(&mut self) -> ChanySel15W<'_, Chany1Spec> {
        ChanySel15W::new(self, 28)
    }
}
#[doc = "Arbitrary channel channel selection register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`chany1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chany1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chany1Spec;
impl crate::RegisterSpec for Chany1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chany1::R`](R) reader structure"]
impl crate::Readable for Chany1Spec {}
#[doc = "`write(|w| ..)` method takes [`chany1::W`](W) writer structure"]
impl crate::Writable for Chany1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHANY1 to value 0"]
impl crate::Resettable for Chany1Spec {}
