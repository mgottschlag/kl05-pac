#[doc = "Register `A2` reader"]
pub struct R(crate::R<A2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A2` writer"]
pub struct W(crate::W<A2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A2_SPEC>;
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
impl From<crate::W<A2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAD` reader - SMBus Address"]
pub struct SAD_R(crate::FieldReader<u8, u8>);
impl SAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAD` writer - SMBus Address"]
pub struct SAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u8 & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - SMBus Address"]
    #[inline(always)]
    pub fn sad(&self) -> SAD_R {
        SAD_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - SMBus Address"]
    #[inline(always)]
    pub fn sad(&mut self) -> SAD_W {
        SAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Address Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a2](index.html) module"]
pub struct A2_SPEC;
impl crate::RegisterSpec for A2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [a2::R](R) reader structure"]
impl crate::Readable for A2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a2::W](W) writer structure"]
impl crate::Writable for A2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets A2 to value 0xc2"]
impl crate::Resettable for A2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc2
    }
}
