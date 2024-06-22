#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    read: Read,
    _reserved1: [u8; 0x0c],
    peek0: Peek0,
    sender0: Sender0,
    status0: Status0,
    config0: Config0,
    write: Write,
    _reserved6: [u8; 0x0c],
    peek1: Peek1,
    sender1: Sender1,
    status1: Status1,
    config1: Config1,
}
impl RegisterBlock {
    #[doc = "0x00 - Read messages from the VideoCore"]
    #[inline(always)]
    pub const fn read(&self) -> &Read {
        &self.read
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn peek0(&self) -> &Peek0 {
        &self.peek0
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn sender0(&self) -> &Sender0 {
        &self.sender0
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn status0(&self) -> &Status0 {
        &self.status0
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn config0(&self) -> &Config0 {
        &self.config0
    }
    #[doc = "0x20 - Write messages to the VideoCore"]
    #[inline(always)]
    pub const fn write(&self) -> &Write {
        &self.write
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn peek1(&self) -> &Peek1 {
        &self.peek1
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn sender1(&self) -> &Sender1 {
        &self.sender1
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn status1(&self) -> &Status1 {
        &self.status1
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn config1(&self) -> &Config1 {
        &self.config1
    }
}
#[doc = "READ (r) register accessor: Read messages from the VideoCore\n\nYou can [`read`](crate::Reg::read) this register and get [`read::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@read`]
module"]
#[doc(alias = "READ")]
pub type Read = crate::Reg<read::ReadSpec>;
#[doc = "Read messages from the VideoCore"]
pub mod read;
#[doc = "PEEK0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`peek0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peek0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek0`]
module"]
#[doc(alias = "PEEK0")]
pub type Peek0 = crate::Reg<peek0::Peek0Spec>;
#[doc = ""]
pub mod peek0;
#[doc = "SENDER0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sender0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sender0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sender0`]
module"]
#[doc(alias = "SENDER0")]
pub type Sender0 = crate::Reg<sender0::Sender0Spec>;
#[doc = ""]
pub mod sender0;
#[doc = "STATUS0 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status0`]
module"]
#[doc(alias = "STATUS0")]
pub type Status0 = crate::Reg<status0::Status0Spec>;
#[doc = ""]
pub mod status0;
#[doc = "CONFIG0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`config0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config0`]
module"]
#[doc(alias = "CONFIG0")]
pub type Config0 = crate::Reg<config0::Config0Spec>;
#[doc = ""]
pub mod config0;
#[doc = "WRITE (w) register accessor: Write messages to the VideoCore\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@write`]
module"]
#[doc(alias = "WRITE")]
pub type Write = crate::Reg<write::WriteSpec>;
#[doc = "Write messages to the VideoCore"]
pub mod write;
#[doc = "PEEK1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`peek1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peek1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek1`]
module"]
#[doc(alias = "PEEK1")]
pub type Peek1 = crate::Reg<peek1::Peek1Spec>;
#[doc = ""]
pub mod peek1;
#[doc = "SENDER1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sender1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sender1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sender1`]
module"]
#[doc(alias = "SENDER1")]
pub type Sender1 = crate::Reg<sender1::Sender1Spec>;
#[doc = ""]
pub mod sender1;
#[doc = "STATUS1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`status1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1`]
module"]
#[doc(alias = "STATUS1")]
pub type Status1 = crate::Reg<status1::Status1Spec>;
#[doc = ""]
pub mod status1;
#[doc = "CONFIG1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`config1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config1`]
module"]
#[doc(alias = "CONFIG1")]
pub type Config1 = crate::Reg<config1::Config1Spec>;
#[doc = ""]
pub mod config1;
