#[doc = "Register `C2` reader"]
pub struct R(crate::R<C2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2` writer"]
pub struct W(crate::W<C2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2_SPEC>;
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
impl From<crate::W<C2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACBFUP` reader - DAC Buffer Upper Limit"]
pub struct DACBFUP_R(crate::FieldReader<bool, bool>);
impl DACBFUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACBFUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACBFUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACBFUP` writer - DAC Buffer Upper Limit"]
pub struct DACBFUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFUP_W<'a> {
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
#[doc = "Field `DACBFRP` reader - DAC Buffer Read Pointer"]
pub struct DACBFRP_R(crate::FieldReader<bool, bool>);
impl DACBFRP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACBFRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACBFRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACBFRP` writer - DAC Buffer Read Pointer"]
pub struct DACBFRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFRP_W<'a> {
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
    #[doc = "Bit 0 - DAC Buffer Upper Limit"]
    #[inline(always)]
    pub fn dacbfup(&self) -> DACBFUP_R {
        DACBFUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DAC Buffer Read Pointer"]
    #[inline(always)]
    pub fn dacbfrp(&self) -> DACBFRP_R {
        DACBFRP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Upper Limit"]
    #[inline(always)]
    pub fn dacbfup(&mut self) -> DACBFUP_W {
        DACBFUP_W { w: self }
    }
    #[doc = "Bit 4 - DAC Buffer Read Pointer"]
    #[inline(always)]
    pub fn dacbfrp(&mut self) -> DACBFRP_W {
        DACBFRP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c2::R](R) reader structure"]
impl crate::Readable for C2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2::W](W) writer structure"]
impl crate::Writable for C2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2 to value 0x01"]
impl crate::Resettable for C2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
