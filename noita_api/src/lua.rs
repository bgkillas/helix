pub use crate::lua_bindings::{LUA_GLOBALSINDEX, lua_State};
use crate::lua_bindings::{Lua51, lua_CFunction};
use noita_api_macros::make_lua_get_tuples;
use std::error::Error;
use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::{
    ffi::{CStr, c_char, c_int},
    ptr, slice,
    sync::LazyLock,
};
pub static LUA: LazyLock<Lua51> =
    LazyLock::new(|| unsafe { Lua51::new("lua51.dll").expect("library to be lua") });
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct LuaState {
    lua: *mut lua_State,
}
#[derive(Debug)]
pub enum LuaError {
    NotUft8Str,
    NullStr,
    WrongArrayLength,
    WrongSizedTypeForArray,
}
impl LuaState {
    #[inline]
    pub fn new(lua: *mut lua_State) -> Self {
        Self { lua }
    }
    #[must_use]
    #[inline]
    pub fn to_integer(self, index: i32) -> isize {
        unsafe { (LUA.lua_tointeger)(self.lua, index) }
    }
    #[must_use]
    #[inline]
    pub fn to_number(self, index: i32) -> f64 {
        unsafe { (LUA.lua_tonumber)(self.lua, index) }
    }
    #[must_use]
    #[inline]
    pub fn to_bool(self, index: i32) -> bool {
        unsafe { (LUA.lua_toboolean)(self.lua, index) > 0 }
    }
    #[inline]
    pub fn to_str<'a>(self, index: i32) -> Result<&'a str, LuaError> {
        let str = self.to_raw_str(index)?;
        str::from_utf8(str).map_err(|_| LuaError::NotUft8Str)
    }
    #[inline]
    pub fn to_raw_str<'a>(self, index: i32) -> Result<&'a RawStr, LuaError> {
        let mut size = 0;
        let buf = unsafe { (LUA.lua_tolstring)(self.lua, index, &raw mut size) };
        if buf.is_null() {
            return Err(LuaError::NullStr);
        }
        let slice = unsafe { slice::from_raw_parts(buf.cast::<u8>(), size) };
        Ok(slice.into())
    }
    #[must_use]
    #[inline]
    pub fn to_cfunction(self, index: i32) -> lua_CFunction {
        unsafe { (LUA.lua_tocfunction)(self.lua, index) }
    }
    #[inline]
    pub fn push_number(self, val: f64) {
        unsafe { (LUA.lua_pushnumber)(self.lua, val) };
    }
    #[inline]
    pub fn push_integer(self, val: isize) {
        unsafe { (LUA.lua_pushinteger)(self.lua, val) };
    }
    #[inline]
    pub fn push_bool(self, val: bool) {
        unsafe { (LUA.lua_pushboolean)(self.lua, i32::from(val)) };
    }
    #[inline]
    pub fn push_str(self, s: &str) {
        unsafe {
            (LUA.lua_pushlstring)(self.lua, s.as_bytes().as_ptr().cast::<c_char>(), s.len());
        }
    }
    #[inline]
    pub fn push_raw_str(self, s: &RawStr) {
        unsafe {
            (LUA.lua_pushlstring)(self.lua, s.as_ptr().cast::<c_char>(), s.len());
        }
    }
    #[inline]
    pub fn push_nil(self) {
        unsafe { (LUA.lua_pushnil)(self.lua) }
    }
    #[inline]
    pub fn get_global(self, name: &CStr) {
        unsafe { (LUA.lua_getfield)(self.lua, LUA_GLOBALSINDEX, name.as_ptr()) };
    }
    #[must_use]
    #[inline]
    pub fn objlen(self, index: i32) -> usize {
        unsafe { (LUA.lua_objlen)(self.lua, index) }
    }
    #[inline]
    pub fn index_table(self, table_index: i32, index_in_table: usize) {
        self.push_integer(index_in_table.cast_signed());
        if table_index < 0 {
            unsafe { (LUA.lua_gettable)(self.lua, table_index - 1) };
        } else {
            unsafe { (LUA.lua_gettable)(self.lua, table_index) };
        }
    }
    #[inline]
    pub fn pop_last(self) {
        unsafe { (LUA.lua_settop)(self.lua, -2) };
    }
    /// Raise an error with message `s`
    ///
    /// This takes String so that it gets deallocated properly, as this functions doesn't return.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn raise_error(self, s: String) -> ! {
        self.push_str(&s);
        drop(s);
        unsafe { (LUA.lua_error)(self.lua) };
        // lua_error does not return.
        unreachable!()
    }
    #[must_use]
    #[inline]
    pub fn is_nil_or_none(self, index: i32) -> bool {
        (unsafe { (LUA.lua_type)(self.lua, index) }) <= 0
    }
    #[inline]
    pub fn create_table(self, narr: c_int, nrec: c_int) {
        unsafe { (LUA.lua_createtable)(self.lua, narr, nrec) };
    }
    #[inline]
    pub fn rawset_table(self, table_index: i32, index_in_table: i32) {
        unsafe { (LUA.lua_rawseti)(self.lua, table_index, index_in_table) };
    }
}

