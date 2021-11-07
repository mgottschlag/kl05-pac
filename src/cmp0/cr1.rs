#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparator Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Analog Comparator is disabled."]
    _0 = 0,
    #[doc = "1: Analog Comparator is enabled."]
    _1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Comparator Module Enable"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::_0,
            true => EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EN_A::_1
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Comparator Module Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog Comparator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
    #[doc = "Analog Comparator is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
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
#[doc = "Comparator Output Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPE_A {
    #[doc = "0: CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
    _0 = 0,
    #[doc = "1: CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
    _1 = 1,
}
impl From<OPE_A> for bool {
    #[inline(always)]
    fn from(variant: OPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPE` reader - Comparator Output Pin Enable"]
pub struct OPE_R(crate::FieldReader<bool, OPE_A>);
impl OPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPE_A {
        match self.bits {
            false => OPE_A::_0,
            true => OPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OPE_A::_1
    }
}
impl core::ops::Deref for OPE_R {
    type Target = crate::FieldReader<bool, OPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPE` writer - Comparator Output Pin Enable"]
pub struct OPE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OPE_A::_0)
    }
    #[doc = "CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OPE_A::_1)
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
#[doc = "Comparator Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COS_A {
    #[doc = "0: Set the filtered comparator output (CMPO) to equal COUT."]
    _0 = 0,
    #[doc = "1: Set the unfiltered comparator output (CMPO) to equal COUTA."]
    _1 = 1,
}
impl From<COS_A> for bool {
    #[inline(always)]
    fn from(variant: COS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COS` reader - Comparator Output Select"]
pub struct COS_R(crate::FieldReader<bool, COS_A>);
impl COS_R {
    pub(crate) fn new(bits: bool) -> Self {
        COS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COS_A {
        match self.bits {
            false => COS_A::_0,
            true => COS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COS_A::_1
    }
}
impl core::ops::Deref for COS_R {
    type Target = crate::FieldReader<bool, COS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COS` writer - Comparator Output Select"]
pub struct COS_W<'a> {
    w: &'a mut W,
}
impl<'a> COS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set the filtered comparator output (CMPO) to equal COUT."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COS_A::_0)
    }
    #[doc = "Set the unfiltered comparator output (CMPO) to equal COUTA."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COS_A::_1)
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
#[doc = "Comparator INVERT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_A {
    #[doc = "0: Does not invert the comparator output."]
    _0 = 0,
    #[doc = "1: Inverts the comparator output."]
    _1 = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Comparator INVERT"]
pub struct INV_R(crate::FieldReader<bool, INV_A>);
impl INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        INV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::_0,
            true => INV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INV_A::_1
    }
}
impl core::ops::Deref for INV_R {
    type Target = crate::FieldReader<bool, INV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV` writer - Comparator INVERT"]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Does not invert the comparator output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV_A::_0)
    }
    #[doc = "Inverts the comparator output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV_A::_1)
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
#[doc = "Power Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMODE_A {
    #[doc = "0: Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
    _0 = 0,
    #[doc = "1: High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
    _1 = 1,
}
impl From<PMODE_A> for bool {
    #[inline(always)]
    fn from(variant: PMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMODE` reader - Power Mode Select"]
pub struct PMODE_R(crate::FieldReader<bool, PMODE_A>);
impl PMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMODE_A {
        match self.bits {
            false => PMODE_A::_0,
            true => PMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PMODE_A::_1
    }
}
impl core::ops::Deref for PMODE_R {
    type Target = crate::FieldReader<bool, PMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMODE` writer - Power Mode Select"]
pub struct PMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PMODE_A::_0)
    }
    #[doc = "High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PMODE_A::_1)
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
#[doc = "Trigger Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGM_A {
    #[doc = "0: Trigger mode is disabled."]
    _0 = 0,
    #[doc = "1: Trigger mode is enabled."]
    _1 = 1,
}
impl From<TRIGM_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGM` reader - Trigger Mode Enable"]
pub struct TRIGM_R(crate::FieldReader<bool, TRIGM_A>);
impl TRIGM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRIGM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGM_A {
        match self.bits {
            false => TRIGM_A::_0,
            true => TRIGM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRIGM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRIGM_A::_1
    }
}
impl core::ops::Deref for TRIGM_R {
    type Target = crate::FieldReader<bool, TRIGM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGM` writer - Trigger Mode Enable"]
pub struct TRIGM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIGM_A::_0)
    }
    #[doc = "Trigger mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIGM_A::_1)
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
#[doc = "Windowing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WE_A {
    #[doc = "0: Windowing mode is not selected."]
    _0 = 0,
    #[doc = "1: Windowing mode is selected."]
    _1 = 1,
}
impl From<WE_A> for bool {
    #[inline(always)]
    fn from(variant: WE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE` reader - Windowing Enable"]
pub struct WE_R(crate::FieldReader<bool, WE_A>);
impl WE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WE_A {
        match self.bits {
            false => WE_A::_0,
            true => WE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WE_A::_1
    }
}
impl core::ops::Deref for WE_R {
    type Target = crate::FieldReader<bool, WE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WE` writer - Windowing Enable"]
pub struct WE_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Windowing mode is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WE_A::_0)
    }
    #[doc = "Windowing mode is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WE_A::_1)
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
#[doc = "Sample Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SE_A {
    #[doc = "0: Sampling mode is not selected."]
    _0 = 0,
    #[doc = "1: Sampling mode is selected."]
    _1 = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SE` reader - Sample Enable"]
pub struct SE_R(crate::FieldReader<bool, SE_A>);
impl SE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::_0,
            true => SE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SE_A::_1
    }
}
impl core::ops::Deref for SE_R {
    type Target = crate::FieldReader<bool, SE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE` writer - Sample Enable"]
pub struct SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sampling mode is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SE_A::_0)
    }
    #[doc = "Sampling mode is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SE_A::_1)
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
    #[doc = "Bit 0 - Comparator Module Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator Output Pin Enable"]
    #[inline(always)]
    pub fn ope(&self) -> OPE_R {
        OPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator Output Select"]
    #[inline(always)]
    pub fn cos(&self) -> COS_R {
        COS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator INVERT"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Power Mode Select"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Trigger Mode Enable"]
    #[inline(always)]
    pub fn trigm(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Windowing Enable"]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sample Enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator Module Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Comparator Output Pin Enable"]
    #[inline(always)]
    pub fn ope(&mut self) -> OPE_W {
        OPE_W { w: self }
    }
    #[doc = "Bit 2 - Comparator Output Select"]
    #[inline(always)]
    pub fn cos(&mut self) -> COS_W {
        COS_W { w: self }
    }
    #[doc = "Bit 3 - Comparator INVERT"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bit 4 - Power Mode Select"]
    #[inline(always)]
    pub fn pmode(&mut self) -> PMODE_W {
        PMODE_W { w: self }
    }
    #[doc = "Bit 5 - Trigger Mode Enable"]
    #[inline(always)]
    pub fn trigm(&mut self) -> TRIGM_W {
        TRIGM_W { w: self }
    }
    #[doc = "Bit 6 - Windowing Enable"]
    #[inline(always)]
    pub fn we(&mut self) -> WE_W {
        WE_W { w: self }
    }
    #[doc = "Bit 7 - Sample Enable"]
    #[inline(always)]
    pub fn se(&mut self) -> SE_W {
        SE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
