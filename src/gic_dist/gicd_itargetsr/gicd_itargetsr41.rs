#[doc = "Register `GICD_ITARGETSR41` reader"]
pub struct R(crate::R<GICD_ITARGETSR41_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR41_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR41_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR41_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR41` writer"]
pub struct W(crate::W<GICD_ITARGETSR41_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR41_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR41_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR41_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT164` reader - Interrupt 164"]
pub type INT164_R = crate::FieldReader;
#[doc = "Field `INT164` writer - Interrupt 164"]
pub type INT164_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR41_SPEC, 8, O>;
#[doc = "Field `INT165` reader - Interrupt 165"]
pub type INT165_R = crate::FieldReader;
#[doc = "Field `INT165` writer - Interrupt 165"]
pub type INT165_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR41_SPEC, 8, O>;
#[doc = "Field `INT166` reader - Interrupt 166"]
pub type INT166_R = crate::FieldReader;
#[doc = "Field `INT166` writer - Interrupt 166"]
pub type INT166_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR41_SPEC, 8, O>;
#[doc = "Field `INT167` reader - Interrupt 167"]
pub type INT167_R = crate::FieldReader;
#[doc = "Field `INT167` writer - Interrupt 167"]
pub type INT167_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR41_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 164"]
    #[inline(always)]
    pub fn int164(&self) -> INT164_R {
        INT164_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 165"]
    #[inline(always)]
    pub fn int165(&self) -> INT165_R {
        INT165_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 166"]
    #[inline(always)]
    pub fn int166(&self) -> INT166_R {
        INT166_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 167"]
    #[inline(always)]
    pub fn int167(&self) -> INT167_R {
        INT167_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR41")
            .field("int164", &format_args!("{}", self.int164().bits()))
            .field("int165", &format_args!("{}", self.int165().bits()))
            .field("int166", &format_args!("{}", self.int166().bits()))
            .field("int167", &format_args!("{}", self.int167().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR41_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 164"]
    #[inline(always)]
    #[must_use]
    pub fn int164(&mut self) -> INT164_W<0> {
        INT164_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 165"]
    #[inline(always)]
    #[must_use]
    pub fn int165(&mut self) -> INT165_W<8> {
        INT165_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 166"]
    #[inline(always)]
    #[must_use]
    pub fn int166(&mut self) -> INT166_W<16> {
        INT166_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 167"]
    #[inline(always)]
    #[must_use]
    pub fn int167(&mut self) -> INT167_W<24> {
        INT167_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 164 - 167\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr41](index.html) module"]
pub struct GICD_ITARGETSR41_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR41_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr41::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR41_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr41::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR41_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR41 to value 0"]
impl crate::Resettable for GICD_ITARGETSR41_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
