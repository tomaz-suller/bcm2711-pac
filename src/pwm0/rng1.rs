#[doc = "Register `RNG1` reader"]
pub struct R(crate::R<RNG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNG1` writer"]
pub struct W(crate::W<RNG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG1_SPEC>;
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
impl From<crate::W<RNG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG1_SPEC>) -> Self {
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
impl core::fmt::Debug for crate::generic::Reg<RNG1_SPEC> {
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
#[doc = "Range for channel 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng1](index.html) module"]
pub struct RNG1_SPEC;
impl crate::RegisterSpec for RNG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng1::R](R) reader structure"]
impl crate::Readable for RNG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rng1::W](W) writer structure"]
impl crate::Writable for RNG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNG1 to value 0x20"]
impl crate::Resettable for RNG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
