#[doc = "Register `RNG2` reader"]
pub struct R(crate::R<RNG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNG2` writer"]
pub struct W(crate::W<RNG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG2_SPEC>;
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
impl From<crate::W<RNG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG2_SPEC>) -> Self {
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
impl core::fmt::Debug for crate::generic::Reg<RNG2_SPEC> {
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
#[doc = "Range for channel 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng2](index.html) module"]
pub struct RNG2_SPEC;
impl crate::RegisterSpec for RNG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng2::R](R) reader structure"]
impl crate::Readable for RNG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rng2::W](W) writer structure"]
impl crate::Writable for RNG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNG2 to value 0x20"]
impl crate::Resettable for RNG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
