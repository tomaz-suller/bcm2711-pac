#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct GICD_ITARGETSR {
    #[doc = "0x00 - Interrupt Processor Target 0 - 3"]
    pub gicd_itargetsr0: GICD_ITARGETSR0,
    #[doc = "0x04 - Interrupt Processor Target 4 - 7"]
    pub gicd_itargetsr1: GICD_ITARGETSR1,
    #[doc = "0x08 - Interrupt Processor Target 8 - 11"]
    pub gicd_itargetsr2: GICD_ITARGETSR2,
    #[doc = "0x0c - Interrupt Processor Target 12 - 15"]
    pub gicd_itargetsr3: GICD_ITARGETSR3,
    #[doc = "0x10 - Interrupt Processor Target 16 - 19"]
    pub gicd_itargetsr4: GICD_ITARGETSR4,
    #[doc = "0x14 - Interrupt Processor Target 20 - 23"]
    pub gicd_itargetsr5: GICD_ITARGETSR5,
    #[doc = "0x18 - Interrupt Processor Target 24 - 27"]
    pub gicd_itargetsr6: GICD_ITARGETSR6,
    #[doc = "0x1c - Interrupt Processor Target 28 - 31"]
    pub gicd_itargetsr7: GICD_ITARGETSR7,
    #[doc = "0x20 - Interrupt Processor Target 32 - 35"]
    pub gicd_itargetsr8: GICD_ITARGETSR8,
    #[doc = "0x24 - Interrupt Processor Target 36 - 39"]
    pub gicd_itargetsr9: GICD_ITARGETSR9,
    #[doc = "0x28 - Interrupt Processor Target 40 - 43"]
    pub gicd_itargetsr10: GICD_ITARGETSR10,
    #[doc = "0x2c - Interrupt Processor Target 44 - 47"]
    pub gicd_itargetsr11: GICD_ITARGETSR11,
    #[doc = "0x30 - Interrupt Processor Target 48 - 51"]
    pub gicd_itargetsr12: GICD_ITARGETSR12,
    #[doc = "0x34 - Interrupt Processor Target 52 - 55"]
    pub gicd_itargetsr13: GICD_ITARGETSR13,
    #[doc = "0x38 - Interrupt Processor Target 56 - 59"]
    pub gicd_itargetsr14: GICD_ITARGETSR14,
    #[doc = "0x3c - Interrupt Processor Target 60 - 63"]
    pub gicd_itargetsr15: GICD_ITARGETSR15,
    #[doc = "0x40 - Interrupt Processor Target 64 - 67"]
    pub gicd_itargetsr16: GICD_ITARGETSR16,
    #[doc = "0x44 - Interrupt Processor Target 68 - 71"]
    pub gicd_itargetsr17: GICD_ITARGETSR17,
    #[doc = "0x48 - Interrupt Processor Target 72 - 75"]
    pub gicd_itargetsr18: GICD_ITARGETSR18,
    #[doc = "0x4c - Interrupt Processor Target 76 - 79"]
    pub gicd_itargetsr19: GICD_ITARGETSR19,
    #[doc = "0x50 - Interrupt Processor Target 80 - 83"]
    pub gicd_itargetsr20: GICD_ITARGETSR20,
    #[doc = "0x54 - Interrupt Processor Target 84 - 87"]
    pub gicd_itargetsr21: GICD_ITARGETSR21,
    #[doc = "0x58 - Interrupt Processor Target 88 - 91"]
    pub gicd_itargetsr22: GICD_ITARGETSR22,
    #[doc = "0x5c - Interrupt Processor Target 92 - 95"]
    pub gicd_itargetsr23: GICD_ITARGETSR23,
    #[doc = "0x60 - Interrupt Processor Target 96 - 99"]
    pub gicd_itargetsr24: GICD_ITARGETSR24,
    #[doc = "0x64 - Interrupt Processor Target 100 - 103"]
    pub gicd_itargetsr25: GICD_ITARGETSR25,
    #[doc = "0x68 - Interrupt Processor Target 104 - 107"]
    pub gicd_itargetsr26: GICD_ITARGETSR26,
    #[doc = "0x6c - Interrupt Processor Target 108 - 111"]
    pub gicd_itargetsr27: GICD_ITARGETSR27,
    #[doc = "0x70 - Interrupt Processor Target 112 - 115"]
    pub gicd_itargetsr28: GICD_ITARGETSR28,
    #[doc = "0x74 - Interrupt Processor Target 116 - 119"]
    pub gicd_itargetsr29: GICD_ITARGETSR29,
    #[doc = "0x78 - Interrupt Processor Target 120 - 123"]
    pub gicd_itargetsr30: GICD_ITARGETSR30,
    #[doc = "0x7c - Interrupt Processor Target 124 - 127"]
    pub gicd_itargetsr31: GICD_ITARGETSR31,
    #[doc = "0x80 - Interrupt Processor Target 128 - 131"]
    pub gicd_itargetsr32: GICD_ITARGETSR32,
    #[doc = "0x84 - Interrupt Processor Target 132 - 135"]
    pub gicd_itargetsr33: GICD_ITARGETSR33,
    #[doc = "0x88 - Interrupt Processor Target 136 - 139"]
    pub gicd_itargetsr34: GICD_ITARGETSR34,
    #[doc = "0x8c - Interrupt Processor Target 140 - 143"]
    pub gicd_itargetsr35: GICD_ITARGETSR35,
    #[doc = "0x90 - Interrupt Processor Target 144 - 147"]
    pub gicd_itargetsr36: GICD_ITARGETSR36,
    #[doc = "0x94 - Interrupt Processor Target 148 - 151"]
    pub gicd_itargetsr37: GICD_ITARGETSR37,
    #[doc = "0x98 - Interrupt Processor Target 152 - 155"]
    pub gicd_itargetsr38: GICD_ITARGETSR38,
    #[doc = "0x9c - Interrupt Processor Target 156 - 159"]
    pub gicd_itargetsr39: GICD_ITARGETSR39,
    #[doc = "0xa0 - Interrupt Processor Target 160 - 163"]
    pub gicd_itargetsr40: GICD_ITARGETSR40,
    #[doc = "0xa4 - Interrupt Processor Target 164 - 167"]
    pub gicd_itargetsr41: GICD_ITARGETSR41,
    #[doc = "0xa8 - Interrupt Processor Target 168 - 171"]
    pub gicd_itargetsr42: GICD_ITARGETSR42,
    #[doc = "0xac - Interrupt Processor Target 172 - 175"]
    pub gicd_itargetsr43: GICD_ITARGETSR43,
    #[doc = "0xb0 - Interrupt Processor Target 176 - 179"]
    pub gicd_itargetsr44: GICD_ITARGETSR44,
    #[doc = "0xb4 - Interrupt Processor Target 180 - 183"]
    pub gicd_itargetsr45: GICD_ITARGETSR45,
    #[doc = "0xb8 - Interrupt Processor Target 184 - 187"]
    pub gicd_itargetsr46: GICD_ITARGETSR46,
    #[doc = "0xbc - Interrupt Processor Target 188 - 191"]
    pub gicd_itargetsr47: GICD_ITARGETSR47,
    #[doc = "0xc0 - Interrupt Processor Target 192 - 195"]
    pub gicd_itargetsr48: GICD_ITARGETSR48,
    #[doc = "0xc4 - Interrupt Processor Target 196 - 199"]
    pub gicd_itargetsr49: GICD_ITARGETSR49,
    #[doc = "0xc8 - Interrupt Processor Target 200 - 203"]
    pub gicd_itargetsr50: GICD_ITARGETSR50,
    #[doc = "0xcc - Interrupt Processor Target 204 - 207"]
    pub gicd_itargetsr51: GICD_ITARGETSR51,
    #[doc = "0xd0 - Interrupt Processor Target 208 - 211"]
    pub gicd_itargetsr52: GICD_ITARGETSR52,
    #[doc = "0xd4 - Interrupt Processor Target 212 - 215"]
    pub gicd_itargetsr53: GICD_ITARGETSR53,
    #[doc = "0xd8 - Interrupt Processor Target 216 - 219"]
    pub gicd_itargetsr54: GICD_ITARGETSR54,
    #[doc = "0xdc - Interrupt Processor Target 220 - 223"]
    pub gicd_itargetsr55: GICD_ITARGETSR55,
}
#[doc = "GICD_ITARGETSR0 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR0_SPEC>`"]
pub type GICD_ITARGETSR0 = crate::Reg<gicd_itargetsr0::GICD_ITARGETSR0_SPEC>;
#[doc = "Interrupt Processor Target 0 - 3"]
pub mod gicd_itargetsr0;
#[doc = "GICD_ITARGETSR1 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR1_SPEC>`"]
pub type GICD_ITARGETSR1 = crate::Reg<gicd_itargetsr1::GICD_ITARGETSR1_SPEC>;
#[doc = "Interrupt Processor Target 4 - 7"]
pub mod gicd_itargetsr1;
#[doc = "GICD_ITARGETSR2 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR2_SPEC>`"]
pub type GICD_ITARGETSR2 = crate::Reg<gicd_itargetsr2::GICD_ITARGETSR2_SPEC>;
#[doc = "Interrupt Processor Target 8 - 11"]
pub mod gicd_itargetsr2;
#[doc = "GICD_ITARGETSR3 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR3_SPEC>`"]
pub type GICD_ITARGETSR3 = crate::Reg<gicd_itargetsr3::GICD_ITARGETSR3_SPEC>;
#[doc = "Interrupt Processor Target 12 - 15"]
pub mod gicd_itargetsr3;
#[doc = "GICD_ITARGETSR4 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR4_SPEC>`"]
pub type GICD_ITARGETSR4 = crate::Reg<gicd_itargetsr4::GICD_ITARGETSR4_SPEC>;
#[doc = "Interrupt Processor Target 16 - 19"]
pub mod gicd_itargetsr4;
#[doc = "GICD_ITARGETSR5 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR5_SPEC>`"]
pub type GICD_ITARGETSR5 = crate::Reg<gicd_itargetsr5::GICD_ITARGETSR5_SPEC>;
#[doc = "Interrupt Processor Target 20 - 23"]
pub mod gicd_itargetsr5;
#[doc = "GICD_ITARGETSR6 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR6_SPEC>`"]
pub type GICD_ITARGETSR6 = crate::Reg<gicd_itargetsr6::GICD_ITARGETSR6_SPEC>;
#[doc = "Interrupt Processor Target 24 - 27"]
pub mod gicd_itargetsr6;
#[doc = "GICD_ITARGETSR7 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR7_SPEC>`"]
pub type GICD_ITARGETSR7 = crate::Reg<gicd_itargetsr7::GICD_ITARGETSR7_SPEC>;
#[doc = "Interrupt Processor Target 28 - 31"]
pub mod gicd_itargetsr7;
#[doc = "GICD_ITARGETSR8 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR8_SPEC>`"]
pub type GICD_ITARGETSR8 = crate::Reg<gicd_itargetsr8::GICD_ITARGETSR8_SPEC>;
#[doc = "Interrupt Processor Target 32 - 35"]
pub mod gicd_itargetsr8;
#[doc = "GICD_ITARGETSR9 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR9_SPEC>`"]
pub type GICD_ITARGETSR9 = crate::Reg<gicd_itargetsr9::GICD_ITARGETSR9_SPEC>;
#[doc = "Interrupt Processor Target 36 - 39"]
pub mod gicd_itargetsr9;
#[doc = "GICD_ITARGETSR10 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR10_SPEC>`"]
pub type GICD_ITARGETSR10 = crate::Reg<gicd_itargetsr10::GICD_ITARGETSR10_SPEC>;
#[doc = "Interrupt Processor Target 40 - 43"]
pub mod gicd_itargetsr10;
#[doc = "GICD_ITARGETSR11 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR11_SPEC>`"]
pub type GICD_ITARGETSR11 = crate::Reg<gicd_itargetsr11::GICD_ITARGETSR11_SPEC>;
#[doc = "Interrupt Processor Target 44 - 47"]
pub mod gicd_itargetsr11;
#[doc = "GICD_ITARGETSR12 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR12_SPEC>`"]
pub type GICD_ITARGETSR12 = crate::Reg<gicd_itargetsr12::GICD_ITARGETSR12_SPEC>;
#[doc = "Interrupt Processor Target 48 - 51"]
pub mod gicd_itargetsr12;
#[doc = "GICD_ITARGETSR13 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR13_SPEC>`"]
pub type GICD_ITARGETSR13 = crate::Reg<gicd_itargetsr13::GICD_ITARGETSR13_SPEC>;
#[doc = "Interrupt Processor Target 52 - 55"]
pub mod gicd_itargetsr13;
#[doc = "GICD_ITARGETSR14 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR14_SPEC>`"]
pub type GICD_ITARGETSR14 = crate::Reg<gicd_itargetsr14::GICD_ITARGETSR14_SPEC>;
#[doc = "Interrupt Processor Target 56 - 59"]
pub mod gicd_itargetsr14;
#[doc = "GICD_ITARGETSR15 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR15_SPEC>`"]
pub type GICD_ITARGETSR15 = crate::Reg<gicd_itargetsr15::GICD_ITARGETSR15_SPEC>;
#[doc = "Interrupt Processor Target 60 - 63"]
pub mod gicd_itargetsr15;
#[doc = "GICD_ITARGETSR16 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR16_SPEC>`"]
pub type GICD_ITARGETSR16 = crate::Reg<gicd_itargetsr16::GICD_ITARGETSR16_SPEC>;
#[doc = "Interrupt Processor Target 64 - 67"]
pub mod gicd_itargetsr16;
#[doc = "GICD_ITARGETSR17 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR17_SPEC>`"]
pub type GICD_ITARGETSR17 = crate::Reg<gicd_itargetsr17::GICD_ITARGETSR17_SPEC>;
#[doc = "Interrupt Processor Target 68 - 71"]
pub mod gicd_itargetsr17;
#[doc = "GICD_ITARGETSR18 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR18_SPEC>`"]
pub type GICD_ITARGETSR18 = crate::Reg<gicd_itargetsr18::GICD_ITARGETSR18_SPEC>;
#[doc = "Interrupt Processor Target 72 - 75"]
pub mod gicd_itargetsr18;
#[doc = "GICD_ITARGETSR19 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR19_SPEC>`"]
pub type GICD_ITARGETSR19 = crate::Reg<gicd_itargetsr19::GICD_ITARGETSR19_SPEC>;
#[doc = "Interrupt Processor Target 76 - 79"]
pub mod gicd_itargetsr19;
#[doc = "GICD_ITARGETSR20 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR20_SPEC>`"]
pub type GICD_ITARGETSR20 = crate::Reg<gicd_itargetsr20::GICD_ITARGETSR20_SPEC>;
#[doc = "Interrupt Processor Target 80 - 83"]
pub mod gicd_itargetsr20;
#[doc = "GICD_ITARGETSR21 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR21_SPEC>`"]
pub type GICD_ITARGETSR21 = crate::Reg<gicd_itargetsr21::GICD_ITARGETSR21_SPEC>;
#[doc = "Interrupt Processor Target 84 - 87"]
pub mod gicd_itargetsr21;
#[doc = "GICD_ITARGETSR22 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR22_SPEC>`"]
pub type GICD_ITARGETSR22 = crate::Reg<gicd_itargetsr22::GICD_ITARGETSR22_SPEC>;
#[doc = "Interrupt Processor Target 88 - 91"]
pub mod gicd_itargetsr22;
#[doc = "GICD_ITARGETSR23 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR23_SPEC>`"]
pub type GICD_ITARGETSR23 = crate::Reg<gicd_itargetsr23::GICD_ITARGETSR23_SPEC>;
#[doc = "Interrupt Processor Target 92 - 95"]
pub mod gicd_itargetsr23;
#[doc = "GICD_ITARGETSR24 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR24_SPEC>`"]
pub type GICD_ITARGETSR24 = crate::Reg<gicd_itargetsr24::GICD_ITARGETSR24_SPEC>;
#[doc = "Interrupt Processor Target 96 - 99"]
pub mod gicd_itargetsr24;
#[doc = "GICD_ITARGETSR25 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR25_SPEC>`"]
pub type GICD_ITARGETSR25 = crate::Reg<gicd_itargetsr25::GICD_ITARGETSR25_SPEC>;
#[doc = "Interrupt Processor Target 100 - 103"]
pub mod gicd_itargetsr25;
#[doc = "GICD_ITARGETSR26 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR26_SPEC>`"]
pub type GICD_ITARGETSR26 = crate::Reg<gicd_itargetsr26::GICD_ITARGETSR26_SPEC>;
#[doc = "Interrupt Processor Target 104 - 107"]
pub mod gicd_itargetsr26;
#[doc = "GICD_ITARGETSR27 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR27_SPEC>`"]
pub type GICD_ITARGETSR27 = crate::Reg<gicd_itargetsr27::GICD_ITARGETSR27_SPEC>;
#[doc = "Interrupt Processor Target 108 - 111"]
pub mod gicd_itargetsr27;
#[doc = "GICD_ITARGETSR28 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR28_SPEC>`"]
pub type GICD_ITARGETSR28 = crate::Reg<gicd_itargetsr28::GICD_ITARGETSR28_SPEC>;
#[doc = "Interrupt Processor Target 112 - 115"]
pub mod gicd_itargetsr28;
#[doc = "GICD_ITARGETSR29 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR29_SPEC>`"]
pub type GICD_ITARGETSR29 = crate::Reg<gicd_itargetsr29::GICD_ITARGETSR29_SPEC>;
#[doc = "Interrupt Processor Target 116 - 119"]
pub mod gicd_itargetsr29;
#[doc = "GICD_ITARGETSR30 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR30_SPEC>`"]
pub type GICD_ITARGETSR30 = crate::Reg<gicd_itargetsr30::GICD_ITARGETSR30_SPEC>;
#[doc = "Interrupt Processor Target 120 - 123"]
pub mod gicd_itargetsr30;
#[doc = "GICD_ITARGETSR31 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR31_SPEC>`"]
pub type GICD_ITARGETSR31 = crate::Reg<gicd_itargetsr31::GICD_ITARGETSR31_SPEC>;
#[doc = "Interrupt Processor Target 124 - 127"]
pub mod gicd_itargetsr31;
#[doc = "GICD_ITARGETSR32 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR32_SPEC>`"]
pub type GICD_ITARGETSR32 = crate::Reg<gicd_itargetsr32::GICD_ITARGETSR32_SPEC>;
#[doc = "Interrupt Processor Target 128 - 131"]
pub mod gicd_itargetsr32;
#[doc = "GICD_ITARGETSR33 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR33_SPEC>`"]
pub type GICD_ITARGETSR33 = crate::Reg<gicd_itargetsr33::GICD_ITARGETSR33_SPEC>;
#[doc = "Interrupt Processor Target 132 - 135"]
pub mod gicd_itargetsr33;
#[doc = "GICD_ITARGETSR34 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR34_SPEC>`"]
pub type GICD_ITARGETSR34 = crate::Reg<gicd_itargetsr34::GICD_ITARGETSR34_SPEC>;
#[doc = "Interrupt Processor Target 136 - 139"]
pub mod gicd_itargetsr34;
#[doc = "GICD_ITARGETSR35 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR35_SPEC>`"]
pub type GICD_ITARGETSR35 = crate::Reg<gicd_itargetsr35::GICD_ITARGETSR35_SPEC>;
#[doc = "Interrupt Processor Target 140 - 143"]
pub mod gicd_itargetsr35;
#[doc = "GICD_ITARGETSR36 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR36_SPEC>`"]
pub type GICD_ITARGETSR36 = crate::Reg<gicd_itargetsr36::GICD_ITARGETSR36_SPEC>;
#[doc = "Interrupt Processor Target 144 - 147"]
pub mod gicd_itargetsr36;
#[doc = "GICD_ITARGETSR37 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR37_SPEC>`"]
pub type GICD_ITARGETSR37 = crate::Reg<gicd_itargetsr37::GICD_ITARGETSR37_SPEC>;
#[doc = "Interrupt Processor Target 148 - 151"]
pub mod gicd_itargetsr37;
#[doc = "GICD_ITARGETSR38 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR38_SPEC>`"]
pub type GICD_ITARGETSR38 = crate::Reg<gicd_itargetsr38::GICD_ITARGETSR38_SPEC>;
#[doc = "Interrupt Processor Target 152 - 155"]
pub mod gicd_itargetsr38;
#[doc = "GICD_ITARGETSR39 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR39_SPEC>`"]
pub type GICD_ITARGETSR39 = crate::Reg<gicd_itargetsr39::GICD_ITARGETSR39_SPEC>;
#[doc = "Interrupt Processor Target 156 - 159"]
pub mod gicd_itargetsr39;
#[doc = "GICD_ITARGETSR40 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR40_SPEC>`"]
pub type GICD_ITARGETSR40 = crate::Reg<gicd_itargetsr40::GICD_ITARGETSR40_SPEC>;
#[doc = "Interrupt Processor Target 160 - 163"]
pub mod gicd_itargetsr40;
#[doc = "GICD_ITARGETSR41 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR41_SPEC>`"]
pub type GICD_ITARGETSR41 = crate::Reg<gicd_itargetsr41::GICD_ITARGETSR41_SPEC>;
#[doc = "Interrupt Processor Target 164 - 167"]
pub mod gicd_itargetsr41;
#[doc = "GICD_ITARGETSR42 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR42_SPEC>`"]
pub type GICD_ITARGETSR42 = crate::Reg<gicd_itargetsr42::GICD_ITARGETSR42_SPEC>;
#[doc = "Interrupt Processor Target 168 - 171"]
pub mod gicd_itargetsr42;
#[doc = "GICD_ITARGETSR43 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR43_SPEC>`"]
pub type GICD_ITARGETSR43 = crate::Reg<gicd_itargetsr43::GICD_ITARGETSR43_SPEC>;
#[doc = "Interrupt Processor Target 172 - 175"]
pub mod gicd_itargetsr43;
#[doc = "GICD_ITARGETSR44 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR44_SPEC>`"]
pub type GICD_ITARGETSR44 = crate::Reg<gicd_itargetsr44::GICD_ITARGETSR44_SPEC>;
#[doc = "Interrupt Processor Target 176 - 179"]
pub mod gicd_itargetsr44;
#[doc = "GICD_ITARGETSR45 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR45_SPEC>`"]
pub type GICD_ITARGETSR45 = crate::Reg<gicd_itargetsr45::GICD_ITARGETSR45_SPEC>;
#[doc = "Interrupt Processor Target 180 - 183"]
pub mod gicd_itargetsr45;
#[doc = "GICD_ITARGETSR46 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR46_SPEC>`"]
pub type GICD_ITARGETSR46 = crate::Reg<gicd_itargetsr46::GICD_ITARGETSR46_SPEC>;
#[doc = "Interrupt Processor Target 184 - 187"]
pub mod gicd_itargetsr46;
#[doc = "GICD_ITARGETSR47 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR47_SPEC>`"]
pub type GICD_ITARGETSR47 = crate::Reg<gicd_itargetsr47::GICD_ITARGETSR47_SPEC>;
#[doc = "Interrupt Processor Target 188 - 191"]
pub mod gicd_itargetsr47;
#[doc = "GICD_ITARGETSR48 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR48_SPEC>`"]
pub type GICD_ITARGETSR48 = crate::Reg<gicd_itargetsr48::GICD_ITARGETSR48_SPEC>;
#[doc = "Interrupt Processor Target 192 - 195"]
pub mod gicd_itargetsr48;
#[doc = "GICD_ITARGETSR49 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR49_SPEC>`"]
pub type GICD_ITARGETSR49 = crate::Reg<gicd_itargetsr49::GICD_ITARGETSR49_SPEC>;
#[doc = "Interrupt Processor Target 196 - 199"]
pub mod gicd_itargetsr49;
#[doc = "GICD_ITARGETSR50 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR50_SPEC>`"]
pub type GICD_ITARGETSR50 = crate::Reg<gicd_itargetsr50::GICD_ITARGETSR50_SPEC>;
#[doc = "Interrupt Processor Target 200 - 203"]
pub mod gicd_itargetsr50;
#[doc = "GICD_ITARGETSR51 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR51_SPEC>`"]
pub type GICD_ITARGETSR51 = crate::Reg<gicd_itargetsr51::GICD_ITARGETSR51_SPEC>;
#[doc = "Interrupt Processor Target 204 - 207"]
pub mod gicd_itargetsr51;
#[doc = "GICD_ITARGETSR52 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR52_SPEC>`"]
pub type GICD_ITARGETSR52 = crate::Reg<gicd_itargetsr52::GICD_ITARGETSR52_SPEC>;
#[doc = "Interrupt Processor Target 208 - 211"]
pub mod gicd_itargetsr52;
#[doc = "GICD_ITARGETSR53 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR53_SPEC>`"]
pub type GICD_ITARGETSR53 = crate::Reg<gicd_itargetsr53::GICD_ITARGETSR53_SPEC>;
#[doc = "Interrupt Processor Target 212 - 215"]
pub mod gicd_itargetsr53;
#[doc = "GICD_ITARGETSR54 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR54_SPEC>`"]
pub type GICD_ITARGETSR54 = crate::Reg<gicd_itargetsr54::GICD_ITARGETSR54_SPEC>;
#[doc = "Interrupt Processor Target 216 - 219"]
pub mod gicd_itargetsr54;
#[doc = "GICD_ITARGETSR55 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR55_SPEC>`"]
pub type GICD_ITARGETSR55 = crate::Reg<gicd_itargetsr55::GICD_ITARGETSR55_SPEC>;
#[doc = "Interrupt Processor Target 220 - 223"]
pub mod gicd_itargetsr55;
