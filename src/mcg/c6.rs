#[doc = "Register `C6` reader"]
pub struct R(crate::R<C6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C6` writer"]
pub struct W(crate::W<C6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C6_SPEC>;
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
impl From<crate::W<C6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CME_A {
    #[doc = "0: External clock monitor is disabled."]
    _0 = 0,
    #[doc = "1: Generate a reset request on loss of external clock."]
    _1 = 1,
}
impl From<CME_A> for bool {
    #[inline(always)]
    fn from(variant: CME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CME` reader - Clock Monitor Enable"]
pub struct CME_R(crate::FieldReader<bool, CME_A>);
impl CME_R {
    pub(crate) fn new(bits: bool) -> Self {
        CME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CME_A {
        match self.bits {
            false => CME_A::_0,
            true => CME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CME_A::_1
    }
}
impl core::ops::Deref for CME_R {
    type Target = crate::FieldReader<bool, CME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CME` writer - Clock Monitor Enable"]
pub struct CME_W<'a> {
    w: &'a mut W,
}
impl<'a> CME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External clock monitor is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CME_A::_0)
    }
    #[doc = "Generate a reset request on loss of external clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CME_A::_1)
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
impl R {
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme(&self) -> CME_R {
        CME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme(&mut self) -> CME_W {
        CME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6](index.html) module"]
pub struct C6_SPEC;
impl crate::RegisterSpec for C6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c6::R](R) reader structure"]
impl crate::Readable for C6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c6::W](W) writer structure"]
impl crate::Writable for C6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C6 to value 0"]
impl crate::Resettable for C6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
