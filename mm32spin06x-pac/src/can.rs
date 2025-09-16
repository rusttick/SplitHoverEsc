#[doc = "Controller area network"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can {
    ptr: *mut u8,
}
unsafe impl Send for Can {}
unsafe impl Sync for Can {}
impl Can {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Basic control register"]
    #[inline(always)]
    pub const fn cr_b(self) -> crate::common::Reg<regs::CrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Peli Mode register"]
    #[inline(always)]
    pub const fn mod_p(self) -> crate::common::Reg<regs::ModP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Basic Command register"]
    #[inline(always)]
    pub const fn cmr_b(self) -> crate::common::Reg<regs::CmrB, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Peli Command register"]
    #[inline(always)]
    pub const fn cmr_p(self) -> crate::common::Reg<regs::CmrP, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt register"]
    #[inline(always)]
    pub const fn ir_b(self) -> crate::common::Reg<regs::IrB, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt register"]
    #[inline(always)]
    pub const fn ir_p(self) -> crate::common::Reg<regs::IrP, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group0_acr_b(self) -> crate::common::Reg<regs::Group0AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Peli Interrupt Enable register"]
    #[inline(always)]
    pub const fn ier_p(self) -> crate::common::Reg<regs::IerP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group0_amr_b(self) -> crate::common::Reg<regs::Group0AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Bus Timing register 0"]
    #[inline(always)]
    pub const fn btr0(self) -> crate::common::Reg<regs::Btr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Bus Timing register 1"]
    #[inline(always)]
    pub const fn btr1(self) -> crate::common::Reg<regs::Btr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Basic TX ID register 0"]
    #[inline(always)]
    pub const fn txid0_b(self) -> crate::common::Reg<regs::Txid0B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Peli Arbitration Lost Capture register"]
    #[inline(always)]
    pub const fn alc_p(self) -> crate::common::Reg<regs::AlcP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Basic TX ID register 1"]
    #[inline(always)]
    pub const fn txid1_b(self) -> crate::common::Reg<regs::Txid1B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Peli Error Code Capture register"]
    #[inline(always)]
    pub const fn ecc_p(self) -> crate::common::Reg<regs::EccP, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Basic TX Data register 0"]
    #[inline(always)]
    pub const fn txdr0_b(self) -> crate::common::Reg<regs::Txdr0B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Peli Error Warning Limit register"]
    #[inline(always)]
    pub const fn ewlr_p(self) -> crate::common::Reg<regs::EwlrP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Basic TX Data register 1"]
    #[inline(always)]
    pub const fn txdr1_b(self) -> crate::common::Reg<regs::Txdr1B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Peli RX Error Counter register"]
    #[inline(always)]
    pub const fn rxerr_p(self) -> crate::common::Reg<regs::RxerrP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Basic Send Data register 2"]
    #[inline(always)]
    pub const fn txdr2_b(self) -> crate::common::Reg<regs::Txdr2B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Basic TX Data register 3"]
    #[inline(always)]
    pub const fn txdr3_b(self) -> crate::common::Reg<regs::Txdr3B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Peli TX Error Counter register"]
    #[inline(always)]
    pub const fn txerr_p(self) -> crate::common::Reg<regs::TxerrP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register 0"]
    #[inline(always)]
    pub const fn group0_acr0_p(self) -> crate::common::Reg<regs::Group0Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Peli Send Frame Format register"]
    #[inline(always)]
    pub const fn sff_p(self) -> crate::common::Reg<regs::SffP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Basic TX Data register 4"]
    #[inline(always)]
    pub const fn txdr4_b(self) -> crate::common::Reg<regs::Txdr4B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Peli Acceptance Code register 1"]
    #[inline(always)]
    pub const fn group0_acr1_p(self) -> crate::common::Reg<regs::Group0Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Basic TX Data register 5"]
    #[inline(always)]
    pub const fn txdr5_b(self) -> crate::common::Reg<regs::Txdr5B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Peli TX ID register 0"]
    #[inline(always)]
    pub const fn txid0_p(self) -> crate::common::Reg<regs::Txid0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Peli Acceptance Code register 2"]
    #[inline(always)]
    pub const fn group0_acr2_p(self) -> crate::common::Reg<regs::Group0Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Basic TX Data register 6"]
    #[inline(always)]
    pub const fn txdr6_b(self) -> crate::common::Reg<regs::Txdr6B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Peli TX ID register 1"]
    #[inline(always)]
    pub const fn txid1_p(self) -> crate::common::Reg<regs::Txid1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Peli Acceptance Code register 3"]
    #[inline(always)]
    pub const fn group0_acr3_p(self) -> crate::common::Reg<regs::Group0Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Peli TX Data register 0"]
    #[inline(always)]
    pub const fn txdata0_p(self) -> crate::common::Reg<regs::Txdata0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Basic TX Data register 7"]
    #[inline(always)]
    pub const fn txdr7_b(self) -> crate::common::Reg<regs::Txdr7B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group0_amr0_p(self) -> crate::common::Reg<regs::Group0Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Peli TX Data register 1"]
    #[inline(always)]
    pub const fn txdata1_p(self) -> crate::common::Reg<regs::Txdata1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group0_amr1_p(self) -> crate::common::Reg<regs::Group0Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Peli TX Data register 2"]
    #[inline(always)]
    pub const fn txdata2_p(self) -> crate::common::Reg<regs::Txdata2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group0_amr2_p(self) -> crate::common::Reg<regs::Group0Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Peli TX Data register 3"]
    #[inline(always)]
    pub const fn txdata3_p(self) -> crate::common::Reg<regs::Txdata3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group0_amr3_p(self) -> crate::common::Reg<regs::Group0Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Peli TX Data register 4"]
    #[inline(always)]
    pub const fn txdata4_p(self) -> crate::common::Reg<regs::Txdata4P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Peli TX Data register 5"]
    #[inline(always)]
    pub const fn txdata5_p(self) -> crate::common::Reg<regs::Txdata5P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Peli TX Data register 6"]
    #[inline(always)]
    pub const fn txdata6_p(self) -> crate::common::Reg<regs::Txdata6P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Peli TX Data register 7"]
    #[inline(always)]
    pub const fn txdata7_p(self) -> crate::common::Reg<regs::Txdata7P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Peli TX Data register 8"]
    #[inline(always)]
    pub const fn txdata8_p(self) -> crate::common::Reg<regs::Txdata8P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Peli TX Data register 9"]
    #[inline(always)]
    pub const fn txdata9_p(self) -> crate::common::Reg<regs::Txdata9P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Clock Divider register"]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::Cdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Filter Mode register 0"]
    #[inline(always)]
    pub const fn afm0(self) -> crate::common::Reg<regs::Afm0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Filter Mode register 1"]
    #[inline(always)]
    pub const fn afm1(self) -> crate::common::Reg<regs::Afm1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Filter Mode register 2"]
    #[inline(always)]
    pub const fn afm2(self) -> crate::common::Reg<regs::Afm2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Filter Group Enable Register 0"]
    #[inline(always)]
    pub const fn fga0(self) -> crate::common::Reg<regs::Fga0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Filter Group Enable Register 1"]
    #[inline(always)]
    pub const fn fga1(self) -> crate::common::Reg<regs::Fga1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Filter Group Enable Register 2"]
    #[inline(always)]
    pub const fn fga2(self) -> crate::common::Reg<regs::Fga2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group1_acr0_p(self) -> crate::common::Reg<regs::Group1Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group1_acr_b(self) -> crate::common::Reg<regs::Group1AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group1_acr1_p(self) -> crate::common::Reg<regs::Group1Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group1_acr2_p(self) -> crate::common::Reg<regs::Group1Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group1_acr3_p(self) -> crate::common::Reg<regs::Group1Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group1_amr0_p(self) -> crate::common::Reg<regs::Group1Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group1_amr_b(self) -> crate::common::Reg<regs::Group1AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group1_amr1_p(self) -> crate::common::Reg<regs::Group1Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group1_amr2_p(self) -> crate::common::Reg<regs::Group1Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group1_amr3_p(self) -> crate::common::Reg<regs::Group1Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group2_acr0_p(self) -> crate::common::Reg<regs::Group2Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group2_acr_b(self) -> crate::common::Reg<regs::Group2AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group2_acr1_p(self) -> crate::common::Reg<regs::Group2Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group2_acr2_p(self) -> crate::common::Reg<regs::Group2Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group2_acr3_p(self) -> crate::common::Reg<regs::Group2Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group2_amr0_p(self) -> crate::common::Reg<regs::Group2Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group2_amr_b(self) -> crate::common::Reg<regs::Group2AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group2_amr1_p(self) -> crate::common::Reg<regs::Group2Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group2_amr2_p(self) -> crate::common::Reg<regs::Group2Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group2_amr3_p(self) -> crate::common::Reg<regs::Group2Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group3_acr0_p(self) -> crate::common::Reg<regs::Group3Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group3_acr_b(self) -> crate::common::Reg<regs::Group3AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group3_acr1_p(self) -> crate::common::Reg<regs::Group3Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group3_acr2_p(self) -> crate::common::Reg<regs::Group3Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group3_acr3_p(self) -> crate::common::Reg<regs::Group3Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group3_amr0_p(self) -> crate::common::Reg<regs::Group3Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group3_amr_b(self) -> crate::common::Reg<regs::Group3AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group3_amr1_p(self) -> crate::common::Reg<regs::Group3Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group3_amr2_p(self) -> crate::common::Reg<regs::Group3Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group3_amr3_p(self) -> crate::common::Reg<regs::Group3Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group4_acr0_p(self) -> crate::common::Reg<regs::Group4Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group4_acr_b(self) -> crate::common::Reg<regs::Group4AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group4_acr1_p(self) -> crate::common::Reg<regs::Group4Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group4_acr2_p(self) -> crate::common::Reg<regs::Group4Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group4_acr3_p(self) -> crate::common::Reg<regs::Group4Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group4_amr0_p(self) -> crate::common::Reg<regs::Group4Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group4_amr_b(self) -> crate::common::Reg<regs::Group4AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group4_amr1_p(self) -> crate::common::Reg<regs::Group4Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group4_amr2_p(self) -> crate::common::Reg<regs::Group4Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group4_amr3_p(self) -> crate::common::Reg<regs::Group4Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group5_acr0_p(self) -> crate::common::Reg<regs::Group5Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group5_acr_b(self) -> crate::common::Reg<regs::Group5AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group5_acr1_p(self) -> crate::common::Reg<regs::Group5Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group5_acr2_p(self) -> crate::common::Reg<regs::Group5Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group5_acr3_p(self) -> crate::common::Reg<regs::Group5Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group5_amr0_p(self) -> crate::common::Reg<regs::Group5Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group5_amr_b(self) -> crate::common::Reg<regs::Group5AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group5_amr1_p(self) -> crate::common::Reg<regs::Group5Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group5_amr2_p(self) -> crate::common::Reg<regs::Group5Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group5_amr3_p(self) -> crate::common::Reg<regs::Group5Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group6_acr0_p(self) -> crate::common::Reg<regs::Group6Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group6_acr_b(self) -> crate::common::Reg<regs::Group6AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group6_acr1_p(self) -> crate::common::Reg<regs::Group6Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group6_acr2_p(self) -> crate::common::Reg<regs::Group6Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group6_acr3_p(self) -> crate::common::Reg<regs::Group6Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group6_amr0_p(self) -> crate::common::Reg<regs::Group6Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group6_amr_b(self) -> crate::common::Reg<regs::Group6AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group6_amr1_p(self) -> crate::common::Reg<regs::Group6Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group6_amr2_p(self) -> crate::common::Reg<regs::Group6Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group6_amr3_p(self) -> crate::common::Reg<regs::Group6Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group7_acr0_p(self) -> crate::common::Reg<regs::Group7Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group7_acr_b(self) -> crate::common::Reg<regs::Group7AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group7_acr1_p(self) -> crate::common::Reg<regs::Group7Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group7_acr2_p(self) -> crate::common::Reg<regs::Group7Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group7_acr3_p(self) -> crate::common::Reg<regs::Group7Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group7_amr0_p(self) -> crate::common::Reg<regs::Group7Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group7_amr_b(self) -> crate::common::Reg<regs::Group7AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group7_amr1_p(self) -> crate::common::Reg<regs::Group7Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group7_amr2_p(self) -> crate::common::Reg<regs::Group7Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group7_amr3_p(self) -> crate::common::Reg<regs::Group7Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group8_acr0_p(self) -> crate::common::Reg<regs::Group8Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group8_acr_b(self) -> crate::common::Reg<regs::Group8AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group8_acr1_p(self) -> crate::common::Reg<regs::Group8Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group8_acr2_p(self) -> crate::common::Reg<regs::Group8Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group8_acr3_p(self) -> crate::common::Reg<regs::Group8Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group8_amr0_p(self) -> crate::common::Reg<regs::Group8Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group8_amr_b(self) -> crate::common::Reg<regs::Group8AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group8_amr1_p(self) -> crate::common::Reg<regs::Group8Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group8_amr2_p(self) -> crate::common::Reg<regs::Group8Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group8_amr3_p(self) -> crate::common::Reg<regs::Group8Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group9_acr0_p(self) -> crate::common::Reg<regs::Group9Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group9_acr_b(self) -> crate::common::Reg<regs::Group9AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group9_acr1_p(self) -> crate::common::Reg<regs::Group9Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group9_acr2_p(self) -> crate::common::Reg<regs::Group9Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group9_acr3_p(self) -> crate::common::Reg<regs::Group9Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group9_amr0_p(self) -> crate::common::Reg<regs::Group9Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group9_amr_b(self) -> crate::common::Reg<regs::Group9AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group9_amr1_p(self) -> crate::common::Reg<regs::Group9Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group9_amr2_p(self) -> crate::common::Reg<regs::Group9Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group9_amr3_p(self) -> crate::common::Reg<regs::Group9Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group10_acr0_p(self) -> crate::common::Reg<regs::Group10Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group10_acr_b(self) -> crate::common::Reg<regs::Group10AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group10_acr1_p(self) -> crate::common::Reg<regs::Group10Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group10_acr2_p(self) -> crate::common::Reg<regs::Group10Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group10_acr3_p(self) -> crate::common::Reg<regs::Group10Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group10_amr0_p(self) -> crate::common::Reg<regs::Group10Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group10_amr_b(self) -> crate::common::Reg<regs::Group10AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group10_amr1_p(self) -> crate::common::Reg<regs::Group10Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group10_amr2_p(self) -> crate::common::Reg<regs::Group10Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group10_amr3_p(self) -> crate::common::Reg<regs::Group10Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group11_acr0_p(self) -> crate::common::Reg<regs::Group11Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group11_acr_b(self) -> crate::common::Reg<regs::Group11AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group11_acr1_p(self) -> crate::common::Reg<regs::Group11Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group11_acr2_p(self) -> crate::common::Reg<regs::Group11Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group11_acr3_p(self) -> crate::common::Reg<regs::Group11Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group11_amr0_p(self) -> crate::common::Reg<regs::Group11Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group11_amr_b(self) -> crate::common::Reg<regs::Group11AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group11_amr1_p(self) -> crate::common::Reg<regs::Group11Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group11_amr2_p(self) -> crate::common::Reg<regs::Group11Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group11_amr3_p(self) -> crate::common::Reg<regs::Group11Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group12_acr0_p(self) -> crate::common::Reg<regs::Group12Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group12_acr_b(self) -> crate::common::Reg<regs::Group12AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group12_acr1_p(self) -> crate::common::Reg<regs::Group12Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group12_acr2_p(self) -> crate::common::Reg<regs::Group12Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group12_acr3_p(self) -> crate::common::Reg<regs::Group12Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group12_amr0_p(self) -> crate::common::Reg<regs::Group12Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group12_amr_b(self) -> crate::common::Reg<regs::Group12AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group12_amr1_p(self) -> crate::common::Reg<regs::Group12Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group12_amr2_p(self) -> crate::common::Reg<regs::Group12Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group12_amr3_p(self) -> crate::common::Reg<regs::Group12Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group13_acr0_p(self) -> crate::common::Reg<regs::Group13Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group13_acr_b(self) -> crate::common::Reg<regs::Group13AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group13_acr1_p(self) -> crate::common::Reg<regs::Group13Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group13_acr2_p(self) -> crate::common::Reg<regs::Group13Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group13_acr3_p(self) -> crate::common::Reg<regs::Group13Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group13_amr0_p(self) -> crate::common::Reg<regs::Group13Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group13_amr_b(self) -> crate::common::Reg<regs::Group13AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group13_amr1_p(self) -> crate::common::Reg<regs::Group13Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group13_amr2_p(self) -> crate::common::Reg<regs::Group13Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group13_amr3_p(self) -> crate::common::Reg<regs::Group13Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group14_acr0_p(self) -> crate::common::Reg<regs::Group14Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group14_acr_b(self) -> crate::common::Reg<regs::Group14AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group14_acr1_p(self) -> crate::common::Reg<regs::Group14Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group14_acr2_p(self) -> crate::common::Reg<regs::Group14Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group14_acr3_p(self) -> crate::common::Reg<regs::Group14Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group14_amr0_p(self) -> crate::common::Reg<regs::Group14Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group14_amr_b(self) -> crate::common::Reg<regs::Group14AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group14_amr1_p(self) -> crate::common::Reg<regs::Group14Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group14_amr2_p(self) -> crate::common::Reg<regs::Group14Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group14_amr3_p(self) -> crate::common::Reg<regs::Group14Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group15_acr0_p(self) -> crate::common::Reg<regs::Group15Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group15_acr_b(self) -> crate::common::Reg<regs::Group15AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group15_acr1_p(self) -> crate::common::Reg<regs::Group15Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group15_acr2_p(self) -> crate::common::Reg<regs::Group15Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group15_acr3_p(self) -> crate::common::Reg<regs::Group15Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group15_amr0_p(self) -> crate::common::Reg<regs::Group15Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group15_amr_b(self) -> crate::common::Reg<regs::Group15AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group15_amr1_p(self) -> crate::common::Reg<regs::Group15Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group15_amr2_p(self) -> crate::common::Reg<regs::Group15Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group15_amr3_p(self) -> crate::common::Reg<regs::Group15Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group16_acr0_p(self) -> crate::common::Reg<regs::Group16Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group16_acr_b(self) -> crate::common::Reg<regs::Group16AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group16_acr1_p(self) -> crate::common::Reg<regs::Group16Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group16_acr2_p(self) -> crate::common::Reg<regs::Group16Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group16_acr3_p(self) -> crate::common::Reg<regs::Group16Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group16_amr0_p(self) -> crate::common::Reg<regs::Group16Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group16_amr_b(self) -> crate::common::Reg<regs::Group16AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group16_amr1_p(self) -> crate::common::Reg<regs::Group16Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group16_amr2_p(self) -> crate::common::Reg<regs::Group16Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group16_amr3_p(self) -> crate::common::Reg<regs::Group16Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group17_acr0_p(self) -> crate::common::Reg<regs::Group17Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group17_acr_b(self) -> crate::common::Reg<regs::Group17AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group17_acr1_p(self) -> crate::common::Reg<regs::Group17Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group17_acr2_p(self) -> crate::common::Reg<regs::Group17Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group17_acr3_p(self) -> crate::common::Reg<regs::Group17Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group17_amr0_p(self) -> crate::common::Reg<regs::Group17Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group17_amr_b(self) -> crate::common::Reg<regs::Group17AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group17_amr1_p(self) -> crate::common::Reg<regs::Group17Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group17_amr2_p(self) -> crate::common::Reg<regs::Group17Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group17_amr3_p(self) -> crate::common::Reg<regs::Group17Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group18_acr0_p(self) -> crate::common::Reg<regs::Group18Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group18_acr_b(self) -> crate::common::Reg<regs::Group18AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group18_acr1_p(self) -> crate::common::Reg<regs::Group18Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group18_acr2_p(self) -> crate::common::Reg<regs::Group18Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group18_acr3_p(self) -> crate::common::Reg<regs::Group18Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group18_amr0_p(self) -> crate::common::Reg<regs::Group18Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group18_amr_b(self) -> crate::common::Reg<regs::Group18AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group18_amr1_p(self) -> crate::common::Reg<regs::Group18Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group18_amr2_p(self) -> crate::common::Reg<regs::Group18Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group18_amr3_p(self) -> crate::common::Reg<regs::Group18Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group19_acr0_p(self) -> crate::common::Reg<regs::Group19Acr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group19_acr_b(self) -> crate::common::Reg<regs::Group19AcrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group19_acr1_p(self) -> crate::common::Reg<regs::Group19Acr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group19_acr2_p(self) -> crate::common::Reg<regs::Group19Acr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group19_acr3_p(self) -> crate::common::Reg<regs::Group19Acr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group19_amr0_p(self) -> crate::common::Reg<regs::Group19Amr0P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group19_amr_b(self) -> crate::common::Reg<regs::Group19AmrB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group19_amr1_p(self) -> crate::common::Reg<regs::Group19Amr1P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ecusize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group19_amr2_p(self) -> crate::common::Reg<regs::Group19Amr2P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group19_amr3_p(self) -> crate::common::Reg<regs::Group19Amr3P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
}
pub mod regs;
