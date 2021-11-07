#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - Source Address Register"]
    pub sar0: crate::Reg<sar::SAR_SPEC>,
    #[doc = "0x104 - Destination Address Register"]
    pub dar0: crate::Reg<dar::DAR_SPEC>,
    _reserved_2_dsr0: [u8; 0x04],
    #[doc = "0x10c - DMA Control Register"]
    pub dcr0: crate::Reg<dcr::DCR_SPEC>,
    #[doc = "0x110 - Source Address Register"]
    pub sar1: crate::Reg<sar::SAR_SPEC>,
    #[doc = "0x114 - Destination Address Register"]
    pub dar1: crate::Reg<dar::DAR_SPEC>,
    _reserved_6_dsr1: [u8; 0x04],
    #[doc = "0x11c - DMA Control Register"]
    pub dcr1: crate::Reg<dcr::DCR_SPEC>,
    #[doc = "0x120 - Source Address Register"]
    pub sar2: crate::Reg<sar::SAR_SPEC>,
    #[doc = "0x124 - Destination Address Register"]
    pub dar2: crate::Reg<dar::DAR_SPEC>,
    _reserved_10_dsr2: [u8; 0x04],
    #[doc = "0x12c - DMA Control Register"]
    pub dcr2: crate::Reg<dcr::DCR_SPEC>,
    #[doc = "0x130 - Source Address Register"]
    pub sar3: crate::Reg<sar::SAR_SPEC>,
    #[doc = "0x134 - Destination Address Register"]
    pub dar3: crate::Reg<dar::DAR_SPEC>,
    _reserved_14_dsr3: [u8; 0x04],
    #[doc = "0x13c - DMA Control Register"]
    pub dcr3: crate::Reg<dcr::DCR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x108 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr0(&self) -> &crate::Reg<dsr_bcr::DSR_BCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(264usize)
                as *const crate::Reg<dsr_bcr::DSR_BCR_SPEC>)
        }
    }
    #[doc = "0x10b - DMA_DSR0 register."]
    #[inline(always)]
    pub fn dsr0(&self) -> &crate::Reg<dsr0::DSR0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(267usize)
                as *const crate::Reg<dsr0::DSR0_SPEC>)
        }
    }
    #[doc = "0x118 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr1(&self) -> &crate::Reg<dsr_bcr::DSR_BCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(280usize)
                as *const crate::Reg<dsr_bcr::DSR_BCR_SPEC>)
        }
    }
    #[doc = "0x11b - DMA_DSR1 register."]
    #[inline(always)]
    pub fn dsr1(&self) -> &crate::Reg<dsr1::DSR1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(283usize)
                as *const crate::Reg<dsr1::DSR1_SPEC>)
        }
    }
    #[doc = "0x128 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr2(&self) -> &crate::Reg<dsr_bcr::DSR_BCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(296usize)
                as *const crate::Reg<dsr_bcr::DSR_BCR_SPEC>)
        }
    }
    #[doc = "0x12b - DMA_DSR2 register."]
    #[inline(always)]
    pub fn dsr2(&self) -> &crate::Reg<dsr2::DSR2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(299usize)
                as *const crate::Reg<dsr2::DSR2_SPEC>)
        }
    }
    #[doc = "0x138 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr3(&self) -> &crate::Reg<dsr_bcr::DSR_BCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(312usize)
                as *const crate::Reg<dsr_bcr::DSR_BCR_SPEC>)
        }
    }
    #[doc = "0x13b - DMA_DSR3 register."]
    #[inline(always)]
    pub fn dsr3(&self) -> &crate::Reg<dsr3::DSR3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(315usize)
                as *const crate::Reg<dsr3::DSR3_SPEC>)
        }
    }
}
#[doc = "SAR register accessor: an alias for `Reg<SAR_SPEC>`"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "Source Address Register"]
pub mod sar;
#[doc = "DAR register accessor: an alias for `Reg<DAR_SPEC>`"]
pub type DAR = crate::Reg<dar::DAR_SPEC>;
#[doc = "Destination Address Register"]
pub mod dar;
#[doc = "DSR_BCR register accessor: an alias for `Reg<DSR_BCR_SPEC>`"]
pub type DSR_BCR = crate::Reg<dsr_bcr::DSR_BCR_SPEC>;
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr;
#[doc = "DSR0 register accessor: an alias for `Reg<DSR0_SPEC>`"]
pub type DSR0 = crate::Reg<dsr0::DSR0_SPEC>;
#[doc = "DMA_DSR0 register."]
pub mod dsr0;
#[doc = "DCR register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "DMA Control Register"]
pub mod dcr;
#[doc = "DSR1 register accessor: an alias for `Reg<DSR1_SPEC>`"]
pub type DSR1 = crate::Reg<dsr1::DSR1_SPEC>;
#[doc = "DMA_DSR1 register."]
pub mod dsr1;
#[doc = "DSR2 register accessor: an alias for `Reg<DSR2_SPEC>`"]
pub type DSR2 = crate::Reg<dsr2::DSR2_SPEC>;
#[doc = "DMA_DSR2 register."]
pub mod dsr2;
#[doc = "DSR3 register accessor: an alias for `Reg<DSR3_SPEC>`"]
pub type DSR3 = crate::Reg<dsr3::DSR3_SPEC>;
#[doc = "DMA_DSR3 register."]
pub mod dsr3;
