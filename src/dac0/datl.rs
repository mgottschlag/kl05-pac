#[doc = "Register `DAT%sL` reader"]
pub struct R(crate::R<DATL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAT%sL` writer"]
pub struct W(crate::W<DATL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATL_SPEC>;
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
impl From<crate::W<DATL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` reader - no description available"]
pub struct DATA0_R(crate::FieldReader<u8, u8>);
impl DATA0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA0` writer - no description available"]
pub struct DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Data Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datl](index.html) module"]
pub struct DATL_SPEC;
impl crate::RegisterSpec for DATL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [datl::R](R) reader structure"]
impl crate::Readable for DATL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datl::W](W) writer structure"]
impl crate::Writable for DATL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAT%sL to value 0"]
impl crate::Resettable for DATL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
