#[doc = "Register `DSR1` reader"]
pub struct R(crate::R<DSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSR1` writer"]
pub struct W(crate::W<DSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSR1_SPEC>;
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
impl From<crate::W<DSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSR1_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_DSR1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr1](index.html) module"]
pub struct DSR1_SPEC;
impl crate::RegisterSpec for DSR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dsr1::R](R) reader structure"]
impl crate::Readable for DSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsr1::W](W) writer structure"]
impl crate::Writable for DSR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSR1 to value 0"]
impl crate::Resettable for DSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
