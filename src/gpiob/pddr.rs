#[doc = "Register `PDDR` reader"]
pub struct R(crate::R<PDDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDR` writer"]
pub struct W(crate::W<PDDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDR_SPEC>;
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
impl From<crate::W<PDDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PDD_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD_A> for u32 {
    #[inline(always)]
    fn from(variant: PDD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PDD` reader - Port Data Direction"]
pub struct PDD_R(crate::FieldReader<u32, PDD_A>);
impl PDD_R {
    pub(crate) fn new(bits: u32) -> Self {
        PDD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PDD_A> {
        match self.bits {
            0 => Some(PDD_A::_0),
            1 => Some(PDD_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDD_A::_1
    }
}
impl core::ops::Deref for PDD_R {
    type Target = crate::FieldReader<u32, PDD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDD` writer - Port Data Direction"]
pub struct PDD_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd(&self) -> PDD_R {
        PDD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd(&mut self) -> PDD_W {
        PDD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Data Direction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddr](index.html) module"]
pub struct PDDR_SPEC;
impl crate::RegisterSpec for PDDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddr::R](R) reader structure"]
impl crate::Readable for PDDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddr::W](W) writer structure"]
impl crate::Writable for PDDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDDR to value 0"]
impl crate::Resettable for PDDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
