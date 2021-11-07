#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x0c - Entry"]
    pub entry: [crate::Reg<entry::ENTRY_SPEC>; 3],
    #[doc = "0x0c - End of Table Marker Register"]
    pub tablemark: crate::Reg<tablemark::TABLEMARK_SPEC>,
    _reserved2: [u8; 0x0fbc],
    #[doc = "0xfcc - System Access Register"]
    pub sysaccess: crate::Reg<sysaccess::SYSACCESS_SPEC>,
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
#[doc = "ENTRY register accessor: an alias for `Reg<ENTRY_SPEC>`"]
pub type ENTRY = crate::Reg<entry::ENTRY_SPEC>;
#[doc = "Entry"]
pub mod entry;
#[doc = "TABLEMARK register accessor: an alias for `Reg<TABLEMARK_SPEC>`"]
pub type TABLEMARK = crate::Reg<tablemark::TABLEMARK_SPEC>;
#[doc = "End of Table Marker Register"]
pub mod tablemark;
#[doc = "SYSACCESS register accessor: an alias for `Reg<SYSACCESS_SPEC>`"]
pub type SYSACCESS = crate::Reg<sysaccess::SYSACCESS_SPEC>;
#[doc = "System Access Register"]
pub mod sysaccess;
#[doc = "PERIPHID register accessor: an alias for `Reg<PERIPHID_SPEC>`"]
pub type PERIPHID = crate::Reg<periphid::PERIPHID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod periphid;
#[doc = "COMPID register accessor: an alias for `Reg<COMPID_SPEC>`"]
pub type COMPID = crate::Reg<compid::COMPID_SPEC>;
#[doc = "Component ID Register"]
pub mod compid;
