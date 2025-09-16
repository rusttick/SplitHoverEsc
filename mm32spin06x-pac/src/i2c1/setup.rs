#[doc = "Register `SETUP` reader"]
pub type R = crate::R<SetupSpec>;
#[doc = "Register `SETUP` writer"]
pub type W = crate::W<SetupSpec>;
#[doc = "Field `CNT` reader - SDA setup"]
pub type CntR = crate::FieldReader;
#[doc = "Field `CNT` writer - SDA setup"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SDA setup"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDA setup"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, SetupSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "SDA Setup Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetupSpec;
impl crate::RegisterSpec for SetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setup::R`](R) reader structure"]
impl crate::Readable for SetupSpec {}
#[doc = "`write(|w| ..)` method takes [`setup::W`](W) writer structure"]
impl crate::Writable for SetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SETUP to value 0x64"]
impl crate::Resettable for SetupSpec {
    const RESET_VALUE: u32 = 0x64;
}
