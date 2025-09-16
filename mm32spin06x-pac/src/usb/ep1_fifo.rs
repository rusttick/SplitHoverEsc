#[doc = "Register `EP1_FIFO` reader"]
pub type R = crate::R<Ep1FifoSpec>;
#[doc = "Register `EP1_FIFO` writer"]
pub type W = crate::W<Ep1FifoSpec>;
#[doc = "Field `EP1_FIFO` reader - EP1 FIFO port"]
pub type Ep1FifoR = crate::FieldReader;
#[doc = "Field `EP1_FIFO` writer - EP1 FIFO port"]
pub type Ep1FifoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EP1 FIFO port"]
    #[inline(always)]
    pub fn ep1_fifo(&self) -> Ep1FifoR {
        Ep1FifoR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EP1 FIFO port"]
    #[inline(always)]
    pub fn ep1_fifo(&mut self) -> Ep1FifoW<'_, Ep1FifoSpec> {
        Ep1FifoW::new(self, 0)
    }
}
#[doc = "EP1 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep1_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep1FifoSpec;
impl crate::RegisterSpec for Ep1FifoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep1_fifo::R`](R) reader structure"]
impl crate::Readable for Ep1FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`ep1_fifo::W`](W) writer structure"]
impl crate::Writable for Ep1FifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP1_FIFO to value 0"]
impl crate::Resettable for Ep1FifoSpec {}
