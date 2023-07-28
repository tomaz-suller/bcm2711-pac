#[doc = "Register `GICD_PIDR3` reader"]
pub struct R(crate::R<GICD_PIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GICD_PIDR3` reader - Peripheral ID 3"]
pub type GICD_PIDR3_R = crate::FieldReader<GICD_PIDR3_A>;
#[doc = "Peripheral ID 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR3_A {
    #[doc = "0: Valid"]
    VALID = 0,
}
impl From<GICD_PIDR3_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_PIDR3_A {
    type Ux = u32;
}
impl GICD_PIDR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GICD_PIDR3_A> {
        match self.bits {
            0 => Some(GICD_PIDR3_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR3_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 3"]
    #[inline(always)]
    pub fn gicd_pidr3(&self) -> GICD_PIDR3_R {
        GICD_PIDR3_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR3")
            .field("gicd_pidr3", &format_args!("{}", self.gicd_pidr3().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_PIDR3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Peripheral ID 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr3](index.html) module"]
pub struct GICD_PIDR3_SPEC;
impl crate::RegisterSpec for GICD_PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr3::R](R) reader structure"]
impl crate::Readable for GICD_PIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR3 to value 0"]
impl crate::Resettable for GICD_PIDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
