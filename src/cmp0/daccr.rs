#[doc = "Register `DACCR` reader"]
pub struct R(crate::R<DACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACCR` writer"]
pub struct W(crate::W<DACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACCR_SPEC>;
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
impl From<crate::W<DACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOSEL` reader - DAC Output Voltage Select"]
pub struct VOSEL_R(crate::FieldReader<u8, u8>);
impl VOSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VOSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOSEL` writer - DAC Output Voltage Select"]
pub struct VOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VOSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u8 & 0x3f);
        self.w
    }
}
#[doc = "Supply Voltage Reference Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRSEL_A {
    #[doc = "0: V is selected as resistor ladder network supply reference V. in1 in"]
    _0 = 0,
    #[doc = "1: V is selected as resistor ladder network supply reference V. in2 in"]
    _1 = 1,
}
impl From<VRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRSEL` reader - Supply Voltage Reference Source Select"]
pub struct VRSEL_R(crate::FieldReader<bool, VRSEL_A>);
impl VRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VRSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRSEL_A {
        match self.bits {
            false => VRSEL_A::_0,
            true => VRSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VRSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VRSEL_A::_1
    }
}
impl core::ops::Deref for VRSEL_R {
    type Target = crate::FieldReader<bool, VRSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VRSEL` writer - Supply Voltage Reference Source Select"]
pub struct VRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VRSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "V is selected as resistor ladder network supply reference V. in1 in"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VRSEL_A::_0)
    }
    #[doc = "V is selected as resistor ladder network supply reference V. in2 in"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VRSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACEN_A {
    #[doc = "0: DAC is disabled."]
    _0 = 0,
    #[doc = "1: DAC is enabled."]
    _1 = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACEN` reader - DAC Enable"]
pub struct DACEN_R(crate::FieldReader<bool, DACEN_A>);
impl DACEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::_0,
            true => DACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACEN_A::_1
    }
}
impl core::ops::Deref for DACEN_R {
    type Target = crate::FieldReader<bool, DACEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACEN` writer - DAC Enable"]
pub struct DACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACEN_A::_0)
    }
    #[doc = "DAC is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&self) -> VOSEL_R {
        VOSEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&self) -> VRSEL_R {
        VRSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&mut self) -> VOSEL_W {
        VOSEL_W { w: self }
    }
    #[doc = "Bit 6 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&mut self) -> VRSEL_W {
        VRSEL_W { w: self }
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daccr](index.html) module"]
pub struct DACCR_SPEC;
impl crate::RegisterSpec for DACCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [daccr::R](R) reader structure"]
impl crate::Readable for DACCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daccr::W](W) writer structure"]
impl crate::Writable for DACCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DACCR to value 0"]
impl crate::Resettable for DACCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
