#[doc = "Register `D` reader"]
pub struct R(crate::R<D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D` writer"]
pub struct W(crate::W<D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_SPEC>;
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
impl From<crate::W<D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R0T0` reader - no description available"]
pub struct R0T0_R(crate::FieldReader<bool, bool>);
impl R0T0_R {
    pub(crate) fn new(bits: bool) -> Self {
        R0T0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R0T0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R0T0` writer - no description available"]
pub struct R0T0_W<'a> {
    w: &'a mut W,
}
impl<'a> R0T0_W<'a> {
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
#[doc = "Field `R1T1` reader - no description available"]
pub struct R1T1_R(crate::FieldReader<bool, bool>);
impl R1T1_R {
    pub(crate) fn new(bits: bool) -> Self {
        R1T1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R1T1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R1T1` writer - no description available"]
pub struct R1T1_W<'a> {
    w: &'a mut W,
}
impl<'a> R1T1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `R2T2` reader - no description available"]
pub struct R2T2_R(crate::FieldReader<bool, bool>);
impl R2T2_R {
    pub(crate) fn new(bits: bool) -> Self {
        R2T2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R2T2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R2T2` writer - no description available"]
pub struct R2T2_W<'a> {
    w: &'a mut W,
}
impl<'a> R2T2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `R3T3` reader - no description available"]
pub struct R3T3_R(crate::FieldReader<bool, bool>);
impl R3T3_R {
    pub(crate) fn new(bits: bool) -> Self {
        R3T3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R3T3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R3T3` writer - no description available"]
pub struct R3T3_W<'a> {
    w: &'a mut W,
}
impl<'a> R3T3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `R4T4` reader - no description available"]
pub struct R4T4_R(crate::FieldReader<bool, bool>);
impl R4T4_R {
    pub(crate) fn new(bits: bool) -> Self {
        R4T4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R4T4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R4T4` writer - no description available"]
pub struct R4T4_W<'a> {
    w: &'a mut W,
}
impl<'a> R4T4_W<'a> {
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
#[doc = "Field `R5T5` reader - no description available"]
pub struct R5T5_R(crate::FieldReader<bool, bool>);
impl R5T5_R {
    pub(crate) fn new(bits: bool) -> Self {
        R5T5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R5T5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R5T5` writer - no description available"]
pub struct R5T5_W<'a> {
    w: &'a mut W,
}
impl<'a> R5T5_W<'a> {
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
#[doc = "Field `R6T6` reader - no description available"]
pub struct R6T6_R(crate::FieldReader<bool, bool>);
impl R6T6_R {
    pub(crate) fn new(bits: bool) -> Self {
        R6T6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R6T6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R6T6` writer - no description available"]
pub struct R6T6_W<'a> {
    w: &'a mut W,
}
impl<'a> R6T6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `R7T7` reader - no description available"]
pub struct R7T7_R(crate::FieldReader<bool, bool>);
impl R7T7_R {
    pub(crate) fn new(bits: bool) -> Self {
        R7T7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R7T7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R7T7` writer - no description available"]
pub struct R7T7_W<'a> {
    w: &'a mut W,
}
impl<'a> R7T7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn r0t0(&self) -> R0T0_R {
        R0T0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn r1t1(&self) -> R1T1_R {
        R1T1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn r2t2(&self) -> R2T2_R {
        R2T2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn r3t3(&self) -> R3T3_R {
        R3T3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn r4t4(&self) -> R4T4_R {
        R4T4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn r5t5(&self) -> R5T5_R {
        R5T5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn r6t6(&self) -> R6T6_R {
        R6T6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn r7t7(&self) -> R7T7_R {
        R7T7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn r0t0(&mut self) -> R0T0_W {
        R0T0_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn r1t1(&mut self) -> R1T1_W {
        R1T1_W { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn r2t2(&mut self) -> R2T2_W {
        R2T2_W { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn r3t3(&mut self) -> R3T3_W {
        R3T3_W { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn r4t4(&mut self) -> R4T4_W {
        R4T4_W { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn r5t5(&mut self) -> R5T5_W {
        R5T5_W { w: self }
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn r6t6(&mut self) -> R6T6_W {
        R6T6_W { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn r7t7(&mut self) -> R7T7_W {
        R7T7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d](index.html) module"]
pub struct D_SPEC;
impl crate::RegisterSpec for D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [d::R](R) reader structure"]
impl crate::Readable for D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d::W](W) writer structure"]
impl crate::Writable for D_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D to value 0"]
impl crate::Resettable for D_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
