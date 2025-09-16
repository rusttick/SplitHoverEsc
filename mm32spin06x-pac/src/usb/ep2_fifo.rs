#[doc = "Register `EP2_FIFO` reader"]
pub type R = crate::R<Ep2FifoSpec>;
#[doc = "Register `EP2_FIFO` writer"]
pub type W = crate::W<Ep2FifoSpec>;
#[doc = "Field `EP2_FIFO` reader - EP2 FIFO port"]
pub type Ep2FifoR = crate::FieldReader;
#[doc = "Field `EP2_FIFO` writer - EP2 FIFO port"]
pub type Ep2FifoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EP2 FIFO port"]
    #[inline(always)]
    pub fn ep2_fifo(&self) -> Ep2FifoR {
        Ep2FifoR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EP2 FIFO port"]
    #[inline(always)]
    pub fn ep2_fifo(&mut self) -> Ep2FifoW<'_, Ep2FifoSpec> {
        Ep2FifoW::new(self, 0)
    }
}
#[doc = "EP2 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep2_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep2FifoSpec;
impl crate::RegisterSpec for Ep2FifoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep2_fifo::R`](R) reader structure"]
impl crate::Readable for Ep2FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`ep2_fifo::W`](W) writer structure"]
impl crate::Writable for Ep2FifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP2_FIFO to value 0"]
impl crate::Resettable for Ep2FifoSpec {}