#[repr(transparent)]
pub struct RawStr([u8]);
impl From<&[u8]> for &RawStr {
    #[inline]
    fn from(value: &[u8]) -> Self {
        #[allow(clippy::as_conversions)]
        unsafe {
            (ptr::from_ref(value) as *const RawStr).as_ref().unwrap()
        }
    }
}
impl Deref for RawStr {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RawStr {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Used for types that can be returned from functions that were defined in rust to lua.
pub trait LuaFnRet {
    fn do_return(self, lua: LuaState) -> c_int;
}

impl LuaFnRet for () {
    #[inline]
    fn do_return(self, _lua: LuaState) -> c_int {
        0
    }
}

impl LuaFnRet for bool {
    #[inline]
    fn do_return(self, lua: LuaState) -> c_int {
        lua.push_bool(self);
        1
    }
}
impl LuaFnRet for isize {
    #[inline]
    fn do_return(self, lua: LuaState) -> c_int {
        lua.push_integer(self);
        1
    }
}

impl<R: LuaFnRet, E: Error> LuaFnRet for Result<R, E> {
    #[inline]
    fn do_return(self, lua: LuaState) -> c_int {
        match self {
            Ok(ok) => ok.do_return(lua),
            Err(err) => unsafe {
                lua.raise_error(format!("Error in rust call: {err}"));
            },
        }
    }
}

impl<T: LuaFnRet> LuaFnRet for Option<T> {
    #[inline]
    fn do_return(self, lua: LuaState) -> c_int {
        if let Some(val) = self {
            val.do_return(lua)
        } else {
            lua.push_nil();
            1
        }
    }
}

impl<T: LuaFnRet> LuaFnRet for Vec<T> {
    #[inline]
    fn do_return(self, lua: LuaState) -> c_int {
        lua.create_table(c_int::try_from(self.len()).unwrap(), 0);
        for (i, el) in self.into_iter().enumerate() {
            let elements = el.do_return(lua);
            assert_eq!(elements, 1, "Vec<T>'s T should only put one value on stack");
            lua.rawset_table(-2, i32::try_from(i + 1).unwrap());
        }
        1
    }
}

impl<T: LuaFnRet, const N: usize> LuaFnRet for [T; N] {
    #[inline]
    fn do_return(self, lua: LuaState) -> c_int {
        lua.create_table(c_int::try_from(self.len()).unwrap(), 0);
        for (i, el) in self.into_iter().enumerate() {
            let elements = el.do_return(lua);
            assert_eq!(elements, 1, "[T; N]'s T should only put one value on stack");
            lua.rawset_table(-2, i32::try_from(i + 1).unwrap());
        }
        1
    }
}

impl LuaFnRet for &RawStr {
    #[inline]
    fn do_return(self, lua: LuaState) -> c_int {
        lua.push_raw_str(self);
        1
    }
}

/// Trait for arguments that can be retrieved from the lua stack.
pub trait LuaGetValue: Sized {
    fn get(lua: LuaState, index: i32) -> Result<(i32, Self), LuaError>;
}

impl LuaGetValue for isize {
    #[inline]
    fn get(lua: LuaState, index: i32) -> Result<(i32, Self), LuaError> {
        Ok((index + 1, lua.to_integer(index)))
    }
}

impl LuaGetValue for f64 {
    #[inline]
    fn get(lua: LuaState, index: i32) -> Result<(i32, Self), LuaError> {
        Ok((index + 1, lua.to_number(index)))
    }
}

impl LuaGetValue for bool {
    #[inline]
    fn get(lua: LuaState, index: i32) -> Result<(i32, Self), LuaError> {
        Ok((index + 1, lua.to_bool(index)))
    }
}

impl LuaGetValue for &str {
    #[inline]
    fn get(lua: LuaState, index: i32) -> Result<(i32, Self), LuaError> {
        Ok((index + 1, lua.to_str(index)?))
    }
}

impl<T: LuaGetValue> LuaGetValue for Option<T> {
    #[inline]
    fn get(lua: LuaState, index: i32) -> Result<(i32, Self), LuaError> {
        if lua.is_nil_or_none(index) {
            Ok((index + 1, None))
        } else {
            let (ind, value) = T::get(lua, index)?;
            Ok((ind, Some(value)))
        }
    }
}

impl<T: LuaGetValue> LuaGetValue for Vec<T> {
    #[inline]
    fn get(lua: LuaState, index: i32) -> Result<(i32, Self), LuaError> {
        let len = lua.objlen(index);
        let mut res = Vec::with_capacity(len);
        for i in 1..=len {
            lua.index_table(index, i);
            let (ind, get) = T::get(lua, -1)?;
            lua.pop_last();
            if ind != 0 {
                return Err(LuaError::WrongSizedTypeForArray);
            }
            res.push(get);
        }
        Ok((index + 1, res))
    }
}

impl<T: LuaGetValue, const N: usize> LuaGetValue for [T; N] {
    #[inline]
    fn get(lua: LuaState, index: i32) -> Result<(i32, Self), LuaError> {
        let len = lua.objlen(index);
        if len != N {
            return Err(LuaError::WrongArrayLength);
        }
        let mut res: [MaybeUninit<T>; N] = MaybeUninit::uninit().into();
        for (i, res) in (1..).zip(res.iter_mut()) {
            lua.index_table(index, i);
            let (ind, get) = T::get(lua, -1)?;
            lua.pop_last();
            if ind != 0 {
                return Err(LuaError::WrongSizedTypeForArray);
            }
            res.write(get);
        }
        let ret: MaybeUninit<[T; N]> = res.into();
        Ok((index + 1, unsafe { ret.assume_init() }))
    }
}
make_lua_get_tuples!(16);
