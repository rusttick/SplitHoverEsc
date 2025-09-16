#[doc = "Register `EP3_FIFO` reader"]
pub type R = crate::R<Ep3FifoSpec>;
#[doc = "Register `EP3_FIFO` writer"]
pub type W = crate::W<Ep3FifoSpec>;
#[doc = "Field `EP3_FIFO` reader - EP3 FIFO port"]
pub type Ep3FifoR = crate::FieldReader;
#[doc = "Field `EP3_FIFO` writer - EP3 FIFO port"]
pub type Ep3FifoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EP3 FIFO port"]
    #[inline(always)]
    pub fn ep3_fifo(&self) -> Ep3FifoR {
        Ep3FifoR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EP3 FIFO port"]
    #[inline(always)]
    pub fn ep3_fifo(&mut self) -> Ep3FifoW<'_, Ep3FifoSpec> {
        Ep3FifoW::new(self, 0)
    }
}
#[doc = "EP3 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep3_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep3FifoSpec;
impl crate::RegisterSpec for Ep3FifoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep3_fifo::R`](R) reader structure"]
impl crate::Readable for Ep3FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`ep3_fifo::W`](W) writer structure"]
impl crate::Writable for Ep3FifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP3_FIFO to value 0"]
impl crate::Resettable for Ep3FifoSpec {}
