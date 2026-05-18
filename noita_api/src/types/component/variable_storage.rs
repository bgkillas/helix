use crate::{
    Component, ComponentBufferVTable, ComponentTrait, ComponentVTable, StdBox, StdPtr, StdString,
};
use noita_api_macros::assert_size_com;
use std::ffi::CStr;
impl ComponentTrait for VariableStorage {
    const NAME: &'static CStr = c"VariableStorageComponent";
    #[inline]
    fn vtable() -> StdBox<ComponentVTable<Self>> {
        StdBox::from(StdPtr::new(0x0104_6b70))
    }
    #[inline]
    fn buffer_vtable() -> StdBox<ComponentBufferVTable<Self>> {
        StdBox::from(StdPtr::new(0x0103_b610))
    }
    #[inline]
    fn free(&mut self) {
        self.name.free();
        self.value_string.free();
    }
}
pub type VariableStorageComponent = Component<VariableStorage>;
#[derive(Debug, Default)]
#[repr(C)]
#[assert_size_com(0x88)]
pub struct VariableStorage {
    pub name: StdString,
    pub value_string: StdString,
    pub value_isize: isize,
    pub value_bool: bool,
    field28_0x7d: u8,
    field29_0x7e: u8,
    field30_0x7f: u8,
    pub value_f32: f32,
    field32_0x84: u8,
    field33_0x85: u8,
    field34_0x86: u8,
    field35_0x87: u8,
}
