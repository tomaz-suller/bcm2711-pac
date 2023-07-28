#[doc = "Register `READ` reader"]
pub struct R(crate::R<READ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READ_SPEC>) -> Self {
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
impl core::fmt::Debug for crate::generic::Reg<READ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Read messages from the VideoCore\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [read](index.html) module"]
pub struct READ_SPEC;
impl crate::RegisterSpec for READ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [read::R](R) reader structure"]
impl crate::Readable for READ_SPEC {
    type Reader = R;
}
