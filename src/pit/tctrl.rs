#[doc = "Register `TCTRL%s` reader"]
pub struct R(crate::R<TCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCTRL%s` writer"]
pub struct W(crate::W<TCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCTRL_SPEC>;
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
impl From<crate::W<TCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN_A {
    #[doc = "0: Timer n is disabled."]
    _0 = 0,
    #[doc = "1: Timer n is enabled."]
    _1 = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Timer Enable"]
pub struct TEN_R(crate::FieldReader<bool, TEN_A>);
impl TEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::_0,
            true => TEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TEN_A::_1
    }
}
impl core::ops::Deref for TEN_R {
    type Target = crate::FieldReader<bool, TEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEN` writer - Timer Enable"]
pub struct TEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer n is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEN_A::_0)
    }
    #[doc = "Timer n is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: Interrupt requests from Timer n are disabled."]
    _0 = 0,
    #[doc = "1: Interrupt will be requested whenever TIF is set."]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Timer Interrupt Enable"]
pub struct TIE_R(crate::FieldReader<bool, TIE_A>);
impl TIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIE_A::_1
    }
}
impl core::ops::Deref for TIE_R {
    type Target = crate::FieldReader<bool, TIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE` writer - Timer Interrupt Enable"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt requests from Timer n are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Interrupt will be requested whenever TIF is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Chain Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHN_A {
    #[doc = "0: Timer is not chained."]
    _0 = 0,
    #[doc = "1: Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    _1 = 1,
}
impl From<CHN_A> for bool {
    #[inline(always)]
    fn from(variant: CHN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHN` reader - Chain Mode"]
pub struct CHN_R(crate::FieldReader<bool, CHN_A>);
impl CHN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHN_A {
        match self.bits {
            false => CHN_A::_0,
            true => CHN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHN_A::_1
    }
}
impl core::ops::Deref for CHN_R {
    type Target = crate::FieldReader<bool, CHN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHN` writer - Chain Mode"]
pub struct CHN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer is not chained."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHN_A::_0)
    }
    #[doc = "Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Chain Mode"]
    #[inline(always)]
    pub fn chn(&self) -> CHN_R {
        CHN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W {
        TEN_W { w: self }
    }
    #[doc = "Bit 1 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Bit 2 - Chain Mode"]
    #[inline(always)]
    pub fn chn(&mut self) -> CHN_W {
        CHN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctrl](index.html) module"]
pub struct TCTRL_SPEC;
impl crate::RegisterSpec for TCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tctrl::R](R) reader structure"]
impl crate::Readable for TCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tctrl::W](W) writer structure"]
impl crate::Writable for TCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCTRL%s to value 0"]
impl crate::Resettable for TCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
