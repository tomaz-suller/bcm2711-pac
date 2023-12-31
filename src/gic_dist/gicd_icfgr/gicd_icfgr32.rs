#[doc = "Register `GICD_ICFGR32` reader"]
pub struct R(crate::R<GICD_ICFGR32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR32` writer"]
pub struct W(crate::W<GICD_ICFGR32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR32_SPEC>;
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
impl From<crate::W<GICD_ICFGR32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDMI_CEC` reader - HDMI CEC"]
pub type HDMI_CEC_R = crate::BitReader<HDMI_CEC_A>;
#[doc = "HDMI CEC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDMI_CEC_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<HDMI_CEC_A> for bool {
    #[inline(always)]
    fn from(variant: HDMI_CEC_A) -> Self {
        variant as u8 != 0
    }
}
impl HDMI_CEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDMI_CEC_A {
        match self.bits {
            false => HDMI_CEC_A::LEVEL,
            true => HDMI_CEC_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == HDMI_CEC_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == HDMI_CEC_A::EDGE
    }
}
#[doc = "Field `HDMI_CEC` writer - HDMI CEC"]
pub type HDMI_CEC_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, HDMI_CEC_A>;
impl<'a, const O: u8> HDMI_CEC_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(HDMI_CEC_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(HDMI_CEC_A::EDGE)
    }
}
#[doc = "Field `HVS` reader - HVS"]
pub type HVS_R = crate::BitReader<HVS_A>;
#[doc = "HVS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVS_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<HVS_A> for bool {
    #[inline(always)]
    fn from(variant: HVS_A) -> Self {
        variant as u8 != 0
    }
}
impl HVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVS_A {
        match self.bits {
            false => HVS_A::LEVEL,
            true => HVS_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == HVS_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == HVS_A::EDGE
    }
}
#[doc = "Field `HVS` writer - HVS"]
pub type HVS_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, HVS_A>;
impl<'a, const O: u8> HVS_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(HVS_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(HVS_A::EDGE)
    }
}
#[doc = "Field `RPIVID` reader - RPIVID"]
pub type RPIVID_R = crate::BitReader<RPIVID_A>;
#[doc = "RPIVID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIVID_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<RPIVID_A> for bool {
    #[inline(always)]
    fn from(variant: RPIVID_A) -> Self {
        variant as u8 != 0
    }
}
impl RPIVID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPIVID_A {
        match self.bits {
            false => RPIVID_A::LEVEL,
            true => RPIVID_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == RPIVID_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == RPIVID_A::EDGE
    }
}
#[doc = "Field `RPIVID` writer - RPIVID"]
pub type RPIVID_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, RPIVID_A>;
impl<'a, const O: u8> RPIVID_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(RPIVID_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(RPIVID_A::EDGE)
    }
}
#[doc = "Field `SDC` reader - SDC"]
pub type SDC_R = crate::BitReader<SDC_A>;
#[doc = "SDC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDC_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SDC_A> for bool {
    #[inline(always)]
    fn from(variant: SDC_A) -> Self {
        variant as u8 != 0
    }
}
impl SDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDC_A {
        match self.bits {
            false => SDC_A::LEVEL,
            true => SDC_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SDC_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SDC_A::EDGE
    }
}
#[doc = "Field `SDC` writer - SDC"]
pub type SDC_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, SDC_A>;
impl<'a, const O: u8> SDC_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SDC_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SDC_A::EDGE)
    }
}
#[doc = "Field `DSI_0` reader - DSI 0"]
pub type DSI_0_R = crate::BitReader<DSI_0_A>;
#[doc = "DSI 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSI_0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DSI_0_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_0_A) -> Self {
        variant as u8 != 0
    }
}
impl DSI_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_0_A {
        match self.bits {
            false => DSI_0_A::LEVEL,
            true => DSI_0_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DSI_0_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DSI_0_A::EDGE
    }
}
#[doc = "Field `DSI_0` writer - DSI 0"]
pub type DSI_0_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, DSI_0_A>;
impl<'a, const O: u8> DSI_0_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(DSI_0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(DSI_0_A::EDGE)
    }
}
#[doc = "Field `PIXEL_VALVE_2` reader - Pixel Valve 2"]
pub type PIXEL_VALVE_2_R = crate::BitReader<PIXEL_VALVE_2_A>;
#[doc = "Pixel Valve 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIXEL_VALVE_2_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<PIXEL_VALVE_2_A> for bool {
    #[inline(always)]
    fn from(variant: PIXEL_VALVE_2_A) -> Self {
        variant as u8 != 0
    }
}
impl PIXEL_VALVE_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXEL_VALVE_2_A {
        match self.bits {
            false => PIXEL_VALVE_2_A::LEVEL,
            true => PIXEL_VALVE_2_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PIXEL_VALVE_2_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == PIXEL_VALVE_2_A::EDGE
    }
}
#[doc = "Field `PIXEL_VALVE_2` writer - Pixel Valve 2"]
pub type PIXEL_VALVE_2_W<'a, const O: u8> =
    crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, PIXEL_VALVE_2_A>;
