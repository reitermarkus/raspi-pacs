#[doc = "Register `CS` reader"]
pub type R = crate::R<CsSpec>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CsSpec>;
#[doc = "Field `M0` reader - System timer match 0"]
pub type M0R = crate::BitReader;
#[doc = "Field `M0` writer - System timer match 0"]
pub type M0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `M1` reader - System timer match 1"]
pub type M1R = crate::BitReader;
#[doc = "Field `M1` writer - System timer match 1"]
pub type M1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `M2` reader - System timer match 2"]
pub type M2R = crate::BitReader;
#[doc = "Field `M2` writer - System timer match 2"]
pub type M2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `M3` reader - System timer match 3"]
pub type M3R = crate::BitReader;
#[doc = "Field `M3` writer - System timer match 3"]
pub type M3W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - System timer match 0"]
    #[inline(always)]
    pub fn m0(&self) -> M0R {
        M0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System timer match 1"]
    #[inline(always)]
    pub fn m1(&self) -> M1R {
        M1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System timer match 2"]
    #[inline(always)]
    pub fn m2(&self) -> M2R {
        M2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System timer match 3"]
    #[inline(always)]
    pub fn m3(&self) -> M3R {
        M3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS")
            .field("m3", &self.m3())
            .field("m2", &self.m2())
            .field("m1", &self.m1())
            .field("m0", &self.m0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - System timer match 0"]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0W<CsSpec> {
        M0W::new(self, 0)
    }
    #[doc = "Bit 1 - System timer match 1"]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1W<CsSpec> {
        M1W::new(self, 1)
    }
    #[doc = "Bit 2 - System timer match 2"]
    #[inline(always)]
    #[must_use]
    pub fn m2(&mut self) -> M2W<CsSpec> {
        M2W::new(self, 2)
    }
    #[doc = "Bit 3 - System timer match 3"]
    #[inline(always)]
    #[must_use]
    pub fn m3(&mut self) -> M3W<CsSpec> {
        M3W::new(self, 3)
    }
}
#[doc = "Control / Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsSpec;
impl crate::RegisterSpec for CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CsSpec {
    const RESET_VALUE: u32 = 0;
}
