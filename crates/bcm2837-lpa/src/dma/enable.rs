#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Field `EN0` reader - Enable DMA engine 0"]
pub type En0R = crate::BitReader;
#[doc = "Field `EN0` writer - Enable DMA engine 0"]
pub type En0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN1` reader - Enable DMA engine 1"]
pub type En1R = crate::BitReader;
#[doc = "Field `EN1` writer - Enable DMA engine 1"]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN2` reader - Enable DMA engine 2"]
pub type En2R = crate::BitReader;
#[doc = "Field `EN2` writer - Enable DMA engine 2"]
pub type En2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN3` reader - Enable DMA engine 3"]
pub type En3R = crate::BitReader;
#[doc = "Field `EN3` writer - Enable DMA engine 3"]
pub type En3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN4` reader - Enable DMA engine 4"]
pub type En4R = crate::BitReader;
#[doc = "Field `EN4` writer - Enable DMA engine 4"]
pub type En4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN5` reader - Enable DMA engine 5"]
pub type En5R = crate::BitReader;
#[doc = "Field `EN5` writer - Enable DMA engine 5"]
pub type En5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN6` reader - Enable DMA engine 6"]
pub type En6R = crate::BitReader;
#[doc = "Field `EN6` writer - Enable DMA engine 6"]
pub type En6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN7` reader - Enable DMA engine 7"]
pub type En7R = crate::BitReader;
#[doc = "Field `EN7` writer - Enable DMA engine 7"]
pub type En7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN8` reader - Enable DMA engine 8"]
pub type En8R = crate::BitReader;
#[doc = "Field `EN8` writer - Enable DMA engine 8"]
pub type En8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN9` reader - Enable DMA engine 9"]
pub type En9R = crate::BitReader;
#[doc = "Field `EN9` writer - Enable DMA engine 9"]
pub type En9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN10` reader - Enable DMA engine 10"]
pub type En10R = crate::BitReader;
#[doc = "Field `EN10` writer - Enable DMA engine 10"]
pub type En10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN11` reader - Enable DMA engine 11"]
pub type En11R = crate::BitReader;
#[doc = "Field `EN11` writer - Enable DMA engine 11"]
pub type En11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN12` reader - Enable DMA engine 12"]
pub type En12R = crate::BitReader;
#[doc = "Field `EN12` writer - Enable DMA engine 12"]
pub type En12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN13` reader - Enable DMA engine 13"]
pub type En13R = crate::BitReader;
#[doc = "Field `EN13` writer - Enable DMA engine 13"]
pub type En13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN14` reader - Enable DMA engine 14"]
pub type En14R = crate::BitReader;
#[doc = "Field `EN14` writer - Enable DMA engine 14"]
pub type En14W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable DMA engine 0"]
    #[inline(always)]
    pub fn en0(&self) -> En0R {
        En0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable DMA engine 1"]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable DMA engine 2"]
    #[inline(always)]
    pub fn en2(&self) -> En2R {
        En2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable DMA engine 3"]
    #[inline(always)]
    pub fn en3(&self) -> En3R {
        En3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable DMA engine 4"]
    #[inline(always)]
    pub fn en4(&self) -> En4R {
        En4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable DMA engine 5"]
    #[inline(always)]
    pub fn en5(&self) -> En5R {
        En5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable DMA engine 6"]
    #[inline(always)]
    pub fn en6(&self) -> En6R {
        En6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable DMA engine 7"]
    #[inline(always)]
    pub fn en7(&self) -> En7R {
        En7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable DMA engine 8"]
    #[inline(always)]
    pub fn en8(&self) -> En8R {
        En8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable DMA engine 9"]
    #[inline(always)]
    pub fn en9(&self) -> En9R {
        En9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable DMA engine 10"]
    #[inline(always)]
    pub fn en10(&self) -> En10R {
        En10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable DMA engine 11"]
    #[inline(always)]
    pub fn en11(&self) -> En11R {
        En11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable DMA engine 12"]
    #[inline(always)]
    pub fn en12(&self) -> En12R {
        En12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable DMA engine 13"]
    #[inline(always)]
    pub fn en13(&self) -> En13R {
        En13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable DMA engine 14"]
    #[inline(always)]
    pub fn en14(&self) -> En14R {
        En14R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE")
            .field("en14", &self.en14())
            .field("en13", &self.en13())
            .field("en12", &self.en12())
            .field("en11", &self.en11())
            .field("en10", &self.en10())
            .field("en9", &self.en9())
            .field("en8", &self.en8())
            .field("en7", &self.en7())
            .field("en6", &self.en6())
            .field("en5", &self.en5())
            .field("en4", &self.en4())
            .field("en3", &self.en3())
            .field("en2", &self.en2())
            .field("en1", &self.en1())
            .field("en0", &self.en0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA engine 0"]
    #[inline(always)]
    #[must_use]
    pub fn en0(&mut self) -> En0W<EnableSpec> {
        En0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable DMA engine 1"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> En1W<EnableSpec> {
        En1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable DMA engine 2"]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> En2W<EnableSpec> {
        En2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable DMA engine 3"]
    #[inline(always)]
    #[must_use]
    pub fn en3(&mut self) -> En3W<EnableSpec> {
        En3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable DMA engine 4"]
    #[inline(always)]
    #[must_use]
    pub fn en4(&mut self) -> En4W<EnableSpec> {
        En4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable DMA engine 5"]
    #[inline(always)]
    #[must_use]
    pub fn en5(&mut self) -> En5W<EnableSpec> {
        En5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable DMA engine 6"]
    #[inline(always)]
    #[must_use]
    pub fn en6(&mut self) -> En6W<EnableSpec> {
        En6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable DMA engine 7"]
    #[inline(always)]
    #[must_use]
    pub fn en7(&mut self) -> En7W<EnableSpec> {
        En7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable DMA engine 8"]
    #[inline(always)]
    #[must_use]
    pub fn en8(&mut self) -> En8W<EnableSpec> {
        En8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable DMA engine 9"]
    #[inline(always)]
    #[must_use]
    pub fn en9(&mut self) -> En9W<EnableSpec> {
        En9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable DMA engine 10"]
    #[inline(always)]
    #[must_use]
    pub fn en10(&mut self) -> En10W<EnableSpec> {
        En10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable DMA engine 11"]
    #[inline(always)]
    #[must_use]
    pub fn en11(&mut self) -> En11W<EnableSpec> {
        En11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable DMA engine 12"]
    #[inline(always)]
    #[must_use]
    pub fn en12(&mut self) -> En12W<EnableSpec> {
        En12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable DMA engine 13"]
    #[inline(always)]
    #[must_use]
    pub fn en13(&mut self) -> En13W<EnableSpec> {
        En13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable DMA engine 14"]
    #[inline(always)]
    #[must_use]
    pub fn en14(&mut self) -> En14W<EnableSpec> {
        En14W::new(self, 14)
    }
}
#[doc = "Global enable bits for each DMA channel\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for EnableSpec {
    const RESET_VALUE: u32 = 0;
}
