#[doc = "Register `FPROT%s` reader"]
pub struct R(crate::R<FPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPROT%s` writer"]
pub struct W(crate::W<FPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPROT_SPEC>;
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
impl From<crate::W<FPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Program Flash Region Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PROT_A {
    #[doc = "0: Program flash region is protected."]
    _0 = 0,
    #[doc = "1: Program flash region is not protected"]
    _1 = 1,
}
impl From<PROT_A> for u8 {
    #[inline(always)]
    fn from(variant: PROT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PROT` reader - Program Flash Region Protect"]
pub struct PROT_R(crate::FieldReader<u8, PROT_A>);
impl PROT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PROT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROT_A> {
        match self.bits {
            0 => Some(PROT_A::_0),
            1 => Some(PROT_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PROT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PROT_A::_1
    }
}
impl core::ops::Deref for PROT_R {
    type Target = crate::FieldReader<u8, PROT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT` writer - Program Flash Region Protect"]
pub struct PROT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Program flash region is protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROT_A::_0)
    }
    #[doc = "Program flash region is not protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROT_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&mut self) -> PROT_W {
        PROT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program Flash Protection Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot](index.html) module"]
pub struct FPROT_SPEC;
impl crate::RegisterSpec for FPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fprot::R](R) reader structure"]
impl crate::Readable for FPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fprot::W](W) writer structure"]
impl crate::Writable for FPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPROT%s to value 0"]
impl crate::Resettable for FPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
