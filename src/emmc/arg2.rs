#[doc = "Register `ARG2` reader"]
pub struct R(crate::R<ARG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARG2` writer"]
pub struct W(crate::W<ARG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARG2_SPEC>;
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
impl From<crate::W<ARG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARG2_SPEC>) -> Self {
        W(writer)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<ARG2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Argument for ACMD23 command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arg2](index.html) module"]
pub struct ARG2_SPEC;
impl crate::RegisterSpec for ARG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arg2::R](R) reader structure"]
impl crate::Readable for ARG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arg2::W](W) writer structure"]
impl crate::Writable for ARG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARG2 to value 0"]
impl crate::Resettable for ARG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
