#[doc = "Register `PSOR` writer"]
pub struct W(crate::W<PSOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSOR_SPEC>;
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
impl From<crate::W<PSOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PTSO_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO_AW> for u32 {
    #[inline(always)]
    fn from(variant: PTSO_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `PTSO` writer - Port Set Output"]
pub struct PTSO_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Set Output"]
    #[inline(always)]
    pub fn ptso(&mut self) -> PTSO_W {
        PTSO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Set Output Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psor](index.html) module"]
pub struct PSOR_SPEC;
impl crate::RegisterSpec for PSOR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [psor::W](W) writer structure"]
impl crate::Writable for PSOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSOR to value 0"]
impl crate::Resettable for PSOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
