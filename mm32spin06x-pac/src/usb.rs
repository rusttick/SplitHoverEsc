#[doc = "Universal serial bus full-speed device interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB_TOP"]
    #[inline(always)]
    pub const fn top(self) -> crate::common::Reg<regs::Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "interrupt state register"]
    #[inline(always)]
    pub const fn int_state(self) -> crate::common::Reg<regs::IntState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "EP interrupt state register"]
    #[inline(always)]
    pub const fn ep_int_state(self) -> crate::common::Reg<regs::EpIntState, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "EP0 interrupt state register"]
    #[inline(always)]
    pub const fn ep0_int_state(self) -> crate::common::Reg<regs::Ep0IntState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "interrupt enable register"]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "EP interrupt enable register"]
    #[inline(always)]
    pub const fn ep_int_en(self) -> crate::common::Reg<regs::EpIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "EP0 interrupt enable register"]
    #[inline(always)]
    pub const fn ep0_int_en(self) -> crate::common::Reg<regs::Ep0IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "EP1 interrupt state register"]
    #[inline(always)]
    pub const fn ep1_int_state(self) -> crate::common::Reg<regs::Ep1IntState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "EP2 interrupt state register"]
    #[inline(always)]
    pub const fn ep2_int_state(self) -> crate::common::Reg<regs::Ep2IntState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "EP3 interrupt state register"]
    #[inline(always)]
    pub const fn ep3_int_state(self) -> crate::common::Reg<regs::Ep3IntState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "EP4 interrupt state register"]
    #[inline(always)]
    pub const fn ep4_int_state(self) -> crate::common::Reg<regs::Ep4IntState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "EP1 interrupt enable register"]
    #[inline(always)]
    pub const fn ep1_int_en(self) -> crate::common::Reg<regs::Ep1IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "EP2 interrupt enable register"]
    #[inline(always)]
    pub const fn ep2_int_en(self) -> crate::common::Reg<regs::Ep2IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "EP3 interrupt enable register"]
    #[inline(always)]
    pub const fn ep3_int_en(self) -> crate::common::Reg<regs::Ep3IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "EP4 interrupt enable register"]
    #[inline(always)]
    pub const fn ep4_int_en(self) -> crate::common::Reg<regs::Ep4IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "USB address"]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "EP Enable End"]
    #[inline(always)]
    pub const fn ep_en(self) -> crate::common::Reg<regs::EpEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "DMA End-point direction register"]
    #[inline(always)]
    pub const fn ep_dma_dir(self) -> crate::common::Reg<regs::EpDmaDir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Endpoint type register"]
    #[inline(always)]
    pub const fn ep_type(self) -> crate::common::Reg<regs::EpType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "End-point index register 1_2"]
    #[inline(always)]
    pub const fn ep_index1_2(self) -> crate::common::Reg<regs::EpIndex12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "End-point index register 3_4"]
    #[inline(always)]
    pub const fn ep_index3_4(self) -> crate::common::Reg<regs::EpIndex34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "TOG CTRL1_4 register"]
    #[inline(always)]
    pub const fn tog_ctrl1_4(self) -> crate::common::Reg<regs::TogCtrl14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "TOG STAT1_4 register"]
    #[inline(always)]
    pub const fn tog_stat1_4(self) -> crate::common::Reg<regs::TogStat14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Setup Date 0 register"]
    #[inline(always)]
    pub const fn setup0(self) -> crate::common::Reg<regs::Setup0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Setup Date 1 register"]
    #[inline(always)]
    pub const fn setup1(self) -> crate::common::Reg<regs::Setup1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Setup Date 2 register"]
    #[inline(always)]
    pub const fn setup2(self) -> crate::common::Reg<regs::Setup2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Setup Date 3 register"]
    #[inline(always)]
    pub const fn setup3(self) -> crate::common::Reg<regs::Setup3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Setup Date 4 register"]
    #[inline(always)]
    pub const fn setup4(self) -> crate::common::Reg<regs::Setup4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Setup Date 5 register"]
    #[inline(always)]
    pub const fn setup5(self) -> crate::common::Reg<regs::Setup5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Setup Date 6 register"]
    #[inline(always)]
    pub const fn setup6(self) -> crate::common::Reg<regs::Setup6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Setup Date 7 register"]
    #[inline(always)]
    pub const fn setup7(self) -> crate::common::Reg<regs::Setup7, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "PACKET SIZEL register"]
    #[inline(always)]
    pub const fn packet_sizel(self) -> crate::common::Reg<regs::PacketSizel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "PACKET SIZEH register"]
    #[inline(always)]
    pub const fn packet_sizeh(self) -> crate::common::Reg<regs::PacketSizeh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "EP0 AVAIL register"]
    #[inline(always)]
    pub const fn ep0_avail(self) -> crate::common::Reg<regs::Ep0Avail, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "EP1 AVAIL register"]
    #[inline(always)]
    pub const fn ep1_avail(self) -> crate::common::Reg<regs::Ep1Avail, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "EP2 AVAIL register"]
    #[inline(always)]
    pub const fn ep2_avail(self) -> crate::common::Reg<regs::Ep2Avail, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "EP3 AVAIL register"]
    #[inline(always)]
    pub const fn ep3_avail(self) -> crate::common::Reg<regs::Ep3Avail, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "EP4 AVAIL register"]
    #[inline(always)]
    pub const fn ep4_avail(self) -> crate::common::Reg<regs::Ep4Avail, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "DMA1 ADDR0 register"]
    #[inline(always)]
    pub const fn dam_addr0(self) -> crate::common::Reg<regs::DamAddr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "DMA1 ADDR1 register"]
    #[inline(always)]
    pub const fn dam_addr1(self) -> crate::common::Reg<regs::DamAddr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "DMA1 ADDR2 register"]
    #[inline(always)]
    pub const fn dam_addr2(self) -> crate::common::Reg<regs::DamAddr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "DMA1 ADDR3 register"]
    #[inline(always)]
    pub const fn dam_addr3(self) -> crate::common::Reg<regs::DamAddr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "DMA NUML register"]
    #[inline(always)]
    pub const fn dma_numl(self) -> crate::common::Reg<regs::DmaNuml, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "DMA NUMH register"]
    #[inline(always)]
    pub const fn dma_numh(self) -> crate::common::Reg<regs::DmaNumh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "EP0 CTRL register"]
    #[inline(always)]
    pub const fn ep0_ctrl(self) -> crate::common::Reg<regs::Ep0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "EP1 CTRL register"]
    #[inline(always)]
    pub const fn ep1_ctrl(self) -> crate::common::Reg<regs::Ep1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "EP2 CTRL register"]
    #[inline(always)]
    pub const fn ep2_ctrl(self) -> crate::common::Reg<regs::Ep2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "EP3 CTRL register"]
    #[inline(always)]
    pub const fn ep3_ctrl(self) -> crate::common::Reg<regs::Ep3Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "EP4 CTRL register"]
    #[inline(always)]
    pub const fn ep4_ctrl(self) -> crate::common::Reg<regs::Ep4Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "EP0 FIFO register"]
    #[inline(always)]
    pub const fn ep0_fifo(self) -> crate::common::Reg<regs::Ep0Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "EP1 FIFO register"]
    #[inline(always)]
    pub const fn ep1_fifo(self) -> crate::common::Reg<regs::Ep1Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "EP2 FIFO register"]
    #[inline(always)]
    pub const fn ep2_fifo(self) -> crate::common::Reg<regs::Ep2Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "EP3 FIFO register"]
    #[inline(always)]
    pub const fn ep3_fifo(self) -> crate::common::Reg<regs::Ep3Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "EP4 FIFO register"]
    #[inline(always)]
    pub const fn ep4_fifo(self) -> crate::common::Reg<regs::Ep4Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "EP MEM register"]
    #[inline(always)]
    pub const fn ep_mem(self) -> crate::common::Reg<regs::EpMem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "EP DMA register"]
    #[inline(always)]
    pub const fn ep_dma(self) -> crate::common::Reg<regs::EpDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "EP HALT register"]
    #[inline(always)]
    pub const fn ep_halt(self) -> crate::common::Reg<regs::EpHalt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Power control register"]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "USB AHB DMA register"]
    #[inline(always)]
    pub const fn usb_ahb_dma(self) -> crate::common::Reg<regs::UsbAhbDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "USB AHB RST register"]
    #[inline(always)]
    pub const fn usb_ahb_rst(self) -> crate::common::Reg<regs::UsbAhbRst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
}
pub mod regs;
