#[doc = "Register `GICD_IPRIORITYR31` reader"]
pub struct R(crate::R<GICD_IPRIORITYR31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR31` writer"]
pub struct W(crate::W<GICD_IPRIORITYR31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR31_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_14` reader - DMA 14"]
pub type DMA_14_R = crate::FieldReader;
#[doc = "Field `DMA_14` writer - DMA 14"]
pub type DMA_14_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR31_SPEC, 8, O>;
#[doc = "Field `AUX` reader - OR of UART1, SPI1 and SPI2"]
pub type AUX_R = crate::FieldReader;
#[doc = "Field `AUX` writer - OR of UART1, SPI1 and SPI2"]
pub type AUX_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR31_SPEC, 8, O>;
#[doc = "Field `ARM` reader - ARM"]
pub type ARM_R = crate::FieldReader;
#[doc = "Field `ARM` writer - ARM"]
pub type ARM_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR31_SPEC, 8, O>;
#[doc = "Field `DMA_15` reader - DMA 15"]
pub type DMA_15_R = crate::FieldReader;
#[doc = "Field `DMA_15` writer - DMA 15"]
pub type DMA_15_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR31_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DMA 14"]
    #[inline(always)]
    pub fn dma_14(&self) -> DMA_14_R {
        DMA_14_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(&self) -> AUX_R {
        AUX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ARM"]
    #[inline(always)]
    pub fn arm(&self) -> ARM_R {
        ARM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DMA 15"]
    #[inline(always)]
    pub fn dma_15(&self) -> DMA_15_R {
        DMA_15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR31")
            .field("dma_14", &format_args!("{}", self.dma_14().bits()))
            .field("aux", &format_args!("{}", self.aux().bits()))
            .field("arm", &format_args!("{}", self.arm().bits()))
            .field("dma_15", &format_args!("{}", self.dma_15().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR31_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA 14"]
    #[inline(always)]
    #[must_use]
    pub fn dma_14(&mut self) -> DMA_14_W<0> {
        DMA_14_W::new(self)
    }
    #[doc = "Bits 8:15 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn aux(&mut self) -> AUX_W<8> {
        AUX_W::new(self)
    }
    #[doc = "Bits 16:23 - ARM"]
    #[inline(always)]
    #[must_use]
    pub fn arm(&mut self) -> ARM_W<16> {
        ARM_W::new(self)
    }
    #[doc = "Bits 24:31 - DMA 15"]
    #[inline(always)]
    #[must_use]
    pub fn dma_15(&mut self) -> DMA_15_W<24> {
        DMA_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 124 - 127 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr31](index.html) module"]
pub struct GICD_IPRIORITYR31_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr31::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr31::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR31_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR31 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