impl<'a, const O: u8> PIXEL_VALVE_2_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(PIXEL_VALVE_2_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(PIXEL_VALVE_2_A::EDGE)
    }
}
#[doc = "Field `CAMERA_0` reader - Camera 0"]
pub type CAMERA_0_R = crate::BitReader<CAMERA_0_A>;
#[doc = "Camera 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAMERA_0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<CAMERA_0_A> for bool {
    #[inline(always)]
    fn from(variant: CAMERA_0_A) -> Self {
        variant as u8 != 0
    }
}
impl CAMERA_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAMERA_0_A {
        match self.bits {
            false => CAMERA_0_A::LEVEL,
            true => CAMERA_0_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == CAMERA_0_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == CAMERA_0_A::EDGE
    }
}
#[doc = "Field `CAMERA_0` writer - Camera 0"]
pub type CAMERA_0_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, CAMERA_0_A>;
impl<'a, const O: u8> CAMERA_0_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(CAMERA_0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(CAMERA_0_A::EDGE)
    }
}
#[doc = "Field `CAMERA_1` reader - Camera 1"]
pub type CAMERA_1_R = crate::BitReader<CAMERA_1_A>;
#[doc = "Camera 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAMERA_1_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<CAMERA_1_A> for bool {
    #[inline(always)]
    fn from(variant: CAMERA_1_A) -> Self {
        variant as u8 != 0
    }
}
impl CAMERA_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAMERA_1_A {
        match self.bits {
            false => CAMERA_1_A::LEVEL,
            true => CAMERA_1_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == CAMERA_1_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == CAMERA_1_A::EDGE
    }
}
#[doc = "Field `CAMERA_1` writer - Camera 1"]
pub type CAMERA_1_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, CAMERA_1_A>;
impl<'a, const O: u8> CAMERA_1_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(CAMERA_1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(CAMERA_1_A::EDGE)
    }
}
#[doc = "Field `HDMI_0` reader - HDMI 0"]
pub type HDMI_0_R = crate::BitReader<HDMI_0_A>;
#[doc = "HDMI 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDMI_0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<HDMI_0_A> for bool {
    #[inline(always)]
    fn from(variant: HDMI_0_A) -> Self {
        variant as u8 != 0
    }
}
impl HDMI_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDMI_0_A {
        match self.bits {
            false => HDMI_0_A::LEVEL,
            true => HDMI_0_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == HDMI_0_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == HDMI_0_A::EDGE
    }
}
#[doc = "Field `HDMI_0` writer - HDMI 0"]
pub type HDMI_0_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, HDMI_0_A>;
impl<'a, const O: u8> HDMI_0_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(HDMI_0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(HDMI_0_A::EDGE)
    }
}
#[doc = "Field `HDMI_1` reader - HDMI 1"]
pub type HDMI_1_R = crate::BitReader<HDMI_1_A>;
#[doc = "HDMI 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDMI_1_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<HDMI_1_A> for bool {
    #[inline(always)]
    fn from(variant: HDMI_1_A) -> Self {
        variant as u8 != 0
    }
}
impl HDMI_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDMI_1_A {
        match self.bits {
            false => HDMI_1_A::LEVEL,
            true => HDMI_1_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == HDMI_1_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == HDMI_1_A::EDGE
    }
}
#[doc = "Field `HDMI_1` writer - HDMI 1"]
pub type HDMI_1_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, HDMI_1_A>;
impl<'a, const O: u8> HDMI_1_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(HDMI_1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(HDMI_1_A::EDGE)
    }
}
#[doc = "Field `PIXEL_VALVE_3` reader - Pixel Valve 3"]
pub type PIXEL_VALVE_3_R = crate::BitReader<PIXEL_VALVE_3_A>;
#[doc = "Pixel Valve 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIXEL_VALVE_3_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<PIXEL_VALVE_3_A> for bool {
    #[inline(always)]
    fn from(variant: PIXEL_VALVE_3_A) -> Self {
        variant as u8 != 0
    }
}
impl PIXEL_VALVE_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXEL_VALVE_3_A {
        match self.bits {
            false => PIXEL_VALVE_3_A::LEVEL,
            true => PIXEL_VALVE_3_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PIXEL_VALVE_3_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == PIXEL_VALVE_3_A::EDGE
    }
}
#[doc = "Field `PIXEL_VALVE_3` writer - Pixel Valve 3"]
pub type PIXEL_VALVE_3_W<'a, const O: u8> =
    crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, PIXEL_VALVE_3_A>;
