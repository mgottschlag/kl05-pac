#[doc = "Register `LVDSC2` reader"]
pub struct R(crate::R<LVDSC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVDSC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVDSC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVDSC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVDSC2` writer"]
pub struct W(crate::W<LVDSC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVDSC2_SPEC>;
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
impl From<crate::W<LVDSC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVDSC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low-Voltage Warning Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVWV_A {
    #[doc = "0: Low trip point selected (VLVW = VLVW1)"]
    _00 = 0,
    #[doc = "1: Mid 1 trip point selected (VLVW = VLVW2)"]
    _01 = 1,
    #[doc = "2: Mid 2 trip point selected (VLVW = VLVW3)"]
    _10 = 2,
    #[doc = "3: High trip point selected (VLVW = VLVW4)"]
    _11 = 3,
}
impl From<LVWV_A> for u8 {
    #[inline(always)]
    fn from(variant: LVWV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LVWV` reader - Low-Voltage Warning Voltage Select"]
pub struct LVWV_R(crate::FieldReader<u8, LVWV_A>);
impl LVWV_R {
    pub(crate) fn new(bits: u8) -> Self {
        LVWV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWV_A {
        match self.bits {
            0 => LVWV_A::_00,
            1 => LVWV_A::_01,
            2 => LVWV_A::_10,
            3 => LVWV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == LVWV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == LVWV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == LVWV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == LVWV_A::_11
    }
}
impl core::ops::Deref for LVWV_R {
    type Target = crate::FieldReader<u8, LVWV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVWV` writer - Low-Voltage Warning Voltage Select"]
pub struct LVWV_W<'a> {
    w: &'a mut W,
}
impl<'a> LVWV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVWV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low trip point selected (VLVW = VLVW1)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LVWV_A::_00)
    }
    #[doc = "Mid 1 trip point selected (VLVW = VLVW2)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LVWV_A::_01)
    }
    #[doc = "Mid 2 trip point selected (VLVW = VLVW3)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LVWV_A::_10)
    }
    #[doc = "High trip point selected (VLVW = VLVW4)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LVWV_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Low-Voltage Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWIE_A {
    #[doc = "0: Hardware interrupt disabled (use polling)"]
    _0 = 0,
    #[doc = "1: Request a hardware interrupt when LVWF = 1"]
    _1 = 1,
}
impl From<LVWIE_A> for bool {
    #[inline(always)]
    fn from(variant: LVWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVWIE` reader - Low-Voltage Warning Interrupt Enable"]
pub struct LVWIE_R(crate::FieldReader<bool, LVWIE_A>);
impl LVWIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVWIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWIE_A {
        match self.bits {
            false => LVWIE_A::_0,
            true => LVWIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVWIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVWIE_A::_1
    }
}
impl core::ops::Deref for LVWIE_R {
    type Target = crate::FieldReader<bool, LVWIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVWIE` writer - Low-Voltage Warning Interrupt Enable"]
pub struct LVWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVWIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVWIE_A::_0)
    }
    #[doc = "Request a hardware interrupt when LVWF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVWIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `LVWACK` writer - Low-Voltage Warning Acknowledge"]
pub struct LVWACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LVWACK_W<'a> {
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
#[doc = "Low-Voltage Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWF_A {
    #[doc = "0: Low-voltage warning event not detected"]
    _0 = 0,
    #[doc = "1: Low-voltage warning event detected"]
    _1 = 1,
}
impl From<LVWF_A> for bool {
    #[inline(always)]
    fn from(variant: LVWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVWF` reader - Low-Voltage Warning Flag"]
pub struct LVWF_R(crate::FieldReader<bool, LVWF_A>);
impl LVWF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVWF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWF_A {
        match self.bits {
            false => LVWF_A::_0,
            true => LVWF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVWF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVWF_A::_1
    }
}
impl core::ops::Deref for LVWF_R {
    type Target = crate::FieldReader<bool, LVWF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-Voltage Warning Voltage Select"]
    #[inline(always)]
    pub fn lvwv(&self) -> LVWV_R {
        LVWV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    pub fn lvwie(&self) -> LVWIE_R {
        LVWIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low-Voltage Warning Flag"]
    #[inline(always)]
    pub fn lvwf(&self) -> LVWF_R {
        LVWF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-Voltage Warning Voltage Select"]
    #[inline(always)]
    pub fn lvwv(&mut self) -> LVWV_W {
        LVWV_W { w: self }
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    pub fn lvwie(&mut self) -> LVWIE_W {
        LVWIE_W { w: self }
    }
    #[doc = "Bit 6 - Low-Voltage Warning Acknowledge"]
    #[inline(always)]
    pub fn lvwack(&mut self) -> LVWACK_W {
        LVWACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Voltage Detect Status And Control 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvdsc2](index.html) module"]
pub struct LVDSC2_SPEC;
impl crate::RegisterSpec for LVDSC2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvdsc2::R](R) reader structure"]
impl crate::Readable for LVDSC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvdsc2::W](W) writer structure"]
impl crate::Writable for LVDSC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LVDSC2 to value 0"]
impl crate::Resettable for LVDSC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
