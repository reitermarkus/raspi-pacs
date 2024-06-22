#[doc = "Register `BLKSIZECNT` reader"]
pub type R = crate::R<BlksizecntSpec>;
#[doc = "Register `BLKSIZECNT` writer"]
pub type W = crate::W<BlksizecntSpec>;
#[doc = "Field `BLKSIZE` reader - Block size in bytes"]
pub type BlksizeR = crate::FieldReader<u16>;
#[doc = "Field `BLKSIZE` writer - Block size in bytes"]
pub type BlksizeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BLKCNT` reader - Number of blocks to be transferred"]
pub type BlkcntR = crate::FieldReader<u16>;
#[doc = "Field `BLKCNT` writer - Number of blocks to be transferred"]
pub type BlkcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - Block size in bytes"]
    #[inline(always)]
    pub fn blksize(&self) -> BlksizeR {
        BlksizeR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - Number of blocks to be transferred"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BlkcntR {
        BlkcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLKSIZECNT")
            .field("blkcnt", &self.blkcnt())
            .field("blksize", &self.blksize())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Block size in bytes"]
    #[inline(always)]
    #[must_use]
    pub fn blksize(&mut self) -> BlksizeW<BlksizecntSpec> {
        BlksizeW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Number of blocks to be transferred"]
    #[inline(always)]
    #[must_use]
    pub fn blkcnt(&mut self) -> BlkcntW<BlksizecntSpec> {
        BlkcntW::new(self, 16)
    }
}
#[doc = "Numer and size in bytes for data block to be transferred\n\nYou can [`read`](crate::Reg::read) this register and get [`blksizecnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blksizecnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlksizecntSpec;
impl crate::RegisterSpec for BlksizecntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blksizecnt::R`](R) reader structure"]
impl crate::Readable for BlksizecntSpec {}
#[doc = "`write(|w| ..)` method takes [`blksizecnt::W`](W) writer structure"]
impl crate::Writable for BlksizecntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLKSIZECNT to value 0"]
impl crate::Resettable for BlksizecntSpec {
    const RESET_VALUE: u32 = 0;
}
