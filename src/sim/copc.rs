#[doc = "Register `COPC` reader"]
pub struct R(crate::R<COPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COPC` writer"]
pub struct W(crate::W<COPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COPC_SPEC>;
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
impl From<crate::W<COPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "COP windowed mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPW_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Windowed mode"]
    _1 = 1,
}
impl From<COPW_A> for bool {
    #[inline(always)]
    fn from(variant: COPW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COPW` reader - COP windowed mode"]
pub struct COPW_R(crate::FieldReader<bool, COPW_A>);
impl COPW_R {
    pub(crate) fn new(bits: bool) -> Self {
        COPW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COPW_A {
        match self.bits {
            false => COPW_A::_0,
            true => COPW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COPW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COPW_A::_1
    }
}
impl core::ops::Deref for COPW_R {
    type Target = crate::FieldReader<bool, COPW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COPW` writer - COP windowed mode"]
pub struct COPW_W<'a> {
    w: &'a mut W,
}
impl<'a> COPW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COPW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COPW_A::_0)
    }
    #[doc = "Windowed mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COPW_A::_1)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "COP Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPCLKS_A {
    #[doc = "0: Internal 1 kHz clock is source to COP"]
    _0 = 0,
    #[doc = "1: Bus clock is source to COP"]
    _1 = 1,
}
impl From<COPCLKS_A> for bool {
    #[inline(always)]
    fn from(variant: COPCLKS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COPCLKS` reader - COP Clock Select"]
pub struct COPCLKS_R(crate::FieldReader<bool, COPCLKS_A>);
impl COPCLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        COPCLKS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COPCLKS_A {
        match self.bits {
            false => COPCLKS_A::_0,
            true => COPCLKS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COPCLKS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COPCLKS_A::_1
    }
}
impl core::ops::Deref for COPCLKS_R {
    type Target = crate::FieldReader<bool, COPCLKS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COPCLKS` writer - COP Clock Select"]
pub struct COPCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> COPCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COPCLKS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal 1 kHz clock is source to COP"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COPCLKS_A::_0)
    }
    #[doc = "Bus clock is source to COP"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COPCLKS_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "COP Watchdog Timeout\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COPT_A {
    #[doc = "0: COP disabled"]
    _00 = 0,
    #[doc = "1: COP timeout after 2^5 LPO cycles or 213 bus clock cycles"]
    _01 = 1,
    #[doc = "2: COP timeout after 2^8 LPO cycles or 216 bus clock cycles"]
    _10 = 2,
    #[doc = "3: COP timeout after 2^10 LPO cycles or 218 bus clock cycles"]
    _11 = 3,
}
impl From<COPT_A> for u8 {
    #[inline(always)]
    fn from(variant: COPT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COPT` reader - COP Watchdog Timeout"]
pub struct COPT_R(crate::FieldReader<u8, COPT_A>);
impl COPT_R {
    pub(crate) fn new(bits: u8) -> Self {
        COPT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COPT_A {
        match self.bits {
            0 => COPT_A::_00,
            1 => COPT_A::_01,
            2 => COPT_A::_10,
            3 => COPT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == COPT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == COPT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == COPT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == COPT_A::_11
    }
}
impl core::ops::Deref for COPT_R {
    type Target = crate::FieldReader<u8, COPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COPT` writer - COP Watchdog Timeout"]
pub struct COPT_W<'a> {
    w: &'a mut W,
}
impl<'a> COPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COPT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "COP disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(COPT_A::_00)
    }
    #[doc = "COP timeout after 2^5 LPO cycles or 213 bus clock cycles"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(COPT_A::_01)
    }
    #[doc = "COP timeout after 2^8 LPO cycles or 216 bus clock cycles"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(COPT_A::_10)
    }
    #[doc = "COP timeout after 2^10 LPO cycles or 218 bus clock cycles"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(COPT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - COP windowed mode"]
    #[inline(always)]
    pub fn copw(&self) -> COPW_R {
        COPW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - COP Clock Select"]
    #[inline(always)]
    pub fn copclks(&self) -> COPCLKS_R {
        COPCLKS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - COP Watchdog Timeout"]
    #[inline(always)]
    pub fn copt(&self) -> COPT_R {
        COPT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - COP windowed mode"]
    #[inline(always)]
    pub fn copw(&mut self) -> COPW_W {
        COPW_W { w: self }
    }
    #[doc = "Bit 1 - COP Clock Select"]
    #[inline(always)]
    pub fn copclks(&mut self) -> COPCLKS_W {
        COPCLKS_W { w: self }
    }
    #[doc = "Bits 2:3 - COP Watchdog Timeout"]
    #[inline(always)]
    pub fn copt(&mut self) -> COPT_W {
        COPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [copc](index.html) module"]
pub struct COPC_SPEC;
impl crate::RegisterSpec for COPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [copc::R](R) reader structure"]
impl crate::Readable for COPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [copc::W](W) writer structure"]
impl crate::Writable for COPC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COPC to value 0x0c"]
impl crate::Resettable for COPC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
