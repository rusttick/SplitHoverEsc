#[doc = "Register `DAM_ADDR1` reader"]
pub type R = crate::R<DamAddr1Spec>;
#[doc = "Field `DMA_ADDR1` reader - USB DAM address1\\[15:8\\]"]
pub type DmaAddr1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - USB DAM address1\\[15:8\\]"]
    #[inline(always)]
    pub fn dma_addr1(&self) -> DmaAddr1R {
        DmaAddr1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA1 ADDR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dam_addr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DamAddr1Spec;
impl crate::RegisterSpec for DamAddr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dam_addr1::R`](R) reader structure"]
impl crate::Readable for DamAddr1Spec {}
#[doc = "`reset()` method sets DAM_ADDR1 to value 0"]
impl crate::Resettable for DamAddr1Spec {}