impl<'a, const O: u8> PIXEL_VALVE_3_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(PIXEL_VALVE_3_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(PIXEL_VALVE_3_A::EDGE)
    }
}
#[doc = "Field `SPI_BSC_SLAVE` reader - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_R = crate::BitReader<SPI_BSC_SLAVE_A>;
#[doc = "SPI/BSC Slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI_BSC_SLAVE_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SPI_BSC_SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: SPI_BSC_SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI_BSC_SLAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI_BSC_SLAVE_A {
        match self.bits {
            false => SPI_BSC_SLAVE_A::LEVEL,
            true => SPI_BSC_SLAVE_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SPI_BSC_SLAVE_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SPI_BSC_SLAVE_A::EDGE
    }
}
#[doc = "Field `SPI_BSC_SLAVE` writer - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_W<'a, const O: u8> =
    crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, SPI_BSC_SLAVE_A>;
impl<'a, const O: u8> SPI_BSC_SLAVE_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SPI_BSC_SLAVE_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SPI_BSC_SLAVE_A::EDGE)
    }
}
#[doc = "Field `DSI_1` reader - DSI 1"]
pub type DSI_1_R = crate::BitReader<DSI_1_A>;
#[doc = "DSI 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSI_1_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DSI_1_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_1_A) -> Self {
        variant as u8 != 0
    }
}
impl DSI_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_1_A {
        match self.bits {
            false => DSI_1_A::LEVEL,
            true => DSI_1_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DSI_1_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DSI_1_A::EDGE
    }
}
#[doc = "Field `DSI_1` writer - DSI 1"]
pub type DSI_1_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, DSI_1_A>;
impl<'a, const O: u8> DSI_1_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(DSI_1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(DSI_1_A::EDGE)
    }
}
#[doc = "Field `PIXEL_VALVE_0` reader - Pixel Valve 0"]
pub type PIXEL_VALVE_0_R = crate::BitReader<PIXEL_VALVE_0_A>;
#[doc = "Pixel Valve 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIXEL_VALVE_0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<PIXEL_VALVE_0_A> for bool {
    #[inline(always)]
    fn from(variant: PIXEL_VALVE_0_A) -> Self {
        variant as u8 != 0
    }
}
impl PIXEL_VALVE_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXEL_VALVE_0_A {
        match self.bits {
            false => PIXEL_VALVE_0_A::LEVEL,
            true => PIXEL_VALVE_0_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PIXEL_VALVE_0_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == PIXEL_VALVE_0_A::EDGE
    }
}
#[doc = "Field `PIXEL_VALVE_0` writer - Pixel Valve 0"]
pub type PIXEL_VALVE_0_W<'a, const O: u8> =
    crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, PIXEL_VALVE_0_A>;
