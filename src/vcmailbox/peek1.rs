#[doc = "Register `PEEK1` reader"]
pub struct R(crate::R<PEEK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEEK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEEK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEEK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEEK1` writer"]
pub struct W(crate::W<PEEK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEEK1_SPEC>;
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
impl From<crate::W<PEEK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEEK1_SPEC>) -> Self {
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
impl core::fmt::Debug for crate::generic::Reg<PEEK1_SPEC> {
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek1](index.html) module"]
pub struct PEEK1_SPEC;
impl crate::RegisterSpec for PEEK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peek1::R](R) reader structure"]
impl crate::Readable for PEEK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peek1::W](W) writer structure"]
impl crate::Writable for PEEK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
