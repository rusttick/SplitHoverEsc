#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_cr_b: [u8; 0x04],
    _reserved_1_cmr: [u8; 0x04],
    sr: Sr,
    _reserved_3_ir: [u8; 0x04],
    _reserved_4_ier_p: [u8; 0x04],
    group0_amr_b: Group0AmrB,
    btr0: Btr0,
    btr1: Btr1,
    _reserved8: [u8; 0x08],
    txid0_b: Txid0B,
    _reserved_9_alc_p: [u8; 0x04],
    _reserved_10_ecc_p: [u8; 0x04],
    _reserved_11_ewlr_p: [u8; 0x04],
    _reserved_12_rxerr_p: [u8; 0x04],
    _reserved_13_txdr3_b: [u8; 0x04],
    _reserved_14_sff_p: [u8; 0x04],
    _reserved_15_txdr5_b: [u8; 0x04],
    _reserved_16_txdr6_b: [u8; 0x04],
    _reserved_17_txdr7_b: [u8; 0x04],
    _reserved_18_txdata1_p: [u8; 0x04],
    _reserved_19_txdata2_p: [u8; 0x04],
    _reserved_20_txdata3_p: [u8; 0x04],
    _reserved_21_txdata4_p: [u8; 0x04],
    txdata5_p: Txdata5P,
    txdata6_p: Txdata6P,
    txdata7_p: Txdata7P,
    txdata8_p: Txdata8P,
    txdata9_p: Txdata9P,
    _reserved27: [u8; 0x08],
    cdr: Cdr,
    afm0: Afm0,
    afm1: Afm1,
    afm2: Afm2,
    fga0: Fga0,
    fga1: Fga1,
    fga2: Fga2,
    _reserved_34_group1_acr: [u8; 0x04],
    group1_acr1_p: Group1Acr1P,
    group1_acr2_p: Group1Acr2P,
    group1_acr3_p: Group1Acr3P,
    _reserved_38_group1_amr: [u8; 0x04],
    group1_amr1_p: Group1Amr1P,
    group1_amr2_p: Group1Amr2P,
    group1_amr3_p: Group1Amr3P,
    _reserved_42_group2_acr: [u8; 0x04],
    group2_acr1_p: Group2Acr1P,
    group2_acr2_p: Group2Acr2P,
    group2_acr3_p: Group2Acr3P,
    _reserved_46_group2_amr: [u8; 0x04],
    group2_amr1_p: Group2Amr1P,
    group2_amr2_p: Group2Amr2P,
    group2_amr3_p: Group2Amr3P,
    _reserved_50_group3_acr: [u8; 0x04],
    group3_acr1_p: Group3Acr1P,
    group3_acr2_p: Group3Acr2P,
    group3_acr3_p: Group3Acr3P,
    _reserved_54_group3_amr: [u8; 0x04],
    group3_amr1_p: Group3Amr1P,
    group3_amr2_p: Group3Amr2P,
    group3_amr3_p: Group3Amr3P,
    _reserved_58_group4_acr: [u8; 0x04],
    group4_acr1_p: Group4Acr1P,
    group4_acr2_p: Group4Acr2P,
    group4_acr3_p: Group4Acr3P,
    _reserved_62_group4_amr: [u8; 0x04],
    group4_amr1_p: Group4Amr1P,
    group4_amr2_p: Group4Amr2P,
    group4_amr3_p: Group4Amr3P,
    _reserved_66_group5_acr: [u8; 0x04],
    group5_acr1_p: Group5Acr1P,
    group5_acr2_p: Group5Acr2P,
    group5_acr3_p: Group5Acr3P,
    _reserved_70_group5_amr: [u8; 0x04],
    group5_amr1_p: Group5Amr1P,
    group5_amr2_p: Group5Amr2P,
    group5_amr3_p: Group5Amr3P,
    _reserved_74_group6_acr: [u8; 0x04],
    group6_acr1_p: Group6Acr1P,
    group6_acr2_p: Group6Acr2P,
    group6_acr3_p: Group6Acr3P,
    _reserved_78_group6_amr: [u8; 0x04],
    group6_amr1_p: Group6Amr1P,
    group6_amr2_p: Group6Amr2P,
    group6_amr3_p: Group6Amr3P,
    _reserved_82_group7_acr: [u8; 0x04],
    group7_acr1_p: Group7Acr1P,
    group7_acr2_p: Group7Acr2P,
    group7_acr3_p: Group7Acr3P,
    _reserved_86_group7_amr: [u8; 0x04],
    group7_amr1_p: Group7Amr1P,
    group7_amr2_p: Group7Amr2P,
    group7_amr3_p: Group7Amr3P,
    _reserved_90_group8_acr: [u8; 0x04],
    group8_acr1_p: Group8Acr1P,
    group8_acr2_p: Group8Acr2P,
    group8_acr3_p: Group8Acr3P,
    _reserved_94_group8_amr: [u8; 0x04],
    group8_amr1_p: Group8Amr1P,
    group8_amr2_p: Group8Amr2P,
    group8_amr3_p: Group8Amr3P,
    _reserved_98_group9_acr: [u8; 0x04],
    group9_acr1_p: Group9Acr1P,
    group9_acr2_p: Group9Acr2P,
    group9_acr3_p: Group9Acr3P,
    _reserved_102_group9_amr: [u8; 0x04],
    group9_amr1_p: Group9Amr1P,
    group9_amr2_p: Group9Amr2P,
    group9_amr3_p: Group9Amr3P,
    _reserved_106_group10_acr: [u8; 0x04],
    group10_acr1_p: Group10Acr1P,
    group10_acr2_p: Group10Acr2P,
    group10_acr3_p: Group10Acr3P,
    _reserved_110_group10_amr: [u8; 0x04],
    group10_amr1_p: Group10Amr1P,
    group10_amr2_p: Group10Amr2P,
    group10_amr3_p: Group10Amr3P,
    _reserved_114_group11_acr: [u8; 0x04],
    group11_acr1_p: Group11Acr1P,
    group11_acr2_p: Group11Acr2P,
    group11_acr3_p: Group11Acr3P,
    _reserved_118_group11_amr: [u8; 0x04],
    group11_amr1_p: Group11Amr1P,
    group11_amr2_p: Group11Amr2P,
    group11_amr3_p: Group11Amr3P,
    _reserved_122_group12_acr: [u8; 0x04],
    group12_acr1_p: Group12Acr1P,
    group12_acr2_p: Group12Acr2P,
    group12_acr3_p: Group12Acr3P,
    _reserved_126_group12_amr: [u8; 0x04],
    group12_amr1_p: Group12Amr1P,
    group12_amr2_p: Group12Amr2P,
    group12_amr3_p: Group12Amr3P,
    _reserved_130_group13_acr: [u8; 0x04],
    group13_acr1_p: Group13Acr1P,
    group13_acr2_p: Group13Acr2P,
    group13_acr3_p: Group13Acr3P,
    _reserved_134_group13_amr: [u8; 0x04],
    group13_amr1_p: Group13Amr1P,
    group13_amr2_p: Group13Amr2P,
    group13_amr3_p: Group13Amr3P,
    _reserved_138_group14_acr: [u8; 0x04],
    group14_acr1_p: Group14Acr1P,
    group14_acr2_p: Group14Acr2P,
    group14_acr3_p: Group14Acr3P,
    _reserved_142_group14_amr: [u8; 0x04],
    group14_amr1_p: Group14Amr1P,
    group14_amr2_p: Group14Amr2P,
    group14_amr3_p: Group14Amr3P,
    _reserved_146_group15_acr: [u8; 0x04],
    group15_acr1_p: Group15Acr1P,
    group15_acr2_p: Group15Acr2P,
    group15_acr3_p: Group15Acr3P,
    _reserved_150_group15_amr: [u8; 0x04],
    group15_amr1_p: Group15Amr1P,
    group15_amr2_p: Group15Amr2P,
    group15_amr3_p: Group15Amr3P,
    _reserved_154_group16_acr: [u8; 0x04],
    group16_acr1_p: Group16Acr1P,
    group16_acr2_p: Group16Acr2P,
    group16_acr3_p: Group16Acr3P,
    _reserved_158_group16_amr: [u8; 0x04],
    group16_amr1_p: Group16Amr1P,
    group16_amr2_p: Group16Amr2P,
    group16_amr3_p: Group16Amr3P,
    _reserved_162_group17_acr: [u8; 0x04],
    group17_acr1_p: Group17Acr1P,
    group17_acr2_p: Group17Acr2P,
    group17_acr3_p: Group17Acr3P,
    _reserved_166_group17_amr: [u8; 0x04],
    group17_amr1_p: Group17Amr1P,
    group17_amr2_p: Group17Amr2P,
    group17_amr3_p: Group17Amr3P,
    _reserved_170_group18_acr: [u8; 0x04],
    group18_acr1_p: Group18Acr1P,
    group18_acr2_p: Group18Acr2P,
    group18_acr3_p: Group18Acr3P,
    _reserved_174_group18_amr: [u8; 0x04],
    group18_amr1_p: Group18Amr1P,
    group18_amr2_p: Group18Amr2P,
    group18_amr3_p: Group18Amr3P,
    _reserved_178_group19_acr: [u8; 0x04],
    group19_acr1_p: Group19Acr1P,
    group19_acr2_p: Group19Acr2P,
    group19_acr3_p: Group19Acr3P,
    _reserved_182_group19_amr: [u8; 0x04],
    group19_amr1_p: Group19Amr1P,
    group19_amr2_p: Group19Amr2P,
    group19_amr3_p: Group19Amr3P,
}
impl RegisterBlock {
    #[doc = "0x00 - Peli Mode register"]
    #[inline(always)]
    pub const fn mod_p(&self) -> &ModP {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Basic control register"]
    #[inline(always)]
    pub const fn cr_b(&self) -> &CrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Peli Command register"]
    #[inline(always)]
    pub const fn cmr_p(&self) -> &CmrP {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Basic Command register"]
    #[inline(always)]
    pub const fn cmr_b(&self) -> &CmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - Interrupt register"]
    #[inline(always)]
    pub const fn ir_p(&self) -> &IrP {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Interrupt register"]
    #[inline(always)]
    pub const fn ir_b(&self) -> &IrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - Peli Interrupt Enable register"]
    #[inline(always)]
    pub const fn ier_p(&self) -> &IerP {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group0_acr_b(&self) -> &Group0AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group0_amr_b(&self) -> &Group0AmrB {
        &self.group0_amr_b
    }
    #[doc = "0x18 - Bus Timing register 0"]
    #[inline(always)]
    pub const fn btr0(&self) -> &Btr0 {
        &self.btr0
    }
    #[doc = "0x1c - Bus Timing register 1"]
    #[inline(always)]
    pub const fn btr1(&self) -> &Btr1 {
        &self.btr1
    }
    #[doc = "0x28 - Basic TX ID register 0"]
    #[inline(always)]
    pub const fn txid0_b(&self) -> &Txid0B {
        &self.txid0_b
    }
    #[doc = "0x2c - Peli Arbitration Lost Capture register"]
    #[inline(always)]
    pub const fn alc_p(&self) -> &AlcP {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - Basic TX ID register 1"]
    #[inline(always)]
    pub const fn txid1_b(&self) -> &Txid1B {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x30 - Peli Error Code Capture register"]
    #[inline(always)]
    pub const fn ecc_p(&self) -> &EccP {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - Basic TX Data register 0"]
    #[inline(always)]
    pub const fn txdr0_b(&self) -> &Txdr0B {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x34 - Peli Error Warning Limit register"]
    #[inline(always)]
    pub const fn ewlr_p(&self) -> &EwlrP {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - Basic TX Data register 1"]
    #[inline(always)]
    pub const fn txdr1_b(&self) -> &Txdr1B {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x38 - Peli RX Error Counter register"]
    #[inline(always)]
    pub const fn rxerr_p(&self) -> &RxerrP {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - Basic Send Data register 2"]
    #[inline(always)]
    pub const fn txdr2_b(&self) -> &Txdr2B {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x3c - Peli TX Error Counter register"]
    #[inline(always)]
    pub const fn txerr_p(&self) -> &TxerrP {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Basic TX Data register 3"]
    #[inline(always)]
    pub const fn txdr3_b(&self) -> &Txdr3B {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x40 - Peli Acceptance Code register 0"]
    #[inline(always)]
    pub const fn group0_acr0_p(&self) -> &Group0Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - Peli Send Frame Format register"]
    #[inline(always)]
    pub const fn sff_p(&self) -> &SffP {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - Basic TX Data register 4"]
    #[inline(always)]
    pub const fn txdr4_b(&self) -> &Txdr4B {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x44 - Peli Acceptance Code register 1"]
    #[inline(always)]
    pub const fn group0_acr1_p(&self) -> &Group0Acr1P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - Peli TX ID register 0"]
    #[inline(always)]
    pub const fn txid0_p(&self) -> &Txid0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - Basic TX Data register 5"]
    #[inline(always)]
    pub const fn txdr5_b(&self) -> &Txdr5B {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x48 - Peli Acceptance Code register 2"]
    #[inline(always)]
    pub const fn group0_acr2_p(&self) -> &Group0Acr2P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - Peli TX ID register 1"]
    #[inline(always)]
    pub const fn txid1_p(&self) -> &Txid1P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - Basic TX Data register 6"]
    #[inline(always)]
    pub const fn txdr6_b(&self) -> &Txdr6B {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x4c - Peli Acceptance Code register 3"]
    #[inline(always)]
    pub const fn group0_acr3_p(&self) -> &Group0Acr3P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - Peli TX Data register 0"]
    #[inline(always)]
    pub const fn txdata0_p(&self) -> &Txdata0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - Basic TX Data register 7"]
    #[inline(always)]
    pub const fn txdr7_b(&self) -> &Txdr7B {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x50 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group0_amr0_p(&self) -> &Group0Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x50 - Peli TX Data register 1"]
    #[inline(always)]
    pub const fn txdata1_p(&self) -> &Txdata1P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x54 - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group0_amr1_p(&self) -> &Group0Amr1P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x54 - Peli TX Data register 2"]
    #[inline(always)]
    pub const fn txdata2_p(&self) -> &Txdata2P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x58 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group0_amr2_p(&self) -> &Group0Amr2P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x58 - Peli TX Data register 3"]
    #[inline(always)]
    pub const fn txdata3_p(&self) -> &Txdata3P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x5c - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group0_amr3_p(&self) -> &Group0Amr3P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x5c - Peli TX Data register 4"]
    #[inline(always)]
    pub const fn txdata4_p(&self) -> &Txdata4P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x60 - Peli TX Data register 5"]
    #[inline(always)]
    pub const fn txdata5_p(&self) -> &Txdata5P {
        &self.txdata5_p
    }
    #[doc = "0x64 - Peli TX Data register 6"]
    #[inline(always)]
    pub const fn txdata6_p(&self) -> &Txdata6P {
        &self.txdata6_p
    }
    #[doc = "0x68 - Peli TX Data register 7"]
    #[inline(always)]
    pub const fn txdata7_p(&self) -> &Txdata7P {
        &self.txdata7_p
    }
    #[doc = "0x6c - Peli TX Data register 8"]
    #[inline(always)]
    pub const fn txdata8_p(&self) -> &Txdata8P {
        &self.txdata8_p
    }
    #[doc = "0x70 - Peli TX Data register 9"]
    #[inline(always)]
    pub const fn txdata9_p(&self) -> &Txdata9P {
        &self.txdata9_p
    }
    #[doc = "0x7c - Clock Divider register"]
    #[inline(always)]
    pub const fn cdr(&self) -> &Cdr {
        &self.cdr
    }
    #[doc = "0x80 - Filter Mode register 0"]
    #[inline(always)]
    pub const fn afm0(&self) -> &Afm0 {
        &self.afm0
    }
    #[doc = "0x84 - Filter Mode register 1"]
    #[inline(always)]
    pub const fn afm1(&self) -> &Afm1 {
        &self.afm1
    }
    #[doc = "0x88 - Filter Mode register 2"]
    #[inline(always)]
    pub const fn afm2(&self) -> &Afm2 {
        &self.afm2
    }
    #[doc = "0x8c - Filter Group Enable Register 0"]
    #[inline(always)]
    pub const fn fga0(&self) -> &Fga0 {
        &self.fga0
    }
    #[doc = "0x90 - Filter Group Enable Register 1"]
    #[inline(always)]
    pub const fn fga1(&self) -> &Fga1 {
        &self.fga1
    }
    #[doc = "0x94 - Filter Group Enable Register 2"]
    #[inline(always)]
    pub const fn fga2(&self) -> &Fga2 {
        &self.fga2
    }
    #[doc = "0x98 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group1_acr0_p(&self) -> &Group1Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x98 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group1_acr_b(&self) -> &Group1AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x9c - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group1_acr1_p(&self) -> &Group1Acr1P {
        &self.group1_acr1_p
    }
    #[doc = "0xa0 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group1_acr2_p(&self) -> &Group1Acr2P {
        &self.group1_acr2_p
    }
    #[doc = "0xa4 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group1_acr3_p(&self) -> &Group1Acr3P {
        &self.group1_acr3_p
    }
    #[doc = "0xa8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group1_amr0_p(&self) -> &Group1Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    #[doc = "0xa8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group1_amr_b(&self) -> &Group1AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    #[doc = "0xac - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group1_amr1_p(&self) -> &Group1Amr1P {
        &self.group1_amr1_p
    }
    #[doc = "0xb0 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group1_amr2_p(&self) -> &Group1Amr2P {
        &self.group1_amr2_p
    }
    #[doc = "0xb4 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group1_amr3_p(&self) -> &Group1Amr3P {
        &self.group1_amr3_p
    }
    #[doc = "0xb8 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group2_acr0_p(&self) -> &Group2Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(184).cast() }
    }
    #[doc = "0xb8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group2_acr_b(&self) -> &Group2AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(184).cast() }
    }
    #[doc = "0xbc - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group2_acr1_p(&self) -> &Group2Acr1P {
        &self.group2_acr1_p
    }
    #[doc = "0xc0 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group2_acr2_p(&self) -> &Group2Acr2P {
        &self.group2_acr2_p
    }
    #[doc = "0xc4 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group2_acr3_p(&self) -> &Group2Acr3P {
        &self.group2_acr3_p
    }
    #[doc = "0xc8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group2_amr0_p(&self) -> &Group2Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xc8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group2_amr_b(&self) -> &Group2AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xcc - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group2_amr1_p(&self) -> &Group2Amr1P {
        &self.group2_amr1_p
    }
    #[doc = "0xd0 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group2_amr2_p(&self) -> &Group2Amr2P {
        &self.group2_amr2_p
    }
    #[doc = "0xd4 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group2_amr3_p(&self) -> &Group2Amr3P {
        &self.group2_amr3_p
    }
    #[doc = "0xd8 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group3_acr0_p(&self) -> &Group3Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(216).cast() }
    }
    #[doc = "0xd8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group3_acr_b(&self) -> &Group3AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(216).cast() }
    }
    #[doc = "0xdc - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group3_acr1_p(&self) -> &Group3Acr1P {
        &self.group3_acr1_p
    }
    #[doc = "0xe0 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group3_acr2_p(&self) -> &Group3Acr2P {
        &self.group3_acr2_p
    }
    #[doc = "0xe4 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group3_acr3_p(&self) -> &Group3Acr3P {
        &self.group3_acr3_p
    }
    #[doc = "0xe8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group3_amr0_p(&self) -> &Group3Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    #[doc = "0xe8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group3_amr_b(&self) -> &Group3AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    #[doc = "0xec - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group3_amr1_p(&self) -> &Group3Amr1P {
        &self.group3_amr1_p
    }
    #[doc = "0xf0 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group3_amr2_p(&self) -> &Group3Amr2P {
        &self.group3_amr2_p
    }
    #[doc = "0xf4 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group3_amr3_p(&self) -> &Group3Amr3P {
        &self.group3_amr3_p
    }
    #[doc = "0xf8 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group4_acr0_p(&self) -> &Group4Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(248).cast() }
    }
    #[doc = "0xf8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group4_acr_b(&self) -> &Group4AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(248).cast() }
    }
    #[doc = "0xfc - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group4_acr1_p(&self) -> &Group4Acr1P {
        &self.group4_acr1_p
    }
    #[doc = "0x100 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group4_acr2_p(&self) -> &Group4Acr2P {
        &self.group4_acr2_p
    }
    #[doc = "0x104 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group4_acr3_p(&self) -> &Group4Acr3P {
        &self.group4_acr3_p
    }
    #[doc = "0x108 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group4_amr0_p(&self) -> &Group4Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x108 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group4_amr_b(&self) -> &Group4AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x10c - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group4_amr1_p(&self) -> &Group4Amr1P {
        &self.group4_amr1_p
    }
    #[doc = "0x110 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group4_amr2_p(&self) -> &Group4Amr2P {
        &self.group4_amr2_p
    }
    #[doc = "0x114 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group4_amr3_p(&self) -> &Group4Amr3P {
        &self.group4_amr3_p
    }
    #[doc = "0x118 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group5_acr0_p(&self) -> &Group5Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x118 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group5_acr_b(&self) -> &Group5AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x11c - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group5_acr1_p(&self) -> &Group5Acr1P {
        &self.group5_acr1_p
    }
    #[doc = "0x120 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group5_acr2_p(&self) -> &Group5Acr2P {
        &self.group5_acr2_p
    }
    #[doc = "0x124 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group5_acr3_p(&self) -> &Group5Acr3P {
        &self.group5_acr3_p
    }
    #[doc = "0x128 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group5_amr0_p(&self) -> &Group5Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x128 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group5_amr_b(&self) -> &Group5AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12c - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group5_amr1_p(&self) -> &Group5Amr1P {
        &self.group5_amr1_p
    }
    #[doc = "0x130 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group5_amr2_p(&self) -> &Group5Amr2P {
        &self.group5_amr2_p
    }
    #[doc = "0x134 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group5_amr3_p(&self) -> &Group5Amr3P {
        &self.group5_amr3_p
    }
    #[doc = "0x138 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group6_acr0_p(&self) -> &Group6Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x138 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group6_acr_b(&self) -> &Group6AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x13c - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group6_acr1_p(&self) -> &Group6Acr1P {
        &self.group6_acr1_p
    }
    #[doc = "0x140 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group6_acr2_p(&self) -> &Group6Acr2P {
        &self.group6_acr2_p
    }
    #[doc = "0x144 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group6_acr3_p(&self) -> &Group6Acr3P {
        &self.group6_acr3_p
    }
    #[doc = "0x148 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group6_amr0_p(&self) -> &Group6Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x148 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group6_amr_b(&self) -> &Group6AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x14c - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group6_amr1_p(&self) -> &Group6Amr1P {
        &self.group6_amr1_p
    }
    #[doc = "0x150 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group6_amr2_p(&self) -> &Group6Amr2P {
        &self.group6_amr2_p
    }
    #[doc = "0x154 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group6_amr3_p(&self) -> &Group6Amr3P {
        &self.group6_amr3_p
    }
    #[doc = "0x158 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group7_acr0_p(&self) -> &Group7Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x158 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group7_acr_b(&self) -> &Group7AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x15c - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group7_acr1_p(&self) -> &Group7Acr1P {
        &self.group7_acr1_p
    }
    #[doc = "0x160 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group7_acr2_p(&self) -> &Group7Acr2P {
        &self.group7_acr2_p
    }
    #[doc = "0x164 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group7_acr3_p(&self) -> &Group7Acr3P {
        &self.group7_acr3_p
    }
    #[doc = "0x168 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group7_amr0_p(&self) -> &Group7Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(360).cast() }
    }
    #[doc = "0x168 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group7_amr_b(&self) -> &Group7AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(360).cast() }
    }
    #[doc = "0x16c - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group7_amr1_p(&self) -> &Group7Amr1P {
        &self.group7_amr1_p
    }
    #[doc = "0x170 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group7_amr2_p(&self) -> &Group7Amr2P {
        &self.group7_amr2_p
    }
    #[doc = "0x174 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group7_amr3_p(&self) -> &Group7Amr3P {
        &self.group7_amr3_p
    }
    #[doc = "0x178 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group8_acr0_p(&self) -> &Group8Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(376).cast() }
    }
    #[doc = "0x178 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group8_acr_b(&self) -> &Group8AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(376).cast() }
    }
    #[doc = "0x17c - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group8_acr1_p(&self) -> &Group8Acr1P {
        &self.group8_acr1_p
    }
    #[doc = "0x180 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group8_acr2_p(&self) -> &Group8Acr2P {
        &self.group8_acr2_p
    }
    #[doc = "0x184 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group8_acr3_p(&self) -> &Group8Acr3P {
        &self.group8_acr3_p
    }
    #[doc = "0x188 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group8_amr0_p(&self) -> &Group8Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(392).cast() }
    }
    #[doc = "0x188 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group8_amr_b(&self) -> &Group8AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(392).cast() }
    }
    #[doc = "0x18c - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group8_amr1_p(&self) -> &Group8Amr1P {
        &self.group8_amr1_p
    }
    #[doc = "0x190 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group8_amr2_p(&self) -> &Group8Amr2P {
        &self.group8_amr2_p
    }
    #[doc = "0x194 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group8_amr3_p(&self) -> &Group8Amr3P {
        &self.group8_amr3_p
    }
    #[doc = "0x198 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group9_acr0_p(&self) -> &Group9Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x198 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group9_acr_b(&self) -> &Group9AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x19c - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group9_acr1_p(&self) -> &Group9Acr1P {
        &self.group9_acr1_p
    }
    #[doc = "0x1a0 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group9_acr2_p(&self) -> &Group9Acr2P {
        &self.group9_acr2_p
    }
    #[doc = "0x1a4 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group9_acr3_p(&self) -> &Group9Acr3P {
        &self.group9_acr3_p
    }
    #[doc = "0x1a8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group9_amr0_p(&self) -> &Group9Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1a8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group9_amr_b(&self) -> &Group9AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1ac - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group9_amr1_p(&self) -> &Group9Amr1P {
        &self.group9_amr1_p
    }
    #[doc = "0x1b0 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group9_amr2_p(&self) -> &Group9Amr2P {
        &self.group9_amr2_p
    }
    #[doc = "0x1b4 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group9_amr3_p(&self) -> &Group9Amr3P {
        &self.group9_amr3_p
    }
    #[doc = "0x1b8 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group10_acr0_p(&self) -> &Group10Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(440).cast() }
    }
    #[doc = "0x1b8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group10_acr_b(&self) -> &Group10AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(440).cast() }
    }
    #[doc = "0x1bc - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group10_acr1_p(&self) -> &Group10Acr1P {
        &self.group10_acr1_p
    }
    #[doc = "0x1c0 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group10_acr2_p(&self) -> &Group10Acr2P {
        &self.group10_acr2_p
    }
    #[doc = "0x1c4 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group10_acr3_p(&self) -> &Group10Acr3P {
        &self.group10_acr3_p
    }
    #[doc = "0x1c8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group10_amr0_p(&self) -> &Group10Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(456).cast() }
    }
    #[doc = "0x1c8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group10_amr_b(&self) -> &Group10AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(456).cast() }
    }
    #[doc = "0x1cc - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group10_amr1_p(&self) -> &Group10Amr1P {
        &self.group10_amr1_p
    }
    #[doc = "0x1d0 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group10_amr2_p(&self) -> &Group10Amr2P {
        &self.group10_amr2_p
    }
    #[doc = "0x1d4 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group10_amr3_p(&self) -> &Group10Amr3P {
        &self.group10_amr3_p
    }
    #[doc = "0x1d8 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group11_acr0_p(&self) -> &Group11Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(472).cast() }
    }
    #[doc = "0x1d8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group11_acr_b(&self) -> &Group11AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(472).cast() }
    }
    #[doc = "0x1dc - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group11_acr1_p(&self) -> &Group11Acr1P {
        &self.group11_acr1_p
    }
    #[doc = "0x1e0 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group11_acr2_p(&self) -> &Group11Acr2P {
        &self.group11_acr2_p
    }
    #[doc = "0x1e4 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group11_acr3_p(&self) -> &Group11Acr3P {
        &self.group11_acr3_p
    }
    #[doc = "0x1e8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group11_amr0_p(&self) -> &Group11Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(488).cast() }
    }
    #[doc = "0x1e8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group11_amr_b(&self) -> &Group11AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(488).cast() }
    }
    #[doc = "0x1ec - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group11_amr1_p(&self) -> &Group11Amr1P {
        &self.group11_amr1_p
    }
    #[doc = "0x1f0 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group11_amr2_p(&self) -> &Group11Amr2P {
        &self.group11_amr2_p
    }
    #[doc = "0x1f4 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group11_amr3_p(&self) -> &Group11Amr3P {
        &self.group11_amr3_p
    }
    #[doc = "0x1f8 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group12_acr0_p(&self) -> &Group12Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(504).cast() }
    }
    #[doc = "0x1f8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group12_acr_b(&self) -> &Group12AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(504).cast() }
    }
    #[doc = "0x1fc - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group12_acr1_p(&self) -> &Group12Acr1P {
        &self.group12_acr1_p
    }
    #[doc = "0x200 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group12_acr2_p(&self) -> &Group12Acr2P {
        &self.group12_acr2_p
    }
    #[doc = "0x204 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group12_acr3_p(&self) -> &Group12Acr3P {
        &self.group12_acr3_p
    }
    #[doc = "0x208 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group12_amr0_p(&self) -> &Group12Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x208 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group12_amr_b(&self) -> &Group12AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x20c - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group12_amr1_p(&self) -> &Group12Amr1P {
        &self.group12_amr1_p
    }
    #[doc = "0x210 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group12_amr2_p(&self) -> &Group12Amr2P {
        &self.group12_amr2_p
    }
    #[doc = "0x214 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group12_amr3_p(&self) -> &Group12Amr3P {
        &self.group12_amr3_p
    }
    #[doc = "0x218 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group13_acr0_p(&self) -> &Group13Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x218 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group13_acr_b(&self) -> &Group13AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x21c - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group13_acr1_p(&self) -> &Group13Acr1P {
        &self.group13_acr1_p
    }
    #[doc = "0x220 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group13_acr2_p(&self) -> &Group13Acr2P {
        &self.group13_acr2_p
    }
    #[doc = "0x224 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group13_acr3_p(&self) -> &Group13Acr3P {
        &self.group13_acr3_p
    }
    #[doc = "0x228 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group13_amr0_p(&self) -> &Group13Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x228 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group13_amr_b(&self) -> &Group13AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x22c - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group13_amr1_p(&self) -> &Group13Amr1P {
        &self.group13_amr1_p
    }
    #[doc = "0x230 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group13_amr2_p(&self) -> &Group13Amr2P {
        &self.group13_amr2_p
    }
    #[doc = "0x234 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group13_amr3_p(&self) -> &Group13Amr3P {
        &self.group13_amr3_p
    }
    #[doc = "0x238 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group14_acr0_p(&self) -> &Group14Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x238 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group14_acr_b(&self) -> &Group14AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x23c - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group14_acr1_p(&self) -> &Group14Acr1P {
        &self.group14_acr1_p
    }
    #[doc = "0x240 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group14_acr2_p(&self) -> &Group14Acr2P {
        &self.group14_acr2_p
    }
    #[doc = "0x244 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group14_acr3_p(&self) -> &Group14Acr3P {
        &self.group14_acr3_p
    }
    #[doc = "0x248 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group14_amr0_p(&self) -> &Group14Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(584).cast() }
    }
    #[doc = "0x248 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group14_amr_b(&self) -> &Group14AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(584).cast() }
    }
    #[doc = "0x24c - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group14_amr1_p(&self) -> &Group14Amr1P {
        &self.group14_amr1_p
    }
    #[doc = "0x250 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group14_amr2_p(&self) -> &Group14Amr2P {
        &self.group14_amr2_p
    }
    #[doc = "0x254 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group14_amr3_p(&self) -> &Group14Amr3P {
        &self.group14_amr3_p
    }
    #[doc = "0x258 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group15_acr0_p(&self) -> &Group15Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(600).cast() }
    }
    #[doc = "0x258 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group15_acr_b(&self) -> &Group15AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(600).cast() }
    }
    #[doc = "0x25c - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group15_acr1_p(&self) -> &Group15Acr1P {
        &self.group15_acr1_p
    }
    #[doc = "0x260 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group15_acr2_p(&self) -> &Group15Acr2P {
        &self.group15_acr2_p
    }
    #[doc = "0x264 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group15_acr3_p(&self) -> &Group15Acr3P {
        &self.group15_acr3_p
    }
    #[doc = "0x268 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group15_amr0_p(&self) -> &Group15Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(616).cast() }
    }
    #[doc = "0x268 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group15_amr_b(&self) -> &Group15AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(616).cast() }
    }
    #[doc = "0x26c - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group15_amr1_p(&self) -> &Group15Amr1P {
        &self.group15_amr1_p
    }
    #[doc = "0x270 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group15_amr2_p(&self) -> &Group15Amr2P {
        &self.group15_amr2_p
    }
    #[doc = "0x274 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group15_amr3_p(&self) -> &Group15Amr3P {
        &self.group15_amr3_p
    }
    #[doc = "0x278 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group16_acr0_p(&self) -> &Group16Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    #[doc = "0x278 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group16_acr_b(&self) -> &Group16AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    #[doc = "0x27c - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group16_acr1_p(&self) -> &Group16Acr1P {
        &self.group16_acr1_p
    }
    #[doc = "0x280 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group16_acr2_p(&self) -> &Group16Acr2P {
        &self.group16_acr2_p
    }
    #[doc = "0x284 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group16_acr3_p(&self) -> &Group16Acr3P {
        &self.group16_acr3_p
    }
    #[doc = "0x288 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group16_amr0_p(&self) -> &Group16Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(648).cast() }
    }
    #[doc = "0x288 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group16_amr_b(&self) -> &Group16AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(648).cast() }
    }
    #[doc = "0x28c - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group16_amr1_p(&self) -> &Group16Amr1P {
        &self.group16_amr1_p
    }
    #[doc = "0x290 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group16_amr2_p(&self) -> &Group16Amr2P {
        &self.group16_amr2_p
    }
    #[doc = "0x294 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group16_amr3_p(&self) -> &Group16Amr3P {
        &self.group16_amr3_p
    }
    #[doc = "0x298 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group17_acr0_p(&self) -> &Group17Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(664).cast() }
    }
    #[doc = "0x298 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group17_acr_b(&self) -> &Group17AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(664).cast() }
    }
    #[doc = "0x29c - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group17_acr1_p(&self) -> &Group17Acr1P {
        &self.group17_acr1_p
    }
    #[doc = "0x2a0 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group17_acr2_p(&self) -> &Group17Acr2P {
        &self.group17_acr2_p
    }
    #[doc = "0x2a4 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group17_acr3_p(&self) -> &Group17Acr3P {
        &self.group17_acr3_p
    }
    #[doc = "0x2a8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group17_amr0_p(&self) -> &Group17Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(680).cast() }
    }
    #[doc = "0x2a8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group17_amr_b(&self) -> &Group17AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(680).cast() }
    }
    #[doc = "0x2ac - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group17_amr1_p(&self) -> &Group17Amr1P {
        &self.group17_amr1_p
    }
    #[doc = "0x2b0 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group17_amr2_p(&self) -> &Group17Amr2P {
        &self.group17_amr2_p
    }
    #[doc = "0x2b4 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group17_amr3_p(&self) -> &Group17Amr3P {
        &self.group17_amr3_p
    }
    #[doc = "0x2b8 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group18_acr0_p(&self) -> &Group18Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(696).cast() }
    }
    #[doc = "0x2b8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group18_acr_b(&self) -> &Group18AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(696).cast() }
    }
    #[doc = "0x2bc - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group18_acr1_p(&self) -> &Group18Acr1P {
        &self.group18_acr1_p
    }
    #[doc = "0x2c0 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group18_acr2_p(&self) -> &Group18Acr2P {
        &self.group18_acr2_p
    }
    #[doc = "0x2c4 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group18_acr3_p(&self) -> &Group18Acr3P {
        &self.group18_acr3_p
    }
    #[doc = "0x2c8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group18_amr0_p(&self) -> &Group18Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(712).cast() }
    }
    #[doc = "0x2c8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group18_amr_b(&self) -> &Group18AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(712).cast() }
    }
    #[doc = "0x2cc - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group18_amr1_p(&self) -> &Group18Amr1P {
        &self.group18_amr1_p
    }
    #[doc = "0x2d0 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group18_amr2_p(&self) -> &Group18Amr2P {
        &self.group18_amr2_p
    }
    #[doc = "0x2d4 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group18_amr3_p(&self) -> &Group18Amr3P {
        &self.group18_amr3_p
    }
    #[doc = "0x2d8 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group19_acr0_p(&self) -> &Group19Acr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(728).cast() }
    }
    #[doc = "0x2d8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group19_acr_b(&self) -> &Group19AcrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(728).cast() }
    }
    #[doc = "0x2dc - Peli Acceptance Code register1"]
    #[inline(always)]
    pub const fn group19_acr1_p(&self) -> &Group19Acr1P {
        &self.group19_acr1_p
    }
    #[doc = "0x2e0 - Peli Acceptance Code register2"]
    #[inline(always)]
    pub const fn group19_acr2_p(&self) -> &Group19Acr2P {
        &self.group19_acr2_p
    }
    #[doc = "0x2e4 - Peli Acceptance Code register3"]
    #[inline(always)]
    pub const fn group19_acr3_p(&self) -> &Group19Acr3P {
        &self.group19_acr3_p
    }
    #[doc = "0x2e8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group19_amr0_p(&self) -> &Group19Amr0P {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(744).cast() }
    }
    #[doc = "0x2e8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group19_amr_b(&self) -> &Group19AmrB {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(744).cast() }
    }
    #[doc = "0x2ec - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group19_amr1_p(&self) -> &Group19Amr1P {
        &self.group19_amr1_p
    }
    #[doc = "0x2f0 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group19_amr2_p(&self) -> &Group19Amr2P {
        &self.group19_amr2_p
    }
    #[doc = "0x2f4 - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group19_amr3_p(&self) -> &Group19Amr3P {
        &self.group19_amr3_p
    }
}
#[doc = "CR_B (rw) register accessor: Basic control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_b`] module"]
#[doc(alias = "CR_B")]
pub type CrB = crate::Reg<cr_b::CrBSpec>;
#[doc = "Basic control register"]
pub mod cr_b;
#[doc = "MOD_P (rw) register accessor: Peli Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mod_p`] module"]
#[doc(alias = "MOD_P")]
pub type ModP = crate::Reg<mod_p::ModPSpec>;
#[doc = "Peli Mode register"]
pub mod mod_p;
#[doc = "CMR_B (w) register accessor: Basic Command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr_b::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr_b`] module"]
#[doc(alias = "CMR_B")]
pub type CmrB = crate::Reg<cmr_b::CmrBSpec>;
#[doc = "Basic Command register"]
pub mod cmr_b;
#[doc = "CMR_P (w) register accessor: Peli Command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr_p::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr_p`] module"]
#[doc(alias = "CMR_P")]
pub type CmrP = crate::Reg<cmr_p::CmrPSpec>;
#[doc = "Peli Command register"]
pub mod cmr_p;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IR_B (r) register accessor: Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_b::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir_b`] module"]
#[doc(alias = "IR_B")]
pub type IrB = crate::Reg<ir_b::IrBSpec>;
#[doc = "Interrupt register"]
pub mod ir_b;
#[doc = "IR_P (r) register accessor: Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_p::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir_p`] module"]
#[doc(alias = "IR_P")]
pub type IrP = crate::Reg<ir_p::IrPSpec>;
#[doc = "Interrupt register"]
pub mod ir_p;
#[doc = "GROUP0_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group0_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group0_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group0_acr_b`] module"]
#[doc(alias = "GROUP0_ACR_B")]
pub type Group0AcrB = crate::Reg<group0_acr_b::Group0AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group0_acr_b;
#[doc = "IER_P (rw) register accessor: Peli Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier_p`] module"]
#[doc(alias = "IER_P")]
pub type IerP = crate::Reg<ier_p::IerPSpec>;
#[doc = "Peli Interrupt Enable register"]
pub mod ier_p;
#[doc = "GROUP0_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group0_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group0_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group0_amr_b`] module"]
#[doc(alias = "GROUP0_AMR_B")]
pub type Group0AmrB = crate::Reg<group0_amr_b::Group0AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group0_amr_b;
#[doc = "BTR0 (rw) register accessor: Bus Timing register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`btr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr0`] module"]
#[doc(alias = "BTR0")]
pub type Btr0 = crate::Reg<btr0::Btr0Spec>;
#[doc = "Bus Timing register 0"]
pub mod btr0;
#[doc = "BTR1 (rw) register accessor: Bus Timing register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`btr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr1`] module"]
#[doc(alias = "BTR1")]
pub type Btr1 = crate::Reg<btr1::Btr1Spec>;
#[doc = "Bus Timing register 1"]
pub mod btr1;
#[doc = "TXID0_B (rw) register accessor: Basic TX ID register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`txid0_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txid0_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txid0_b`] module"]
#[doc(alias = "TXID0_B")]
pub type Txid0B = crate::Reg<txid0_b::Txid0BSpec>;
#[doc = "Basic TX ID register 0"]
pub mod txid0_b;
#[doc = "TXID1_B (rw) register accessor: Basic TX ID register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txid1_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txid1_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txid1_b`] module"]
#[doc(alias = "TXID1_B")]
pub type Txid1B = crate::Reg<txid1_b::Txid1BSpec>;
#[doc = "Basic TX ID register 1"]
pub mod txid1_b;
#[doc = "ALC_P (rw) register accessor: Peli Arbitration Lost Capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`alc_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alc_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alc_p`] module"]
#[doc(alias = "ALC_P")]
pub type AlcP = crate::Reg<alc_p::AlcPSpec>;
#[doc = "Peli Arbitration Lost Capture register"]
pub mod alc_p;
#[doc = "TXDR0_B (rw) register accessor: Basic TX Data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr0_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr0_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr0_b`] module"]
#[doc(alias = "TXDR0_B")]
pub type Txdr0B = crate::Reg<txdr0_b::Txdr0BSpec>;
#[doc = "Basic TX Data register 0"]
pub mod txdr0_b;
#[doc = "ECC_P (r) register accessor: Peli Error Code Capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_p::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_p`] module"]
#[doc(alias = "ECC_P")]
pub type EccP = crate::Reg<ecc_p::EccPSpec>;
#[doc = "Peli Error Code Capture register"]
pub mod ecc_p;
#[doc = "TXDR1_B (rw) register accessor: Basic TX Data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr1_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr1_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr1_b`] module"]
#[doc(alias = "TXDR1_B")]
pub type Txdr1B = crate::Reg<txdr1_b::Txdr1BSpec>;
#[doc = "Basic TX Data register 1"]
pub mod txdr1_b;
#[doc = "EWLR_P (rw) register accessor: Peli Error Warning Limit register\n\nYou can [`read`](crate::Reg::read) this register and get [`ewlr_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewlr_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ewlr_p`] module"]
#[doc(alias = "EWLR_P")]
pub type EwlrP = crate::Reg<ewlr_p::EwlrPSpec>;
#[doc = "Peli Error Warning Limit register"]
pub mod ewlr_p;
#[doc = "TXDR2_B (rw) register accessor: Basic Send Data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr2_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr2_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr2_b`] module"]
#[doc(alias = "TXDR2_B")]
pub type Txdr2B = crate::Reg<txdr2_b::Txdr2BSpec>;
#[doc = "Basic Send Data register 2"]
pub mod txdr2_b;
#[doc = "RXERR_P (rw) register accessor: Peli RX Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxerr_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxerr_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxerr_p`] module"]
#[doc(alias = "RXERR_P")]
pub type RxerrP = crate::Reg<rxerr_p::RxerrPSpec>;
#[doc = "Peli RX Error Counter register"]
pub mod rxerr_p;
#[doc = "TXDR3_B (rw) register accessor: Basic TX Data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr3_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr3_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr3_b`] module"]
#[doc(alias = "TXDR3_B")]
pub type Txdr3B = crate::Reg<txdr3_b::Txdr3BSpec>;
#[doc = "Basic TX Data register 3"]
pub mod txdr3_b;
#[doc = "TXERR_P (rw) register accessor: Peli TX Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`txerr_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txerr_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txerr_p`] module"]
#[doc(alias = "TXERR_P")]
pub type TxerrP = crate::Reg<txerr_p::TxerrPSpec>;
#[doc = "Peli TX Error Counter register"]
pub mod txerr_p;
#[doc = "TXDR4_B (rw) register accessor: Basic TX Data register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr4_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr4_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr4_b`] module"]
#[doc(alias = "TXDR4_B")]
pub type Txdr4B = crate::Reg<txdr4_b::Txdr4BSpec>;
#[doc = "Basic TX Data register 4"]
pub mod txdr4_b;
#[doc = "SFF_P (rw) register accessor: Peli Send Frame Format register\n\nYou can [`read`](crate::Reg::read) this register and get [`sff_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sff_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sff_p`] module"]
#[doc(alias = "SFF_P")]
pub type SffP = crate::Reg<sff_p::SffPSpec>;
#[doc = "Peli Send Frame Format register"]
pub mod sff_p;
#[doc = "GROUP0_ACR0_P (rw) register accessor: Peli Acceptance Code register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group0_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group0_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group0_acr0_p`] module"]
#[doc(alias = "GROUP0_ACR0_P")]
pub type Group0Acr0P = crate::Reg<group0_acr0_p::Group0Acr0PSpec>;
#[doc = "Peli Acceptance Code register 0"]
pub mod group0_acr0_p;
#[doc = "TXDR5_B (rw) register accessor: Basic TX Data register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr5_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr5_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr5_b`] module"]
#[doc(alias = "TXDR5_B")]
pub type Txdr5B = crate::Reg<txdr5_b::Txdr5BSpec>;
#[doc = "Basic TX Data register 5"]
pub mod txdr5_b;
#[doc = "TXID0_P (rw) register accessor: Peli TX ID register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`txid0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txid0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txid0_p`] module"]
#[doc(alias = "TXID0_P")]
pub type Txid0P = crate::Reg<txid0_p::Txid0PSpec>;
#[doc = "Peli TX ID register 0"]
pub mod txid0_p;
#[doc = "GROUP0_ACR1_P (rw) register accessor: Peli Acceptance Code register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group0_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group0_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group0_acr1_p`] module"]
#[doc(alias = "GROUP0_ACR1_P")]
pub type Group0Acr1P = crate::Reg<group0_acr1_p::Group0Acr1PSpec>;
#[doc = "Peli Acceptance Code register 1"]
pub mod group0_acr1_p;
#[doc = "TXDR6_B (rw) register accessor: Basic TX Data register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr6_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr6_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr6_b`] module"]
#[doc(alias = "TXDR6_B")]
pub type Txdr6B = crate::Reg<txdr6_b::Txdr6BSpec>;
#[doc = "Basic TX Data register 6"]
pub mod txdr6_b;
#[doc = "TXID1_P (rw) register accessor: Peli TX ID register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txid1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txid1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txid1_p`] module"]
#[doc(alias = "TXID1_P")]
pub type Txid1P = crate::Reg<txid1_p::Txid1PSpec>;
#[doc = "Peli TX ID register 1"]
pub mod txid1_p;
#[doc = "GROUP0_ACR2_P (rw) register accessor: Peli Acceptance Code register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group0_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group0_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group0_acr2_p`] module"]
#[doc(alias = "GROUP0_ACR2_P")]
pub type Group0Acr2P = crate::Reg<group0_acr2_p::Group0Acr2PSpec>;
#[doc = "Peli Acceptance Code register 2"]
pub mod group0_acr2_p;
#[doc = "TXDR7_B (rw) register accessor: Basic TX Data register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr7_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr7_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr7_b`] module"]
#[doc(alias = "TXDR7_B")]
pub type Txdr7B = crate::Reg<txdr7_b::Txdr7BSpec>;
#[doc = "Basic TX Data register 7"]
pub mod txdr7_b;
#[doc = "TXDATA0_P (rw) register accessor: Peli TX Data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata0_p`] module"]
#[doc(alias = "TXDATA0_P")]
pub type Txdata0P = crate::Reg<txdata0_p::Txdata0PSpec>;
#[doc = "Peli TX Data register 0"]
pub mod txdata0_p;
#[doc = "GROUP0_ACR3_P (rw) register accessor: Peli Acceptance Code register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group0_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group0_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group0_acr3_p`] module"]
#[doc(alias = "GROUP0_ACR3_P")]
pub type Group0Acr3P = crate::Reg<group0_acr3_p::Group0Acr3PSpec>;
#[doc = "Peli Acceptance Code register 3"]
pub mod group0_acr3_p;
#[doc = "TXDATA1_P (rw) register accessor: Peli TX Data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata1_p`] module"]
#[doc(alias = "TXDATA1_P")]
pub type Txdata1P = crate::Reg<txdata1_p::Txdata1PSpec>;
#[doc = "Peli TX Data register 1"]
pub mod txdata1_p;
#[doc = "GROUP0_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group0_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group0_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group0_amr0_p`] module"]
#[doc(alias = "GROUP0_AMR0_P")]
pub type Group0Amr0P = crate::Reg<group0_amr0_p::Group0Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group0_amr0_p;
#[doc = "TXDATA2_P (rw) register accessor: Peli TX Data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata2_p`] module"]
#[doc(alias = "TXDATA2_P")]
pub type Txdata2P = crate::Reg<txdata2_p::Txdata2PSpec>;
#[doc = "Peli TX Data register 2"]
pub mod txdata2_p;
#[doc = "GROUP0_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group0_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group0_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group0_amr1_p`] module"]
#[doc(alias = "GROUP0_AMR1_P")]
pub type Group0Amr1P = crate::Reg<group0_amr1_p::Group0Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group0_amr1_p;
#[doc = "TXDATA3_P (rw) register accessor: Peli TX Data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata3_p`] module"]
#[doc(alias = "TXDATA3_P")]
pub type Txdata3P = crate::Reg<txdata3_p::Txdata3PSpec>;
#[doc = "Peli TX Data register 3"]
pub mod txdata3_p;
#[doc = "GROUP0_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group0_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group0_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group0_amr2_p`] module"]
#[doc(alias = "GROUP0_AMR2_P")]
pub type Group0Amr2P = crate::Reg<group0_amr2_p::Group0Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group0_amr2_p;
#[doc = "TXDATA4_P (rw) register accessor: Peli TX Data register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata4_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata4_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata4_p`] module"]
#[doc(alias = "TXDATA4_P")]
pub type Txdata4P = crate::Reg<txdata4_p::Txdata4PSpec>;
#[doc = "Peli TX Data register 4"]
pub mod txdata4_p;
#[doc = "GROUP0_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group0_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group0_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group0_amr3_p`] module"]
#[doc(alias = "GROUP0_AMR3_P")]
pub type Group0Amr3P = crate::Reg<group0_amr3_p::Group0Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group0_amr3_p;
#[doc = "TXDATA5_P (rw) register accessor: Peli TX Data register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata5_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata5_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata5_p`] module"]
#[doc(alias = "TXDATA5_P")]
pub type Txdata5P = crate::Reg<txdata5_p::Txdata5PSpec>;
#[doc = "Peli TX Data register 5"]
pub mod txdata5_p;
#[doc = "TXDATA6_P (rw) register accessor: Peli TX Data register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata6_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata6_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata6_p`] module"]
#[doc(alias = "TXDATA6_P")]
pub type Txdata6P = crate::Reg<txdata6_p::Txdata6PSpec>;
#[doc = "Peli TX Data register 6"]
pub mod txdata6_p;
#[doc = "TXDATA7_P (rw) register accessor: Peli TX Data register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata7_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata7_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata7_p`] module"]
#[doc(alias = "TXDATA7_P")]
pub type Txdata7P = crate::Reg<txdata7_p::Txdata7PSpec>;
#[doc = "Peli TX Data register 7"]
pub mod txdata7_p;
#[doc = "TXDATA8_P (rw) register accessor: Peli TX Data register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata8_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata8_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata8_p`] module"]
#[doc(alias = "TXDATA8_P")]
pub type Txdata8P = crate::Reg<txdata8_p::Txdata8PSpec>;
#[doc = "Peli TX Data register 8"]
pub mod txdata8_p;
#[doc = "TXDATA9_P (rw) register accessor: Peli TX Data register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata9_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata9_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata9_p`] module"]
#[doc(alias = "TXDATA9_P")]
pub type Txdata9P = crate::Reg<txdata9_p::Txdata9PSpec>;
#[doc = "Peli TX Data register 9"]
pub mod txdata9_p;
#[doc = "CDR (rw) register accessor: Clock Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr`] module"]
#[doc(alias = "CDR")]
pub type Cdr = crate::Reg<cdr::CdrSpec>;
#[doc = "Clock Divider register"]
pub mod cdr;
#[doc = "AFM0 (rw) register accessor: Filter Mode register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`afm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm0`] module"]
#[doc(alias = "AFM0")]
pub type Afm0 = crate::Reg<afm0::Afm0Spec>;
#[doc = "Filter Mode register 0"]
pub mod afm0;
#[doc = "AFM1 (rw) register accessor: Filter Mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`afm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm1`] module"]
#[doc(alias = "AFM1")]
pub type Afm1 = crate::Reg<afm1::Afm1Spec>;
#[doc = "Filter Mode register 1"]
pub mod afm1;
#[doc = "AFM2 (rw) register accessor: Filter Mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`afm2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afm2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm2`] module"]
#[doc(alias = "AFM2")]
pub type Afm2 = crate::Reg<afm2::Afm2Spec>;
#[doc = "Filter Mode register 2"]
pub mod afm2;
#[doc = "FGA0 (rw) register accessor: Filter Group Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fga0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fga0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fga0`] module"]
#[doc(alias = "FGA0")]
pub type Fga0 = crate::Reg<fga0::Fga0Spec>;
#[doc = "Filter Group Enable Register 0"]
pub mod fga0;
#[doc = "FGA1 (rw) register accessor: Filter Group Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fga1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fga1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fga1`] module"]
#[doc(alias = "FGA1")]
pub type Fga1 = crate::Reg<fga1::Fga1Spec>;
#[doc = "Filter Group Enable Register 1"]
pub mod fga1;
#[doc = "FGA2 (rw) register accessor: Filter Group Enable Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fga2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fga2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fga2`] module"]
#[doc(alias = "FGA2")]
pub type Fga2 = crate::Reg<fga2::Fga2Spec>;
#[doc = "Filter Group Enable Register 2"]
pub mod fga2;
#[doc = "GROUP1_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group1_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group1_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group1_acr_b`] module"]
#[doc(alias = "GROUP1_ACR_B")]
pub type Group1AcrB = crate::Reg<group1_acr_b::Group1AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group1_acr_b;
#[doc = "GROUP1_ACR0_P (rw) register accessor: Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group1_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group1_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group1_acr0_p`] module"]
#[doc(alias = "GROUP1_ACR0_P")]
pub type Group1Acr0P = crate::Reg<group1_acr0_p::Group1Acr0PSpec>;
#[doc = "Peli Acceptance Code register0"]
pub mod group1_acr0_p;
#[doc = "GROUP1_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group1_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group1_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group1_acr1_p`] module"]
#[doc(alias = "GROUP1_ACR1_P")]
pub type Group1Acr1P = crate::Reg<group1_acr1_p::Group1Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group1_acr1_p;
#[doc = "GROUP1_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group1_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group1_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group1_acr2_p`] module"]
#[doc(alias = "GROUP1_ACR2_P")]
pub type Group1Acr2P = crate::Reg<group1_acr2_p::Group1Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group1_acr2_p;
#[doc = "GROUP1_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group1_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group1_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group1_acr3_p`] module"]
#[doc(alias = "GROUP1_ACR3_P")]
pub type Group1Acr3P = crate::Reg<group1_acr3_p::Group1Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group1_acr3_p;
#[doc = "GROUP1_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group1_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group1_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group1_amr_b`] module"]
#[doc(alias = "GROUP1_AMR_B")]
pub type Group1AmrB = crate::Reg<group1_amr_b::Group1AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group1_amr_b;
#[doc = "GROUP1_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group1_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group1_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group1_amr0_p`] module"]
#[doc(alias = "GROUP1_AMR0_P")]
pub type Group1Amr0P = crate::Reg<group1_amr0_p::Group1Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group1_amr0_p;
#[doc = "GROUP1_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group1_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group1_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group1_amr1_p`] module"]
#[doc(alias = "GROUP1_AMR1_P")]
pub type Group1Amr1P = crate::Reg<group1_amr1_p::Group1Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group1_amr1_p;
#[doc = "GROUP1_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group1_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group1_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group1_amr2_p`] module"]
#[doc(alias = "GROUP1_AMR2_P")]
pub type Group1Amr2P = crate::Reg<group1_amr2_p::Group1Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group1_amr2_p;
#[doc = "GROUP1_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group1_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group1_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group1_amr3_p`] module"]
#[doc(alias = "GROUP1_AMR3_P")]
pub type Group1Amr3P = crate::Reg<group1_amr3_p::Group1Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group1_amr3_p;
#[doc = "GROUP2_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group2_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group2_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group2_acr_b`] module"]
#[doc(alias = "GROUP2_ACR_B")]
pub type Group2AcrB = crate::Reg<group2_acr_b::Group2AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group2_acr_b;
#[doc = "GROUP2_ACR0_P (rw) register accessor: Peli Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group2_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group2_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group2_acr0_p`] module"]
#[doc(alias = "GROUP2_ACR0_P")]
pub type Group2Acr0P = crate::Reg<group2_acr0_p::Group2Acr0PSpec>;
#[doc = "Peli Acceptance Code register"]
pub mod group2_acr0_p;
#[doc = "GROUP2_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group2_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group2_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group2_acr1_p`] module"]
#[doc(alias = "GROUP2_ACR1_P")]
pub type Group2Acr1P = crate::Reg<group2_acr1_p::Group2Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group2_acr1_p;
#[doc = "GROUP2_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group2_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group2_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group2_acr2_p`] module"]
#[doc(alias = "GROUP2_ACR2_P")]
pub type Group2Acr2P = crate::Reg<group2_acr2_p::Group2Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group2_acr2_p;
#[doc = "GROUP2_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group2_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group2_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group2_acr3_p`] module"]
#[doc(alias = "GROUP2_ACR3_P")]
pub type Group2Acr3P = crate::Reg<group2_acr3_p::Group2Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group2_acr3_p;
#[doc = "GROUP2_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group2_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group2_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group2_amr_b`] module"]
#[doc(alias = "GROUP2_AMR_B")]
pub type Group2AmrB = crate::Reg<group2_amr_b::Group2AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group2_amr_b;
#[doc = "GROUP2_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group2_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group2_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group2_amr0_p`] module"]
#[doc(alias = "GROUP2_AMR0_P")]
pub type Group2Amr0P = crate::Reg<group2_amr0_p::Group2Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group2_amr0_p;
#[doc = "GROUP2_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group2_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group2_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group2_amr1_p`] module"]
#[doc(alias = "GROUP2_AMR1_P")]
pub type Group2Amr1P = crate::Reg<group2_amr1_p::Group2Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group2_amr1_p;
#[doc = "GROUP2_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group2_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group2_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group2_amr2_p`] module"]
#[doc(alias = "GROUP2_AMR2_P")]
pub type Group2Amr2P = crate::Reg<group2_amr2_p::Group2Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group2_amr2_p;
#[doc = "GROUP2_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group2_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group2_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group2_amr3_p`] module"]
#[doc(alias = "GROUP2_AMR3_P")]
pub type Group2Amr3P = crate::Reg<group2_amr3_p::Group2Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group2_amr3_p;
#[doc = "GROUP3_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group3_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group3_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group3_acr_b`] module"]
#[doc(alias = "GROUP3_ACR_B")]
pub type Group3AcrB = crate::Reg<group3_acr_b::Group3AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group3_acr_b;
#[doc = "GROUP3_ACR0_P (rw) register accessor: Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group3_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group3_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group3_acr0_p`] module"]
#[doc(alias = "GROUP3_ACR0_P")]
pub type Group3Acr0P = crate::Reg<group3_acr0_p::Group3Acr0PSpec>;
#[doc = "Peli Acceptance Code register0"]
pub mod group3_acr0_p;
#[doc = "GROUP3_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group3_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group3_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group3_acr1_p`] module"]
#[doc(alias = "GROUP3_ACR1_P")]
pub type Group3Acr1P = crate::Reg<group3_acr1_p::Group3Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group3_acr1_p;
#[doc = "GROUP3_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group3_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group3_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group3_acr2_p`] module"]
#[doc(alias = "GROUP3_ACR2_P")]
pub type Group3Acr2P = crate::Reg<group3_acr2_p::Group3Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group3_acr2_p;
#[doc = "GROUP3_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group3_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group3_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group3_acr3_p`] module"]
#[doc(alias = "GROUP3_ACR3_P")]
pub type Group3Acr3P = crate::Reg<group3_acr3_p::Group3Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group3_acr3_p;
#[doc = "GROUP3_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group3_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group3_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group3_amr_b`] module"]
#[doc(alias = "GROUP3_AMR_B")]
pub type Group3AmrB = crate::Reg<group3_amr_b::Group3AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group3_amr_b;
#[doc = "GROUP3_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group3_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group3_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group3_amr0_p`] module"]
#[doc(alias = "GROUP3_AMR0_P")]
pub type Group3Amr0P = crate::Reg<group3_amr0_p::Group3Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group3_amr0_p;
#[doc = "GROUP3_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group3_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group3_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group3_amr1_p`] module"]
#[doc(alias = "GROUP3_AMR1_P")]
pub type Group3Amr1P = crate::Reg<group3_amr1_p::Group3Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group3_amr1_p;
#[doc = "GROUP3_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group3_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group3_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group3_amr2_p`] module"]
#[doc(alias = "GROUP3_AMR2_P")]
pub type Group3Amr2P = crate::Reg<group3_amr2_p::Group3Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group3_amr2_p;
#[doc = "GROUP3_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group3_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group3_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group3_amr3_p`] module"]
#[doc(alias = "GROUP3_AMR3_P")]
pub type Group3Amr3P = crate::Reg<group3_amr3_p::Group3Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group3_amr3_p;
#[doc = "GROUP4_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group4_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group4_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group4_acr_b`] module"]
#[doc(alias = "GROUP4_ACR_B")]
pub type Group4AcrB = crate::Reg<group4_acr_b::Group4AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group4_acr_b;
#[doc = "GROUP4_ACR0_P (rw) register accessor: Peli Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group4_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group4_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group4_acr0_p`] module"]
#[doc(alias = "GROUP4_ACR0_P")]
pub type Group4Acr0P = crate::Reg<group4_acr0_p::Group4Acr0PSpec>;
#[doc = "Peli Acceptance Code register"]
pub mod group4_acr0_p;
#[doc = "GROUP4_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group4_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group4_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group4_acr1_p`] module"]
#[doc(alias = "GROUP4_ACR1_P")]
pub type Group4Acr1P = crate::Reg<group4_acr1_p::Group4Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group4_acr1_p;
#[doc = "GROUP4_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group4_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group4_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group4_acr2_p`] module"]
#[doc(alias = "GROUP4_ACR2_P")]
pub type Group4Acr2P = crate::Reg<group4_acr2_p::Group4Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group4_acr2_p;
#[doc = "GROUP4_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group4_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group4_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group4_acr3_p`] module"]
#[doc(alias = "GROUP4_ACR3_P")]
pub type Group4Acr3P = crate::Reg<group4_acr3_p::Group4Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group4_acr3_p;
#[doc = "GROUP4_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group4_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group4_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group4_amr_b`] module"]
#[doc(alias = "GROUP4_AMR_B")]
pub type Group4AmrB = crate::Reg<group4_amr_b::Group4AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group4_amr_b;
#[doc = "GROUP4_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group4_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group4_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group4_amr0_p`] module"]
#[doc(alias = "GROUP4_AMR0_P")]
pub type Group4Amr0P = crate::Reg<group4_amr0_p::Group4Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group4_amr0_p;
#[doc = "GROUP4_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group4_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group4_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group4_amr1_p`] module"]
#[doc(alias = "GROUP4_AMR1_P")]
pub type Group4Amr1P = crate::Reg<group4_amr1_p::Group4Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group4_amr1_p;
#[doc = "GROUP4_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group4_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group4_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group4_amr2_p`] module"]
#[doc(alias = "GROUP4_AMR2_P")]
pub type Group4Amr2P = crate::Reg<group4_amr2_p::Group4Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group4_amr2_p;
#[doc = "GROUP4_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group4_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group4_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group4_amr3_p`] module"]
#[doc(alias = "GROUP4_AMR3_P")]
pub type Group4Amr3P = crate::Reg<group4_amr3_p::Group4Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group4_amr3_p;
#[doc = "GROUP5_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group5_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group5_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group5_acr_b`] module"]
#[doc(alias = "GROUP5_ACR_B")]
pub type Group5AcrB = crate::Reg<group5_acr_b::Group5AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group5_acr_b;
#[doc = "GROUP5_ACR0_P (rw) register accessor: Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group5_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group5_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group5_acr0_p`] module"]
#[doc(alias = "GROUP5_ACR0_P")]
pub type Group5Acr0P = crate::Reg<group5_acr0_p::Group5Acr0PSpec>;
#[doc = "Peli Acceptance Code register0"]
pub mod group5_acr0_p;
#[doc = "GROUP5_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group5_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group5_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group5_acr1_p`] module"]
#[doc(alias = "GROUP5_ACR1_P")]
pub type Group5Acr1P = crate::Reg<group5_acr1_p::Group5Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group5_acr1_p;
#[doc = "GROUP5_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group5_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group5_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group5_acr2_p`] module"]
#[doc(alias = "GROUP5_ACR2_P")]
pub type Group5Acr2P = crate::Reg<group5_acr2_p::Group5Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group5_acr2_p;
#[doc = "GROUP5_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group5_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group5_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group5_acr3_p`] module"]
#[doc(alias = "GROUP5_ACR3_P")]
pub type Group5Acr3P = crate::Reg<group5_acr3_p::Group5Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group5_acr3_p;
#[doc = "GROUP5_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group5_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group5_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group5_amr_b`] module"]
#[doc(alias = "GROUP5_AMR_B")]
pub type Group5AmrB = crate::Reg<group5_amr_b::Group5AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group5_amr_b;
#[doc = "GROUP5_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group5_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group5_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group5_amr0_p`] module"]
#[doc(alias = "GROUP5_AMR0_P")]
pub type Group5Amr0P = crate::Reg<group5_amr0_p::Group5Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group5_amr0_p;
#[doc = "GROUP5_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group5_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group5_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group5_amr1_p`] module"]
#[doc(alias = "GROUP5_AMR1_P")]
pub type Group5Amr1P = crate::Reg<group5_amr1_p::Group5Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group5_amr1_p;
#[doc = "GROUP5_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group5_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group5_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group5_amr2_p`] module"]
#[doc(alias = "GROUP5_AMR2_P")]
pub type Group5Amr2P = crate::Reg<group5_amr2_p::Group5Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group5_amr2_p;
#[doc = "GROUP5_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group5_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group5_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group5_amr3_p`] module"]
#[doc(alias = "GROUP5_AMR3_P")]
pub type Group5Amr3P = crate::Reg<group5_amr3_p::Group5Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group5_amr3_p;
#[doc = "GROUP6_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group6_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group6_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group6_acr_b`] module"]
#[doc(alias = "GROUP6_ACR_B")]
pub type Group6AcrB = crate::Reg<group6_acr_b::Group6AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group6_acr_b;
#[doc = "GROUP6_ACR0_P (rw) register accessor: Peli Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group6_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group6_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group6_acr0_p`] module"]
#[doc(alias = "GROUP6_ACR0_P")]
pub type Group6Acr0P = crate::Reg<group6_acr0_p::Group6Acr0PSpec>;
#[doc = "Peli Acceptance Code register"]
pub mod group6_acr0_p;
#[doc = "GROUP6_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group6_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group6_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group6_acr1_p`] module"]
#[doc(alias = "GROUP6_ACR1_P")]
pub type Group6Acr1P = crate::Reg<group6_acr1_p::Group6Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group6_acr1_p;
#[doc = "GROUP6_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group6_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group6_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group6_acr2_p`] module"]
#[doc(alias = "GROUP6_ACR2_P")]
pub type Group6Acr2P = crate::Reg<group6_acr2_p::Group6Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group6_acr2_p;
#[doc = "GROUP6_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group6_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group6_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group6_acr3_p`] module"]
#[doc(alias = "GROUP6_ACR3_P")]
pub type Group6Acr3P = crate::Reg<group6_acr3_p::Group6Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group6_acr3_p;
#[doc = "GROUP6_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group6_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group6_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group6_amr_b`] module"]
#[doc(alias = "GROUP6_AMR_B")]
pub type Group6AmrB = crate::Reg<group6_amr_b::Group6AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group6_amr_b;
#[doc = "GROUP6_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group6_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group6_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group6_amr0_p`] module"]
#[doc(alias = "GROUP6_AMR0_P")]
pub type Group6Amr0P = crate::Reg<group6_amr0_p::Group6Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group6_amr0_p;
#[doc = "GROUP6_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group6_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group6_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group6_amr1_p`] module"]
#[doc(alias = "GROUP6_AMR1_P")]
pub type Group6Amr1P = crate::Reg<group6_amr1_p::Group6Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group6_amr1_p;
#[doc = "GROUP6_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group6_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group6_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group6_amr2_p`] module"]
#[doc(alias = "GROUP6_AMR2_P")]
pub type Group6Amr2P = crate::Reg<group6_amr2_p::Group6Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group6_amr2_p;
#[doc = "GROUP6_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group6_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group6_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group6_amr3_p`] module"]
#[doc(alias = "GROUP6_AMR3_P")]
pub type Group6Amr3P = crate::Reg<group6_amr3_p::Group6Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group6_amr3_p;
#[doc = "GROUP7_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group7_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group7_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group7_acr_b`] module"]
#[doc(alias = "GROUP7_ACR_B")]
pub type Group7AcrB = crate::Reg<group7_acr_b::Group7AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group7_acr_b;
#[doc = "GROUP7_ACR0_P (rw) register accessor: Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group7_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group7_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group7_acr0_p`] module"]
#[doc(alias = "GROUP7_ACR0_P")]
pub type Group7Acr0P = crate::Reg<group7_acr0_p::Group7Acr0PSpec>;
#[doc = "Peli Acceptance Code register0"]
pub mod group7_acr0_p;
#[doc = "GROUP7_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group7_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group7_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group7_acr1_p`] module"]
#[doc(alias = "GROUP7_ACR1_P")]
pub type Group7Acr1P = crate::Reg<group7_acr1_p::Group7Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group7_acr1_p;
#[doc = "GROUP7_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group7_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group7_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group7_acr2_p`] module"]
#[doc(alias = "GROUP7_ACR2_P")]
pub type Group7Acr2P = crate::Reg<group7_acr2_p::Group7Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group7_acr2_p;
#[doc = "GROUP7_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group7_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group7_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group7_acr3_p`] module"]
#[doc(alias = "GROUP7_ACR3_P")]
pub type Group7Acr3P = crate::Reg<group7_acr3_p::Group7Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group7_acr3_p;
#[doc = "GROUP7_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group7_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group7_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group7_amr_b`] module"]
#[doc(alias = "GROUP7_AMR_B")]
pub type Group7AmrB = crate::Reg<group7_amr_b::Group7AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group7_amr_b;
#[doc = "GROUP7_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group7_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group7_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group7_amr0_p`] module"]
#[doc(alias = "GROUP7_AMR0_P")]
pub type Group7Amr0P = crate::Reg<group7_amr0_p::Group7Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group7_amr0_p;
#[doc = "GROUP7_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group7_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group7_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group7_amr1_p`] module"]
#[doc(alias = "GROUP7_AMR1_P")]
pub type Group7Amr1P = crate::Reg<group7_amr1_p::Group7Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group7_amr1_p;
#[doc = "GROUP7_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group7_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group7_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group7_amr2_p`] module"]
#[doc(alias = "GROUP7_AMR2_P")]
pub type Group7Amr2P = crate::Reg<group7_amr2_p::Group7Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group7_amr2_p;
#[doc = "GROUP7_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group7_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group7_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group7_amr3_p`] module"]
#[doc(alias = "GROUP7_AMR3_P")]
pub type Group7Amr3P = crate::Reg<group7_amr3_p::Group7Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group7_amr3_p;
#[doc = "GROUP8_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group8_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group8_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group8_acr_b`] module"]
#[doc(alias = "GROUP8_ACR_B")]
pub type Group8AcrB = crate::Reg<group8_acr_b::Group8AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group8_acr_b;
#[doc = "GROUP8_ACR0_P (rw) register accessor: Peli Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group8_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group8_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group8_acr0_p`] module"]
#[doc(alias = "GROUP8_ACR0_P")]
pub type Group8Acr0P = crate::Reg<group8_acr0_p::Group8Acr0PSpec>;
#[doc = "Peli Acceptance Code register"]
pub mod group8_acr0_p;
#[doc = "GROUP8_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group8_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group8_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group8_acr1_p`] module"]
#[doc(alias = "GROUP8_ACR1_P")]
pub type Group8Acr1P = crate::Reg<group8_acr1_p::Group8Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group8_acr1_p;
#[doc = "GROUP8_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group8_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group8_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group8_acr2_p`] module"]
#[doc(alias = "GROUP8_ACR2_P")]
pub type Group8Acr2P = crate::Reg<group8_acr2_p::Group8Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group8_acr2_p;
#[doc = "GROUP8_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group8_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group8_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group8_acr3_p`] module"]
#[doc(alias = "GROUP8_ACR3_P")]
pub type Group8Acr3P = crate::Reg<group8_acr3_p::Group8Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group8_acr3_p;
#[doc = "GROUP8_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group8_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group8_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group8_amr_b`] module"]
#[doc(alias = "GROUP8_AMR_B")]
pub type Group8AmrB = crate::Reg<group8_amr_b::Group8AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group8_amr_b;
#[doc = "GROUP8_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group8_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group8_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group8_amr0_p`] module"]
#[doc(alias = "GROUP8_AMR0_P")]
pub type Group8Amr0P = crate::Reg<group8_amr0_p::Group8Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group8_amr0_p;
#[doc = "GROUP8_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group8_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group8_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group8_amr1_p`] module"]
#[doc(alias = "GROUP8_AMR1_P")]
pub type Group8Amr1P = crate::Reg<group8_amr1_p::Group8Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group8_amr1_p;
#[doc = "GROUP8_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group8_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group8_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group8_amr2_p`] module"]
#[doc(alias = "GROUP8_AMR2_P")]
pub type Group8Amr2P = crate::Reg<group8_amr2_p::Group8Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group8_amr2_p;
#[doc = "GROUP8_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group8_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group8_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group8_amr3_p`] module"]
#[doc(alias = "GROUP8_AMR3_P")]
pub type Group8Amr3P = crate::Reg<group8_amr3_p::Group8Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group8_amr3_p;
#[doc = "GROUP9_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group9_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group9_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group9_acr_b`] module"]
#[doc(alias = "GROUP9_ACR_B")]
pub type Group9AcrB = crate::Reg<group9_acr_b::Group9AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group9_acr_b;
#[doc = "GROUP9_ACR0_P (rw) register accessor: Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group9_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group9_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group9_acr0_p`] module"]
#[doc(alias = "GROUP9_ACR0_P")]
pub type Group9Acr0P = crate::Reg<group9_acr0_p::Group9Acr0PSpec>;
#[doc = "Peli Acceptance Code register0"]
pub mod group9_acr0_p;
#[doc = "GROUP9_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group9_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group9_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group9_acr1_p`] module"]
#[doc(alias = "GROUP9_ACR1_P")]
pub type Group9Acr1P = crate::Reg<group9_acr1_p::Group9Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group9_acr1_p;
#[doc = "GROUP9_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group9_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group9_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group9_acr2_p`] module"]
#[doc(alias = "GROUP9_ACR2_P")]
pub type Group9Acr2P = crate::Reg<group9_acr2_p::Group9Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group9_acr2_p;
#[doc = "GROUP9_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group9_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group9_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group9_acr3_p`] module"]
#[doc(alias = "GROUP9_ACR3_P")]
pub type Group9Acr3P = crate::Reg<group9_acr3_p::Group9Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group9_acr3_p;
#[doc = "GROUP9_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group9_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group9_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group9_amr_b`] module"]
#[doc(alias = "GROUP9_AMR_B")]
pub type Group9AmrB = crate::Reg<group9_amr_b::Group9AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group9_amr_b;
#[doc = "GROUP9_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group9_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group9_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group9_amr0_p`] module"]
#[doc(alias = "GROUP9_AMR0_P")]
pub type Group9Amr0P = crate::Reg<group9_amr0_p::Group9Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group9_amr0_p;
#[doc = "GROUP9_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group9_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group9_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group9_amr1_p`] module"]
#[doc(alias = "GROUP9_AMR1_P")]
pub type Group9Amr1P = crate::Reg<group9_amr1_p::Group9Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group9_amr1_p;
#[doc = "GROUP9_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group9_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group9_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group9_amr2_p`] module"]
#[doc(alias = "GROUP9_AMR2_P")]
pub type Group9Amr2P = crate::Reg<group9_amr2_p::Group9Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group9_amr2_p;
#[doc = "GROUP9_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group9_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group9_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group9_amr3_p`] module"]
#[doc(alias = "GROUP9_AMR3_P")]
pub type Group9Amr3P = crate::Reg<group9_amr3_p::Group9Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group9_amr3_p;
#[doc = "GROUP10_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group10_acr_b`] module"]
#[doc(alias = "GROUP10_ACR_B")]
pub type Group10AcrB = crate::Reg<group10_acr_b::Group10AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group10_acr_b;
#[doc = "GROUP10_ACR0_P (rw) register accessor: Peli Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group10_acr0_p`] module"]
#[doc(alias = "GROUP10_ACR0_P")]
pub type Group10Acr0P = crate::Reg<group10_acr0_p::Group10Acr0PSpec>;
#[doc = "Peli Acceptance Code register"]
pub mod group10_acr0_p;
#[doc = "GROUP10_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group10_acr1_p`] module"]
#[doc(alias = "GROUP10_ACR1_P")]
pub type Group10Acr1P = crate::Reg<group10_acr1_p::Group10Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group10_acr1_p;
#[doc = "GROUP10_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group10_acr2_p`] module"]
#[doc(alias = "GROUP10_ACR2_P")]
pub type Group10Acr2P = crate::Reg<group10_acr2_p::Group10Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group10_acr2_p;
#[doc = "GROUP10_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group10_acr3_p`] module"]
#[doc(alias = "GROUP10_ACR3_P")]
pub type Group10Acr3P = crate::Reg<group10_acr3_p::Group10Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group10_acr3_p;
#[doc = "GROUP10_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group10_amr_b`] module"]
#[doc(alias = "GROUP10_AMR_B")]
pub type Group10AmrB = crate::Reg<group10_amr_b::Group10AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group10_amr_b;
#[doc = "GROUP10_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group10_amr0_p`] module"]
#[doc(alias = "GROUP10_AMR0_P")]
pub type Group10Amr0P = crate::Reg<group10_amr0_p::Group10Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group10_amr0_p;
#[doc = "GROUP10_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group10_amr1_p`] module"]
#[doc(alias = "GROUP10_AMR1_P")]
pub type Group10Amr1P = crate::Reg<group10_amr1_p::Group10Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group10_amr1_p;
#[doc = "GROUP10_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group10_amr2_p`] module"]
#[doc(alias = "GROUP10_AMR2_P")]
pub type Group10Amr2P = crate::Reg<group10_amr2_p::Group10Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group10_amr2_p;
#[doc = "GROUP10_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group10_amr3_p`] module"]
#[doc(alias = "GROUP10_AMR3_P")]
pub type Group10Amr3P = crate::Reg<group10_amr3_p::Group10Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group10_amr3_p;
#[doc = "GROUP11_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group11_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group11_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group11_acr_b`] module"]
#[doc(alias = "GROUP11_ACR_B")]
pub type Group11AcrB = crate::Reg<group11_acr_b::Group11AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group11_acr_b;
#[doc = "GROUP11_ACR0_P (rw) register accessor: Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group11_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group11_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group11_acr0_p`] module"]
#[doc(alias = "GROUP11_ACR0_P")]
pub type Group11Acr0P = crate::Reg<group11_acr0_p::Group11Acr0PSpec>;
#[doc = "Peli Acceptance Code register0"]
pub mod group11_acr0_p;
#[doc = "GROUP11_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group11_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group11_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group11_acr1_p`] module"]
#[doc(alias = "GROUP11_ACR1_P")]
pub type Group11Acr1P = crate::Reg<group11_acr1_p::Group11Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group11_acr1_p;
#[doc = "GROUP11_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group11_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group11_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group11_acr2_p`] module"]
#[doc(alias = "GROUP11_ACR2_P")]
pub type Group11Acr2P = crate::Reg<group11_acr2_p::Group11Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group11_acr2_p;
#[doc = "GROUP11_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group11_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group11_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group11_acr3_p`] module"]
#[doc(alias = "GROUP11_ACR3_P")]
pub type Group11Acr3P = crate::Reg<group11_acr3_p::Group11Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group11_acr3_p;
#[doc = "GROUP11_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group11_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group11_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group11_amr_b`] module"]
#[doc(alias = "GROUP11_AMR_B")]
pub type Group11AmrB = crate::Reg<group11_amr_b::Group11AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group11_amr_b;
#[doc = "GROUP11_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group11_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group11_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group11_amr0_p`] module"]
#[doc(alias = "GROUP11_AMR0_P")]
pub type Group11Amr0P = crate::Reg<group11_amr0_p::Group11Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group11_amr0_p;
#[doc = "GROUP11_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group11_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group11_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group11_amr1_p`] module"]
#[doc(alias = "GROUP11_AMR1_P")]
pub type Group11Amr1P = crate::Reg<group11_amr1_p::Group11Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group11_amr1_p;
#[doc = "GROUP11_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group11_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group11_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group11_amr2_p`] module"]
#[doc(alias = "GROUP11_AMR2_P")]
pub type Group11Amr2P = crate::Reg<group11_amr2_p::Group11Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group11_amr2_p;
#[doc = "GROUP11_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group11_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group11_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group11_amr3_p`] module"]
#[doc(alias = "GROUP11_AMR3_P")]
pub type Group11Amr3P = crate::Reg<group11_amr3_p::Group11Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group11_amr3_p;
#[doc = "GROUP12_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group12_acr_b`] module"]
#[doc(alias = "GROUP12_ACR_B")]
pub type Group12AcrB = crate::Reg<group12_acr_b::Group12AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group12_acr_b;
#[doc = "GROUP12_ACR0_P (rw) register accessor: Peli Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group12_acr0_p`] module"]
#[doc(alias = "GROUP12_ACR0_P")]
pub type Group12Acr0P = crate::Reg<group12_acr0_p::Group12Acr0PSpec>;
#[doc = "Peli Acceptance Code register"]
pub mod group12_acr0_p;
#[doc = "GROUP12_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group12_acr1_p`] module"]
#[doc(alias = "GROUP12_ACR1_P")]
pub type Group12Acr1P = crate::Reg<group12_acr1_p::Group12Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group12_acr1_p;
#[doc = "GROUP12_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group12_acr2_p`] module"]
#[doc(alias = "GROUP12_ACR2_P")]
pub type Group12Acr2P = crate::Reg<group12_acr2_p::Group12Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group12_acr2_p;
#[doc = "GROUP12_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group12_acr3_p`] module"]
#[doc(alias = "GROUP12_ACR3_P")]
pub type Group12Acr3P = crate::Reg<group12_acr3_p::Group12Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group12_acr3_p;
#[doc = "GROUP12_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group12_amr_b`] module"]
#[doc(alias = "GROUP12_AMR_B")]
pub type Group12AmrB = crate::Reg<group12_amr_b::Group12AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group12_amr_b;
#[doc = "GROUP12_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group12_amr0_p`] module"]
#[doc(alias = "GROUP12_AMR0_P")]
pub type Group12Amr0P = crate::Reg<group12_amr0_p::Group12Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group12_amr0_p;
#[doc = "GROUP12_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group12_amr1_p`] module"]
#[doc(alias = "GROUP12_AMR1_P")]
pub type Group12Amr1P = crate::Reg<group12_amr1_p::Group12Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group12_amr1_p;
#[doc = "GROUP12_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group12_amr2_p`] module"]
#[doc(alias = "GROUP12_AMR2_P")]
pub type Group12Amr2P = crate::Reg<group12_amr2_p::Group12Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group12_amr2_p;
#[doc = "GROUP12_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group12_amr3_p`] module"]
#[doc(alias = "GROUP12_AMR3_P")]
pub type Group12Amr3P = crate::Reg<group12_amr3_p::Group12Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group12_amr3_p;
#[doc = "GROUP13_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group13_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group13_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group13_acr_b`] module"]
#[doc(alias = "GROUP13_ACR_B")]
pub type Group13AcrB = crate::Reg<group13_acr_b::Group13AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group13_acr_b;
#[doc = "GROUP13_ACR0_P (rw) register accessor: Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group13_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group13_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group13_acr0_p`] module"]
#[doc(alias = "GROUP13_ACR0_P")]
pub type Group13Acr0P = crate::Reg<group13_acr0_p::Group13Acr0PSpec>;
#[doc = "Peli Acceptance Code register0"]
pub mod group13_acr0_p;
#[doc = "GROUP13_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group13_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group13_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group13_acr1_p`] module"]
#[doc(alias = "GROUP13_ACR1_P")]
pub type Group13Acr1P = crate::Reg<group13_acr1_p::Group13Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group13_acr1_p;
#[doc = "GROUP13_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group13_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group13_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group13_acr2_p`] module"]
#[doc(alias = "GROUP13_ACR2_P")]
pub type Group13Acr2P = crate::Reg<group13_acr2_p::Group13Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group13_acr2_p;
#[doc = "GROUP13_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group13_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group13_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group13_acr3_p`] module"]
#[doc(alias = "GROUP13_ACR3_P")]
pub type Group13Acr3P = crate::Reg<group13_acr3_p::Group13Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group13_acr3_p;
#[doc = "GROUP13_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group13_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group13_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group13_amr_b`] module"]
#[doc(alias = "GROUP13_AMR_B")]
pub type Group13AmrB = crate::Reg<group13_amr_b::Group13AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group13_amr_b;
#[doc = "GROUP13_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group13_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group13_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group13_amr0_p`] module"]
#[doc(alias = "GROUP13_AMR0_P")]
pub type Group13Amr0P = crate::Reg<group13_amr0_p::Group13Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group13_amr0_p;
#[doc = "GROUP13_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group13_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group13_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group13_amr1_p`] module"]
#[doc(alias = "GROUP13_AMR1_P")]
pub type Group13Amr1P = crate::Reg<group13_amr1_p::Group13Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group13_amr1_p;
#[doc = "GROUP13_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group13_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group13_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group13_amr2_p`] module"]
#[doc(alias = "GROUP13_AMR2_P")]
pub type Group13Amr2P = crate::Reg<group13_amr2_p::Group13Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group13_amr2_p;
#[doc = "GROUP13_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group13_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group13_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group13_amr3_p`] module"]
#[doc(alias = "GROUP13_AMR3_P")]
pub type Group13Amr3P = crate::Reg<group13_amr3_p::Group13Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group13_amr3_p;
#[doc = "GROUP14_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group14_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group14_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group14_acr_b`] module"]
#[doc(alias = "GROUP14_ACR_B")]
pub type Group14AcrB = crate::Reg<group14_acr_b::Group14AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group14_acr_b;
#[doc = "GROUP14_ACR0_P (rw) register accessor: Peli Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group14_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group14_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group14_acr0_p`] module"]
#[doc(alias = "GROUP14_ACR0_P")]
pub type Group14Acr0P = crate::Reg<group14_acr0_p::Group14Acr0PSpec>;
#[doc = "Peli Acceptance Code register"]
pub mod group14_acr0_p;
#[doc = "GROUP14_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group14_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group14_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group14_acr1_p`] module"]
#[doc(alias = "GROUP14_ACR1_P")]
pub type Group14Acr1P = crate::Reg<group14_acr1_p::Group14Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group14_acr1_p;
#[doc = "GROUP14_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group14_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group14_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group14_acr2_p`] module"]
#[doc(alias = "GROUP14_ACR2_P")]
pub type Group14Acr2P = crate::Reg<group14_acr2_p::Group14Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group14_acr2_p;
#[doc = "GROUP14_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group14_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group14_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group14_acr3_p`] module"]
#[doc(alias = "GROUP14_ACR3_P")]
pub type Group14Acr3P = crate::Reg<group14_acr3_p::Group14Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group14_acr3_p;
#[doc = "GROUP14_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group14_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group14_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group14_amr_b`] module"]
#[doc(alias = "GROUP14_AMR_B")]
pub type Group14AmrB = crate::Reg<group14_amr_b::Group14AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group14_amr_b;
#[doc = "GROUP14_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group14_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group14_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group14_amr0_p`] module"]
#[doc(alias = "GROUP14_AMR0_P")]
pub type Group14Amr0P = crate::Reg<group14_amr0_p::Group14Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group14_amr0_p;
#[doc = "GROUP14_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group14_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group14_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group14_amr1_p`] module"]
#[doc(alias = "GROUP14_AMR1_P")]
pub type Group14Amr1P = crate::Reg<group14_amr1_p::Group14Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group14_amr1_p;
#[doc = "GROUP14_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group14_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group14_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group14_amr2_p`] module"]
#[doc(alias = "GROUP14_AMR2_P")]
pub type Group14Amr2P = crate::Reg<group14_amr2_p::Group14Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group14_amr2_p;
#[doc = "GROUP14_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group14_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group14_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group14_amr3_p`] module"]
#[doc(alias = "GROUP14_AMR3_P")]
pub type Group14Amr3P = crate::Reg<group14_amr3_p::Group14Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group14_amr3_p;
#[doc = "GROUP15_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group15_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group15_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group15_acr_b`] module"]
#[doc(alias = "GROUP15_ACR_B")]
pub type Group15AcrB = crate::Reg<group15_acr_b::Group15AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group15_acr_b;
#[doc = "GROUP15_ACR0_P (rw) register accessor: Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group15_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group15_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group15_acr0_p`] module"]
#[doc(alias = "GROUP15_ACR0_P")]
pub type Group15Acr0P = crate::Reg<group15_acr0_p::Group15Acr0PSpec>;
#[doc = "Peli Acceptance Code register0"]
pub mod group15_acr0_p;
#[doc = "GROUP15_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group15_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group15_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group15_acr1_p`] module"]
#[doc(alias = "GROUP15_ACR1_P")]
pub type Group15Acr1P = crate::Reg<group15_acr1_p::Group15Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group15_acr1_p;
#[doc = "GROUP15_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group15_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group15_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group15_acr2_p`] module"]
#[doc(alias = "GROUP15_ACR2_P")]
pub type Group15Acr2P = crate::Reg<group15_acr2_p::Group15Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group15_acr2_p;
#[doc = "GROUP15_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group15_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group15_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group15_acr3_p`] module"]
#[doc(alias = "GROUP15_ACR3_P")]
pub type Group15Acr3P = crate::Reg<group15_acr3_p::Group15Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group15_acr3_p;
#[doc = "GROUP15_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group15_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group15_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group15_amr_b`] module"]
#[doc(alias = "GROUP15_AMR_B")]
pub type Group15AmrB = crate::Reg<group15_amr_b::Group15AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group15_amr_b;
#[doc = "GROUP15_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group15_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group15_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group15_amr0_p`] module"]
#[doc(alias = "GROUP15_AMR0_P")]
pub type Group15Amr0P = crate::Reg<group15_amr0_p::Group15Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group15_amr0_p;
#[doc = "GROUP15_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group15_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group15_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group15_amr1_p`] module"]
#[doc(alias = "GROUP15_AMR1_P")]
pub type Group15Amr1P = crate::Reg<group15_amr1_p::Group15Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group15_amr1_p;
#[doc = "GROUP15_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group15_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group15_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group15_amr2_p`] module"]
#[doc(alias = "GROUP15_AMR2_P")]
pub type Group15Amr2P = crate::Reg<group15_amr2_p::Group15Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group15_amr2_p;
#[doc = "GROUP15_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group15_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group15_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group15_amr3_p`] module"]
#[doc(alias = "GROUP15_AMR3_P")]
pub type Group15Amr3P = crate::Reg<group15_amr3_p::Group15Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group15_amr3_p;
#[doc = "GROUP16_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group16_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group16_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group16_acr_b`] module"]
#[doc(alias = "GROUP16_ACR_B")]
pub type Group16AcrB = crate::Reg<group16_acr_b::Group16AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group16_acr_b;
#[doc = "GROUP16_ACR0_P (rw) register accessor: Peli Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group16_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group16_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group16_acr0_p`] module"]
#[doc(alias = "GROUP16_ACR0_P")]
pub type Group16Acr0P = crate::Reg<group16_acr0_p::Group16Acr0PSpec>;
#[doc = "Peli Acceptance Code register"]
pub mod group16_acr0_p;
#[doc = "GROUP16_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group16_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group16_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group16_acr1_p`] module"]
#[doc(alias = "GROUP16_ACR1_P")]
pub type Group16Acr1P = crate::Reg<group16_acr1_p::Group16Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group16_acr1_p;
#[doc = "GROUP16_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group16_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group16_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group16_acr2_p`] module"]
#[doc(alias = "GROUP16_ACR2_P")]
pub type Group16Acr2P = crate::Reg<group16_acr2_p::Group16Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group16_acr2_p;
#[doc = "GROUP16_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group16_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group16_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group16_acr3_p`] module"]
#[doc(alias = "GROUP16_ACR3_P")]
pub type Group16Acr3P = crate::Reg<group16_acr3_p::Group16Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group16_acr3_p;
#[doc = "GROUP16_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group16_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group16_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group16_amr_b`] module"]
#[doc(alias = "GROUP16_AMR_B")]
pub type Group16AmrB = crate::Reg<group16_amr_b::Group16AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group16_amr_b;
#[doc = "GROUP16_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group16_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group16_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group16_amr0_p`] module"]
#[doc(alias = "GROUP16_AMR0_P")]
pub type Group16Amr0P = crate::Reg<group16_amr0_p::Group16Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group16_amr0_p;
#[doc = "GROUP16_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group16_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group16_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group16_amr1_p`] module"]
#[doc(alias = "GROUP16_AMR1_P")]
pub type Group16Amr1P = crate::Reg<group16_amr1_p::Group16Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group16_amr1_p;
#[doc = "GROUP16_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group16_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group16_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group16_amr2_p`] module"]
#[doc(alias = "GROUP16_AMR2_P")]
pub type Group16Amr2P = crate::Reg<group16_amr2_p::Group16Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group16_amr2_p;
#[doc = "GROUP16_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group16_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group16_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group16_amr3_p`] module"]
#[doc(alias = "GROUP16_AMR3_P")]
pub type Group16Amr3P = crate::Reg<group16_amr3_p::Group16Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group16_amr3_p;
#[doc = "GROUP17_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group17_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group17_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group17_acr_b`] module"]
#[doc(alias = "GROUP17_ACR_B")]
pub type Group17AcrB = crate::Reg<group17_acr_b::Group17AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group17_acr_b;
#[doc = "GROUP17_ACR0_P (rw) register accessor: Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group17_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group17_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group17_acr0_p`] module"]
#[doc(alias = "GROUP17_ACR0_P")]
pub type Group17Acr0P = crate::Reg<group17_acr0_p::Group17Acr0PSpec>;
#[doc = "Peli Acceptance Code register0"]
pub mod group17_acr0_p;
#[doc = "GROUP17_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group17_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group17_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group17_acr1_p`] module"]
#[doc(alias = "GROUP17_ACR1_P")]
pub type Group17Acr1P = crate::Reg<group17_acr1_p::Group17Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group17_acr1_p;
#[doc = "GROUP17_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group17_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group17_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group17_acr2_p`] module"]
#[doc(alias = "GROUP17_ACR2_P")]
pub type Group17Acr2P = crate::Reg<group17_acr2_p::Group17Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group17_acr2_p;
#[doc = "GROUP17_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group17_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group17_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group17_acr3_p`] module"]
#[doc(alias = "GROUP17_ACR3_P")]
pub type Group17Acr3P = crate::Reg<group17_acr3_p::Group17Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group17_acr3_p;
#[doc = "GROUP17_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group17_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group17_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group17_amr_b`] module"]
#[doc(alias = "GROUP17_AMR_B")]
pub type Group17AmrB = crate::Reg<group17_amr_b::Group17AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group17_amr_b;
#[doc = "GROUP17_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group17_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group17_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group17_amr0_p`] module"]
#[doc(alias = "GROUP17_AMR0_P")]
pub type Group17Amr0P = crate::Reg<group17_amr0_p::Group17Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group17_amr0_p;
#[doc = "GROUP17_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group17_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group17_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group17_amr1_p`] module"]
#[doc(alias = "GROUP17_AMR1_P")]
pub type Group17Amr1P = crate::Reg<group17_amr1_p::Group17Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group17_amr1_p;
#[doc = "GROUP17_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group17_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group17_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group17_amr2_p`] module"]
#[doc(alias = "GROUP17_AMR2_P")]
pub type Group17Amr2P = crate::Reg<group17_amr2_p::Group17Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group17_amr2_p;
#[doc = "GROUP17_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group17_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group17_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group17_amr3_p`] module"]
#[doc(alias = "GROUP17_AMR3_P")]
pub type Group17Amr3P = crate::Reg<group17_amr3_p::Group17Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group17_amr3_p;
#[doc = "GROUP18_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group18_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group18_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group18_acr_b`] module"]
#[doc(alias = "GROUP18_ACR_B")]
pub type Group18AcrB = crate::Reg<group18_acr_b::Group18AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group18_acr_b;
#[doc = "GROUP18_ACR0_P (rw) register accessor: Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group18_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group18_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group18_acr0_p`] module"]
#[doc(alias = "GROUP18_ACR0_P")]
pub type Group18Acr0P = crate::Reg<group18_acr0_p::Group18Acr0PSpec>;
#[doc = "Peli Acceptance Code register0"]
pub mod group18_acr0_p;
#[doc = "GROUP18_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group18_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group18_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group18_acr1_p`] module"]
#[doc(alias = "GROUP18_ACR1_P")]
pub type Group18Acr1P = crate::Reg<group18_acr1_p::Group18Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group18_acr1_p;
#[doc = "GROUP18_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group18_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group18_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group18_acr2_p`] module"]
#[doc(alias = "GROUP18_ACR2_P")]
pub type Group18Acr2P = crate::Reg<group18_acr2_p::Group18Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group18_acr2_p;
#[doc = "GROUP18_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group18_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group18_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group18_acr3_p`] module"]
#[doc(alias = "GROUP18_ACR3_P")]
pub type Group18Acr3P = crate::Reg<group18_acr3_p::Group18Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group18_acr3_p;
#[doc = "GROUP18_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group18_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group18_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group18_amr_b`] module"]
#[doc(alias = "GROUP18_AMR_B")]
pub type Group18AmrB = crate::Reg<group18_amr_b::Group18AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group18_amr_b;
#[doc = "GROUP18_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group18_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group18_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group18_amr0_p`] module"]
#[doc(alias = "GROUP18_AMR0_P")]
pub type Group18Amr0P = crate::Reg<group18_amr0_p::Group18Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group18_amr0_p;
#[doc = "GROUP18_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group18_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group18_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group18_amr1_p`] module"]
#[doc(alias = "GROUP18_AMR1_P")]
pub type Group18Amr1P = crate::Reg<group18_amr1_p::Group18Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group18_amr1_p;
#[doc = "GROUP18_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group18_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group18_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group18_amr2_p`] module"]
#[doc(alias = "GROUP18_AMR2_P")]
pub type Group18Amr2P = crate::Reg<group18_amr2_p::Group18Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group18_amr2_p;
#[doc = "GROUP18_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group18_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group18_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group18_amr3_p`] module"]
#[doc(alias = "GROUP18_AMR3_P")]
pub type Group18Amr3P = crate::Reg<group18_amr3_p::Group18Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group18_amr3_p;
#[doc = "GROUP19_ACR_B (rw) register accessor: Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group19_acr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group19_acr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group19_acr_b`] module"]
#[doc(alias = "GROUP19_ACR_B")]
pub type Group19AcrB = crate::Reg<group19_acr_b::Group19AcrBSpec>;
#[doc = "Basic Acceptance Code register"]
pub mod group19_acr_b;
#[doc = "GROUP19_ACR0_P (rw) register accessor: Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group19_acr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group19_acr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group19_acr0_p`] module"]
#[doc(alias = "GROUP19_ACR0_P")]
pub type Group19Acr0P = crate::Reg<group19_acr0_p::Group19Acr0PSpec>;
#[doc = "Peli Acceptance Code register0"]
pub mod group19_acr0_p;
#[doc = "GROUP19_ACR1_P (rw) register accessor: Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group19_acr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group19_acr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group19_acr1_p`] module"]
#[doc(alias = "GROUP19_ACR1_P")]
pub type Group19Acr1P = crate::Reg<group19_acr1_p::Group19Acr1PSpec>;
#[doc = "Peli Acceptance Code register1"]
pub mod group19_acr1_p;
#[doc = "GROUP19_ACR2_P (rw) register accessor: Peli Acceptance Code register2\n\nYou can [`read`](crate::Reg::read) this register and get [`group19_acr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group19_acr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group19_acr2_p`] module"]
#[doc(alias = "GROUP19_ACR2_P")]
pub type Group19Acr2P = crate::Reg<group19_acr2_p::Group19Acr2PSpec>;
#[doc = "Peli Acceptance Code register2"]
pub mod group19_acr2_p;
#[doc = "GROUP19_ACR3_P (rw) register accessor: Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group19_acr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group19_acr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group19_acr3_p`] module"]
#[doc(alias = "GROUP19_ACR3_P")]
pub type Group19Acr3P = crate::Reg<group19_acr3_p::Group19Acr3PSpec>;
#[doc = "Peli Acceptance Code register3"]
pub mod group19_acr3_p;
#[doc = "GROUP19_AMR_B (rw) register accessor: Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group19_amr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group19_amr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group19_amr_b`] module"]
#[doc(alias = "GROUP19_AMR_B")]
pub type Group19AmrB = crate::Reg<group19_amr_b::Group19AmrBSpec>;
#[doc = "Basic Acceptance Mask register"]
pub mod group19_amr_b;
#[doc = "GROUP19_AMR0_P (rw) register accessor: Peli Acceptance Mask register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`group19_amr0_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group19_amr0_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group19_amr0_p`] module"]
#[doc(alias = "GROUP19_AMR0_P")]
pub type Group19Amr0P = crate::Reg<group19_amr0_p::Group19Amr0PSpec>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group19_amr0_p;
#[doc = "GROUP19_AMR1_P (rw) register accessor: Peli Acceptance Mask register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group19_amr1_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group19_amr1_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group19_amr1_p`] module"]
#[doc(alias = "GROUP19_AMR1_P")]
pub type Group19Amr1P = crate::Reg<group19_amr1_p::Group19Amr1PSpec>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group19_amr1_p;
#[doc = "GROUP19_AMR2_P (rw) register accessor: Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group19_amr2_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group19_amr2_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group19_amr2_p`] module"]
#[doc(alias = "GROUP19_AMR2_P")]
pub type Group19Amr2P = crate::Reg<group19_amr2_p::Group19Amr2PSpec>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group19_amr2_p;
#[doc = "GROUP19_AMR3_P (rw) register accessor: Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group19_amr3_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group19_amr3_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group19_amr3_p`] module"]
#[doc(alias = "GROUP19_AMR3_P")]
pub type Group19Amr3P = crate::Reg<group19_amr3_p::Group19Amr3PSpec>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group19_amr3_p;
