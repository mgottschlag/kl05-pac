#[doc = "Register `REGSC` reader"]
pub struct R(crate::R<REGSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGSC` writer"]
pub struct W(crate::W<REGSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGSC_SPEC>;
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
impl From<crate::W<REGSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bandgap Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGBE_A {
    #[doc = "0: Bandgap buffer not enabled"]
    _0 = 0,
    #[doc = "1: Bandgap buffer enabled"]
    _1 = 1,
}
impl From<BGBE_A> for bool {
    #[inline(always)]
    fn from(variant: BGBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGBE` reader - Bandgap Buffer Enable"]
pub struct BGBE_R(crate::FieldReader<bool, BGBE_A>);
impl BGBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGBE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGBE_A {
        match self.bits {
            false => BGBE_A::_0,
            true => BGBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BGBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BGBE_A::_1
    }
}
impl core::ops::Deref for BGBE_R {
    type Target = crate::FieldReader<bool, BGBE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGBE` writer - Bandgap Buffer Enable"]
pub struct BGBE_W<'a> {
    w: &'a mut W,
}
impl<'a> BGBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGBE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bandgap buffer not enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGBE_A::_0)
    }
    #[doc = "Bandgap buffer enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGBE_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Regulator In Run Regulation Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGONS_A {
    #[doc = "0: Regulator is in stop regulation or in transition to/from it"]
    _0 = 0,
    #[doc = "1: Regulator is in run regulation"]
    _1 = 1,
}
impl From<REGONS_A> for bool {
    #[inline(always)]
    fn from(variant: REGONS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGONS` reader - Regulator In Run Regulation Status"]
pub struct REGONS_R(crate::FieldReader<bool, REGONS_A>);
impl REGONS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGONS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGONS_A {
        match self.bits {
            false => REGONS_A::_0,
            true => REGONS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REGONS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REGONS_A::_1
    }
}
impl core::ops::Deref for REGONS_R {
    type Target = crate::FieldReader<bool, REGONS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Acknowledge Isolation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKISO_A {
    #[doc = "0: Peripherals and I/O pads are in normal run state"]
    _0 = 0,
    #[doc = "1: Certain peripherals and I/O pads are in an isolated and latched state"]
    _1 = 1,
}
impl From<ACKISO_A> for bool {
    #[inline(always)]
    fn from(variant: ACKISO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKISO` reader - Acknowledge Isolation"]
pub struct ACKISO_R(crate::FieldReader<bool, ACKISO_A>);
impl ACKISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKISO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKISO_A {
        match self.bits {
            false => ACKISO_A::_0,
            true => ACKISO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACKISO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACKISO_A::_1
    }
}
impl core::ops::Deref for ACKISO_R {
    type Target = crate::FieldReader<bool, ACKISO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKISO` writer - Acknowledge Isolation"]
pub struct ACKISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKISO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACKISO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Peripherals and I/O pads are in normal run state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKISO_A::_0)
    }
    #[doc = "Certain peripherals and I/O pads are in an isolated and latched state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKISO_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Bandgap Enable In VLPx Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGEN_A {
    #[doc = "0: Bandgap voltage reference is disabled in VLPx , LLS , and VLLSx modes"]
    _0 = 0,
    #[doc = "1: Bandgap voltage reference is enabled in VLPx , LLS , and VLLSx modes"]
    _1 = 1,
}
impl From<BGEN_A> for bool {
    #[inline(always)]
    fn from(variant: BGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGEN` reader - Bandgap Enable In VLPx Operation"]
pub struct BGEN_R(crate::FieldReader<bool, BGEN_A>);
impl BGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGEN_A {
        match self.bits {
            false => BGEN_A::_0,
            true => BGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BGEN_A::_1
    }
}
impl core::ops::Deref for BGEN_R {
    type Target = crate::FieldReader<bool, BGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGEN` writer - Bandgap Enable In VLPx Operation"]
pub struct BGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bandgap voltage reference is disabled in VLPx , LLS , and VLLSx modes"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGEN_A::_0)
    }
    #[doc = "Bandgap voltage reference is enabled in VLPx , LLS , and VLLSx modes"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline(always)]
    pub fn bgbe(&self) -> BGBE_R {
        BGBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Regulator In Run Regulation Status"]
    #[inline(always)]
    pub fn regons(&self) -> REGONS_R {
        REGONS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Isolation"]
    #[inline(always)]
    pub fn ackiso(&self) -> ACKISO_R {
        ACKISO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bandgap Enable In VLPx Operation"]
    #[inline(always)]
    pub fn bgen(&self) -> BGEN_R {
        BGEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline(always)]
    pub fn bgbe(&mut self) -> BGBE_W {
        BGBE_W { w: self }
    }
    #[doc = "Bit 3 - Acknowledge Isolation"]
    #[inline(always)]
    pub fn ackiso(&mut self) -> ACKISO_W {
        ACKISO_W { w: self }
    }
    #[doc = "Bit 4 - Bandgap Enable In VLPx Operation"]
    #[inline(always)]
    pub fn bgen(&mut self) -> BGEN_W {
        BGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Regulator Status And Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regsc](index.html) module"]
pub struct REGSC_SPEC;
impl crate::RegisterSpec for REGSC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [regsc::R](R) reader structure"]
impl crate::Readable for REGSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regsc::W](W) writer structure"]
impl crate::Writable for REGSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REGSC to value 0x04"]
impl crate::Resettable for REGSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
