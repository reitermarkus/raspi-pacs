#[doc = "Register `WRITE` writer"]
pub type W = crate::W<WriteSpec>;
impl core::fmt::Debug for crate::generic::Reg<WriteSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Write messages to the VideoCore\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WriteSpec;
impl crate::RegisterSpec for WriteSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`write::W`](W) writer structure"]
impl crate::Writable for WriteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
