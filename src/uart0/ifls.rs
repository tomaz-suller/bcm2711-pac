#[doc = "Register `IFLS` reader"]
pub struct R(crate::R<IFLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFLS` writer"]
pub struct W(crate::W<IFLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFLS_SPEC>;
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
impl From<crate::W<IFLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXIFLSEL` reader - TXIFLSEL"]
pub type TXIFLSEL_R = crate::FieldReader;
#[doc = "Field `TXIFLSEL` writer - TXIFLSEL"]
pub type TXIFLSEL_W<'a, const O: u8> = crate::FieldWriter<'a, IFLS_SPEC, 3, O>;
#[doc = "Field `RXIFLSEL` reader - RXIFLSEL"]
pub type RXIFLSEL_R = crate::FieldReader;
#[doc = "Field `RXIFLSEL` writer - RXIFLSEL"]
pub type RXIFLSEL_W<'a, const O: u8> = crate::FieldWriter<'a, IFLS_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:2 - TXIFLSEL"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TXIFLSEL_R {
        TXIFLSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - RXIFLSEL"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RXIFLSEL_R {
        RXIFLSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFLS")
            .field("txiflsel", &format_args!("{}", self.txiflsel().bits()))
            .field("rxiflsel", &format_args!("{}", self.rxiflsel().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IFLS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - TXIFLSEL"]
    #[inline(always)]
    #[must_use]
    pub fn txiflsel(&mut self) -> TXIFLSEL_W<0> {
        TXIFLSEL_W::new(self)
    }
    #[doc = "Bits 3:5 - RXIFLSEL"]
    #[inline(always)]
    #[must_use]
    pub fn rxiflsel(&mut self) -> RXIFLSEL_W<3> {
        RXIFLSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt FIFO Level Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifls](index.html) module"]
pub struct IFLS_SPEC;
impl crate::RegisterSpec for IFLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifls::R](R) reader structure"]
impl crate::Readable for IFLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifls::W](W) writer structure"]
impl crate::Writable for IFLS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFLS to value 0"]
impl crate::Resettable for IFLS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
