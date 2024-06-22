#[doc = "Register `GICD_SGIR` writer"]
pub type W = crate::W<GicdSgirSpec>;
impl core::fmt::Debug for crate::generic::Reg<GicdSgirSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Software Generated Interrupt Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_sgir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdSgirSpec;
impl crate::RegisterSpec for GicdSgirSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gicd_sgir::W`](W) writer structure"]
impl crate::Writable for GicdSgirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
