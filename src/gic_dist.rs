#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Distributor Control Register"]
    pub gicd_ctlr: GICD_CTLR,
    #[doc = "0x04 - Interrupt Controller Type Register"]
    pub gicd_typer: GICD_TYPER,
    #[doc = "0x08 - Distributor Implementer Identification Register"]
    pub gicd_iidr: GICD_IIDR,
    _reserved3: [u8; 0x74],
    #[doc = "0x80..0x9c - Interrupt Group Registers"]
    pub gicd_igroupr: GICD_IGROUPR,
    _reserved4: [u8; 0x64],
    #[doc = "0x100..0x11c - Interrupt Set-Enable Registers"]
    pub gicd_isenabler: GICD_ISENABLER,
    _reserved5: [u8; 0x64],
    #[doc = "0x180..0x19c - Interrupt Clear-Enable Registers"]
    pub gicd_icenabler: GICD_ICENABLER,
    _reserved6: [u8; 0x64],
    #[doc = "0x200..0x21c - Interrupt Set-Pending Registers"]
    pub gicd_ispendr: GICD_ISPENDR,
    _reserved7: [u8; 0x64],
    #[doc = "0x280..0x29c - Interrupt Clear-Pending Registers"]
    pub gicd_icpendr: GICD_ICPENDR,
    _reserved8: [u8; 0x64],
    #[doc = "0x300..0x31c - Interrupt Set-Active Registers"]
    pub gicd_isactiver: GICD_ISACTIVER,
    _reserved9: [u8; 0x64],
    #[doc = "0x380..0x39c - Interrupt Clear-Active Registers"]
    pub gicd_icactiver: GICD_ICACTIVER,
    _reserved10: [u8; 0x64],
    #[doc = "0x400..0x4e0 - Interrupt Priority"]
    pub gicd_ipriorityr: GICD_IPRIORITYR,
    _reserved11: [u8; 0x0320],
    #[doc = "0x800..0x8e0 - Interrupt Processor Targets"]
    pub gicd_itargetsr: GICD_ITARGETSR,
    _reserved12: [u8; 0x0320],
    #[doc = "0xc00..0xc38 - Interrupt Configuration"]
    pub gicd_icfgr: GICD_ICFGR,
    _reserved13: [u8; 0xc8],
    #[doc = "0xd00 - Private Peripheral Interrupt Status Register"]
    pub gicd_ppisr: GICD_PPISR,
    #[doc = "0xd04 - Shared Peripheral Interrupt Status Registers"]
    pub gicd_spisr0: GICD_SPISR0,
    #[doc = "0xd08 - Shared Peripheral Interrupt Status Registers"]
    pub gicd_spisr1: GICD_SPISR1,
    #[doc = "0xd0c - Shared Peripheral Interrupt Status Registers"]
    pub gicd_spisr2: GICD_SPISR2,
    #[doc = "0xd10 - Shared Peripheral Interrupt Status Registers"]
    pub gicd_spisr3: GICD_SPISR3,
    #[doc = "0xd14 - Shared Peripheral Interrupt Status Registers"]
    pub gicd_spisr4: GICD_SPISR4,
    #[doc = "0xd18 - Shared Peripheral Interrupt Status Registers"]
    pub gicd_spisr5: GICD_SPISR5,
    _reserved20: [u8; 0x01e4],
    #[doc = "0xf00 - Software Generated Interrupt Register"]
    pub gicd_sgir: GICD_SGIR,
    _reserved21: [u8; 0x0c],
    #[doc = "0xf10 - SGI Clear-Pending Registers"]
    pub gicd_cpendsgirn: GICD_CPENDSGIRN,
    _reserved22: [u8; 0x0c],
    #[doc = "0xf20 - SGI Set-Pending Registers"]
    pub gicd_spendsgirn: GICD_SPENDSGIRN,
    _reserved23: [u8; 0xac],
    #[doc = "0xfd0 - Peripheral ID 4"]
    pub gicd_pidr4: GICD_PIDR4,
    #[doc = "0xfd4 - Peripheral ID 5"]
    pub gicd_pidr5: GICD_PIDR5,
    #[doc = "0xfd8 - Peripheral ID 6"]
    pub gicd_pidr6: GICD_PIDR6,
    #[doc = "0xfdc - Peripheral ID 7"]
    pub gicd_pidr7: GICD_PIDR7,
    #[doc = "0xfe0 - Peripheral ID 0"]
    pub gicd_pidr0: GICD_PIDR0,
    #[doc = "0xfe4 - Peripheral ID 1"]
    pub gicd_pidr1: GICD_PIDR1,
    #[doc = "0xfe8 - Peripheral ID 2"]
    pub gicd_pidr2: GICD_PIDR2,
    #[doc = "0xfec - Peripheral ID 3"]
    pub gicd_pidr3: GICD_PIDR3,
    #[doc = "0xff0 - Component ID 0"]
    pub gicd_cidr0: GICD_CIDR0,
    #[doc = "0xff4 - Component ID 1"]
    pub gicd_cidr1: GICD_CIDR1,
    #[doc = "0xff8 - Component ID 2"]
    pub gicd_cidr2: GICD_CIDR2,
    #[doc = "0xffc - Component ID 3"]
    pub gicd_cidr3: GICD_CIDR3,
}
#[doc = "GICD_CTLR (rw) register accessor: an alias for `Reg<GICD_CTLR_SPEC>`"]
pub type GICD_CTLR = crate::Reg<gicd_ctlr::GICD_CTLR_SPEC>;
#[doc = "Distributor Control Register"]
pub mod gicd_ctlr;
#[doc = "GICD_TYPER (r) register accessor: an alias for `Reg<GICD_TYPER_SPEC>`"]
pub type GICD_TYPER = crate::Reg<gicd_typer::GICD_TYPER_SPEC>;
#[doc = "Interrupt Controller Type Register"]
pub mod gicd_typer;
#[doc = "GICD_IIDR (r) register accessor: an alias for `Reg<GICD_IIDR_SPEC>`"]
pub type GICD_IIDR = crate::Reg<gicd_iidr::GICD_IIDR_SPEC>;
#[doc = "Distributor Implementer Identification Register"]
pub mod gicd_iidr;
#[doc = "Interrupt Group Registers"]
pub use self::gicd_igroupr::GICD_IGROUPR;
#[doc = r"Cluster"]
#[doc = "Interrupt Group Registers"]
pub mod gicd_igroupr;
#[doc = "Interrupt Set-Enable Registers"]
pub use self::gicd_isenabler::GICD_ISENABLER;
#[doc = r"Cluster"]
#[doc = "Interrupt Set-Enable Registers"]
pub mod gicd_isenabler;
#[doc = "Interrupt Clear-Enable Registers"]
pub use self::gicd_icenabler::GICD_ICENABLER;
#[doc = r"Cluster"]
#[doc = "Interrupt Clear-Enable Registers"]
pub mod gicd_icenabler;
#[doc = "Interrupt Set-Pending Registers"]
pub use self::gicd_ispendr::GICD_ISPENDR;
#[doc = r"Cluster"]
#[doc = "Interrupt Set-Pending Registers"]
pub mod gicd_ispendr;
#[doc = "Interrupt Clear-Pending Registers"]
pub use self::gicd_icpendr::GICD_ICPENDR;
#[doc = r"Cluster"]
#[doc = "Interrupt Clear-Pending Registers"]
pub mod gicd_icpendr;
#[doc = "Interrupt Set-Active Registers"]
pub use self::gicd_isactiver::GICD_ISACTIVER;
#[doc = r"Cluster"]
#[doc = "Interrupt Set-Active Registers"]
pub mod gicd_isactiver;
#[doc = "Interrupt Clear-Active Registers"]
pub use self::gicd_icactiver::GICD_ICACTIVER;
#[doc = r"Cluster"]
#[doc = "Interrupt Clear-Active Registers"]
pub mod gicd_icactiver;
#[doc = "Interrupt Priority"]
pub use self::gicd_ipriorityr::GICD_IPRIORITYR;
#[doc = r"Cluster"]
#[doc = "Interrupt Priority"]
pub mod gicd_ipriorityr;
#[doc = "Interrupt Processor Targets"]
pub use self::gicd_itargetsr::GICD_ITARGETSR;
#[doc = r"Cluster"]
#[doc = "Interrupt Processor Targets"]
pub mod gicd_itargetsr;
#[doc = "Interrupt Configuration"]
pub use self::gicd_icfgr::GICD_ICFGR;
#[doc = r"Cluster"]
#[doc = "Interrupt Configuration"]
pub mod gicd_icfgr;
#[doc = "GICD_PPISR (rw) register accessor: an alias for `Reg<GICD_PPISR_SPEC>`"]
pub type GICD_PPISR = crate::Reg<gicd_ppisr::GICD_PPISR_SPEC>;
#[doc = "Private Peripheral Interrupt Status Register"]
pub mod gicd_ppisr;
#[doc = "GICD_SPISR0 (rw) register accessor: an alias for `Reg<GICD_SPISR0_SPEC>`"]
pub type GICD_SPISR0 = crate::Reg<gicd_spisr0::GICD_SPISR0_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr0;
#[doc = "GICD_SPISR1 (rw) register accessor: an alias for `Reg<GICD_SPISR1_SPEC>`"]
pub type GICD_SPISR1 = crate::Reg<gicd_spisr1::GICD_SPISR1_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr1;
#[doc = "GICD_SPISR2 (rw) register accessor: an alias for `Reg<GICD_SPISR2_SPEC>`"]
pub type GICD_SPISR2 = crate::Reg<gicd_spisr2::GICD_SPISR2_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr2;
#[doc = "GICD_SPISR3 (rw) register accessor: an alias for `Reg<GICD_SPISR3_SPEC>`"]
pub type GICD_SPISR3 = crate::Reg<gicd_spisr3::GICD_SPISR3_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr3;
#[doc = "GICD_SPISR4 (rw) register accessor: an alias for `Reg<GICD_SPISR4_SPEC>`"]
pub type GICD_SPISR4 = crate::Reg<gicd_spisr4::GICD_SPISR4_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr4;
#[doc = "GICD_SPISR5 (rw) register accessor: an alias for `Reg<GICD_SPISR5_SPEC>`"]
pub type GICD_SPISR5 = crate::Reg<gicd_spisr5::GICD_SPISR5_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr5;
#[doc = "GICD_SGIR (w) register accessor: an alias for `Reg<GICD_SGIR_SPEC>`"]
pub type GICD_SGIR = crate::Reg<gicd_sgir::GICD_SGIR_SPEC>;
#[doc = "Software Generated Interrupt Register"]
pub mod gicd_sgir;
#[doc = "GICD_CPENDSGIRn (rw) register accessor: an alias for `Reg<GICD_CPENDSGIRN_SPEC>`"]
pub type GICD_CPENDSGIRN = crate::Reg<gicd_cpendsgirn::GICD_CPENDSGIRN_SPEC>;
#[doc = "SGI Clear-Pending Registers"]
pub mod gicd_cpendsgirn;
#[doc = "GICD_SPENDSGIRn (rw) register accessor: an alias for `Reg<GICD_SPENDSGIRN_SPEC>`"]
pub type GICD_SPENDSGIRN = crate::Reg<gicd_spendsgirn::GICD_SPENDSGIRN_SPEC>;
#[doc = "SGI Set-Pending Registers"]
pub mod gicd_spendsgirn;
#[doc = "GICD_PIDR4 (r) register accessor: an alias for `Reg<GICD_PIDR4_SPEC>`"]
pub type GICD_PIDR4 = crate::Reg<gicd_pidr4::GICD_PIDR4_SPEC>;
#[doc = "Peripheral ID 4"]
pub mod gicd_pidr4;
#[doc = "GICD_PIDR5 (r) register accessor: an alias for `Reg<GICD_PIDR5_SPEC>`"]
pub type GICD_PIDR5 = crate::Reg<gicd_pidr5::GICD_PIDR5_SPEC>;
#[doc = "Peripheral ID 5"]
pub mod gicd_pidr5;
#[doc = "GICD_PIDR6 (r) register accessor: an alias for `Reg<GICD_PIDR6_SPEC>`"]
pub type GICD_PIDR6 = crate::Reg<gicd_pidr6::GICD_PIDR6_SPEC>;
#[doc = "Peripheral ID 6"]
pub mod gicd_pidr6;
#[doc = "GICD_PIDR7 (r) register accessor: an alias for `Reg<GICD_PIDR7_SPEC>`"]
pub type GICD_PIDR7 = crate::Reg<gicd_pidr7::GICD_PIDR7_SPEC>;
#[doc = "Peripheral ID 7"]
pub mod gicd_pidr7;
#[doc = "GICD_PIDR0 (r) register accessor: an alias for `Reg<GICD_PIDR0_SPEC>`"]
pub type GICD_PIDR0 = crate::Reg<gicd_pidr0::GICD_PIDR0_SPEC>;
#[doc = "Peripheral ID 0"]
pub mod gicd_pidr0;
#[doc = "GICD_PIDR1 (r) register accessor: an alias for `Reg<GICD_PIDR1_SPEC>`"]
pub type GICD_PIDR1 = crate::Reg<gicd_pidr1::GICD_PIDR1_SPEC>;
#[doc = "Peripheral ID 1"]
pub mod gicd_pidr1;
#[doc = "GICD_PIDR2 (r) register accessor: an alias for `Reg<GICD_PIDR2_SPEC>`"]
pub type GICD_PIDR2 = crate::Reg<gicd_pidr2::GICD_PIDR2_SPEC>;
#[doc = "Peripheral ID 2"]
pub mod gicd_pidr2;
#[doc = "GICD_PIDR3 (r) register accessor: an alias for `Reg<GICD_PIDR3_SPEC>`"]
pub type GICD_PIDR3 = crate::Reg<gicd_pidr3::GICD_PIDR3_SPEC>;
#[doc = "Peripheral ID 3"]
pub mod gicd_pidr3;
#[doc = "GICD_CIDR0 (r) register accessor: an alias for `Reg<GICD_CIDR0_SPEC>`"]
pub type GICD_CIDR0 = crate::Reg<gicd_cidr0::GICD_CIDR0_SPEC>;
#[doc = "Component ID 0"]
pub mod gicd_cidr0;
#[doc = "GICD_CIDR1 (r) register accessor: an alias for `Reg<GICD_CIDR1_SPEC>`"]
pub type GICD_CIDR1 = crate::Reg<gicd_cidr1::GICD_CIDR1_SPEC>;
#[doc = "Component ID 1"]
pub mod gicd_cidr1;
#[doc = "GICD_CIDR2 (r) register accessor: an alias for `Reg<GICD_CIDR2_SPEC>`"]
pub type GICD_CIDR2 = crate::Reg<gicd_cidr2::GICD_CIDR2_SPEC>;
#[doc = "Component ID 2"]
pub mod gicd_cidr2;
#[doc = "GICD_CIDR3 (r) register accessor: an alias for `Reg<GICD_CIDR3_SPEC>`"]
pub type GICD_CIDR3 = crate::Reg<gicd_cidr3::GICD_CIDR3_SPEC>;
#[doc = "Component ID 3"]
pub mod gicd_cidr3;
