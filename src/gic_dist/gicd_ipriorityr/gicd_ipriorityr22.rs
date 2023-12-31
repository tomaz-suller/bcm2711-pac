#[doc = "Register `GICD_IPRIORITYR22` reader"]
pub struct R(crate::R<GICD_IPRIORITYR22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR22` writer"]
pub struct W(crate::W<GICD_IPRIORITYR22_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR22_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR22_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR22_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT88` reader - Interrupt 88"]
pub type INT88_R = crate::FieldReader;
#[doc = "Field `INT88` writer - Interrupt 88"]
pub type INT88_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR22_SPEC, 8, O>;
#[doc = "Field `INT89` reader - Interrupt 89"]
pub type INT89_R = crate::FieldReader;
#[doc = "Field `INT89` writer - Interrupt 89"]
pub type INT89_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR22_SPEC, 8, O>;
#[doc = "Field `INT90` reader - Interrupt 90"]
pub type INT90_R = crate::FieldReader;
#[doc = "Field `INT90` writer - Interrupt 90"]
pub type INT90_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR22_SPEC, 8, O>;
#[doc = "Field `INT91` reader - Interrupt 91"]
pub type INT91_R = crate::FieldReader;
#[doc = "Field `INT91` writer - Interrupt 91"]
pub type INT91_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR22_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 88"]
    #[inline(always)]
    pub fn int88(&self) -> INT88_R {
        INT88_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 89"]
    #[inline(always)]
    pub fn int89(&self) -> INT89_R {
        INT89_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 90"]
    #[inline(always)]
    pub fn int90(&self) -> INT90_R {
        INT90_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 91"]
    #[inline(always)]
    pub fn int91(&self) -> INT91_R {
        INT91_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR22")
            .field("int88", &format_args!("{}", self.int88().bits()))
            .field("int89", &format_args!("{}", self.int89().bits()))
            .field("int90", &format_args!("{}", self.int90().bits()))
            .field("int91", &format_args!("{}", self.int91().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR22_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 88"]
    #[inline(always)]
    #[must_use]
    pub fn int88(&mut self) -> INT88_W<0> {
        INT88_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 89"]
    #[inline(always)]
    #[must_use]
    pub fn int89(&mut self) -> INT89_W<8> {
        INT89_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 90"]
    #[inline(always)]
    #[must_use]
    pub fn int90(&mut self) -> INT90_W<16> {
        INT90_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 91"]
    #[inline(always)]
    #[must_use]
    pub fn int91(&mut self) -> INT91_W<24> {
        INT91_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 88 - 91 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr22](index.html) module"]
pub struct GICD_IPRIORITYR22_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr22::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR22_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr22::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR22_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR22 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
