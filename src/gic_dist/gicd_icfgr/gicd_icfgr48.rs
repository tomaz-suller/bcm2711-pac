#[doc = "Register `GICD_ICFGR48` reader"]
pub struct R(crate::R<GICD_ICFGR48_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR48_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR48_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR48_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR48` writer"]
pub struct W(crate::W<GICD_ICFGR48_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR48_SPEC>;
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
impl From<crate::W<GICD_ICFGR48_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR48_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT192` reader - Interrupt 192"]
pub type INT192_R = crate::BitReader<INT192_A>;
#[doc = "Interrupt 192\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT192_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT192_A> for bool {
    #[inline(always)]
    fn from(variant: INT192_A) -> Self {
        variant as u8 != 0
    }
}
impl INT192_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT192_A {
        match self.bits {
            false => INT192_A::LEVEL,
            true => INT192_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT192_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT192_A::EDGE
    }
}
#[doc = "Field `INT192` writer - Interrupt 192"]
pub type INT192_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT192_A>;
impl<'a, const O: u8> INT192_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT192_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT192_A::EDGE)
    }
}
#[doc = "Field `INT193` reader - Interrupt 193"]
pub type INT193_R = crate::BitReader<INT193_A>;
#[doc = "Interrupt 193\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT193_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT193_A> for bool {
    #[inline(always)]
    fn from(variant: INT193_A) -> Self {
        variant as u8 != 0
    }
}
impl INT193_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT193_A {
        match self.bits {
            false => INT193_A::LEVEL,
            true => INT193_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT193_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT193_A::EDGE
    }
}
#[doc = "Field `INT193` writer - Interrupt 193"]
pub type INT193_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT193_A>;
impl<'a, const O: u8> INT193_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT193_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT193_A::EDGE)
    }
}
#[doc = "Field `INT194` reader - Interrupt 194"]
pub type INT194_R = crate::BitReader<INT194_A>;
#[doc = "Interrupt 194\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT194_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT194_A> for bool {
    #[inline(always)]
    fn from(variant: INT194_A) -> Self {
        variant as u8 != 0
    }
}
impl INT194_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT194_A {
        match self.bits {
            false => INT194_A::LEVEL,
            true => INT194_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT194_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT194_A::EDGE
    }
}
#[doc = "Field `INT194` writer - Interrupt 194"]
pub type INT194_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT194_A>;
impl<'a, const O: u8> INT194_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT194_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT194_A::EDGE)
    }
}
#[doc = "Field `INT195` reader - Interrupt 195"]
pub type INT195_R = crate::BitReader<INT195_A>;
#[doc = "Interrupt 195\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT195_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT195_A> for bool {
    #[inline(always)]
    fn from(variant: INT195_A) -> Self {
        variant as u8 != 0
    }
}
impl INT195_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT195_A {
        match self.bits {
            false => INT195_A::LEVEL,
            true => INT195_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT195_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT195_A::EDGE
    }
}
#[doc = "Field `INT195` writer - Interrupt 195"]
pub type INT195_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT195_A>;
impl<'a, const O: u8> INT195_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT195_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT195_A::EDGE)
    }
}
#[doc = "Field `INT196` reader - Interrupt 196"]
pub type INT196_R = crate::BitReader<INT196_A>;
#[doc = "Interrupt 196\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT196_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT196_A> for bool {
    #[inline(always)]
    fn from(variant: INT196_A) -> Self {
        variant as u8 != 0
    }
}
impl INT196_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT196_A {
        match self.bits {
            false => INT196_A::LEVEL,
            true => INT196_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT196_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT196_A::EDGE
    }
}
#[doc = "Field `INT196` writer - Interrupt 196"]
pub type INT196_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT196_A>;
impl<'a, const O: u8> INT196_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT196_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT196_A::EDGE)
    }
}
#[doc = "Field `INT197` reader - Interrupt 197"]
pub type INT197_R = crate::BitReader<INT197_A>;
#[doc = "Interrupt 197\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT197_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT197_A> for bool {
    #[inline(always)]
    fn from(variant: INT197_A) -> Self {
        variant as u8 != 0
    }
}
impl INT197_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT197_A {
        match self.bits {
            false => INT197_A::LEVEL,
            true => INT197_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT197_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT197_A::EDGE
    }
}
#[doc = "Field `INT197` writer - Interrupt 197"]
pub type INT197_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT197_A>;
impl<'a, const O: u8> INT197_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT197_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT197_A::EDGE)
    }
}
#[doc = "Field `INT198` reader - Interrupt 198"]
pub type INT198_R = crate::BitReader<INT198_A>;
#[doc = "Interrupt 198\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT198_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT198_A> for bool {
    #[inline(always)]
    fn from(variant: INT198_A) -> Self {
        variant as u8 != 0
    }
}
impl INT198_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT198_A {
        match self.bits {
            false => INT198_A::LEVEL,
            true => INT198_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT198_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT198_A::EDGE
    }
}
#[doc = "Field `INT198` writer - Interrupt 198"]
pub type INT198_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT198_A>;
impl<'a, const O: u8> INT198_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT198_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT198_A::EDGE)
    }
}
#[doc = "Field `INT199` reader - Interrupt 199"]
pub type INT199_R = crate::BitReader<INT199_A>;
#[doc = "Interrupt 199\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT199_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT199_A> for bool {
    #[inline(always)]
    fn from(variant: INT199_A) -> Self {
        variant as u8 != 0
    }
}
impl INT199_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT199_A {
        match self.bits {
            false => INT199_A::LEVEL,
            true => INT199_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT199_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT199_A::EDGE
    }
}
#[doc = "Field `INT199` writer - Interrupt 199"]
pub type INT199_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT199_A>;
impl<'a, const O: u8> INT199_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT199_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT199_A::EDGE)
    }
}
#[doc = "Field `INT200` reader - Interrupt 200"]
pub type INT200_R = crate::BitReader<INT200_A>;
#[doc = "Interrupt 200\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT200_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT200_A> for bool {
    #[inline(always)]
    fn from(variant: INT200_A) -> Self {
        variant as u8 != 0
    }
}
impl INT200_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT200_A {
        match self.bits {
            false => INT200_A::LEVEL,
            true => INT200_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT200_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT200_A::EDGE
    }
}
#[doc = "Field `INT200` writer - Interrupt 200"]
pub type INT200_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT200_A>;
impl<'a, const O: u8> INT200_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT200_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT200_A::EDGE)
    }
}
#[doc = "Field `INT201` reader - Interrupt 201"]
pub type INT201_R = crate::BitReader<INT201_A>;
#[doc = "Interrupt 201\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT201_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT201_A> for bool {
    #[inline(always)]
    fn from(variant: INT201_A) -> Self {
        variant as u8 != 0
    }
}
impl INT201_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT201_A {
        match self.bits {
            false => INT201_A::LEVEL,
            true => INT201_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT201_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT201_A::EDGE
    }
}
#[doc = "Field `INT201` writer - Interrupt 201"]
pub type INT201_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT201_A>;
impl<'a, const O: u8> INT201_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT201_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT201_A::EDGE)
    }
}
#[doc = "Field `INT202` reader - Interrupt 202"]
pub type INT202_R = crate::BitReader<INT202_A>;
#[doc = "Interrupt 202\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT202_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT202_A> for bool {
    #[inline(always)]
    fn from(variant: INT202_A) -> Self {
        variant as u8 != 0
    }
}
impl INT202_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT202_A {
        match self.bits {
            false => INT202_A::LEVEL,
            true => INT202_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT202_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT202_A::EDGE
    }
}
#[doc = "Field `INT202` writer - Interrupt 202"]
pub type INT202_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT202_A>;
impl<'a, const O: u8> INT202_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT202_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT202_A::EDGE)
    }
}
#[doc = "Field `INT203` reader - Interrupt 203"]
pub type INT203_R = crate::BitReader<INT203_A>;
#[doc = "Interrupt 203\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT203_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT203_A> for bool {
    #[inline(always)]
    fn from(variant: INT203_A) -> Self {
        variant as u8 != 0
    }
}
impl INT203_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT203_A {
        match self.bits {
            false => INT203_A::LEVEL,
            true => INT203_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT203_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT203_A::EDGE
    }
}
#[doc = "Field `INT203` writer - Interrupt 203"]
pub type INT203_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT203_A>;
impl<'a, const O: u8> INT203_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT203_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT203_A::EDGE)
    }
}
#[doc = "Field `INT204` reader - Interrupt 204"]
pub type INT204_R = crate::BitReader<INT204_A>;
#[doc = "Interrupt 204\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT204_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT204_A> for bool {
    #[inline(always)]
    fn from(variant: INT204_A) -> Self {
        variant as u8 != 0
    }
}
impl INT204_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT204_A {
        match self.bits {
            false => INT204_A::LEVEL,
            true => INT204_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT204_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT204_A::EDGE
    }
}
#[doc = "Field `INT204` writer - Interrupt 204"]
pub type INT204_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT204_A>;
impl<'a, const O: u8> INT204_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT204_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT204_A::EDGE)
    }
}
#[doc = "Field `INT205` reader - Interrupt 205"]
pub type INT205_R = crate::BitReader<INT205_A>;
#[doc = "Interrupt 205\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT205_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT205_A> for bool {
    #[inline(always)]
    fn from(variant: INT205_A) -> Self {
        variant as u8 != 0
    }
}
impl INT205_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT205_A {
        match self.bits {
            false => INT205_A::LEVEL,
            true => INT205_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT205_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT205_A::EDGE
    }
}
#[doc = "Field `INT205` writer - Interrupt 205"]
pub type INT205_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT205_A>;
impl<'a, const O: u8> INT205_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT205_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT205_A::EDGE)
    }
}
#[doc = "Field `INT206` reader - Interrupt 206"]
pub type INT206_R = crate::BitReader<INT206_A>;
#[doc = "Interrupt 206\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT206_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT206_A> for bool {
    #[inline(always)]
    fn from(variant: INT206_A) -> Self {
        variant as u8 != 0
    }
}
impl INT206_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT206_A {
        match self.bits {
            false => INT206_A::LEVEL,
            true => INT206_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT206_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT206_A::EDGE
    }
}
#[doc = "Field `INT206` writer - Interrupt 206"]
pub type INT206_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT206_A>;
impl<'a, const O: u8> INT206_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT206_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT206_A::EDGE)
    }
}
#[doc = "Field `INT207` reader - Interrupt 207"]
pub type INT207_R = crate::BitReader<INT207_A>;
#[doc = "Interrupt 207\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT207_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT207_A> for bool {
    #[inline(always)]
    fn from(variant: INT207_A) -> Self {
        variant as u8 != 0
    }
}
impl INT207_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT207_A {
        match self.bits {
            false => INT207_A::LEVEL,
            true => INT207_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT207_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT207_A::EDGE
    }
}
#[doc = "Field `INT207` writer - Interrupt 207"]
pub type INT207_W<'a, const O: u8> = crate::BitWriter<'a, GICD_ICFGR48_SPEC, O, INT207_A>;
impl<'a, const O: u8> INT207_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT207_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT207_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 192"]
    #[inline(always)]
    pub fn int192(&self) -> INT192_R {
        INT192_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 193"]
    #[inline(always)]
    pub fn int193(&self) -> INT193_R {
        INT193_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 194"]
    #[inline(always)]
    pub fn int194(&self) -> INT194_R {
        INT194_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 195"]
    #[inline(always)]
    pub fn int195(&self) -> INT195_R {
        INT195_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 196"]
    #[inline(always)]
    pub fn int196(&self) -> INT196_R {
        INT196_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 197"]
    #[inline(always)]
    pub fn int197(&self) -> INT197_R {
        INT197_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 198"]
    #[inline(always)]
    pub fn int198(&self) -> INT198_R {
        INT198_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 199"]
    #[inline(always)]
    pub fn int199(&self) -> INT199_R {
        INT199_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 200"]
    #[inline(always)]
    pub fn int200(&self) -> INT200_R {
        INT200_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 201"]
    #[inline(always)]
    pub fn int201(&self) -> INT201_R {
        INT201_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 202"]
    #[inline(always)]
    pub fn int202(&self) -> INT202_R {
        INT202_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 203"]
    #[inline(always)]
    pub fn int203(&self) -> INT203_R {
        INT203_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 204"]
    #[inline(always)]
    pub fn int204(&self) -> INT204_R {
        INT204_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 205"]
    #[inline(always)]
    pub fn int205(&self) -> INT205_R {
        INT205_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 206"]
    #[inline(always)]
    pub fn int206(&self) -> INT206_R {
        INT206_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 207"]
    #[inline(always)]
    pub fn int207(&self) -> INT207_R {
        INT207_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR48")
            .field("int192", &format_args!("{}", self.int192().bit()))
            .field("int193", &format_args!("{}", self.int193().bit()))
            .field("int194", &format_args!("{}", self.int194().bit()))
            .field("int195", &format_args!("{}", self.int195().bit()))
            .field("int196", &format_args!("{}", self.int196().bit()))
            .field("int197", &format_args!("{}", self.int197().bit()))
            .field("int198", &format_args!("{}", self.int198().bit()))
            .field("int199", &format_args!("{}", self.int199().bit()))
            .field("int200", &format_args!("{}", self.int200().bit()))
            .field("int201", &format_args!("{}", self.int201().bit()))
            .field("int202", &format_args!("{}", self.int202().bit()))
            .field("int203", &format_args!("{}", self.int203().bit()))
            .field("int204", &format_args!("{}", self.int204().bit()))
            .field("int205", &format_args!("{}", self.int205().bit()))
            .field("int206", &format_args!("{}", self.int206().bit()))
            .field("int207", &format_args!("{}", self.int207().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GICD_ICFGR48_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 192"]
    #[inline(always)]
    #[must_use]
    pub fn int192(&mut self) -> INT192_W<1> {
        INT192_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt 193"]
    #[inline(always)]
    #[must_use]
    pub fn int193(&mut self) -> INT193_W<3> {
        INT193_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt 194"]
    #[inline(always)]
    #[must_use]
    pub fn int194(&mut self) -> INT194_W<5> {
        INT194_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt 195"]
    #[inline(always)]
    #[must_use]
    pub fn int195(&mut self) -> INT195_W<7> {
        INT195_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt 196"]
    #[inline(always)]
    #[must_use]
    pub fn int196(&mut self) -> INT196_W<9> {
        INT196_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt 197"]
    #[inline(always)]
    #[must_use]
    pub fn int197(&mut self) -> INT197_W<11> {
        INT197_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt 198"]
    #[inline(always)]
    #[must_use]
    pub fn int198(&mut self) -> INT198_W<13> {
        INT198_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt 199"]
    #[inline(always)]
    #[must_use]
    pub fn int199(&mut self) -> INT199_W<15> {
        INT199_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 200"]
    #[inline(always)]
    #[must_use]
    pub fn int200(&mut self) -> INT200_W<17> {
        INT200_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 201"]
    #[inline(always)]
    #[must_use]
    pub fn int201(&mut self) -> INT201_W<19> {
        INT201_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 202"]
    #[inline(always)]
    #[must_use]
    pub fn int202(&mut self) -> INT202_W<21> {
        INT202_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 203"]
    #[inline(always)]
    #[must_use]
    pub fn int203(&mut self) -> INT203_W<23> {
        INT203_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 204"]
    #[inline(always)]
    #[must_use]
    pub fn int204(&mut self) -> INT204_W<25> {
        INT204_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 205"]
    #[inline(always)]
    #[must_use]
    pub fn int205(&mut self) -> INT205_W<27> {
        INT205_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 206"]
    #[inline(always)]
    #[must_use]
    pub fn int206(&mut self) -> INT206_W<29> {
        INT206_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 207"]
    #[inline(always)]
    #[must_use]
    pub fn int207(&mut self) -> INT207_W<31> {
        INT207_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Configuration 192 - 207\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr48](index.html) module"]
pub struct GICD_ICFGR48_SPEC;
impl crate::RegisterSpec for GICD_ICFGR48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr48::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR48_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr48::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR48_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR48 to value 0"]
impl crate::Resettable for GICD_ICFGR48_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
