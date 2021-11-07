#[doc = "Register `C4` reader"]
pub struct R(crate::R<C4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C4` writer"]
pub struct W(crate::W<C4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C4_SPEC>;
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
impl From<crate::W<C4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCFTRIM` reader - Slow Internal Reference Clock Fine Trim"]
pub struct SCFTRIM_R(crate::FieldReader<bool, bool>);
impl SCFTRIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCFTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCFTRIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCFTRIM` writer - Slow Internal Reference Clock Fine Trim"]
pub struct SCFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCFTRIM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `FCTRIM` reader - Fast Internal Reference Clock Trim Setting"]
pub struct FCTRIM_R(crate::FieldReader<u8, u8>);
impl FCTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FCTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCTRIM` writer - Fast Internal Reference Clock Trim Setting"]
pub struct FCTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FCTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u8 & 0x0f) << 1);
        self.w
    }
}
#[doc = "DCO Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRST_DRS_A {
    #[doc = "0: Encoding 0 - Low range (reset default)."]
    _00 = 0,
    #[doc = "1: Encoding 1 - Mid range."]
    _01 = 1,
    #[doc = "2: Encoding 2 - Mid-high range."]
    _10 = 2,
    #[doc = "3: Encoding 3 - High range."]
    _11 = 3,
}
impl From<DRST_DRS_A> for u8 {
    #[inline(always)]
    fn from(variant: DRST_DRS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DRST_DRS` reader - DCO Range Select"]
pub struct DRST_DRS_R(crate::FieldReader<u8, DRST_DRS_A>);
impl DRST_DRS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRST_DRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRST_DRS_A {
        match self.bits {
            0 => DRST_DRS_A::_00,
            1 => DRST_DRS_A::_01,
            2 => DRST_DRS_A::_10,
            3 => DRST_DRS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == DRST_DRS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == DRST_DRS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == DRST_DRS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == DRST_DRS_A::_11
    }
}
impl core::ops::Deref for DRST_DRS_R {
    type Target = crate::FieldReader<u8, DRST_DRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRST_DRS` writer - DCO Range Select"]
pub struct DRST_DRS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRST_DRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRST_DRS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Encoding 0 - Low range (reset default)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DRST_DRS_A::_00)
    }
    #[doc = "Encoding 1 - Mid range."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DRST_DRS_A::_01)
    }
    #[doc = "Encoding 2 - Mid-high range."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DRST_DRS_A::_10)
    }
    #[doc = "Encoding 3 - High range."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DRST_DRS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u8 & 0x03) << 5);
        self.w
    }
}
#[doc = "DCO Maximum Frequency with 32.768 kHz Reference\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMX32_A {
    #[doc = "0: DCO has a default range of 25%."]
    _0 = 0,
    #[doc = "1: DCO is fine-tuned for maximum frequency with 32.768 kHz reference."]
    _1 = 1,
}
impl From<DMX32_A> for bool {
    #[inline(always)]
    fn from(variant: DMX32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMX32` reader - DCO Maximum Frequency with 32.768 kHz Reference"]
pub struct DMX32_R(crate::FieldReader<bool, DMX32_A>);
impl DMX32_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMX32_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMX32_A {
        match self.bits {
            false => DMX32_A::_0,
            true => DMX32_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMX32_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMX32_A::_1
    }
}
impl core::ops::Deref for DMX32_R {
    type Target = crate::FieldReader<bool, DMX32_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMX32` writer - DCO Maximum Frequency with 32.768 kHz Reference"]
pub struct DMX32_W<'a> {
    w: &'a mut W,
}
impl<'a> DMX32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMX32_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DCO has a default range of 25%."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMX32_A::_0)
    }
    #[doc = "DCO is fine-tuned for maximum frequency with 32.768 kHz reference."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMX32_A::_1)
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
    #[doc = "Bit 0 - Slow Internal Reference Clock Fine Trim"]
    #[inline(always)]
    pub fn scftrim(&self) -> SCFTRIM_R {
        SCFTRIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - Fast Internal Reference Clock Trim Setting"]
    #[inline(always)]
    pub fn fctrim(&self) -> FCTRIM_R {
        FCTRIM_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - DCO Range Select"]
    #[inline(always)]
    pub fn drst_drs(&self) -> DRST_DRS_R {
        DRST_DRS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - DCO Maximum Frequency with 32.768 kHz Reference"]
    #[inline(always)]
    pub fn dmx32(&self) -> DMX32_R {
        DMX32_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slow Internal Reference Clock Fine Trim"]
    #[inline(always)]
    pub fn scftrim(&mut self) -> SCFTRIM_W {
        SCFTRIM_W { w: self }
    }
    #[doc = "Bits 1:4 - Fast Internal Reference Clock Trim Setting"]
    #[inline(always)]
    pub fn fctrim(&mut self) -> FCTRIM_W {
        FCTRIM_W { w: self }
    }
    #[doc = "Bits 5:6 - DCO Range Select"]
    #[inline(always)]
    pub fn drst_drs(&mut self) -> DRST_DRS_W {
        DRST_DRS_W { w: self }
    }
    #[doc = "Bit 7 - DCO Maximum Frequency with 32.768 kHz Reference"]
    #[inline(always)]
    pub fn dmx32(&mut self) -> DMX32_W {
        DMX32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4](index.html) module"]
pub struct C4_SPEC;
impl crate::RegisterSpec for C4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c4::R](R) reader structure"]
impl crate::Readable for C4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c4::W](W) writer structure"]
impl crate::Writable for C4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C4 to value 0"]
impl crate::Resettable for C4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
