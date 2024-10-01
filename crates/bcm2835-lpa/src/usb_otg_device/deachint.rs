#[doc = "Register `DEACHINT` reader"]
pub type R = crate::R<DeachintSpec>;
#[doc = "Register `DEACHINT` writer"]
pub type W = crate::W<DeachintSpec>;
#[doc = "Field `IEP1INT` reader - IN endpoint 1interrupt bit"]
pub type Iep1intR = crate::BitReader;
#[doc = "Field `IEP1INT` writer - IN endpoint 1interrupt bit"]
pub type Iep1intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEP1INT` reader - OUT endpoint 1 interrupt bit"]
pub type Oep1intR = crate::BitReader;
#[doc = "Field `OEP1INT` writer - OUT endpoint 1 interrupt bit"]
pub type Oep1intW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    pub fn iep1int(&self) -> Iep1intR {
        Iep1intR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    pub fn oep1int(&self) -> Oep1intR {
        Oep1intR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEACHINT")
            .field("iep1int", &self.iep1int())
            .field("oep1int", &self.oep1int())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn iep1int(&mut self) -> Iep1intW<DeachintSpec> {
        Iep1intW::new(self, 1)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn oep1int(&mut self) -> Oep1intW<DeachintSpec> {
        Oep1intW::new(self, 17)
    }
}
#[doc = "OTG_HS device each endpoint interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`deachint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deachint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeachintSpec;
impl crate::RegisterSpec for DeachintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deachint::R`](R) reader structure"]
impl crate::Readable for DeachintSpec {}
#[doc = "`write(|w| ..)` method takes [`deachint::W`](W) writer structure"]
impl crate::Writable for DeachintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEACHINT to value 0"]
impl crate::Resettable for DeachintSpec {
    const RESET_VALUE: u32 = 0;
}
