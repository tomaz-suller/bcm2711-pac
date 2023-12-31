#[doc = "Register `DLEN` reader"]
pub struct R(crate::R<DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLEN` writer"]
pub struct W(crate::W<DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLEN_SPEC>;
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
impl From<crate::W<DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLEN` reader - Data length"]
pub type DLEN_R = crate::FieldReader<u16>;
#[doc = "Field `DLEN` writer - Data length"]
pub type DLEN_W<'a, const O: u8> = crate::FieldWriter<'a, DLEN_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Data length"]
    #[inline(always)]
    pub fn dlen(&self) -> DLEN_R {
        DLEN_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLEN")
            .field("dlen", &format_args!("{}", self.dlen().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data length"]
    #[inline(always)]
    #[must_use]
    pub fn dlen(&mut self) -> DLEN_W<0> {
        DLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlen](index.html) module"]
pub struct DLEN_SPEC;
impl crate::RegisterSpec for DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlen::R](R) reader structure"]
impl crate::Readable for DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlen::W](W) writer structure"]
impl crate::Writable for DLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLEN to value 0"]
impl crate::Resettable for DLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
