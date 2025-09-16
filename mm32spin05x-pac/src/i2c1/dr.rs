#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DrSpec>;
#[doc = "Field `DAT` reader - This register contains the data to be transimitted or received on the i2c bus."]
pub type DatR = crate::FieldReader;
#[doc = "Field `DAT` writer - This register contains the data to be transimitted or received on the i2c bus."]
pub type DatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMD` reader - This bit controls whether a read or a write is perormed"]
pub type CmdR = crate::BitReader;
#[doc = "Field `CMD` writer - This bit controls whether a read or a write is perormed"]
pub type CmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - This register contains the data to be transimitted or received on the i2c bus."]
    #[inline(always)]
    pub fn dat(&self) -> DatR {
        DatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - This bit controls whether a read or a write is perormed"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register contains the data to be transimitted or received on the i2c bus."]
    #[inline(always)]
    pub fn dat(&mut self) -> DatW<'_, DrSpec> {
        DatW::new(self, 0)
    }
    #[doc = "Bit 8 - This bit controls whether a read or a write is perormed"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, DrSpec> {
        CmdW::new(self, 8)
    }
}
#[doc = "Data Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR to value 0x01"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0x01;
}
