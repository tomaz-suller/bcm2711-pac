#[doc = "Register `IMSC` reader"]
pub struct R(crate::R<IMSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMSC` writer"]
pub struct W(crate::W<IMSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMSC_SPEC>;
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
impl From<crate::W<IMSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIMIM` reader - RIMIM"]
pub type RIMIM_R = crate::BitReader;
#[doc = "Field `RIMIM` writer - RIMIM"]
pub type RIMIM_W<'a, const O: u8> = crate::BitWriter<'a, IMSC_SPEC, O>;
#[doc = "Field `CTSMIM` reader - CTSMIM"]
pub type CTSMIM_R = crate::BitReader;
#[doc = "Field `CTSMIM` writer - CTSMIM"]
pub type CTSMIM_W<'a, const O: u8> = crate::BitWriter<'a, IMSC_SPEC, O>;
#[doc = "Field `DCDMIM` reader - DCDMIM"]
pub type DCDMIM_R = crate::BitReader;
#[doc = "Field `DCDMIM` writer - DCDMIM"]
pub type DCDMIM_W<'a, const O: u8> = crate::BitWriter<'a, IMSC_SPEC, O>;
#[doc = "Field `DSRMIM` reader - DSRMIM"]
pub type DSRMIM_R = crate::BitReader;
#[doc = "Field `DSRMIM` writer - DSRMIM"]
pub type DSRMIM_W<'a, const O: u8> = crate::BitWriter<'a, IMSC_SPEC, O>;
#[doc = "Field `RXIM` reader - RXIM"]
pub type RXIM_R = crate::BitReader;
#[doc = "Field `RXIM` writer - RXIM"]
pub type RXIM_W<'a, const O: u8> = crate::BitWriter<'a, IMSC_SPEC, O>;
#[doc = "Field `TXIM` reader - TXIM"]
pub type TXIM_R = crate::BitReader;
#[doc = "Field `TXIM` writer - TXIM"]
pub type TXIM_W<'a, const O: u8> = crate::BitWriter<'a, IMSC_SPEC, O>;
#[doc = "Field `RTIM` reader - RTIM"]
pub type RTIM_R = crate::BitReader;
#[doc = "Field `RTIM` writer - RTIM"]
pub type RTIM_W<'a, const O: u8> = crate::BitWriter<'a, IMSC_SPEC, O>;
#[doc = "Field `FEIM` reader - FEIM"]
pub type FEIM_R = crate::BitReader;
#[doc = "Field `FEIM` writer - FEIM"]
pub type FEIM_W<'a, const O: u8> = crate::BitWriter<'a, IMSC_SPEC, O>;
#[doc = "Field `PEIM` reader - PEIM"]
pub type PEIM_R = crate::BitReader;
#[doc = "Field `PEIM` writer - PEIM"]
pub type PEIM_W<'a, const O: u8> = crate::BitWriter<'a, IMSC_SPEC, O>;
#[doc = "Field `BEIM` reader - BEIM"]
pub type BEIM_R = crate::BitReader;
#[doc = "Field `BEIM` writer - BEIM"]
pub type BEIM_W<'a, const O: u8> = crate::BitWriter<'a, IMSC_SPEC, O>;
#[doc = "Field `OEIM` reader - OEIM"]
pub type OEIM_R = crate::BitReader;
#[doc = "Field `OEIM` writer - OEIM"]
pub type OEIM_W<'a, const O: u8> = crate::BitWriter<'a, IMSC_SPEC, O>;
impl R {
    #[doc = "Bit 0 - RIMIM"]
    #[inline(always)]
    pub fn rimim(&self) -> RIMIM_R {
        RIMIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSMIM"]
    #[inline(always)]
    pub fn ctsmim(&self) -> CTSMIM_R {
        CTSMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCDMIM"]
    #[inline(always)]
    pub fn dcdmim(&self) -> DCDMIM_R {
        DCDMIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSRMIM"]
    #[inline(always)]
    pub fn dsrmim(&self) -> DSRMIM_R {
        DSRMIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXIM"]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXIM"]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTIM"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FEIM"]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PEIM"]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BEIM"]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OEIM"]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMSC")
            .field("rimim", &format_args!("{}", self.rimim().bit()))
            .field("ctsmim", &format_args!("{}", self.ctsmim().bit()))
            .field("dcdmim", &format_args!("{}", self.dcdmim().bit()))
            .field("dsrmim", &format_args!("{}", self.dsrmim().bit()))
            .field("rxim", &format_args!("{}", self.rxim().bit()))
            .field("txim", &format_args!("{}", self.txim().bit()))
            .field("rtim", &format_args!("{}", self.rtim().bit()))
            .field("feim", &format_args!("{}", self.feim().bit()))
            .field("peim", &format_args!("{}", self.peim().bit()))
            .field("beim", &format_args!("{}", self.beim().bit()))
            .field("oeim", &format_args!("{}", self.oeim().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IMSC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - RIMIM"]
    #[inline(always)]
    #[must_use]
    pub fn rimim(&mut self) -> RIMIM_W<0> {
        RIMIM_W::new(self)
    }
    #[doc = "Bit 1 - CTSMIM"]
    #[inline(always)]
    #[must_use]
    pub fn ctsmim(&mut self) -> CTSMIM_W<1> {
        CTSMIM_W::new(self)
    }
    #[doc = "Bit 2 - DCDMIM"]
    #[inline(always)]
    #[must_use]
    pub fn dcdmim(&mut self) -> DCDMIM_W<2> {
        DCDMIM_W::new(self)
    }
    #[doc = "Bit 3 - DSRMIM"]
    #[inline(always)]
    #[must_use]
    pub fn dsrmim(&mut self) -> DSRMIM_W<3> {
        DSRMIM_W::new(self)
    }
    #[doc = "Bit 4 - RXIM"]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RXIM_W<4> {
        RXIM_W::new(self)
    }
    #[doc = "Bit 5 - TXIM"]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TXIM_W<5> {
        TXIM_W::new(self)
    }
    #[doc = "Bit 6 - RTIM"]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RTIM_W<6> {
        RTIM_W::new(self)
    }
    #[doc = "Bit 7 - FEIM"]
    #[inline(always)]
    #[must_use]
    pub fn feim(&mut self) -> FEIM_W<7> {
        FEIM_W::new(self)
    }
    #[doc = "Bit 8 - PEIM"]
    #[inline(always)]
    #[must_use]
    pub fn peim(&mut self) -> PEIM_W<8> {
        PEIM_W::new(self)
    }
    #[doc = "Bit 9 - BEIM"]
    #[inline(always)]
    #[must_use]
    pub fn beim(&mut self) -> BEIM_W<9> {
        BEIM_W::new(self)
    }
    #[doc = "Bit 10 - OEIM"]
    #[inline(always)]
    #[must_use]
    pub fn oeim(&mut self) -> OEIM_W<10> {
        OEIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask set_Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imsc](index.html) module"]
pub struct IMSC_SPEC;
impl crate::RegisterSpec for IMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imsc::R](R) reader structure"]
impl crate::Readable for IMSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imsc::W](W) writer structure"]
impl crate::Writable for IMSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMSC to value 0"]
impl crate::Resettable for IMSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
