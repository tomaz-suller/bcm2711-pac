#[doc = "Register `GICD_ITARGETSR24` reader"]
pub struct R(crate::R<GICD_ITARGETSR24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR24` writer"]
pub struct W(crate::W<GICD_ITARGETSR24_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR24_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR24_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR24_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_0` reader - Timer 0"]
pub type TIMER_0_R = crate::FieldReader;
#[doc = "Field `TIMER_0` writer - Timer 0"]
pub type TIMER_0_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR24_SPEC, 8, O>;
#[doc = "Field `TIMER_1` reader - Timer 1"]
pub type TIMER_1_R = crate::FieldReader;
#[doc = "Field `TIMER_1` writer - Timer 1"]
pub type TIMER_1_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR24_SPEC, 8, O>;
#[doc = "Field `TIMER_2` reader - Timer 2"]
pub type TIMER_2_R = crate::FieldReader;
#[doc = "Field `TIMER_2` writer - Timer 2"]
pub type TIMER_2_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR24_SPEC, 8, O>;
#[doc = "Field `TIMER_3` reader - Timer 3"]
pub type TIMER_3_R = crate::FieldReader;
#[doc = "Field `TIMER_3` writer - Timer 3"]
pub type TIMER_3_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_ITARGETSR24_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timer 0"]
    #[inline(always)]
    pub fn timer_0(&self) -> TIMER_0_R {
        TIMER_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timer 1"]
    #[inline(always)]
    pub fn timer_1(&self) -> TIMER_1_R {
        TIMER_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Timer 2"]
    #[inline(always)]
    pub fn timer_2(&self) -> TIMER_2_R {
        TIMER_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Timer 3"]
    #[inline(always)]
    pub fn timer_3(&self) -> TIMER_3_R {
        TIMER_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR24")
            .field("timer_0", &format_args!("{}", self.timer_0().bits()))
            .field("timer_1", &format_args!("{}", self.timer_1().bits()))
            .field("timer_2", &format_args!("{}", self.timer_2().bits()))
            .field("timer_3", &format_args!("{}", self.timer_3().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR24_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer_0(&mut self) -> TIMER_0_W<0> {
        TIMER_0_W::new(self)
    }
    #[doc = "Bits 8:15 - Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer_1(&mut self) -> TIMER_1_W<8> {
        TIMER_1_W::new(self)
    }
    #[doc = "Bits 16:23 - Timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer_2(&mut self) -> TIMER_2_W<16> {
        TIMER_2_W::new(self)
    }
    #[doc = "Bits 24:31 - Timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_3(&mut self) -> TIMER_3_W<24> {
        TIMER_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 96 - 99\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr24](index.html) module"]
pub struct GICD_ITARGETSR24_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr24::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR24_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr24::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR24_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR24 to value 0"]
impl crate::Resettable for GICD_ITARGETSR24_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
