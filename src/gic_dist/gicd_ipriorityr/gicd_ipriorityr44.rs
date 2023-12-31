#[doc = "Register `GICD_IPRIORITYR44` reader"]
pub struct R(crate::R<GICD_IPRIORITYR44_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR44_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR44_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR44_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR44` writer"]
pub struct W(crate::W<GICD_IPRIORITYR44_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR44_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR44_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR44_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT176` reader - Interrupt 176"]
pub type INT176_R = crate::FieldReader;
#[doc = "Field `INT176` writer - Interrupt 176"]
pub type INT176_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR44_SPEC, 8, O>;
#[doc = "Field `INT177` reader - Interrupt 177"]
pub type INT177_R = crate::FieldReader;
#[doc = "Field `INT177` writer - Interrupt 177"]
pub type INT177_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR44_SPEC, 8, O>;
#[doc = "Field `INT178` reader - Interrupt 178"]
pub type INT178_R = crate::FieldReader;
#[doc = "Field `INT178` writer - Interrupt 178"]
pub type INT178_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR44_SPEC, 8, O>;
#[doc = "Field `INT179` reader - Interrupt 179"]
pub type INT179_R = crate::FieldReader;
#[doc = "Field `INT179` writer - Interrupt 179"]
pub type INT179_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR44_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 176"]
    #[inline(always)]
    pub fn int176(&self) -> INT176_R {
        INT176_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 177"]
    #[inline(always)]
    pub fn int177(&self) -> INT177_R {
        INT177_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 178"]
    #[inline(always)]
    pub fn int178(&self) -> INT178_R {
        INT178_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 179"]
    #[inline(always)]
    pub fn int179(&self) -> INT179_R {
        INT179_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR44")
            .field("int176", &format_args!("{}", self.int176().bits()))
            .field("int177", &format_args!("{}", self.int177().bits()))
            .field("int178", &format_args!("{}", self.int178().bits()))
            .field("int179", &format_args!("{}", self.int179().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR44_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 176"]
    #[inline(always)]
    #[must_use]
    pub fn int176(&mut self) -> INT176_W<0> {
        INT176_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 177"]
    #[inline(always)]
    #[must_use]
    pub fn int177(&mut self) -> INT177_W<8> {
        INT177_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 178"]
    #[inline(always)]
    #[must_use]
    pub fn int178(&mut self) -> INT178_W<16> {
        INT178_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 179"]
    #[inline(always)]
    #[must_use]
    pub fn int179(&mut self) -> INT179_W<24> {
        INT179_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 176 - 179 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr44](index.html) module"]
pub struct GICD_IPRIORITYR44_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR44_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr44::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR44_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr44::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR44_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR44 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR44_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
