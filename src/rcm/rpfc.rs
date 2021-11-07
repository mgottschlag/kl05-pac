#[doc = "Register `RPFC` reader"]
pub struct R(crate::R<RPFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPFC` writer"]
pub struct W(crate::W<RPFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPFC_SPEC>;
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
impl From<crate::W<RPFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reset Pin Filter Select in Run and Wait Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTFLTSRW_A {
    #[doc = "0: All filtering disabled"]
    _00 = 0,
    #[doc = "1: Bus clock filter enabled for normal operation"]
    _01 = 1,
    #[doc = "2: LPO clock filter enabled for normal operation"]
    _10 = 2,
}
impl From<RSTFLTSRW_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTFLTSRW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RSTFLTSRW` reader - Reset Pin Filter Select in Run and Wait Modes"]
pub struct RSTFLTSRW_R(crate::FieldReader<u8, RSTFLTSRW_A>);
impl RSTFLTSRW_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSTFLTSRW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSTFLTSRW_A> {
        match self.bits {
            0 => Some(RSTFLTSRW_A::_00),
            1 => Some(RSTFLTSRW_A::_01),
            2 => Some(RSTFLTSRW_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == RSTFLTSRW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == RSTFLTSRW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == RSTFLTSRW_A::_10
    }
}
impl core::ops::Deref for RSTFLTSRW_R {
    type Target = crate::FieldReader<u8, RSTFLTSRW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTFLTSRW` writer - Reset Pin Filter Select in Run and Wait Modes"]
pub struct RSTFLTSRW_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFLTSRW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTFLTSRW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_00)
    }
    #[doc = "Bus clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_01)
    }
    #[doc = "LPO clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Reset Pin Filter Select in Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFLTSS_A {
    #[doc = "0: All filtering disabled"]
    _0 = 0,
    #[doc = "1: LPO clock filter enabled"]
    _1 = 1,
}
impl From<RSTFLTSS_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFLTSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFLTSS` reader - Reset Pin Filter Select in Stop Mode"]
pub struct RSTFLTSS_R(crate::FieldReader<bool, RSTFLTSS_A>);
impl RSTFLTSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTFLTSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFLTSS_A {
        match self.bits {
            false => RSTFLTSS_A::_0,
            true => RSTFLTSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RSTFLTSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RSTFLTSS_A::_1
    }
}
impl core::ops::Deref for RSTFLTSS_R {
    type Target = crate::FieldReader<bool, RSTFLTSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTFLTSS` writer - Reset Pin Filter Select in Stop Mode"]
pub struct RSTFLTSS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFLTSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTFLTSS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTFLTSS_A::_0)
    }
    #[doc = "LPO clock filter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTFLTSS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Reset Pin Filter Select in Run and Wait Modes"]
    #[inline(always)]
    pub fn rstfltsrw(&self) -> RSTFLTSRW_R {
        RSTFLTSRW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Reset Pin Filter Select in Stop Mode"]
    #[inline(always)]
    pub fn rstfltss(&self) -> RSTFLTSS_R {
        RSTFLTSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reset Pin Filter Select in Run and Wait Modes"]
    #[inline(always)]
    pub fn rstfltsrw(&mut self) -> RSTFLTSRW_W {
        RSTFLTSRW_W { w: self }
    }
    #[doc = "Bit 2 - Reset Pin Filter Select in Stop Mode"]
    #[inline(always)]
    pub fn rstfltss(&mut self) -> RSTFLTSS_W {
        RSTFLTSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Pin Filter Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpfc](index.html) module"]
pub struct RPFC_SPEC;
impl crate::RegisterSpec for RPFC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rpfc::R](R) reader structure"]
impl crate::Readable for RPFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpfc::W](W) writer structure"]
impl crate::Writable for RPFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPFC to value 0"]
impl crate::Resettable for RPFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
