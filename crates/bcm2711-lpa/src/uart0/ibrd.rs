#[doc = "Register `IBRD` reader"]
pub type R = crate::R<IbrdSpec>;
#[doc = "Register `IBRD` writer"]
pub type W = crate::W<IbrdSpec>;
#[doc = "Field `BAUDDIVINT` reader - BAUDDIVINT"]
pub type BauddivintR = crate::FieldReader<u16>;
#[doc = "Field `BAUDDIVINT` writer - BAUDDIVINT"]
pub type BauddivintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BAUDDIVINT"]
    #[inline(always)]
    pub fn bauddivint(&self) -> BauddivintR {
        BauddivintR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBRD")
            .field("bauddivint", &self.bauddivint())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BAUDDIVINT"]
    #[inline(always)]
    #[must_use]
    pub fn bauddivint(&mut self) -> BauddivintW<IbrdSpec> {
        BauddivintW::new(self, 0)
    }
}
#[doc = "Integer Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IbrdSpec;
impl crate::RegisterSpec for IbrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibrd::R`](R) reader structure"]
impl crate::Readable for IbrdSpec {}
#[doc = "`write(|w| ..)` method takes [`ibrd::W`](W) writer structure"]
impl crate::Writable for IbrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IBRD to value 0"]
impl crate::Resettable for IbrdSpec {
    const RESET_VALUE: u32 = 0;
}
