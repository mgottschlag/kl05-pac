#[doc = "Register `ATCVL` reader"]
pub struct R(crate::R<ATCVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATCVL` writer"]
pub struct W(crate::W<ATCVL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCVL_SPEC>;
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
impl From<crate::W<ATCVL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCVL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATCVL` reader - ATM Compare Value Low"]
pub struct ATCVL_R(crate::FieldReader<u8, u8>);
impl ATCVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATCVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATCVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATCVL` writer - ATM Compare Value Low"]
pub struct ATCVL_W<'a> {
    w: &'a mut W,
}
impl<'a> ATCVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ATM Compare Value Low"]
    #[inline(always)]
    pub fn atcvl(&self) -> ATCVL_R {
        ATCVL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATM Compare Value Low"]
    #[inline(always)]
    pub fn atcvl(&mut self) -> ATCVL_W {
        ATCVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Auto Trim Compare Value Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcvl](index.html) module"]
pub struct ATCVL_SPEC;
impl crate::RegisterSpec for ATCVL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [atcvl::R](R) reader structure"]
impl crate::Readable for ATCVL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atcvl::W](W) writer structure"]
impl crate::Writable for ATCVL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATCVL to value 0"]
impl crate::Resettable for ATCVL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
