#[doc = "Register `VID` reader"]
pub struct R(crate::R<VID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VID_SPEC>) -> Self {
        R(reader)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<VID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "OTG_HS vendor ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vid](index.html) module"]
pub struct VID_SPEC;
impl crate::RegisterSpec for VID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vid::R](R) reader structure"]
impl crate::Readable for VID_SPEC {
    type Reader = R;
}
