#[doc = "Register `PG` reader"]
pub struct R(crate::R<PG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PG` writer"]
pub struct W(crate::W<PG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_SPEC>;
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
impl From<crate::W<PG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PG` reader - Plus-Side Gain"]
pub struct PG_R(crate::FieldReader<u16, u16>);
impl PG_R {
    pub(crate) fn new(bits: u16) -> Self {
        PG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG` writer - Plus-Side Gain"]
pub struct PG_W<'a> {
    w: &'a mut W,
}
impl<'a> PG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Plus-Side Gain"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Plus-Side Gain"]
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg](index.html) module"]
pub struct PG_SPEC;
impl crate::RegisterSpec for PG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg::R](R) reader structure"]
impl crate::Readable for PG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg::W](W) writer structure"]
impl crate::Writable for PG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PG to value 0x8200"]
impl crate::Resettable for PG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8200
    }
}
