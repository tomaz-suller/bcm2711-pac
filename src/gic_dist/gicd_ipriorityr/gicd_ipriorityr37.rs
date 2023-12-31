#[doc = "Register `GICD_IPRIORITYR37` reader"]
pub struct R(crate::R<GICD_IPRIORITYR37_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR37_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR37_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR37_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR37` writer"]
pub struct W(crate::W<GICD_IPRIORITYR37_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR37_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR37_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR37_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_3` reader - GPIO 3"]
pub type GPIO_3_R = crate::FieldReader;
#[doc = "Field `GPIO_3` writer - GPIO 3"]
pub type GPIO_3_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR37_SPEC, 8, O>;
#[doc = "Field `I2C` reader - OR of all I2C"]
pub type I2C_R = crate::FieldReader;
#[doc = "Field `I2C` writer - OR of all I2C"]
pub type I2C_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR37_SPEC, 8, O>;
#[doc = "Field `SPI` reader - OR of all SPI"]
pub type SPI_R = crate::FieldReader;
#[doc = "Field `SPI` writer - OR of all SPI"]
pub type SPI_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR37_SPEC, 8, O>;
#[doc = "Field `PCM_I2S` reader - PCM/I2S"]
pub type PCM_I2S_R = crate::FieldReader;
#[doc = "Field `PCM_I2S` writer - PCM/I2S"]
pub type PCM_I2S_W<'a, const O: u8> = crate::FieldWriter<'a, GICD_IPRIORITYR37_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(&self) -> GPIO_3_R {
        GPIO_3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OR of all I2C"]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - OR of all SPI"]
    #[inline(always)]
    pub fn spi(&self) -> SPI_R {
        SPI_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(&self) -> PCM_I2S_R {
        PCM_I2S_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR37")
            .field("gpio_3", &format_args!("{}", self.gpio_3().bits()))
            .field("i2c", &format_args!("{}", self.i2c().bits()))
            .field("spi", &format_args!("{}", self.spi().bits()))
            .field("pcm_i2s", &format_args!("{}", self.pcm_i2s().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR37_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_3(&mut self) -> GPIO_3_W<0> {
        GPIO_3_W::new(self)
    }
    #[doc = "Bits 8:15 - OR of all I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2C_W<8> {
        I2C_W::new(self)
    }
    #[doc = "Bits 16:23 - OR of all SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spi(&mut self) -> SPI_W<16> {
        SPI_W::new(self)
    }
    #[doc = "Bits 24:31 - PCM/I2S"]
    #[inline(always)]
    #[must_use]
    pub fn pcm_i2s(&mut self) -> PCM_I2S_W<24> {
        PCM_I2S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 148 - 151 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr37](index.html) module"]
pub struct GICD_IPRIORITYR37_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR37_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr37::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR37_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr37::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR37_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR37 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR37_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
