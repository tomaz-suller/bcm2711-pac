#[doc = "Register `INTERRUPT` reader"]
pub struct R(crate::R<INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT` writer"]
pub struct W(crate::W<INTERRUPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTERRUPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_DONE` reader - Command has finished"]
pub type CMD_DONE_R = crate::BitReader;
#[doc = "Field `CMD_DONE` writer - Command has finished"]
pub type CMD_DONE_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `DATA_DONE` reader - Data transfer has finished"]
pub type DATA_DONE_R = crate::BitReader;
#[doc = "Field `DATA_DONE` writer - Data transfer has finished"]
pub type DATA_DONE_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `BLOCK_GAP` reader - Data transfer has stopped at block gap"]
pub type BLOCK_GAP_R = crate::BitReader;
#[doc = "Field `BLOCK_GAP` writer - Data transfer has stopped at block gap"]
pub type BLOCK_GAP_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `WRITE_RDY` reader - DATA can be written to"]
pub type WRITE_RDY_R = crate::BitReader;
#[doc = "Field `WRITE_RDY` writer - DATA can be written to"]
pub type WRITE_RDY_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `READ_RDY` reader - DATA contains data to be read"]
pub type READ_RDY_R = crate::BitReader;
#[doc = "Field `READ_RDY` writer - DATA contains data to be read"]
pub type READ_RDY_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `CARD` reader - Card made interrupt request"]
pub type CARD_R = crate::BitReader;
#[doc = "Field `CARD` writer - Card made interrupt request"]
pub type CARD_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `RETUNE` reader - Clock retune request"]
pub type RETUNE_R = crate::BitReader;
#[doc = "Field `RETUNE` writer - Clock retune request"]
pub type RETUNE_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `BOOTACK` reader - Boot has been acknowledged"]
pub type BOOTACK_R = crate::BitReader;
#[doc = "Field `BOOTACK` writer - Boot has been acknowledged"]
pub type BOOTACK_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `ENDBOOT` reader - Boot operation has terminated"]
pub type ENDBOOT_R = crate::BitReader;
#[doc = "Field `ENDBOOT` writer - Boot operation has terminated"]
pub type ENDBOOT_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `ERR` reader - An error has occured"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `CTO_ERR` reader - Command timeout"]
pub type CTO_ERR_R = crate::BitReader;
#[doc = "Field `CTO_ERR` writer - Command timeout"]
pub type CTO_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `CCRC_ERR` reader - Command CRC error"]
pub type CCRC_ERR_R = crate::BitReader;
#[doc = "Field `CCRC_ERR` writer - Command CRC error"]
pub type CCRC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `CEND_ERR` reader - Command end bit error (not 1)"]
pub type CEND_ERR_R = crate::BitReader;
#[doc = "Field `CEND_ERR` writer - Command end bit error (not 1)"]
pub type CEND_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `CBAD_ERR` reader - Incorrect response command index"]
pub type CBAD_ERR_R = crate::BitReader;
#[doc = "Field `CBAD_ERR` writer - Incorrect response command index"]
pub type CBAD_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `DTO_ERR` reader - Data timeout"]
pub type DTO_ERR_R = crate::BitReader;
#[doc = "Field `DTO_ERR` writer - Data timeout"]
pub type DTO_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `DCRC_ERR` reader - Data CRC error"]
pub type DCRC_ERR_R = crate::BitReader;
#[doc = "Field `DCRC_ERR` writer - Data CRC error"]
pub type DCRC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `DEND_ERR` reader - Data end bit error (not 1)"]
pub type DEND_ERR_R = crate::BitReader;
#[doc = "Field `DEND_ERR` writer - Data end bit error (not 1)"]
pub type DEND_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
#[doc = "Field `ACMD_ERR` reader - Auto command error"]
pub type ACMD_ERR_R = crate::BitReader;
#[doc = "Field `ACMD_ERR` writer - Auto command error"]
pub type ACMD_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Command has finished"]
    #[inline(always)]
    pub fn cmd_done(&self) -> CMD_DONE_R {
        CMD_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data transfer has finished"]
    #[inline(always)]
    pub fn data_done(&self) -> DATA_DONE_R {
        DATA_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data transfer has stopped at block gap"]
    #[inline(always)]
    pub fn block_gap(&self) -> BLOCK_GAP_R {
        BLOCK_GAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DATA can be written to"]
    #[inline(always)]
    pub fn write_rdy(&self) -> WRITE_RDY_R {
        WRITE_RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DATA contains data to be read"]
    #[inline(always)]
    pub fn read_rdy(&self) -> READ_RDY_R {
        READ_RDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Card made interrupt request"]
    #[inline(always)]
    pub fn card(&self) -> CARD_R {
        CARD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Clock retune request"]
    #[inline(always)]
    pub fn retune(&self) -> RETUNE_R {
        RETUNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Boot has been acknowledged"]
    #[inline(always)]
    pub fn bootack(&self) -> BOOTACK_R {
        BOOTACK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot operation has terminated"]
    #[inline(always)]
    pub fn endboot(&self) -> ENDBOOT_R {
        ENDBOOT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - An error has occured"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Command timeout"]
    #[inline(always)]
    pub fn cto_err(&self) -> CTO_ERR_R {
        CTO_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC error"]
    #[inline(always)]
    pub fn ccrc_err(&self) -> CCRC_ERR_R {
        CCRC_ERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command end bit error (not 1)"]
    #[inline(always)]
    pub fn cend_err(&self) -> CEND_ERR_R {
        CEND_ERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Incorrect response command index"]
    #[inline(always)]
    pub fn cbad_err(&self) -> CBAD_ERR_R {
        CBAD_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data timeout"]
    #[inline(always)]
    pub fn dto_err(&self) -> DTO_ERR_R {
        DTO_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC error"]
    #[inline(always)]
    pub fn dcrc_err(&self) -> DCRC_ERR_R {
        DCRC_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data end bit error (not 1)"]
    #[inline(always)]
    pub fn dend_err(&self) -> DEND_ERR_R {
        DEND_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto command error"]
    #[inline(always)]
    pub fn acmd_err(&self) -> ACMD_ERR_R {
        ACMD_ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT")
            .field("acmd_err", &format_args!("{}", self.acmd_err().bit()))
            .field("dend_err", &format_args!("{}", self.dend_err().bit()))
            .field("dcrc_err", &format_args!("{}", self.dcrc_err().bit()))
            .field("dto_err", &format_args!("{}", self.dto_err().bit()))
            .field("cbad_err", &format_args!("{}", self.cbad_err().bit()))
            .field("cend_err", &format_args!("{}", self.cend_err().bit()))
            .field("ccrc_err", &format_args!("{}", self.ccrc_err().bit()))
            .field("cto_err", &format_args!("{}", self.cto_err().bit()))
            .field("err", &format_args!("{}", self.err().bit()))
            .field("endboot", &format_args!("{}", self.endboot().bit()))
            .field("bootack", &format_args!("{}", self.bootack().bit()))
            .field("retune", &format_args!("{}", self.retune().bit()))
            .field("card", &format_args!("{}", self.card().bit()))
            .field("read_rdy", &format_args!("{}", self.read_rdy().bit()))
            .field("write_rdy", &format_args!("{}", self.write_rdy().bit()))
            .field("block_gap", &format_args!("{}", self.block_gap().bit()))
            .field("data_done", &format_args!("{}", self.data_done().bit()))
            .field("cmd_done", &format_args!("{}", self.cmd_done().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERRUPT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Command has finished"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_done(&mut self) -> CMD_DONE_W<0> {
        CMD_DONE_W::new(self)
    }
    #[doc = "Bit 1 - Data transfer has finished"]
    #[inline(always)]
    #[must_use]
    pub fn data_done(&mut self) -> DATA_DONE_W<1> {
        DATA_DONE_W::new(self)
    }
    #[doc = "Bit 2 - Data transfer has stopped at block gap"]
    #[inline(always)]
    #[must_use]
    pub fn block_gap(&mut self) -> BLOCK_GAP_W<2> {
        BLOCK_GAP_W::new(self)
    }
    #[doc = "Bit 4 - DATA can be written to"]
    #[inline(always)]
    #[must_use]
    pub fn write_rdy(&mut self) -> WRITE_RDY_W<4> {
        WRITE_RDY_W::new(self)
    }
    #[doc = "Bit 5 - DATA contains data to be read"]
    #[inline(always)]
    #[must_use]
    pub fn read_rdy(&mut self) -> READ_RDY_W<5> {
        READ_RDY_W::new(self)
    }
    #[doc = "Bit 8 - Card made interrupt request"]
    #[inline(always)]
    #[must_use]
    pub fn card(&mut self) -> CARD_W<8> {
        CARD_W::new(self)
    }
    #[doc = "Bit 12 - Clock retune request"]
    #[inline(always)]
    #[must_use]
    pub fn retune(&mut self) -> RETUNE_W<12> {
        RETUNE_W::new(self)
    }
    #[doc = "Bit 13 - Boot has been acknowledged"]
    #[inline(always)]
    #[must_use]
    pub fn bootack(&mut self) -> BOOTACK_W<13> {
        BOOTACK_W::new(self)
    }
    #[doc = "Bit 14 - Boot operation has terminated"]
    #[inline(always)]
    #[must_use]
    pub fn endboot(&mut self) -> ENDBOOT_W<14> {
        ENDBOOT_W::new(self)
    }
    #[doc = "Bit 16 - Command timeout"]
    #[inline(always)]
    #[must_use]
    pub fn cto_err(&mut self) -> CTO_ERR_W<16> {
        CTO_ERR_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn ccrc_err(&mut self) -> CCRC_ERR_W<17> {
        CCRC_ERR_W::new(self)
    }
    #[doc = "Bit 18 - Command end bit error (not 1)"]
    #[inline(always)]
    #[must_use]
    pub fn cend_err(&mut self) -> CEND_ERR_W<18> {
        CEND_ERR_W::new(self)
    }
    #[doc = "Bit 19 - Incorrect response command index"]
    #[inline(always)]
    #[must_use]
    pub fn cbad_err(&mut self) -> CBAD_ERR_W<19> {
        CBAD_ERR_W::new(self)
    }
    #[doc = "Bit 20 - Data timeout"]
    #[inline(always)]
    #[must_use]
    pub fn dto_err(&mut self) -> DTO_ERR_W<20> {
        DTO_ERR_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn dcrc_err(&mut self) -> DCRC_ERR_W<21> {
        DCRC_ERR_W::new(self)
    }
    #[doc = "Bit 22 - Data end bit error (not 1)"]
    #[inline(always)]
    #[must_use]
    pub fn dend_err(&mut self) -> DEND_ERR_W<22> {
        DEND_ERR_W::new(self)
    }
    #[doc = "Bit 24 - Auto command error"]
    #[inline(always)]
    #[must_use]
    pub fn acmd_err(&mut self) -> ACMD_ERR_W<24> {
        ACMD_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt](index.html) module"]
pub struct INTERRUPT_SPEC;
impl crate::RegisterSpec for INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt::R](R) reader structure"]
impl crate::Readable for INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt::W](W) writer structure"]
impl crate::Writable for INTERRUPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERRUPT to value 0"]
impl crate::Resettable for INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
