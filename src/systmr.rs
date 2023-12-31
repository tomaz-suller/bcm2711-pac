#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Control / Status"]
    pub cs: CS,
    #[doc = "0x04 - Lower 32 bits for the free running counter"]
    pub clo: CLO,
    #[doc = "0x08 - Higher 32 bits for the free running counter"]
    pub chi: CHI,
    #[doc = "0x0c - Compare channel 0"]
    pub c0: C0,
    #[doc = "0x10 - Compare channel 1"]
    pub c1: C1,
    #[doc = "0x14 - Compare channel 2"]
    pub c2: C2,
    #[doc = "0x18 - Compare channel 3"]
    pub c3: C3,
}
#[doc = "CS (rw) register accessor: an alias for `Reg<CS_SPEC>`"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "Control / Status"]
pub mod cs;
#[doc = "CLO (r) register accessor: an alias for `Reg<CLO_SPEC>`"]
pub type CLO = crate::Reg<clo::CLO_SPEC>;
#[doc = "Lower 32 bits for the free running counter"]
pub mod clo;
#[doc = "CHI (r) register accessor: an alias for `Reg<CHI_SPEC>`"]
pub type CHI = crate::Reg<chi::CHI_SPEC>;
#[doc = "Higher 32 bits for the free running counter"]
pub mod chi;
#[doc = "C0 (rw) register accessor: an alias for `Reg<C0_SPEC>`"]
pub type C0 = crate::Reg<c0::C0_SPEC>;
#[doc = "Compare channel 0"]
pub mod c0;
#[doc = "C1 (rw) register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "Compare channel 1"]
pub mod c1;
#[doc = "C2 (rw) register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "Compare channel 2"]
pub mod c2;
#[doc = "C3 (rw) register accessor: an alias for `Reg<C3_SPEC>`"]
pub type C3 = crate::Reg<c3::C3_SPEC>;
#[doc = "Compare channel 3"]
pub mod c3;
