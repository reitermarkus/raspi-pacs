#[doc = "Register `GPFEN1` reader"]
pub type R = crate::R<Gpfen1Spec>;
#[doc = "Register `GPFEN1` writer"]
pub type W = crate::W<Gpfen1Spec>;
#[doc = "Field `FEN32` reader - Falling edge enabled 32"]
pub type Fen32R = crate::BitReader;
#[doc = "Field `FEN32` writer - Falling edge enabled 32"]
pub type Fen32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN33` reader - Falling edge enabled 33"]
pub type Fen33R = crate::BitReader;
#[doc = "Field `FEN33` writer - Falling edge enabled 33"]
pub type Fen33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN34` reader - Falling edge enabled 34"]
pub type Fen34R = crate::BitReader;
#[doc = "Field `FEN34` writer - Falling edge enabled 34"]
pub type Fen34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN35` reader - Falling edge enabled 35"]
pub type Fen35R = crate::BitReader;
#[doc = "Field `FEN35` writer - Falling edge enabled 35"]
pub type Fen35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN36` reader - Falling edge enabled 36"]
pub type Fen36R = crate::BitReader;
#[doc = "Field `FEN36` writer - Falling edge enabled 36"]
pub type Fen36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN37` reader - Falling edge enabled 37"]
pub type Fen37R = crate::BitReader;
#[doc = "Field `FEN37` writer - Falling edge enabled 37"]
pub type Fen37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN38` reader - Falling edge enabled 38"]
pub type Fen38R = crate::BitReader;
#[doc = "Field `FEN38` writer - Falling edge enabled 38"]
pub type Fen38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN39` reader - Falling edge enabled 39"]
pub type Fen39R = crate::BitReader;
#[doc = "Field `FEN39` writer - Falling edge enabled 39"]
pub type Fen39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN40` reader - Falling edge enabled 40"]
pub type Fen40R = crate::BitReader;
#[doc = "Field `FEN40` writer - Falling edge enabled 40"]
pub type Fen40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN41` reader - Falling edge enabled 41"]
pub type Fen41R = crate::BitReader;
#[doc = "Field `FEN41` writer - Falling edge enabled 41"]
pub type Fen41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN42` reader - Falling edge enabled 42"]
pub type Fen42R = crate::BitReader;
#[doc = "Field `FEN42` writer - Falling edge enabled 42"]
pub type Fen42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN43` reader - Falling edge enabled 43"]
pub type Fen43R = crate::BitReader;
#[doc = "Field `FEN43` writer - Falling edge enabled 43"]
pub type Fen43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN44` reader - Falling edge enabled 44"]
pub type Fen44R = crate::BitReader;
#[doc = "Field `FEN44` writer - Falling edge enabled 44"]
pub type Fen44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN45` reader - Falling edge enabled 45"]
pub type Fen45R = crate::BitReader;
#[doc = "Field `FEN45` writer - Falling edge enabled 45"]
pub type Fen45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN46` reader - Falling edge enabled 46"]
pub type Fen46R = crate::BitReader;
#[doc = "Field `FEN46` writer - Falling edge enabled 46"]
pub type Fen46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN47` reader - Falling edge enabled 47"]
pub type Fen47R = crate::BitReader;
#[doc = "Field `FEN47` writer - Falling edge enabled 47"]
pub type Fen47W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN48` reader - Falling edge enabled 48"]
pub type Fen48R = crate::BitReader;
#[doc = "Field `FEN48` writer - Falling edge enabled 48"]
pub type Fen48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN49` reader - Falling edge enabled 49"]
pub type Fen49R = crate::BitReader;
#[doc = "Field `FEN49` writer - Falling edge enabled 49"]
pub type Fen49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN50` reader - Falling edge enabled 50"]
pub type Fen50R = crate::BitReader;
#[doc = "Field `FEN50` writer - Falling edge enabled 50"]
pub type Fen50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN51` reader - Falling edge enabled 51"]
pub type Fen51R = crate::BitReader;
#[doc = "Field `FEN51` writer - Falling edge enabled 51"]
pub type Fen51W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN52` reader - Falling edge enabled 52"]
pub type Fen52R = crate::BitReader;
#[doc = "Field `FEN52` writer - Falling edge enabled 52"]
pub type Fen52W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN53` reader - Falling edge enabled 53"]
pub type Fen53R = crate::BitReader;
#[doc = "Field `FEN53` writer - Falling edge enabled 53"]
pub type Fen53W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Falling edge enabled 32"]
    #[inline(always)]
    pub fn fen32(&self) -> Fen32R {
        Fen32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge enabled 33"]
    #[inline(always)]
    pub fn fen33(&self) -> Fen33R {
        Fen33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge enabled 34"]
    #[inline(always)]
    pub fn fen34(&self) -> Fen34R {
        Fen34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling edge enabled 35"]
    #[inline(always)]
    pub fn fen35(&self) -> Fen35R {
        Fen35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling edge enabled 36"]
    #[inline(always)]
    pub fn fen36(&self) -> Fen36R {
        Fen36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling edge enabled 37"]
    #[inline(always)]
    pub fn fen37(&self) -> Fen37R {
        Fen37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling edge enabled 38"]
    #[inline(always)]
    pub fn fen38(&self) -> Fen38R {
        Fen38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling edge enabled 39"]
    #[inline(always)]
    pub fn fen39(&self) -> Fen39R {
        Fen39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling edge enabled 40"]
    #[inline(always)]
    pub fn fen40(&self) -> Fen40R {
        Fen40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling edge enabled 41"]
    #[inline(always)]
    pub fn fen41(&self) -> Fen41R {
        Fen41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling edge enabled 42"]
    #[inline(always)]
    pub fn fen42(&self) -> Fen42R {
        Fen42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling edge enabled 43"]
    #[inline(always)]
    pub fn fen43(&self) -> Fen43R {
        Fen43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling edge enabled 44"]
    #[inline(always)]
    pub fn fen44(&self) -> Fen44R {
        Fen44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling edge enabled 45"]
    #[inline(always)]
    pub fn fen45(&self) -> Fen45R {
        Fen45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling edge enabled 46"]
    #[inline(always)]
    pub fn fen46(&self) -> Fen46R {
        Fen46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling edge enabled 47"]
    #[inline(always)]
    pub fn fen47(&self) -> Fen47R {
        Fen47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling edge enabled 48"]
    #[inline(always)]
    pub fn fen48(&self) -> Fen48R {
        Fen48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling edge enabled 49"]
    #[inline(always)]
    pub fn fen49(&self) -> Fen49R {
        Fen49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling edge enabled 50"]
    #[inline(always)]
    pub fn fen50(&self) -> Fen50R {
        Fen50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Falling edge enabled 51"]
    #[inline(always)]
    pub fn fen51(&self) -> Fen51R {
        Fen51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Falling edge enabled 52"]
    #[inline(always)]
    pub fn fen52(&self) -> Fen52R {
        Fen52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling edge enabled 53"]
    #[inline(always)]
    pub fn fen53(&self) -> Fen53R {
        Fen53R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPFEN1")
            .field("fen32", &self.fen32())
            .field("fen33", &self.fen33())
            .field("fen34", &self.fen34())
            .field("fen35", &self.fen35())
            .field("fen36", &self.fen36())
            .field("fen37", &self.fen37())
            .field("fen38", &self.fen38())
            .field("fen39", &self.fen39())
            .field("fen40", &self.fen40())
            .field("fen41", &self.fen41())
            .field("fen42", &self.fen42())
            .field("fen43", &self.fen43())
            .field("fen44", &self.fen44())
            .field("fen45", &self.fen45())
            .field("fen46", &self.fen46())
            .field("fen47", &self.fen47())
            .field("fen48", &self.fen48())
            .field("fen49", &self.fen49())
            .field("fen50", &self.fen50())
            .field("fen51", &self.fen51())
            .field("fen52", &self.fen52())
            .field("fen53", &self.fen53())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Falling edge enabled 32"]
    #[inline(always)]
    #[must_use]
    pub fn fen32(&mut self) -> Fen32W<Gpfen1Spec> {
        Fen32W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling edge enabled 33"]
    #[inline(always)]
    #[must_use]
    pub fn fen33(&mut self) -> Fen33W<Gpfen1Spec> {
        Fen33W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling edge enabled 34"]
    #[inline(always)]
    #[must_use]
    pub fn fen34(&mut self) -> Fen34W<Gpfen1Spec> {
        Fen34W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling edge enabled 35"]
    #[inline(always)]
    #[must_use]
    pub fn fen35(&mut self) -> Fen35W<Gpfen1Spec> {
        Fen35W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling edge enabled 36"]
    #[inline(always)]
    #[must_use]
    pub fn fen36(&mut self) -> Fen36W<Gpfen1Spec> {
        Fen36W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling edge enabled 37"]
    #[inline(always)]
    #[must_use]
    pub fn fen37(&mut self) -> Fen37W<Gpfen1Spec> {
        Fen37W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling edge enabled 38"]
    #[inline(always)]
    #[must_use]
    pub fn fen38(&mut self) -> Fen38W<Gpfen1Spec> {
        Fen38W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling edge enabled 39"]
    #[inline(always)]
    #[must_use]
    pub fn fen39(&mut self) -> Fen39W<Gpfen1Spec> {
        Fen39W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling edge enabled 40"]
    #[inline(always)]
    #[must_use]
    pub fn fen40(&mut self) -> Fen40W<Gpfen1Spec> {
        Fen40W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling edge enabled 41"]
    #[inline(always)]
    #[must_use]
    pub fn fen41(&mut self) -> Fen41W<Gpfen1Spec> {
        Fen41W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling edge enabled 42"]
    #[inline(always)]
    #[must_use]
    pub fn fen42(&mut self) -> Fen42W<Gpfen1Spec> {
        Fen42W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling edge enabled 43"]
    #[inline(always)]
    #[must_use]
    pub fn fen43(&mut self) -> Fen43W<Gpfen1Spec> {
        Fen43W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling edge enabled 44"]
    #[inline(always)]
    #[must_use]
    pub fn fen44(&mut self) -> Fen44W<Gpfen1Spec> {
        Fen44W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling edge enabled 45"]
    #[inline(always)]
    #[must_use]
    pub fn fen45(&mut self) -> Fen45W<Gpfen1Spec> {
        Fen45W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling edge enabled 46"]
    #[inline(always)]
    #[must_use]
    pub fn fen46(&mut self) -> Fen46W<Gpfen1Spec> {
        Fen46W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling edge enabled 47"]
    #[inline(always)]
    #[must_use]
    pub fn fen47(&mut self) -> Fen47W<Gpfen1Spec> {
        Fen47W::new(self, 15)
    }
    #[doc = "Bit 16 - Falling edge enabled 48"]
    #[inline(always)]
    #[must_use]
    pub fn fen48(&mut self) -> Fen48W<Gpfen1Spec> {
        Fen48W::new(self, 16)
    }
    #[doc = "Bit 17 - Falling edge enabled 49"]
    #[inline(always)]
    #[must_use]
    pub fn fen49(&mut self) -> Fen49W<Gpfen1Spec> {
        Fen49W::new(self, 17)
    }
    #[doc = "Bit 18 - Falling edge enabled 50"]
    #[inline(always)]
    #[must_use]
    pub fn fen50(&mut self) -> Fen50W<Gpfen1Spec> {
        Fen50W::new(self, 18)
    }
    #[doc = "Bit 19 - Falling edge enabled 51"]
    #[inline(always)]
    #[must_use]
    pub fn fen51(&mut self) -> Fen51W<Gpfen1Spec> {
        Fen51W::new(self, 19)
    }
    #[doc = "Bit 20 - Falling edge enabled 52"]
    #[inline(always)]
    #[must_use]
    pub fn fen52(&mut self) -> Fen52W<Gpfen1Spec> {
        Fen52W::new(self, 20)
    }
    #[doc = "Bit 21 - Falling edge enabled 53"]
    #[inline(always)]
    #[must_use]
    pub fn fen53(&mut self) -> Fen53W<Gpfen1Spec> {
        Fen53W::new(self, 21)
    }
}
#[doc = "GPIO Pin Falling Edge Detect Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpfen1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpfen1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpfen1Spec;
impl crate::RegisterSpec for Gpfen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpfen1::R`](R) reader structure"]
impl crate::Readable for Gpfen1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpfen1::W`](W) writer structure"]
impl crate::Writable for Gpfen1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
