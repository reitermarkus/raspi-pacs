#[doc = "Register `GPFSEL5` reader"]
pub type R = crate::R<Gpfsel5Spec>;
#[doc = "Register `GPFSEL5` writer"]
pub type W = crate::W<Gpfsel5Spec>;
#[doc = "Function Select 50"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel50 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    Reserved1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT0"]
    Sd1Dat0 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel50> for u8 {
    #[inline(always)]
    fn from(variant: Fsel50) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel50 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel50 {}
#[doc = "Field `FSEL50` reader - Function Select 50"]
pub type Fsel50R = crate::FieldReader<Fsel50>;
impl Fsel50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel50 {
        match self.bits {
            0 => Fsel50::Input,
            1 => Fsel50::Output,
            4 => Fsel50::Reserved0,
            5 => Fsel50::Reserved1,
            6 => Fsel50::Reserved2,
            7 => Fsel50::Sd1Dat0,
            3 => Fsel50::Reserved4,
            2 => Fsel50::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel50::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel50::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel50::Reserved0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fsel50::Reserved1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel50::Reserved2
    }
    #[doc = "Pin is connected to SD1_DAT0"]
    #[inline(always)]
    pub fn is_sd1_dat0(&self) -> bool {
        *self == Fsel50::Sd1Dat0
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel50::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel50::Reserved5
    }
}
#[doc = "Field `FSEL50` writer - Function Select 50"]
pub type Fsel50W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel50, crate::Safe>;
impl<'a, REG> Fsel50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel50::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel50::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel50::Reserved0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel50::Reserved1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel50::Reserved2)
    }
    #[doc = "Pin is connected to SD1_DAT0"]
    #[inline(always)]
    pub fn sd1_dat0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel50::Sd1Dat0)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel50::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel50::Reserved5)
    }
}
#[doc = "Function Select 51"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel51 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    Reserved1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT1"]
    Sd1Dat1 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel51> for u8 {
    #[inline(always)]
    fn from(variant: Fsel51) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel51 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel51 {}
#[doc = "Field `FSEL51` reader - Function Select 51"]
pub type Fsel51R = crate::FieldReader<Fsel51>;
impl Fsel51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel51 {
        match self.bits {
            0 => Fsel51::Input,
            1 => Fsel51::Output,
            4 => Fsel51::Reserved0,
            5 => Fsel51::Reserved1,
            6 => Fsel51::Reserved2,
            7 => Fsel51::Sd1Dat1,
            3 => Fsel51::Reserved4,
            2 => Fsel51::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel51::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel51::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel51::Reserved0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fsel51::Reserved1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel51::Reserved2
    }
    #[doc = "Pin is connected to SD1_DAT1"]
    #[inline(always)]
    pub fn is_sd1_dat1(&self) -> bool {
        *self == Fsel51::Sd1Dat1
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel51::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel51::Reserved5
    }
}
#[doc = "Field `FSEL51` writer - Function Select 51"]
pub type Fsel51W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel51, crate::Safe>;
impl<'a, REG> Fsel51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel51::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel51::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel51::Reserved0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel51::Reserved1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel51::Reserved2)
    }
    #[doc = "Pin is connected to SD1_DAT1"]
    #[inline(always)]
    pub fn sd1_dat1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel51::Sd1Dat1)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel51::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel51::Reserved5)
    }
}
#[doc = "Function Select 52"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel52 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    Reserved1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT2"]
    Sd1Dat2 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel52> for u8 {
    #[inline(always)]
    fn from(variant: Fsel52) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel52 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel52 {}
#[doc = "Field `FSEL52` reader - Function Select 52"]
pub type Fsel52R = crate::FieldReader<Fsel52>;
impl Fsel52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel52 {
        match self.bits {
            0 => Fsel52::Input,
            1 => Fsel52::Output,
            4 => Fsel52::Reserved0,
            5 => Fsel52::Reserved1,
            6 => Fsel52::Reserved2,
            7 => Fsel52::Sd1Dat2,
            3 => Fsel52::Reserved4,
            2 => Fsel52::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel52::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel52::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel52::Reserved0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fsel52::Reserved1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel52::Reserved2
    }
    #[doc = "Pin is connected to SD1_DAT2"]
    #[inline(always)]
    pub fn is_sd1_dat2(&self) -> bool {
        *self == Fsel52::Sd1Dat2
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel52::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel52::Reserved5
    }
}
#[doc = "Field `FSEL52` writer - Function Select 52"]
pub type Fsel52W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel52, crate::Safe>;
impl<'a, REG> Fsel52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel52::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel52::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel52::Reserved0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel52::Reserved1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel52::Reserved2)
    }
    #[doc = "Pin is connected to SD1_DAT2"]
    #[inline(always)]
    pub fn sd1_dat2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel52::Sd1Dat2)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel52::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel52::Reserved5)
    }
}
#[doc = "Function Select 53"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel53 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    Reserved1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT3"]
    Sd1Dat3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel53> for u8 {
    #[inline(always)]
    fn from(variant: Fsel53) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel53 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel53 {}
#[doc = "Field `FSEL53` reader - Function Select 53"]
pub type Fsel53R = crate::FieldReader<Fsel53>;
impl Fsel53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel53 {
        match self.bits {
            0 => Fsel53::Input,
            1 => Fsel53::Output,
            4 => Fsel53::Reserved0,
            5 => Fsel53::Reserved1,
            6 => Fsel53::Reserved2,
            7 => Fsel53::Sd1Dat3,
            3 => Fsel53::Reserved4,
            2 => Fsel53::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel53::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel53::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel53::Reserved0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fsel53::Reserved1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel53::Reserved2
    }
    #[doc = "Pin is connected to SD1_DAT3"]
    #[inline(always)]
    pub fn is_sd1_dat3(&self) -> bool {
        *self == Fsel53::Sd1Dat3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel53::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel53::Reserved5
    }
}
#[doc = "Field `FSEL53` writer - Function Select 53"]
pub type Fsel53W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel53, crate::Safe>;
impl<'a, REG> Fsel53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel53::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel53::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel53::Reserved0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel53::Reserved1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel53::Reserved2)
    }
    #[doc = "Pin is connected to SD1_DAT3"]
    #[inline(always)]
    pub fn sd1_dat3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel53::Sd1Dat3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel53::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel53::Reserved5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 50"]
    #[inline(always)]
    pub fn fsel50(&self) -> Fsel50R {
        Fsel50R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 51"]
    #[inline(always)]
    pub fn fsel51(&self) -> Fsel51R {
        Fsel51R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 52"]
    #[inline(always)]
    pub fn fsel52(&self) -> Fsel52R {
        Fsel52R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 53"]
    #[inline(always)]
    pub fn fsel53(&self) -> Fsel53R {
        Fsel53R::new(((self.bits >> 9) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPFSEL5")
            .field("fsel50", &self.fsel50())
            .field("fsel51", &self.fsel51())
            .field("fsel52", &self.fsel52())
            .field("fsel53", &self.fsel53())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 50"]
    #[inline(always)]
    #[must_use]
    pub fn fsel50(&mut self) -> Fsel50W<Gpfsel5Spec> {
        Fsel50W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Select 51"]
    #[inline(always)]
    #[must_use]
    pub fn fsel51(&mut self) -> Fsel51W<Gpfsel5Spec> {
        Fsel51W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Select 52"]
    #[inline(always)]
    #[must_use]
    pub fn fsel52(&mut self) -> Fsel52W<Gpfsel5Spec> {
        Fsel52W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Select 53"]
    #[inline(always)]
    #[must_use]
    pub fn fsel53(&mut self) -> Fsel53W<Gpfsel5Spec> {
        Fsel53W::new(self, 9)
    }
}
#[doc = "GPIO Function Select 5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpfsel5::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpfsel5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpfsel5Spec;
impl crate::RegisterSpec for Gpfsel5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpfsel5::R`](R) reader structure"]
impl crate::Readable for Gpfsel5Spec {}
#[doc = "`write(|w| ..)` method takes [`gpfsel5::W`](W) writer structure"]
impl crate::Writable for Gpfsel5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
