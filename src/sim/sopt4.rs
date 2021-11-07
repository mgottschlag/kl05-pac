#[doc = "Register `SOPT4` reader"]
pub struct R(crate::R<SOPT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT4` writer"]
pub struct W(crate::W<SOPT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT4_SPEC>;
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
impl From<crate::W<SOPT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TPM1 channel 0 input capture source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1CH0SRC_A {
    #[doc = "0: TPM1_CH0 signal"]
    _0 = 0,
    #[doc = "1: CMP0 output"]
    _1 = 1,
}
impl From<TPM1CH0SRC_A> for bool {
    #[inline(always)]
    fn from(variant: TPM1CH0SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM1CH0SRC` reader - TPM1 channel 0 input capture source select"]
pub struct TPM1CH0SRC_R(crate::FieldReader<bool, TPM1CH0SRC_A>);
impl TPM1CH0SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPM1CH0SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM1CH0SRC_A {
        match self.bits {
            false => TPM1CH0SRC_A::_0,
            true => TPM1CH0SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TPM1CH0SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TPM1CH0SRC_A::_1
    }
}
impl core::ops::Deref for TPM1CH0SRC_R {
    type Target = crate::FieldReader<bool, TPM1CH0SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPM1CH0SRC` writer - TPM1 channel 0 input capture source select"]
pub struct TPM1CH0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM1CH0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM1CH0SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TPM1_CH0 signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM1CH0SRC_A::_0)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM1CH0SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "TPM0 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM0CLKSEL_A {
    #[doc = "0: TPM0 external clock driven by TPM_CLKIN0 pin."]
    _0 = 0,
    #[doc = "1: TPM0 external clock driven by TPM_CLKIN1 pin."]
    _1 = 1,
}
impl From<TPM0CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TPM0CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM0CLKSEL` reader - TPM0 External Clock Pin Select"]
pub struct TPM0CLKSEL_R(crate::FieldReader<bool, TPM0CLKSEL_A>);
impl TPM0CLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPM0CLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM0CLKSEL_A {
        match self.bits {
            false => TPM0CLKSEL_A::_0,
            true => TPM0CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TPM0CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TPM0CLKSEL_A::_1
    }
}
impl core::ops::Deref for TPM0CLKSEL_R {
    type Target = crate::FieldReader<bool, TPM0CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPM0CLKSEL` writer - TPM0 External Clock Pin Select"]
pub struct TPM0CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM0CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM0CLKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TPM0 external clock driven by TPM_CLKIN0 pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM0CLKSEL_A::_0)
    }
    #[doc = "TPM0 external clock driven by TPM_CLKIN1 pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM0CLKSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "TPM1 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1CLKSEL_A {
    #[doc = "0: TPM1 external clock driven by TPM_CLKIN0 pin."]
    _0 = 0,
    #[doc = "1: TPM1 external clock driven by TPM_CLKIN1 pin."]
    _1 = 1,
}
impl From<TPM1CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TPM1CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM1CLKSEL` reader - TPM1 External Clock Pin Select"]
pub struct TPM1CLKSEL_R(crate::FieldReader<bool, TPM1CLKSEL_A>);
impl TPM1CLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPM1CLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM1CLKSEL_A {
        match self.bits {
            false => TPM1CLKSEL_A::_0,
            true => TPM1CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TPM1CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TPM1CLKSEL_A::_1
    }
}
impl core::ops::Deref for TPM1CLKSEL_R {
    type Target = crate::FieldReader<bool, TPM1CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPM1CLKSEL` writer - TPM1 External Clock Pin Select"]
pub struct TPM1CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM1CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM1CLKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TPM1 external clock driven by TPM_CLKIN0 pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM1CLKSEL_A::_0)
    }
    #[doc = "TPM1 external clock driven by TPM_CLKIN1 pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM1CLKSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - TPM1 channel 0 input capture source select"]
    #[inline(always)]
    pub fn tpm1ch0src(&self) -> TPM1CH0SRC_R {
        TPM1CH0SRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TPM0 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm0clksel(&self) -> TPM0CLKSEL_R {
        TPM0CLKSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm1clksel(&self) -> TPM1CLKSEL_R {
        TPM1CLKSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - TPM1 channel 0 input capture source select"]
    #[inline(always)]
    pub fn tpm1ch0src(&mut self) -> TPM1CH0SRC_W {
        TPM1CH0SRC_W { w: self }
    }
    #[doc = "Bit 24 - TPM0 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm0clksel(&mut self) -> TPM0CLKSEL_W {
        TPM0CLKSEL_W { w: self }
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm1clksel(&mut self) -> TPM1CLKSEL_W {
        TPM1CLKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt4](index.html) module"]
pub struct SOPT4_SPEC;
impl crate::RegisterSpec for SOPT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt4::R](R) reader structure"]
impl crate::Readable for SOPT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt4::W](W) writer structure"]
impl crate::Writable for SOPT4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOPT4 to value 0"]
impl crate::Resettable for SOPT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
