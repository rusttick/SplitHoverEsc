#[doc = "Register `DAM_ADDR3` reader"]
pub type R = crate::R<DamAddr3Spec>;
#[doc = "Field `DMA_ADDR3` reader - USB DAM address3\\[31:24\\]"]
pub type DmaAddr3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - USB DAM address3\\[31:24\\]"]
    #[inline(always)]
    pub fn dma_addr3(&self) -> DmaAddr3R {
        DmaAddr3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA1 ADDR3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dam_addr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DamAddr3Spec;
impl crate::RegisterSpec for DamAddr3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dam_addr3::R`](R) reader structure"]
impl crate::Readable for DamAddr3Spec {}
#[doc = "`reset()` method sets DAM_ADDR3 to value 0"]
impl crate::Resettable for DamAddr3Spec {}