impl<'a, const O: u8> PIXEL_VALVE_0_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(PIXEL_VALVE_0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(PIXEL_VALVE_0_A::EDGE)
    }
}
#[doc = "Field `PIXEL_VALVE_1_2` reader - OR of Pixel Valve 1 and 2"]
pub type PIXEL_VALVE_1_2_R = crate::BitReader<PIXEL_VALVE_1_2_A>;
#[doc = "OR of Pixel Valve 1 and 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIXEL_VALVE_1_2_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<PIXEL_VALVE_1_2_A> for bool {
    #[inline(always)]
    fn from(variant: PIXEL_VALVE_1_2_A) -> Self {
        variant as u8 != 0
    }
}
impl PIXEL_VALVE_1_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXEL_VALVE_1_2_A {
        match self.bits {
            false => PIXEL_VALVE_1_2_A::LEVEL,
            true => PIXEL_VALVE_1_2_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PIXEL_VALVE_1_2_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == PIXEL_VALVE_1_2_A::EDGE
    }
}
#[doc = "Field `PIXEL_VALVE_1_2` writer - OR of Pixel Valve 1 and 2"]
pub type PIXEL_VALVE_1_2_W<'a, const O: u8> =
    crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, PIXEL_VALVE_1_2_A>;
impl<'a, const O: u8> PIXEL_VALVE_1_2_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(PIXEL_VALVE_1_2_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(PIXEL_VALVE_1_2_A::EDGE)
    }
}
#[doc = "Field `CPR` reader - CPR"]
pub type CPR_R = crate::BitReader<CPR_A>;
#[doc = "CPR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPR_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<CPR_A> for bool {
    #[inline(always)]
    fn from(variant: CPR_A) -> Self {
        variant as u8 != 0
    }
}
impl CPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPR_A {
        match self.bits {
            false => CPR_A::LEVEL,
            true => CPR_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == CPR_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == CPR_A::EDGE
    }
}
#[doc = "Field `CPR` writer - CPR"]
pub type CPR_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR32_SPEC, O, CPR_A>;
impl<'a, const O: u8> CPR_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(CPR_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(CPR_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(&self) -> HDMI_CEC_R {
        HDMI_CEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HVS"]
    #[inline(always)]
    pub fn hvs(&self) -> HVS_R {
        HVS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - RPIVID"]
    #[inline(always)]
    pub fn rpivid(&self) -> RPIVID_R {
        RPIVID_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - SDC"]
    #[inline(always)]
    pub fn sdc(&self) -> SDC_R {
        SDC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - DSI 0"]
    #[inline(always)]
    pub fn dsi_0(&self) -> DSI_0_R {
        DSI_0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Pixel Valve 2"]
    #[inline(always)]
    pub fn pixel_valve_2(&self) -> PIXEL_VALVE_2_R {
        PIXEL_VALVE_2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Camera 0"]
    #[inline(always)]
    pub fn camera_0(&self) -> CAMERA_0_R {
        CAMERA_0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Camera 1"]
    #[inline(always)]
    pub fn camera_1(&self) -> CAMERA_1_R {
        CAMERA_1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - HDMI 0"]
    #[inline(always)]
    pub fn hdmi_0(&self) -> HDMI_0_R {
        HDMI_0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - HDMI 1"]
    #[inline(always)]
    pub fn hdmi_1(&self) -> HDMI_1_R {
        HDMI_1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Pixel Valve 3"]
    #[inline(always)]
    pub fn pixel_valve_3(&self) -> PIXEL_VALVE_3_R {
        PIXEL_VALVE_3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - SPI/BSC Slave"]
    #[inline(always)]
    pub fn spi_bsc_slave(&self) -> SPI_BSC_SLAVE_R {
        SPI_BSC_SLAVE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - DSI 1"]
    #[inline(always)]
    pub fn dsi_1(&self) -> DSI_1_R {
        DSI_1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Pixel Valve 0"]
    #[inline(always)]
    pub fn pixel_valve_0(&self) -> PIXEL_VALVE_0_R {
        PIXEL_VALVE_0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn pixel_valve_1_2(&self) -> PIXEL_VALVE_1_2_R {
        PIXEL_VALVE_1_2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - CPR"]
    #[inline(always)]
    pub fn cpr(&self) -> CPR_R {
        CPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR32")
            .field("hdmi_cec", &format_args!("{}", self.hdmi_cec().bit()))
            .field("hvs", &format_args!("{}", self.hvs().bit()))
            .field("rpivid", &format_args!("{}", self.rpivid().bit()))
            .field("sdc", &format_args!("{}", self.sdc().bit()))
            .field("dsi_0", &format_args!("{}", self.dsi_0().bit()))
            .field(
                "pixel_valve_2",
                &format_args!("{}", self.pixel_valve_2().bit()),
            )
            .field("camera_0", &format_args!("{}", self.camera_0().bit()))
            .field("camera_1", &format_args!("{}", self.camera_1().bit()))
            .field("hdmi_0", &format_args!("{}", self.hdmi_0().bit()))
            .field("hdmi_1", &format_args!("{}", self.hdmi_1().bit()))
            .field(
                "pixel_valve_3",
                &format_args!("{}", self.pixel_valve_3().bit()),
            )
            .field(
                "spi_bsc_slave",
                &format_args!("{}", self.spi_bsc_slave().bit()),
            )
            .field("dsi_1", &format_args!("{}", self.dsi_1().bit()))
            .field(
                "pixel_valve_0",
                &format_args!("{}", self.pixel_valve_0().bit()),
            )
            .field(
                "pixel_valve_1_2",
                &format_args!("{}", self.pixel_valve_1_2().bit()),
            )
            .field("cpr", &format_args!("{}", self.cpr().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_ICFGR32_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - HDMI CEC"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_cec(&mut self) -> HDMI_CEC_W<1> {
        HDMI_CEC_W::new(self)
    }
    #[doc = "Bit 3 - HVS"]
    #[inline(always)]
    #[must_use]
    pub fn hvs(&mut self) -> HVS_W<3> {
        HVS_W::new(self)
    }
    #[doc = "Bit 5 - RPIVID"]
    #[inline(always)]
    #[must_use]
    pub fn rpivid(&mut self) -> RPIVID_W<5> {
        RPIVID_W::new(self)
    }
    #[doc = "Bit 7 - SDC"]
    #[inline(always)]
    #[must_use]
    pub fn sdc(&mut self) -> SDC_W<7> {
        SDC_W::new(self)
    }
    #[doc = "Bit 9 - DSI 0"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_0(&mut self) -> DSI_0_W<9> {
        DSI_0_W::new(self)
    }
    #[doc = "Bit 11 - Pixel Valve 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_2(&mut self) -> PIXEL_VALVE_2_W<11> {
        PIXEL_VALVE_2_W::new(self)
    }
    #[doc = "Bit 13 - Camera 0"]
    #[inline(always)]
    #[must_use]
    pub fn camera_0(&mut self) -> CAMERA_0_W<13> {
        CAMERA_0_W::new(self)
    }
    #[doc = "Bit 15 - Camera 1"]
    #[inline(always)]
    #[must_use]
    pub fn camera_1(&mut self) -> CAMERA_1_W<15> {
        CAMERA_1_W::new(self)
    }
    #[doc = "Bit 17 - HDMI 0"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_0(&mut self) -> HDMI_0_W<17> {
        HDMI_0_W::new(self)
    }
    #[doc = "Bit 19 - HDMI 1"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_1(&mut self) -> HDMI_1_W<19> {
        HDMI_1_W::new(self)
    }
    #[doc = "Bit 21 - Pixel Valve 3"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_3(&mut self) -> PIXEL_VALVE_3_W<21> {
        PIXEL_VALVE_3_W::new(self)
    }
    #[doc = "Bit 23 - SPI/BSC Slave"]
    #[inline(always)]
    #[must_use]
    pub fn spi_bsc_slave(&mut self) -> SPI_BSC_SLAVE_W<23> {
        SPI_BSC_SLAVE_W::new(self)
    }
    #[doc = "Bit 25 - DSI 1"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_1(&mut self) -> DSI_1_W<25> {
        DSI_1_W::new(self)
    }
    #[doc = "Bit 27 - Pixel Valve 0"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_0(&mut self) -> PIXEL_VALVE_0_W<27> {
        PIXEL_VALVE_0_W::new(self)
    }
    #[doc = "Bit 29 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_1_2(&mut self) -> PIXEL_VALVE_1_2_W<29> {
        PIXEL_VALVE_1_2_W::new(self)
    }
    #[doc = "Bit 31 - CPR"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CPR_W<31> {
        CPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Configuration 128 - 143\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr32](index.html) module"]
pub struct GICD_ICFGR32_SPEC;
impl crate::RegisterSpec for GICD_ICFGR32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr32::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr32::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR32_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR32 to value 0"]
impl crate::Resettable for GICD_ICFGR32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
