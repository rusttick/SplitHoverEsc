#[doc = "Register `DAM_ADDR2` reader"]
pub type R = crate::R<DamAddr2Spec>;
#[doc = "Field `DMA_ADDR2` reader - USB DAM address2\\[23:16\\]"]
pub type DmaAddr2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - USB DAM address2\\[23:16\\]"]
    #[inline(always)]
    pub fn dma_addr2(&self) -> DmaAddr2R {
        DmaAddr2R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA1 ADDR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dam_addr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DamAddr2Spec;
impl crate::RegisterSpec for DamAddr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dam_addr2::R`](R) reader structure"]
impl crate::Readable for DamAddr2Spec {}
#[doc = "`reset()` method sets DAM_ADDR2 to value 0"]
impl crate::Resettable for DamAddr2Spec {}
