#[doc = "Register `GICD_ITARGETSR47` reader"]
pub struct R(crate::R<GICD_ITARGETSR47_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR47_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR47_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR47_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR47` writer"]
pub struct W(crate::W<GICD_ITARGETSR47_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR47_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR47_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR47_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT188` reader - Interrupt 188"]
pub type INT188_R = crate::FieldReader;
#[doc = "Field `INT188` writer - Interrupt 188"]
pub type INT188_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR47_SPEC, 8, O>;
#[doc = "Field `INT189` reader - Interrupt 189"]
pub type INT189_R = crate::FieldReader;
#[doc = "Field `INT189` writer - Interrupt 189"]
pub type INT189_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR47_SPEC, 8, O>;
#[doc = "Field `INT190` reader - Interrupt 190"]
pub type INT190_R = crate::FieldReader;
#[doc = "Field `INT190` writer - Interrupt 190"]
pub type INT190_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR47_SPEC, 8, O>;
#[doc = "Field `INT191` reader - Interrupt 191"]
pub type INT191_R = crate::FieldReader;
#[doc = "Field `INT191` writer - Interrupt 191"]
pub type INT191_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR47_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 188"]
    #[inline(always)]
    pub fn int188(&self) -> INT188_R {
        INT188_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 189"]
    #[inline(always)]
    pub fn int189(&self) -> INT189_R {
        INT189_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 190"]
    #[inline(always)]
    pub fn int190(&self) -> INT190_R {
        INT190_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 191"]
    #[inline(always)]
    pub fn int191(&self) -> INT191_R {
        INT191_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR47")
            .field("int188", &format_args!("{}", self.int188().bits()))
            .field("int189", &format_args!("{}", self.int189().bits()))
            .field("int190", &format_args!("{}", self.int190().bits()))
            .field("int191", &format_args!("{}", self.int191().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR47_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 188"]
    #[inline(always)]
    #[must_use]
    pub fn int188(&mut self) -> INT188_W<0> {
        INT188_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 189"]
    #[inline(always)]
    #[must_use]
    pub fn int189(&mut self) -> INT189_W<8> {
        INT189_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 190"]
    #[inline(always)]
    #[must_use]
    pub fn int190(&mut self) -> INT190_W<16> {
        INT190_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 191"]
    #[inline(always)]
    #[must_use]
    pub fn int191(&mut self) -> INT191_W<24> {
        INT191_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 188 - 191\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr47](index.html) module"]
pub struct GICD_ITARGETSR47_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR47_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr47::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR47_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr47::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR47_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR47 to value 0"]
impl crate::Resettable for GICD_ITARGETSR47_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
