#[doc = "Register `GICC_IIDR` reader"]
pub type R = crate::R<GiccIidrSpec>;
#[doc = "Register `GICC_IIDR` writer"]
pub type W = crate::W<GiccIidrSpec>;
#[doc = "ID\n\nValue on reset: 33690683"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Id {
    #[doc = "33690683: ID is valid"]
    Valid = 33690683,
    #[doc = "0: ID is *NOT* valid"]
    Invalid = 0,
}
impl From<Id> for u32 {
    #[inline(always)]
    fn from(variant: Id) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Id {
    type Ux = u32;
}
impl crate::IsEnum for Id {}
#[doc = "Field `ID` reader - ID"]
pub type IdR = crate::FieldReader<Id>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id {
        match self.bits {
            33690683 => Id::Valid,
            _ => Id::Invalid,
        }
    }
    #[doc = "ID is valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Id::Valid
    }
    #[doc = "ID is *NOT* valid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), Id::Invalid)
    }
}
#[doc = "Field `ID` writer - ID"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 32, Id, crate::Safe>;
impl<'a, REG> IdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "ID is valid"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(Id::Valid)
    }
    #[doc = "ID is *NOT* valid"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut crate::W<REG> {
        self.variant(Id::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_IIDR").field("id", &self.id()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<GiccIidrSpec> {
        IdW::new(self, 0)
    }
}
#[doc = "CPU Interface Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_iidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_iidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiccIidrSpec;
impl crate::RegisterSpec for GiccIidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_iidr::R`](R) reader structure"]
impl crate::Readable for GiccIidrSpec {}
#[doc = "`write(|w| ..)` method takes [`gicc_iidr::W`](W) writer structure"]
impl crate::Writable for GiccIidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_IIDR to value 0x0202_143b"]
impl crate::Resettable for GiccIidrSpec {
    const RESET_VALUE: u32 = 0x0202_143b;
}
