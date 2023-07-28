#[doc = "Register `GICC_EOIR` writer"]
pub struct W(crate::W<GICC_EOIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_EOIR_SPEC>;
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
impl From<crate::W<GICC_EOIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_EOIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT_ID` writer - Interrupt ID"]
pub type INTERRUPT_ID_W<'a, const O: u8> = crate::FieldWriter<'a, GICC_EOIR_SPEC, 10, O, u16>;
#[doc = "Field `CPUID` writer - CPUID that requested a software interrupt, 0 otherwise"]
pub type CPUID_W<'a, const O: u8> = crate::FieldWriter<'a, GICC_EOIR_SPEC, 3, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICC_EOIR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:9 - Interrupt ID"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<0> {
        INTERRUPT_ID_W::new(self)
    }
    #[doc = "Bits 10:12 - CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<10> {
        CPUID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "End of Interrupt\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_eoir](index.html) module"]
pub struct GICC_EOIR_SPEC;
impl crate::RegisterSpec for GICC_EOIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gicc_eoir::W](W) writer structure"]
impl crate::Writable for GICC_EOIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICC_EOIR to value 0"]
impl crate::Resettable for GICC_EOIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
