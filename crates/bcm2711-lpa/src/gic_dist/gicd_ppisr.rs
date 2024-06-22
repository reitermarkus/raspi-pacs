#[doc = "Register `GICD_PPISR` reader"]
pub type R = crate::R<GicdPpisrSpec>;
#[doc = "Register `GICD_PPISR` writer"]
pub type W = crate::W<GicdPpisrSpec>;
#[doc = "Field `ID25` reader - Virtual maintenance interrupt"]
pub type Id25R = crate::BitReader;
#[doc = "Field `ID25` writer - Virtual maintenance interrupt"]
pub type Id25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID26` reader - Hypervisor timer event"]
pub type Id26R = crate::BitReader;
#[doc = "Field `ID26` writer - Hypervisor timer event"]
pub type Id26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID27` reader - Virtual timer event"]
pub type Id27R = crate::BitReader;
#[doc = "Field `ID27` writer - Virtual timer event"]
pub type Id27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID28` reader - nLEGACYFIQ signal"]
pub type Id28R = crate::BitReader;
#[doc = "Field `ID28` writer - nLEGACYFIQ signal"]
pub type Id28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID29` reader - Secure physical timer event"]
pub type Id29R = crate::BitReader;
#[doc = "Field `ID29` writer - Secure physical timer event"]
pub type Id29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID30` reader - Non-secure physical timer event"]
pub type Id30R = crate::BitReader;
#[doc = "Field `ID30` writer - Non-secure physical timer event"]
pub type Id30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID31` reader - nLEGACYIRQ signal"]
pub type Id31R = crate::BitReader;
#[doc = "Field `ID31` writer - nLEGACYIRQ signal"]
pub type Id31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - Virtual maintenance interrupt"]
    #[inline(always)]
    pub fn id25(&self) -> Id25R {
        Id25R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hypervisor timer event"]
    #[inline(always)]
    pub fn id26(&self) -> Id26R {
        Id26R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Virtual timer event"]
    #[inline(always)]
    pub fn id27(&self) -> Id27R {
        Id27R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - nLEGACYFIQ signal"]
    #[inline(always)]
    pub fn id28(&self) -> Id28R {
        Id28R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Secure physical timer event"]
    #[inline(always)]
    pub fn id29(&self) -> Id29R {
        Id29R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Non-secure physical timer event"]
    #[inline(always)]
    pub fn id30(&self) -> Id30R {
        Id30R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - nLEGACYIRQ signal"]
    #[inline(always)]
    pub fn id31(&self) -> Id31R {
        Id31R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PPISR")
            .field("id31", &self.id31())
            .field("id30", &self.id30())
            .field("id29", &self.id29())
            .field("id28", &self.id28())
            .field("id27", &self.id27())
            .field("id26", &self.id26())
            .field("id25", &self.id25())
            .finish()
    }
}
impl W {
    #[doc = "Bit 9 - Virtual maintenance interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn id25(&mut self) -> Id25W<GicdPpisrSpec> {
        Id25W::new(self, 9)
    }
    #[doc = "Bit 10 - Hypervisor timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id26(&mut self) -> Id26W<GicdPpisrSpec> {
        Id26W::new(self, 10)
    }
    #[doc = "Bit 11 - Virtual timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id27(&mut self) -> Id27W<GicdPpisrSpec> {
        Id27W::new(self, 11)
    }
    #[doc = "Bit 12 - nLEGACYFIQ signal"]
    #[inline(always)]
    #[must_use]
    pub fn id28(&mut self) -> Id28W<GicdPpisrSpec> {
        Id28W::new(self, 12)
    }
    #[doc = "Bit 13 - Secure physical timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id29(&mut self) -> Id29W<GicdPpisrSpec> {
        Id29W::new(self, 13)
    }
    #[doc = "Bit 14 - Non-secure physical timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id30(&mut self) -> Id30W<GicdPpisrSpec> {
        Id30W::new(self, 14)
    }
    #[doc = "Bit 15 - nLEGACYIRQ signal"]
    #[inline(always)]
    #[must_use]
    pub fn id31(&mut self) -> Id31W<GicdPpisrSpec> {
        Id31W::new(self, 15)
    }
}
#[doc = "Private Peripheral Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ppisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ppisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdPpisrSpec;
impl crate::RegisterSpec for GicdPpisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ppisr::R`](R) reader structure"]
impl crate::Readable for GicdPpisrSpec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ppisr::W`](W) writer structure"]
impl crate::Writable for GicdPpisrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_PPISR to value 0"]
impl crate::Resettable for GicdPpisrSpec {
    const RESET_VALUE: u32 = 0;
}
