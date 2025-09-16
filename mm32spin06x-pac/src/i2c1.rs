#[doc = "Inter integrated circuit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c1 {
    ptr: *mut u8,
}
unsafe impl Send for I2c1 {}
unsafe impl Sync for I2c1 {}
impl I2c1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Target Register"]
    #[inline(always)]
    pub const fn tar(self) -> crate::common::Reg<regs::Tar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Slave Address Register"]
    #[inline(always)]
    pub const fn sar(self) -> crate::common::Reg<regs::Sar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Data Command Register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SCL High Period Count for Std. Speed Register"]
    #[inline(always)]
    pub const fn sshr(self) -> crate::common::Reg<regs::Sshr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SCL Low Period Count for Std. Speed Register"]
    #[inline(always)]
    pub const fn sslr(self) -> crate::common::Reg<regs::Sslr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SCL High Period Count for Fast Speed Register"]
    #[inline(always)]
    pub const fn fshr(self) -> crate::common::Reg<regs::Fshr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SCL Low Period Count for Fast Speed Register"]
    #[inline(always)]
    pub const fn fslr(self) -> crate::common::Reg<regs::Fslr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "RAW Interrupt Status Register"]
    #[inline(always)]
    pub const fn rawisr(self) -> crate::common::Reg<regs::Rawisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Receive FIFO Threshold Level Register"]
    #[inline(always)]
    pub const fn rxtlr(self) -> crate::common::Reg<regs::Rxtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Transmit FIFO Threshold Level Register"]
    #[inline(always)]
    pub const fn txtlr(self) -> crate::common::Reg<regs::Txtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Clear All Interrupt Register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Clear RX_UNDER Interrupt Register"]
    #[inline(always)]
    pub const fn rx_under(self) -> crate::common::Reg<regs::RxUnder, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Clear RX_OVER Interrupt Register"]
    #[inline(always)]
    pub const fn rx_over(self) -> crate::common::Reg<regs::RxOver, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Clear TX_OVER Interrupt Register"]
    #[inline(always)]
    pub const fn tx_over(self) -> crate::common::Reg<regs::TxOver, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Clear RD_REQ Interrupt Register"]
    #[inline(always)]
    pub const fn rd_req(self) -> crate::common::Reg<regs::RdReq, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Clear TX_ABRT Interrupt Register"]
    #[inline(always)]
    pub const fn tx_abrt(self) -> crate::common::Reg<regs::TxAbrt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Clear RX_DONE Interrupt Register"]
    #[inline(always)]
    pub const fn rx_done(self) -> crate::common::Reg<regs::RxDone, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Clear ACTIVITY Interrupt Register"]
    #[inline(always)]
    pub const fn activ(self) -> crate::common::Reg<regs::Activ, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Clear STOP_DET Interrupt Register"]
    #[inline(always)]
    pub const fn stop(self) -> crate::common::Reg<regs::Stop, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Clear START_DET Interrupt Register"]
    #[inline(always)]
    pub const fn start(self) -> crate::common::Reg<regs::Start, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Clear GEN_CALL Interrupt Register"]
    #[inline(always)]
    pub const fn gc(self) -> crate::common::Reg<regs::Gc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Enable Register"]
    #[inline(always)]
    pub const fn enr(self) -> crate::common::Reg<regs::Enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Transmit FIFO Level Register"]
    #[inline(always)]
    pub const fn txflr(self) -> crate::common::Reg<regs::Txflr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Receive FIFO Level Register"]
    #[inline(always)]
    pub const fn rxflr(self) -> crate::common::Reg<regs::Rxflr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "SDA Hold Time Register"]
    #[inline(always)]
    pub const fn hold(self) -> crate::common::Reg<regs::Hold, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "DMA Control Register"]
    #[inline(always)]
    pub const fn dma(self) -> crate::common::Reg<regs::Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "SDA Setup Time Register"]
    #[inline(always)]
    pub const fn setup(self) -> crate::common::Reg<regs::Setup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "ACK General Call Register"]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Slave Address Mask Register"]
    #[inline(always)]
    pub const fn slvmask(self) -> crate::common::Reg<regs::Slvmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Receiver Address Register"]
    #[inline(always)]
    pub const fn slvrcvaddr(self) -> crate::common::Reg<regs::Slvrcvaddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
}
pub mod regs;
