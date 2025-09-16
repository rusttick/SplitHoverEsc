#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `IRQ_SEL` reader - Interruput select"]
pub type IrqSelR = crate::BitReader;
#[doc = "Field `IRQ_SEL` writer - Interruput select"]
pub type IrqSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_CLR` reader - Interruput clear"]
pub type IrqClrR = crate::BitReader;
#[doc = "Field `IRQ_CLR` writer - Interruput clear"]
pub type IrqClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interruput select"]
    #[inline(always)]
    pub fn irq_sel(&self) -> IrqSelR {
        IrqSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interruput clear"]
    #[inline(always)]
    pub fn irq_clr(&self) -> IrqClrR {
        IrqClrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interruput select"]
    #[inline(always)]
    pub fn irq_sel(&mut self) -> IrqSelW<'_, CrSpec> {
        IrqSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Interruput clear"]
    #[inline(always)]
    pub fn irq_clr(&mut self) -> IrqClrW<'_, CrSpec> {
        IrqClrW::new(self, 1)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
