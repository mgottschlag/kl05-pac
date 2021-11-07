#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB Position Register"]
    pub position: crate::Reg<position::POSITION_SPEC>,
    #[doc = "0x04 - MTB Master Register"]
    pub master: crate::Reg<master::MASTER_SPEC>,
    #[doc = "0x08 - MTB Flow Register"]
    pub flow: crate::Reg<flow::FLOW_SPEC>,
    #[doc = "0x0c - MTB Base Register"]
    pub base: crate::Reg<base::BASE_SPEC>,
    _reserved4: [u8; 0x0ef0],
    #[doc = "0xf00 - Integration Mode Control Register"]
    pub modectrl: crate::Reg<modectrl::MODECTRL_SPEC>,
    _reserved5: [u8; 0x9c],
    #[doc = "0xfa0 - Claim TAG Set Register"]
    pub tagset: crate::Reg<tagset::TAGSET_SPEC>,
    #[doc = "0xfa4 - Claim TAG Clear Register"]
    pub tagclear: crate::Reg<tagclear::TAGCLEAR_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0xfb0 - Lock Access Register"]
    pub lockaccess: crate::Reg<lockaccess::LOCKACCESS_SPEC>,
    #[doc = "0xfb4 - Lock Status Register"]
    pub lockstat: crate::Reg<lockstat::LOCKSTAT_SPEC>,
    #[doc = "0xfb8 - Authentication Status Register"]
    pub authstat: crate::Reg<authstat::AUTHSTAT_SPEC>,
    #[doc = "0xfbc - Device Architecture Register"]
    pub devicearch: crate::Reg<devicearch::DEVICEARCH_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0xfc8 - Device Configuration Register"]
    pub devicecfg: crate::Reg<devicecfg::DEVICECFG_SPEC>,
    #[doc = "0xfcc - Device Type Identifier Register"]
    pub devicetypid: crate::Reg<devicetypid::DEVICETYPID_SPEC>,
    #[doc = "0xfd0 - Peripheral ID Register"]
    pub periphid4: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfd4 - Peripheral ID Register"]
    pub periphid5: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfd8 - Peripheral ID Register"]
    pub periphid6: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfdc - Peripheral ID Register"]
    pub periphid7: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfe0 - Peripheral ID Register"]
    pub periphid0: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfe4 - Peripheral ID Register"]
    pub periphid1: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfe8 - Peripheral ID Register"]
    pub periphid2: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfec - Peripheral ID Register"]
    pub periphid3: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xff0..0x1000 - Component ID Register"]
    pub compid: [crate::Reg<compid::COMPID_SPEC>; 4],
}
#[doc = "POSITION register accessor: an alias for `Reg<POSITION_SPEC>`"]
pub type POSITION = crate::Reg<position::POSITION_SPEC>;
#[doc = "MTB Position Register"]
pub mod position;
#[doc = "MASTER register accessor: an alias for `Reg<MASTER_SPEC>`"]
pub type MASTER = crate::Reg<master::MASTER_SPEC>;
#[doc = "MTB Master Register"]
pub mod master;
#[doc = "FLOW register accessor: an alias for `Reg<FLOW_SPEC>`"]
pub type FLOW = crate::Reg<flow::FLOW_SPEC>;
#[doc = "MTB Flow Register"]
pub mod flow;
#[doc = "BASE register accessor: an alias for `Reg<BASE_SPEC>`"]
pub type BASE = crate::Reg<base::BASE_SPEC>;
#[doc = "MTB Base Register"]
pub mod base;
#[doc = "MODECTRL register accessor: an alias for `Reg<MODECTRL_SPEC>`"]
pub type MODECTRL = crate::Reg<modectrl::MODECTRL_SPEC>;
#[doc = "Integration Mode Control Register"]
pub mod modectrl;
#[doc = "TAGSET register accessor: an alias for `Reg<TAGSET_SPEC>`"]
pub type TAGSET = crate::Reg<tagset::TAGSET_SPEC>;
#[doc = "Claim TAG Set Register"]
pub mod tagset;
#[doc = "TAGCLEAR register accessor: an alias for `Reg<TAGCLEAR_SPEC>`"]
pub type TAGCLEAR = crate::Reg<tagclear::TAGCLEAR_SPEC>;
#[doc = "Claim TAG Clear Register"]
pub mod tagclear;
#[doc = "LOCKACCESS register accessor: an alias for `Reg<LOCKACCESS_SPEC>`"]
pub type LOCKACCESS = crate::Reg<lockaccess::LOCKACCESS_SPEC>;
#[doc = "Lock Access Register"]
pub mod lockaccess;
#[doc = "LOCKSTAT register accessor: an alias for `Reg<LOCKSTAT_SPEC>`"]
pub type LOCKSTAT = crate::Reg<lockstat::LOCKSTAT_SPEC>;
#[doc = "Lock Status Register"]
pub mod lockstat;
#[doc = "AUTHSTAT register accessor: an alias for `Reg<AUTHSTAT_SPEC>`"]
pub type AUTHSTAT = crate::Reg<authstat::AUTHSTAT_SPEC>;
#[doc = "Authentication Status Register"]
pub mod authstat;
#[doc = "DEVICEARCH register accessor: an alias for `Reg<DEVICEARCH_SPEC>`"]
pub type DEVICEARCH = crate::Reg<devicearch::DEVICEARCH_SPEC>;
#[doc = "Device Architecture Register"]
pub mod devicearch;
#[doc = "DEVICECFG register accessor: an alias for `Reg<DEVICECFG_SPEC>`"]
pub type DEVICECFG = crate::Reg<devicecfg::DEVICECFG_SPEC>;
#[doc = "Device Configuration Register"]
pub mod devicecfg;
#[doc = "DEVICETYPID register accessor: an alias for `Reg<DEVICETYPID_SPEC>`"]
pub type DEVICETYPID = crate::Reg<devicetypid::DEVICETYPID_SPEC>;
#[doc = "Device Type Identifier Register"]
pub mod devicetypid;
#[doc = "PERIPHID register accessor: an alias for `Reg<PERIPHID_SPEC>`"]
pub type PERIPHID = crate::Reg<periphid::PERIPHID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod periphid;
#[doc = "COMPID register accessor: an alias for `Reg<COMPID_SPEC>`"]
pub type COMPID = crate::Reg<compid::COMPID_SPEC>;
#[doc = "Component ID Register"]
pub mod compid;
