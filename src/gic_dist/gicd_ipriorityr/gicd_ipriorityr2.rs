#[doc = "Register `GICD_IPRIORITYR2` reader"]
pub struct R(crate::R<GICD_IPRIORITYR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR2` writer"]
pub struct W(crate::W<GICD_IPRIORITYR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR2_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT8` reader - Interrupt 8"]
pub type INT8_R = crate::FieldReader;
#[doc = "Field `INT8` writer - Interrupt 8"]
pub type INT8_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR2_SPEC, 8, O>;
#[doc = "Field `INT9` reader - Interrupt 9"]
pub type INT9_R = crate::FieldReader;
#[doc = "Field `INT9` writer - Interrupt 9"]
pub type INT9_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR2_SPEC, 8, O>;
#[doc = "Field `INT10` reader - Interrupt 10"]
pub type INT10_R = crate::FieldReader;
#[doc = "Field `INT10` writer - Interrupt 10"]
pub type INT10_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR2_SPEC, 8, O>;
#[doc = "Field `INT11` reader - Interrupt 11"]
pub type INT11_R = crate::FieldReader;
#[doc = "Field `INT11` writer - Interrupt 11"]
pub type INT11_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR2_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR2")
            .field("int8", &format_args!("{}", self.int8().bits()))
            .field("int9", &format_args!("{}", self.int9().bits()))
            .field("int10", &format_args!("{}", self.int10().bits()))
            .field("int11", &format_args!("{}", self.int11().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn int8(&mut self) -> INT8_W<0> {
        INT8_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> INT9_W<8> {
        INT9_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn int10(&mut self) -> INT10_W<16> {
        INT10_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> INT11_W<24> {
        INT11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 8 - 11 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr2](index.html) module"]
pub struct GICD_IPRIORITYR2_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr2::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr2::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR2 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
