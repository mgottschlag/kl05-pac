#[doc = "Register `ME` reader"]
pub struct R(crate::R<ME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ME` writer"]
pub struct W(crate::W<ME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ME_SPEC>;
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
impl From<crate::W<ME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup Module Enable For Module 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME0_A {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<WUME0_A> for bool {
    #[inline(always)]
    fn from(variant: WUME0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME0` reader - Wakeup Module Enable For Module 0"]
pub struct WUME0_R(crate::FieldReader<bool, WUME0_A>);
impl WUME0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUME0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUME0_A {
        match self.bits {
            false => WUME0_A::_0,
            true => WUME0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUME0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUME0_A::_1
    }
}
impl core::ops::Deref for WUME0_R {
    type Target = crate::FieldReader<bool, WUME0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUME0` writer - Wakeup Module Enable For Module 0"]
pub struct WUME0_W<'a> {
    w: &'a mut W,
}
impl<'a> WUME0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUME0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME0_A::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME0_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Wakeup Module Enable for Module 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME1_A {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<WUME1_A> for bool {
    #[inline(always)]
    fn from(variant: WUME1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME1` reader - Wakeup Module Enable for Module 1"]
pub struct WUME1_R(crate::FieldReader<bool, WUME1_A>);
impl WUME1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUME1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUME1_A {
        match self.bits {
            false => WUME1_A::_0,
            true => WUME1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUME1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUME1_A::_1
    }
}
impl core::ops::Deref for WUME1_R {
    type Target = crate::FieldReader<bool, WUME1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUME1` writer - Wakeup Module Enable for Module 1"]
pub struct WUME1_W<'a> {
    w: &'a mut W,
}
impl<'a> WUME1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUME1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME1_A::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Wakeup Module Enable For Module 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME2_A {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<WUME2_A> for bool {
    #[inline(always)]
    fn from(variant: WUME2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME2` reader - Wakeup Module Enable For Module 2"]
pub struct WUME2_R(crate::FieldReader<bool, WUME2_A>);
impl WUME2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUME2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUME2_A {
        match self.bits {
            false => WUME2_A::_0,
            true => WUME2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUME2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUME2_A::_1
    }
}
impl core::ops::Deref for WUME2_R {
    type Target = crate::FieldReader<bool, WUME2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUME2` writer - Wakeup Module Enable For Module 2"]
pub struct WUME2_W<'a> {
    w: &'a mut W,
}
impl<'a> WUME2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUME2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME2_A::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Wakeup Module Enable For Module 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME3_A {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<WUME3_A> for bool {
    #[inline(always)]
    fn from(variant: WUME3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME3` reader - Wakeup Module Enable For Module 3"]
pub struct WUME3_R(crate::FieldReader<bool, WUME3_A>);
impl WUME3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUME3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUME3_A {
        match self.bits {
            false => WUME3_A::_0,
            true => WUME3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUME3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUME3_A::_1
    }
}
impl core::ops::Deref for WUME3_R {
    type Target = crate::FieldReader<bool, WUME3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUME3` writer - Wakeup Module Enable For Module 3"]
pub struct WUME3_W<'a> {
    w: &'a mut W,
}
impl<'a> WUME3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUME3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME3_A::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Wakeup Module Enable For Module 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME4_A {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<WUME4_A> for bool {
    #[inline(always)]
    fn from(variant: WUME4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME4` reader - Wakeup Module Enable For Module 4"]
pub struct WUME4_R(crate::FieldReader<bool, WUME4_A>);
impl WUME4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUME4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUME4_A {
        match self.bits {
            false => WUME4_A::_0,
            true => WUME4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUME4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUME4_A::_1
    }
}
impl core::ops::Deref for WUME4_R {
    type Target = crate::FieldReader<bool, WUME4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUME4` writer - Wakeup Module Enable For Module 4"]
pub struct WUME4_W<'a> {
    w: &'a mut W,
}
impl<'a> WUME4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUME4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME4_A::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Wakeup Module Enable For Module 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME5_A {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<WUME5_A> for bool {
    #[inline(always)]
    fn from(variant: WUME5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME5` reader - Wakeup Module Enable For Module 5"]
pub struct WUME5_R(crate::FieldReader<bool, WUME5_A>);
impl WUME5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUME5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUME5_A {
        match self.bits {
            false => WUME5_A::_0,
            true => WUME5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUME5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUME5_A::_1
    }
}
impl core::ops::Deref for WUME5_R {
    type Target = crate::FieldReader<bool, WUME5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUME5` writer - Wakeup Module Enable For Module 5"]
pub struct WUME5_W<'a> {
    w: &'a mut W,
}
impl<'a> WUME5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUME5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME5_A::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Wakeup Module Enable For Module 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME6_A {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<WUME6_A> for bool {
    #[inline(always)]
    fn from(variant: WUME6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME6` reader - Wakeup Module Enable For Module 6"]
pub struct WUME6_R(crate::FieldReader<bool, WUME6_A>);
impl WUME6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUME6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUME6_A {
        match self.bits {
            false => WUME6_A::_0,
            true => WUME6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUME6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUME6_A::_1
    }
}
impl core::ops::Deref for WUME6_R {
    type Target = crate::FieldReader<bool, WUME6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUME6` writer - Wakeup Module Enable For Module 6"]
pub struct WUME6_W<'a> {
    w: &'a mut W,
}
impl<'a> WUME6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUME6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME6_A::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Wakeup Module Enable For Module 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME7_A {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<WUME7_A> for bool {
    #[inline(always)]
    fn from(variant: WUME7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME7` reader - Wakeup Module Enable For Module 7"]
pub struct WUME7_R(crate::FieldReader<bool, WUME7_A>);
impl WUME7_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUME7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUME7_A {
        match self.bits {
            false => WUME7_A::_0,
            true => WUME7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUME7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUME7_A::_1
    }
}
impl core::ops::Deref for WUME7_R {
    type Target = crate::FieldReader<bool, WUME7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUME7` writer - Wakeup Module Enable For Module 7"]
pub struct WUME7_W<'a> {
    w: &'a mut W,
}
impl<'a> WUME7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUME7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME7_A::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Module Enable For Module 0"]
    #[inline(always)]
    pub fn wume0(&self) -> WUME0_R {
        WUME0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Module Enable for Module 1"]
    #[inline(always)]
    pub fn wume1(&self) -> WUME1_R {
        WUME1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Module Enable For Module 2"]
    #[inline(always)]
    pub fn wume2(&self) -> WUME2_R {
        WUME2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup Module Enable For Module 3"]
    #[inline(always)]
    pub fn wume3(&self) -> WUME3_R {
        WUME3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Module Enable For Module 4"]
    #[inline(always)]
    pub fn wume4(&self) -> WUME4_R {
        WUME4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup Module Enable For Module 5"]
    #[inline(always)]
    pub fn wume5(&self) -> WUME5_R {
        WUME5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wakeup Module Enable For Module 6"]
    #[inline(always)]
    pub fn wume6(&self) -> WUME6_R {
        WUME6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup Module Enable For Module 7"]
    #[inline(always)]
    pub fn wume7(&self) -> WUME7_R {
        WUME7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Module Enable For Module 0"]
    #[inline(always)]
    pub fn wume0(&mut self) -> WUME0_W {
        WUME0_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Module Enable for Module 1"]
    #[inline(always)]
    pub fn wume1(&mut self) -> WUME1_W {
        WUME1_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Module Enable For Module 2"]
    #[inline(always)]
    pub fn wume2(&mut self) -> WUME2_W {
        WUME2_W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Module Enable For Module 3"]
    #[inline(always)]
    pub fn wume3(&mut self) -> WUME3_W {
        WUME3_W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Module Enable For Module 4"]
    #[inline(always)]
    pub fn wume4(&mut self) -> WUME4_W {
        WUME4_W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Module Enable For Module 5"]
    #[inline(always)]
    pub fn wume5(&mut self) -> WUME5_W {
        WUME5_W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Module Enable For Module 6"]
    #[inline(always)]
    pub fn wume6(&mut self) -> WUME6_W {
        WUME6_W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Module Enable For Module 7"]
    #[inline(always)]
    pub fn wume7(&mut self) -> WUME7_W {
        WUME7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Module Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [me](index.html) module"]
pub struct ME_SPEC;
impl crate::RegisterSpec for ME_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [me::R](R) reader structure"]
impl crate::Readable for ME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [me::W](W) writer structure"]
impl crate::Writable for ME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ME to value 0"]
impl crate::Resettable for ME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
