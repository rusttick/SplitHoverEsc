#[doc = "Register `DAM_ADDR0` reader"]
pub type R = crate::R<DamAddr0Spec>;
#[doc = "Field `DMA_ADDR0` reader - USB DAM address0\\[7:0\\]"]
pub type DmaAddr0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - USB DAM address0\\[7:0\\]"]
    #[inline(always)]
    pub fn dma_addr0(&self) -> DmaAddr0R {
        DmaAddr0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA1 ADDR0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dam_addr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DamAddr0Spec;
impl crate::RegisterSpec for DamAddr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dam_addr0::R`](R) reader structure"]
impl crate::Readable for DamAddr0Spec {}
#[doc = "`reset()` method sets DAM_ADDR0 to value 0"]
impl crate::Resettable for DamAddr0Spec {}
