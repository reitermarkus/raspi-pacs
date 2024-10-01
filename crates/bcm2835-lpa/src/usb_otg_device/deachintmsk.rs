#[doc = "Register `DEACHINTMSK` reader"]
pub type R = crate::R<DeachintmskSpec>;
#[doc = "Register `DEACHINTMSK` writer"]
pub type W = crate::W<DeachintmskSpec>;
#[doc = "Field `IEP1INTM` reader - IN Endpoint 1 interrupt mask bit"]
pub type Iep1intmR = crate::BitReader;
#[doc = "Field `IEP1INTM` writer - IN Endpoint 1 interrupt mask bit"]
pub type Iep1intmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEP1INTM` reader - OUT Endpoint 1 interrupt mask bit"]
pub type Oep1intmR = crate::BitReader;
#[doc = "Field `OEP1INTM` writer - OUT Endpoint 1 interrupt mask bit"]
pub type Oep1intmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn iep1intm(&self) -> Iep1intmR {
        Iep1intmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn oep1intm(&self) -> Oep1intmR {
        Oep1intmR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEACHINTMSK")
            .field("iep1intm", &self.iep1intm())
            .field("oep1intm", &self.oep1intm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn iep1intm(&mut self) -> Iep1intmW<DeachintmskSpec> {
        Iep1intmW::new(self, 1)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn oep1intm(&mut self) -> Oep1intmW<DeachintmskSpec> {
        Oep1intmW::new(self, 17)
    }
}
#[doc = "OTG_HS device each endpoint interrupt register mask\n\nYou can [`read`](crate::Reg::read) this register and get [`deachintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deachintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeachintmskSpec;
impl crate::RegisterSpec for DeachintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deachintmsk::R`](R) reader structure"]
impl crate::Readable for DeachintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`deachintmsk::W`](W) writer structure"]
impl crate::Writable for DeachintmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEACHINTMSK to value 0"]
impl crate::Resettable for DeachintmskSpec {
    const RESET_VALUE: u32 = 0;
}
