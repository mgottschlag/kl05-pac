#[doc = "Register `CFG2` reader"]
pub struct R(crate::R<CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG2` writer"]
pub struct W(crate::W<CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG2_SPEC>;
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
impl From<crate::W<CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Long Sample Time Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADLSTS_A {
    #[doc = "0: Default longest sample time; 20 extra ADCK cycles; 24 ADCK cycles total."]
    _00 = 0,
    #[doc = "1: 12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    _01 = 1,
    #[doc = "2: 6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    _10 = 2,
    #[doc = "3: 2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    _11 = 3,
}
impl From<ADLSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADLSTS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADLSTS` reader - Long Sample Time Select"]
pub struct ADLSTS_R(crate::FieldReader<u8, ADLSTS_A>);
impl ADLSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADLSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLSTS_A {
        match self.bits {
            0 => ADLSTS_A::_00,
            1 => ADLSTS_A::_01,
            2 => ADLSTS_A::_10,
            3 => ADLSTS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == ADLSTS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == ADLSTS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == ADLSTS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == ADLSTS_A::_11
    }
}
impl core::ops::Deref for ADLSTS_R {
    type Target = crate::FieldReader<u8, ADLSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADLSTS` writer - Long Sample Time Select"]
pub struct ADLSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADLSTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADLSTS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Default longest sample time; 20 extra ADCK cycles; 24 ADCK cycles total."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADLSTS_A::_00)
    }
    #[doc = "12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADLSTS_A::_01)
    }
    #[doc = "6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADLSTS_A::_10)
    }
    #[doc = "2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADLSTS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "High-Speed Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADHSC_A {
    #[doc = "0: Normal conversion sequence selected."]
    _0 = 0,
    #[doc = "1: High-speed conversion sequence selected with 2 additional ADCK cycles to total conversion time."]
    _1 = 1,
}
impl From<ADHSC_A> for bool {
    #[inline(always)]
    fn from(variant: ADHSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADHSC` reader - High-Speed Configuration"]
pub struct ADHSC_R(crate::FieldReader<bool, ADHSC_A>);
impl ADHSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADHSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADHSC_A {
        match self.bits {
            false => ADHSC_A::_0,
            true => ADHSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADHSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADHSC_A::_1
    }
}
impl core::ops::Deref for ADHSC_R {
    type Target = crate::FieldReader<bool, ADHSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADHSC` writer - High-Speed Configuration"]
pub struct ADHSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADHSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal conversion sequence selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADHSC_A::_0)
    }
    #[doc = "High-speed conversion sequence selected with 2 additional ADCK cycles to total conversion time."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADHSC_A::_1)
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
#[doc = "Asynchronous Clock Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADACKEN_A {
    #[doc = "0: Asynchronous clock output disabled; Asynchronous clock is enabled only if selected by ADICLK and a conversion is active."]
    _0 = 0,
    #[doc = "1: Asynchronous clock and clock output is enabled regardless of the state of the ADC."]
    _1 = 1,
}
impl From<ADACKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADACKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADACKEN` reader - Asynchronous Clock Output Enable"]
pub struct ADACKEN_R(crate::FieldReader<bool, ADACKEN_A>);
impl ADACKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADACKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACKEN_A {
        match self.bits {
            false => ADACKEN_A::_0,
            true => ADACKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADACKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADACKEN_A::_1
    }
}
impl core::ops::Deref for ADACKEN_R {
    type Target = crate::FieldReader<bool, ADACKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADACKEN` writer - Asynchronous Clock Output Enable"]
pub struct ADACKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADACKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADACKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Asynchronous clock output disabled; Asynchronous clock is enabled only if selected by ADICLK and a conversion is active."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADACKEN_A::_0)
    }
    #[doc = "Asynchronous clock and clock output is enabled regardless of the state of the ADC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADACKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "ADC Mux Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXSEL_A {
    #[doc = "0: ADxxa channels are selected."]
    _0 = 0,
    #[doc = "1: ADxxb channels are selected."]
    _1 = 1,
}
impl From<MUXSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MUXSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUXSEL` reader - ADC Mux Select"]
pub struct MUXSEL_R(crate::FieldReader<bool, MUXSEL_A>);
impl MUXSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUXSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUXSEL_A {
        match self.bits {
            false => MUXSEL_A::_0,
            true => MUXSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MUXSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MUXSEL_A::_1
    }
}
impl core::ops::Deref for MUXSEL_R {
    type Target = crate::FieldReader<bool, MUXSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUXSEL` writer - ADC Mux Select"]
pub struct MUXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADxxa channels are selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MUXSEL_A::_0)
    }
    #[doc = "ADxxb channels are selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MUXSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Long Sample Time Select"]
    #[inline(always)]
    pub fn adlsts(&self) -> ADLSTS_R {
        ADLSTS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - High-Speed Configuration"]
    #[inline(always)]
    pub fn adhsc(&self) -> ADHSC_R {
        ADHSC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Asynchronous Clock Output Enable"]
    #[inline(always)]
    pub fn adacken(&self) -> ADACKEN_R {
        ADACKEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC Mux Select"]
    #[inline(always)]
    pub fn muxsel(&self) -> MUXSEL_R {
        MUXSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Long Sample Time Select"]
    #[inline(always)]
    pub fn adlsts(&mut self) -> ADLSTS_W {
        ADLSTS_W { w: self }
    }
    #[doc = "Bit 2 - High-Speed Configuration"]
    #[inline(always)]
    pub fn adhsc(&mut self) -> ADHSC_W {
        ADHSC_W { w: self }
    }
    #[doc = "Bit 3 - Asynchronous Clock Output Enable"]
    #[inline(always)]
    pub fn adacken(&mut self) -> ADACKEN_W {
        ADACKEN_W { w: self }
    }
    #[doc = "Bit 4 - ADC Mux Select"]
    #[inline(always)]
    pub fn muxsel(&mut self) -> MUXSEL_W {
        MUXSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](index.html) module"]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg2::R](R) reader structure"]
impl crate::Readable for CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg2::W](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
