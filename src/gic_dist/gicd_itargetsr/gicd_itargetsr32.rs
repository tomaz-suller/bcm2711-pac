#[doc = "Register `GICD_ITARGETSR32` reader"]
pub struct R(crate::R<GICD_ITARGETSR32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR32` writer"]
pub struct W(crate::W<GICD_ITARGETSR32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR32_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDMI_CEC` reader - HDMI CEC"]
pub type HDMI_CEC_R = crate::FieldReader;
#[doc = "Field `HDMI_CEC` writer - HDMI CEC"]
pub type HDMI_CEC_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR32_SPEC, 8, O>;
#[doc = "Field `HVS` reader - HVS"]
pub type HVS_R = crate::FieldReader;
#[doc = "Field `HVS` writer - HVS"]
pub type HVS_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR32_SPEC, 8, O>;
#[doc = "Field `RPIVID` reader - RPIVID"]
pub type RPIVID_R = crate::FieldReader;
#[doc = "Field `RPIVID` writer - RPIVID"]
pub type RPIVID_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR32_SPEC, 8, O>;
#[doc = "Field `SDC` reader - SDC"]
pub type SDC_R = crate::FieldReader;
#[doc = "Field `SDC` writer - SDC"]
pub type SDC_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR32_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(&self) -> HDMI_CEC_R {
        HDMI_CEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - HVS"]
    #[inline(always)]
    pub fn hvs(&self) -> HVS_R {
        HVS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RPIVID"]
    #[inline(always)]
    pub fn rpivid(&self) -> RPIVID_R {
        RPIVID_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SDC"]
    #[inline(always)]
    pub fn sdc(&self) -> SDC_R {
        SDC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR32")
            .field("hdmi_cec", &format_args!("{}", self.hdmi_cec().bits()))
            .field("hvs", &format_args!("{}", self.hvs().bits()))
            .field("rpivid", &format_args!("{}", self.rpivid().bits()))
            .field("sdc", &format_args!("{}", self.sdc().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR32_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDMI CEC"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_cec(&mut self) -> HDMI_CEC_W<0> {
        HDMI_CEC_W::new(self)
    }
    #[doc = "Bits 8:15 - HVS"]
    #[inline(always)]
    #[must_use]
    pub fn hvs(&mut self) -> HVS_W<8> {
        HVS_W::new(self)
    }
    #[doc = "Bits 16:23 - RPIVID"]
    #[inline(always)]
    #[must_use]
    pub fn rpivid(&mut self) -> RPIVID_W<16> {
        RPIVID_W::new(self)
    }
    #[doc = "Bits 24:31 - SDC"]
    #[inline(always)]
    #[must_use]
    pub fn sdc(&mut self) -> SDC_W<24> {
        SDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 128 - 131\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr32](index.html) module"]
pub struct GICD_ITARGETSR32_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr32::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr32::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR32_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR32 to value 0"]
impl crate::Resettable for GICD_ITARGETSR32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
