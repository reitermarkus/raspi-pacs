#[doc = "Register `GPEDS0` reader"]
pub struct R(crate::R<GPEDS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPEDS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPEDS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPEDS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPEDS0` writer"]
pub struct W(crate::W<GPEDS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPEDS0_SPEC>;
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
impl From<crate::W<GPEDS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPEDS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDS0` reader - Event detected 0"]
pub type EDS0_R = crate::BitReader<bool>;
#[doc = "Field `EDS0` writer - Event detected 0"]
pub type EDS0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS1` reader - Event detected 1"]
pub type EDS1_R = crate::BitReader<bool>;
#[doc = "Field `EDS1` writer - Event detected 1"]
pub type EDS1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS2` reader - Event detected 2"]
pub type EDS2_R = crate::BitReader<bool>;
#[doc = "Field `EDS2` writer - Event detected 2"]
pub type EDS2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS3` reader - Event detected 3"]
pub type EDS3_R = crate::BitReader<bool>;
#[doc = "Field `EDS3` writer - Event detected 3"]
pub type EDS3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS4` reader - Event detected 4"]
pub type EDS4_R = crate::BitReader<bool>;
#[doc = "Field `EDS4` writer - Event detected 4"]
pub type EDS4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS5` reader - Event detected 5"]
pub type EDS5_R = crate::BitReader<bool>;
#[doc = "Field `EDS5` writer - Event detected 5"]
pub type EDS5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS6` reader - Event detected 6"]
pub type EDS6_R = crate::BitReader<bool>;
#[doc = "Field `EDS6` writer - Event detected 6"]
pub type EDS6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS7` reader - Event detected 7"]
pub type EDS7_R = crate::BitReader<bool>;
#[doc = "Field `EDS7` writer - Event detected 7"]
pub type EDS7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS8` reader - Event detected 8"]
pub type EDS8_R = crate::BitReader<bool>;
#[doc = "Field `EDS8` writer - Event detected 8"]
pub type EDS8_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS9` reader - Event detected 9"]
pub type EDS9_R = crate::BitReader<bool>;
#[doc = "Field `EDS9` writer - Event detected 9"]
pub type EDS9_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS10` reader - Event detected 10"]
pub type EDS10_R = crate::BitReader<bool>;
#[doc = "Field `EDS10` writer - Event detected 10"]
pub type EDS10_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS11` reader - Event detected 11"]
pub type EDS11_R = crate::BitReader<bool>;
#[doc = "Field `EDS11` writer - Event detected 11"]
pub type EDS11_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS12` reader - Event detected 12"]
pub type EDS12_R = crate::BitReader<bool>;
#[doc = "Field `EDS12` writer - Event detected 12"]
pub type EDS12_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS13` reader - Event detected 13"]
pub type EDS13_R = crate::BitReader<bool>;
#[doc = "Field `EDS13` writer - Event detected 13"]
pub type EDS13_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS14` reader - Event detected 14"]
pub type EDS14_R = crate::BitReader<bool>;
#[doc = "Field `EDS14` writer - Event detected 14"]
pub type EDS14_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS15` reader - Event detected 15"]
pub type EDS15_R = crate::BitReader<bool>;
#[doc = "Field `EDS15` writer - Event detected 15"]
pub type EDS15_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS16` reader - Event detected 16"]
pub type EDS16_R = crate::BitReader<bool>;
#[doc = "Field `EDS16` writer - Event detected 16"]
pub type EDS16_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS17` reader - Event detected 17"]
pub type EDS17_R = crate::BitReader<bool>;
#[doc = "Field `EDS17` writer - Event detected 17"]
pub type EDS17_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS18` reader - Event detected 18"]
pub type EDS18_R = crate::BitReader<bool>;
#[doc = "Field `EDS18` writer - Event detected 18"]
pub type EDS18_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS19` reader - Event detected 19"]
pub type EDS19_R = crate::BitReader<bool>;
#[doc = "Field `EDS19` writer - Event detected 19"]
pub type EDS19_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS20` reader - Event detected 20"]
pub type EDS20_R = crate::BitReader<bool>;
#[doc = "Field `EDS20` writer - Event detected 20"]
pub type EDS20_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS21` reader - Event detected 21"]
pub type EDS21_R = crate::BitReader<bool>;
#[doc = "Field `EDS21` writer - Event detected 21"]
pub type EDS21_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS22` reader - Event detected 22"]
pub type EDS22_R = crate::BitReader<bool>;
#[doc = "Field `EDS22` writer - Event detected 22"]
pub type EDS22_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS23` reader - Event detected 23"]
pub type EDS23_R = crate::BitReader<bool>;
#[doc = "Field `EDS23` writer - Event detected 23"]
pub type EDS23_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS24` reader - Event detected 24"]
pub type EDS24_R = crate::BitReader<bool>;
#[doc = "Field `EDS24` writer - Event detected 24"]
pub type EDS24_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS25` reader - Event detected 25"]
pub type EDS25_R = crate::BitReader<bool>;
#[doc = "Field `EDS25` writer - Event detected 25"]
pub type EDS25_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS26` reader - Event detected 26"]
pub type EDS26_R = crate::BitReader<bool>;
#[doc = "Field `EDS26` writer - Event detected 26"]
pub type EDS26_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS27` reader - Event detected 27"]
pub type EDS27_R = crate::BitReader<bool>;
#[doc = "Field `EDS27` writer - Event detected 27"]
pub type EDS27_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS28` reader - Event detected 28"]
pub type EDS28_R = crate::BitReader<bool>;
#[doc = "Field `EDS28` writer - Event detected 28"]
pub type EDS28_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS29` reader - Event detected 29"]
pub type EDS29_R = crate::BitReader<bool>;
#[doc = "Field `EDS29` writer - Event detected 29"]
pub type EDS29_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS30` reader - Event detected 30"]
pub type EDS30_R = crate::BitReader<bool>;
#[doc = "Field `EDS30` writer - Event detected 30"]
pub type EDS30_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
#[doc = "Field `EDS31` reader - Event detected 31"]
pub type EDS31_R = crate::BitReader<bool>;
#[doc = "Field `EDS31` writer - Event detected 31"]
pub type EDS31_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Event detected 0"]
    #[inline(always)]
    pub fn eds0(&self) -> EDS0_R {
        EDS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event detected 1"]
    #[inline(always)]
    pub fn eds1(&self) -> EDS1_R {
        EDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event detected 2"]
    #[inline(always)]
    pub fn eds2(&self) -> EDS2_R {
        EDS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event detected 3"]
    #[inline(always)]
    pub fn eds3(&self) -> EDS3_R {
        EDS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event detected 4"]
    #[inline(always)]
    pub fn eds4(&self) -> EDS4_R {
        EDS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event detected 5"]
    #[inline(always)]
    pub fn eds5(&self) -> EDS5_R {
        EDS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event detected 6"]
    #[inline(always)]
    pub fn eds6(&self) -> EDS6_R {
        EDS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event detected 7"]
    #[inline(always)]
    pub fn eds7(&self) -> EDS7_R {
        EDS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event detected 8"]
    #[inline(always)]
    pub fn eds8(&self) -> EDS8_R {
        EDS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event detected 9"]
    #[inline(always)]
    pub fn eds9(&self) -> EDS9_R {
        EDS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event detected 10"]
    #[inline(always)]
    pub fn eds10(&self) -> EDS10_R {
        EDS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event detected 11"]
    #[inline(always)]
    pub fn eds11(&self) -> EDS11_R {
        EDS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event detected 12"]
    #[inline(always)]
    pub fn eds12(&self) -> EDS12_R {
        EDS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event detected 13"]
    #[inline(always)]
    pub fn eds13(&self) -> EDS13_R {
        EDS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event detected 14"]
    #[inline(always)]
    pub fn eds14(&self) -> EDS14_R {
        EDS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event detected 15"]
    #[inline(always)]
    pub fn eds15(&self) -> EDS15_R {
        EDS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event detected 16"]
    #[inline(always)]
    pub fn eds16(&self) -> EDS16_R {
        EDS16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event detected 17"]
    #[inline(always)]
    pub fn eds17(&self) -> EDS17_R {
        EDS17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event detected 18"]
    #[inline(always)]
    pub fn eds18(&self) -> EDS18_R {
        EDS18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Event detected 19"]
    #[inline(always)]
    pub fn eds19(&self) -> EDS19_R {
        EDS19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Event detected 20"]
    #[inline(always)]
    pub fn eds20(&self) -> EDS20_R {
        EDS20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Event detected 21"]
    #[inline(always)]
    pub fn eds21(&self) -> EDS21_R {
        EDS21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Event detected 22"]
    #[inline(always)]
    pub fn eds22(&self) -> EDS22_R {
        EDS22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Event detected 23"]
    #[inline(always)]
    pub fn eds23(&self) -> EDS23_R {
        EDS23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Event detected 24"]
    #[inline(always)]
    pub fn eds24(&self) -> EDS24_R {
        EDS24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Event detected 25"]
    #[inline(always)]
    pub fn eds25(&self) -> EDS25_R {
        EDS25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Event detected 26"]
    #[inline(always)]
    pub fn eds26(&self) -> EDS26_R {
        EDS26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Event detected 27"]
    #[inline(always)]
    pub fn eds27(&self) -> EDS27_R {
        EDS27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Event detected 28"]
    #[inline(always)]
    pub fn eds28(&self) -> EDS28_R {
        EDS28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Event detected 29"]
    #[inline(always)]
    pub fn eds29(&self) -> EDS29_R {
        EDS29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Event detected 30"]
    #[inline(always)]
    pub fn eds30(&self) -> EDS30_R {
        EDS30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Event detected 31"]
    #[inline(always)]
    pub fn eds31(&self) -> EDS31_R {
        EDS31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event detected 0"]
    #[inline(always)]
    #[must_use]
    pub fn eds0(&mut self) -> EDS0_W<0> {
        EDS0_W::new(self)
    }
    #[doc = "Bit 1 - Event detected 1"]
    #[inline(always)]
    #[must_use]
    pub fn eds1(&mut self) -> EDS1_W<1> {
        EDS1_W::new(self)
    }
    #[doc = "Bit 2 - Event detected 2"]
    #[inline(always)]
    #[must_use]
    pub fn eds2(&mut self) -> EDS2_W<2> {
        EDS2_W::new(self)
    }
    #[doc = "Bit 3 - Event detected 3"]
    #[inline(always)]
    #[must_use]
    pub fn eds3(&mut self) -> EDS3_W<3> {
        EDS3_W::new(self)
    }
    #[doc = "Bit 4 - Event detected 4"]
    #[inline(always)]
    #[must_use]
    pub fn eds4(&mut self) -> EDS4_W<4> {
        EDS4_W::new(self)
    }
    #[doc = "Bit 5 - Event detected 5"]
    #[inline(always)]
    #[must_use]
    pub fn eds5(&mut self) -> EDS5_W<5> {
        EDS5_W::new(self)
    }
    #[doc = "Bit 6 - Event detected 6"]
    #[inline(always)]
    #[must_use]
    pub fn eds6(&mut self) -> EDS6_W<6> {
        EDS6_W::new(self)
    }
    #[doc = "Bit 7 - Event detected 7"]
    #[inline(always)]
    #[must_use]
    pub fn eds7(&mut self) -> EDS7_W<7> {
        EDS7_W::new(self)
    }
    #[doc = "Bit 8 - Event detected 8"]
    #[inline(always)]
    #[must_use]
    pub fn eds8(&mut self) -> EDS8_W<8> {
        EDS8_W::new(self)
    }
    #[doc = "Bit 9 - Event detected 9"]
    #[inline(always)]
    #[must_use]
    pub fn eds9(&mut self) -> EDS9_W<9> {
        EDS9_W::new(self)
    }
    #[doc = "Bit 10 - Event detected 10"]
    #[inline(always)]
    #[must_use]
    pub fn eds10(&mut self) -> EDS10_W<10> {
        EDS10_W::new(self)
    }
    #[doc = "Bit 11 - Event detected 11"]
    #[inline(always)]
    #[must_use]
    pub fn eds11(&mut self) -> EDS11_W<11> {
        EDS11_W::new(self)
    }
    #[doc = "Bit 12 - Event detected 12"]
    #[inline(always)]
    #[must_use]
    pub fn eds12(&mut self) -> EDS12_W<12> {
        EDS12_W::new(self)
    }
    #[doc = "Bit 13 - Event detected 13"]
    #[inline(always)]
    #[must_use]
    pub fn eds13(&mut self) -> EDS13_W<13> {
        EDS13_W::new(self)
    }
    #[doc = "Bit 14 - Event detected 14"]
    #[inline(always)]
    #[must_use]
    pub fn eds14(&mut self) -> EDS14_W<14> {
        EDS14_W::new(self)
    }
    #[doc = "Bit 15 - Event detected 15"]
    #[inline(always)]
    #[must_use]
    pub fn eds15(&mut self) -> EDS15_W<15> {
        EDS15_W::new(self)
    }
    #[doc = "Bit 16 - Event detected 16"]
    #[inline(always)]
    #[must_use]
    pub fn eds16(&mut self) -> EDS16_W<16> {
        EDS16_W::new(self)
    }
    #[doc = "Bit 17 - Event detected 17"]
    #[inline(always)]
    #[must_use]
    pub fn eds17(&mut self) -> EDS17_W<17> {
        EDS17_W::new(self)
    }
    #[doc = "Bit 18 - Event detected 18"]
    #[inline(always)]
    #[must_use]
    pub fn eds18(&mut self) -> EDS18_W<18> {
        EDS18_W::new(self)
    }
    #[doc = "Bit 19 - Event detected 19"]
    #[inline(always)]
    #[must_use]
    pub fn eds19(&mut self) -> EDS19_W<19> {
        EDS19_W::new(self)
    }
    #[doc = "Bit 20 - Event detected 20"]
    #[inline(always)]
    #[must_use]
    pub fn eds20(&mut self) -> EDS20_W<20> {
        EDS20_W::new(self)
    }
    #[doc = "Bit 21 - Event detected 21"]
    #[inline(always)]
    #[must_use]
    pub fn eds21(&mut self) -> EDS21_W<21> {
        EDS21_W::new(self)
    }
    #[doc = "Bit 22 - Event detected 22"]
    #[inline(always)]
    #[must_use]
    pub fn eds22(&mut self) -> EDS22_W<22> {
        EDS22_W::new(self)
    }
    #[doc = "Bit 23 - Event detected 23"]
    #[inline(always)]
    #[must_use]
    pub fn eds23(&mut self) -> EDS23_W<23> {
        EDS23_W::new(self)
    }
    #[doc = "Bit 24 - Event detected 24"]
    #[inline(always)]
    #[must_use]
    pub fn eds24(&mut self) -> EDS24_W<24> {
        EDS24_W::new(self)
    }
    #[doc = "Bit 25 - Event detected 25"]
    #[inline(always)]
    #[must_use]
    pub fn eds25(&mut self) -> EDS25_W<25> {
        EDS25_W::new(self)
    }
    #[doc = "Bit 26 - Event detected 26"]
    #[inline(always)]
    #[must_use]
    pub fn eds26(&mut self) -> EDS26_W<26> {
        EDS26_W::new(self)
    }
    #[doc = "Bit 27 - Event detected 27"]
    #[inline(always)]
    #[must_use]
    pub fn eds27(&mut self) -> EDS27_W<27> {
        EDS27_W::new(self)
    }
    #[doc = "Bit 28 - Event detected 28"]
    #[inline(always)]
    #[must_use]
    pub fn eds28(&mut self) -> EDS28_W<28> {
        EDS28_W::new(self)
    }
    #[doc = "Bit 29 - Event detected 29"]
    #[inline(always)]
    #[must_use]
    pub fn eds29(&mut self) -> EDS29_W<29> {
        EDS29_W::new(self)
    }
    #[doc = "Bit 30 - Event detected 30"]
    #[inline(always)]
    #[must_use]
    pub fn eds30(&mut self) -> EDS30_W<30> {
        EDS30_W::new(self)
    }
    #[doc = "Bit 31 - Event detected 31"]
    #[inline(always)]
    #[must_use]
    pub fn eds31(&mut self) -> EDS31_W<31> {
        EDS31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Event Detect Status 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpeds0](index.html) module"]
pub struct GPEDS0_SPEC;
impl crate::RegisterSpec for GPEDS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpeds0::R](R) reader structure"]
impl crate::Readable for GPEDS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpeds0::W](W) writer structure"]
impl crate::Writable for GPEDS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
