#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Mode Protection register"]
    pub pmprot: crate::Reg<pmprot::PMPROT_SPEC>,
    #[doc = "0x01 - Power Mode Control register"]
    pub pmctrl: crate::Reg<pmctrl::PMCTRL_SPEC>,
    #[doc = "0x02 - Stop Control Register"]
    pub stopctrl: crate::Reg<stopctrl::STOPCTRL_SPEC>,
    #[doc = "0x03 - Power Mode Status register"]
    pub pmstat: crate::Reg<pmstat::PMSTAT_SPEC>,
}
#[doc = "PMPROT register accessor: an alias for `Reg<PMPROT_SPEC>`"]
pub type PMPROT = crate::Reg<pmprot::PMPROT_SPEC>;
#[doc = "Power Mode Protection register"]
pub mod pmprot;
#[doc = "PMCTRL register accessor: an alias for `Reg<PMCTRL_SPEC>`"]
pub type PMCTRL = crate::Reg<pmctrl::PMCTRL_SPEC>;
#[doc = "Power Mode Control register"]
pub mod pmctrl;
#[doc = "STOPCTRL register accessor: an alias for `Reg<STOPCTRL_SPEC>`"]
pub type STOPCTRL = crate::Reg<stopctrl::STOPCTRL_SPEC>;
#[doc = "Stop Control Register"]
pub mod stopctrl;
#[doc = "PMSTAT register accessor: an alias for `Reg<PMSTAT_SPEC>`"]
pub type PMSTAT = crate::Reg<pmstat::PMSTAT_SPEC>;
#[doc = "Power Mode Status register"]
pub mod pmstat;
