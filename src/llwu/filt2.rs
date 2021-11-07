#[doc = "Register `FILT2` reader"]
pub struct R(crate::R<FILT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILT2` writer"]
pub struct W(crate::W<FILT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILT2_SPEC>;
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
impl From<crate::W<FILT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Filter Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTSEL_A {
    #[doc = "0: Select LLWU_P0 for filter"]
    _0000 = 0,
    #[doc = "15: Select LLWU_P15 for filter"]
    _1111 = 15,
}
impl From<FILTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FILTSEL` reader - Filter Pin Select"]
pub struct FILTSEL_R(crate::FieldReader<u8, FILTSEL_A>);
impl FILTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FILTSEL_A> {
        match self.bits {
            0 => Some(FILTSEL_A::_0000),
            15 => Some(FILTSEL_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == FILTSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        **self == FILTSEL_A::_1111
    }
}
impl core::ops::Deref for FILTSEL_R {
    type Target = crate::FieldReader<u8, FILTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTSEL` writer - Filter Pin Select"]
pub struct FILTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select LLWU_P0 for filter"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(FILTSEL_A::_0000)
    }
    #[doc = "Select LLWU_P15 for filter"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(FILTSEL_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Digital Filter On External Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTE_A {
    #[doc = "0: Filter disabled"]
    _00 = 0,
    #[doc = "1: Filter posedge detect enabled"]
    _01 = 1,
    #[doc = "2: Filter negedge detect enabled"]
    _10 = 2,
    #[doc = "3: Filter any edge detect enabled"]
    _11 = 3,
}
impl From<FILTE_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FILTE` reader - Digital Filter On External Pin"]
pub struct FILTE_R(crate::FieldReader<u8, FILTE_A>);
impl FILTE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTE_A {
        match self.bits {
            0 => FILTE_A::_00,
            1 => FILTE_A::_01,
            2 => FILTE_A::_10,
            3 => FILTE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FILTE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FILTE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FILTE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FILTE_A::_11
    }
}
impl core::ops::Deref for FILTE_R {
    type Target = crate::FieldReader<u8, FILTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTE` writer - Digital Filter On External Pin"]
pub struct FILTE_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FILTE_A::_00)
    }
    #[doc = "Filter posedge detect enabled"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FILTE_A::_01)
    }
    #[doc = "Filter negedge detect enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FILTE_A::_10)
    }
    #[doc = "Filter any edge detect enabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FILTE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u8 & 0x03) << 5);
        self.w
    }
}
#[doc = "Filter Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTF_A {
    #[doc = "0: Pin Filter 2 was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Pin Filter 2 was a wakeup source"]
    _1 = 1,
}
impl From<FILTF_A> for bool {
    #[inline(always)]
    fn from(variant: FILTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTF` reader - Filter Detect Flag"]
pub struct FILTF_R(crate::FieldReader<bool, FILTF_A>);
impl FILTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FILTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTF_A {
        match self.bits {
            false => FILTF_A::_0,
            true => FILTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FILTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FILTF_A::_1
    }
}
impl core::ops::Deref for FILTF_R {
    type Target = crate::FieldReader<bool, FILTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTF` writer - Filter Detect Flag"]
pub struct FILTF_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Filter 2 was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTF_A::_0)
    }
    #[doc = "Pin Filter 2 was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTF_A::_1)
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
    #[doc = "Bits 0:3 - Filter Pin Select"]
    #[inline(always)]
    pub fn filtsel(&self) -> FILTSEL_R {
        FILTSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - Digital Filter On External Pin"]
    #[inline(always)]
    pub fn filte(&self) -> FILTE_R {
        FILTE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Filter Detect Flag"]
    #[inline(always)]
    pub fn filtf(&self) -> FILTF_R {
        FILTF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Filter Pin Select"]
    #[inline(always)]
    pub fn filtsel(&mut self) -> FILTSEL_W {
        FILTSEL_W { w: self }
    }
    #[doc = "Bits 5:6 - Digital Filter On External Pin"]
    #[inline(always)]
    pub fn filte(&mut self) -> FILTE_W {
        FILTE_W { w: self }
    }
    #[doc = "Bit 7 - Filter Detect Flag"]
    #[inline(always)]
    pub fn filtf(&mut self) -> FILTF_W {
        FILTF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Filter 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filt2](index.html) module"]
pub struct FILT2_SPEC;
impl crate::RegisterSpec for FILT2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [filt2::R](R) reader structure"]
impl crate::Readable for FILT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filt2::W](W) writer structure"]
impl crate::Writable for FILT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILT2 to value 0"]
impl crate::Resettable for FILT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
