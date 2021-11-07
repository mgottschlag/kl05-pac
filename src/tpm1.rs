#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x04 - Counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x08 - Modulo"]
    pub mod_: crate::Reg<mod_::MOD_SPEC>,
    #[doc = "0x0c - Channel (n) Status and Control"]
    pub c0sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x10 - Channel (n) Value"]
    pub c0v: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x14 - Channel (n) Status and Control"]
    pub c1sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x18 - Channel (n) Value"]
    pub c1v: crate::Reg<cv::CV_SPEC>,
    _reserved7: [u8; 0x34],
    #[doc = "0x50 - Capture and Compare Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved8: [u8; 0x30],
    #[doc = "0x84 - Configuration"]
    pub conf: crate::Reg<conf::CONF_SPEC>,
}
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "Status and Control"]
pub mod sc;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "MOD register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Modulo"]
pub mod mod_;
#[doc = "CSC register accessor: an alias for `Reg<CSC_SPEC>`"]
pub type CSC = crate::Reg<csc::CSC_SPEC>;
#[doc = "Channel (n) Status and Control"]
pub mod csc;
#[doc = "CV register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Channel (n) Value"]
pub mod cv;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Capture and Compare Status"]
pub mod status;
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configuration"]
pub mod conf;
