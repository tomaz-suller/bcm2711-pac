#[doc = "Register `GICD_IPRIORITYR38` reader"]
pub struct R(crate::R<GICD_IPRIORITYR38_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR38_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR38_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR38_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR38` writer"]
pub struct W(crate::W<GICD_IPRIORITYR38_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR38_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR38_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR38_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDHOST` reader - SDHOST"]
pub type SDHOST_R = crate::FieldReader;
#[doc = "Field `SDHOST` writer - SDHOST"]
pub type SDHOST_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR38_SPEC, 8, O>;
#[doc = "Field `UART` reader - OR of all PL011 UARTs"]
pub type UART_R = crate::FieldReader;
#[doc = "Field `UART` writer - OR of all PL011 UARTs"]
pub type UART_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR38_SPEC, 8, O>;
#[doc = "Field `ETH_PCIE` reader - OR of all ETH_PCIe L2"]
pub type ETH_PCIE_R = crate::FieldReader;
#[doc = "Field `ETH_PCIE` writer - OR of all ETH_PCIe L2"]
pub type ETH_PCIE_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR38_SPEC, 8, O>;
#[doc = "Field `VEC` reader - VEC"]
pub type VEC_R = crate::FieldReader;
#[doc = "Field `VEC` writer - VEC"]
pub type VEC_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR38_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SDHOST"]
    #[inline(always)]
    pub fn sdhost(&self) -> SDHOST_R {
        SDHOST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(&self) -> UART_R {
        UART_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(&self) -> ETH_PCIE_R {
        ETH_PCIE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - VEC"]
    #[inline(always)]
    pub fn vec(&self) -> VEC_R {
        VEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR38")
            .field("sdhost", &format_args!("{}", self.sdhost().bits()))
            .field("uart", &format_args!("{}", self.uart().bits()))
            .field("eth_pcie", &format_args!("{}", self.eth_pcie().bits()))
            .field("vec", &format_args!("{}", self.vec().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR38_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDHOST"]
    #[inline(always)]
    #[must_use]
    pub fn sdhost(&mut self) -> SDHOST_W<0> {
        SDHOST_W::new(self)
    }
    #[doc = "Bits 8:15 - OR of all PL011 UARTs"]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UART_W<8> {
        UART_W::new(self)
    }
    #[doc = "Bits 16:23 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie(&mut self) -> ETH_PCIE_W<16> {
        ETH_PCIE_W::new(self)
    }
    #[doc = "Bits 24:31 - VEC"]
    #[inline(always)]
    #[must_use]
    pub fn vec(&mut self) -> VEC_W<24> {
        VEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 152 - 155 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr38](index.html) module"]
pub struct GICD_IPRIORITYR38_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR38_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr38::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR38_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr38::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR38_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR38 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR38_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
