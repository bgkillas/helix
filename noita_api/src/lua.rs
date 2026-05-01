pub use crate::lua_bindings::{LUA_GLOBALSINDEX, lua_State};
use crate::lua_bindings::{Lua51, lua_CFunction};
use eyre::{Context as _, OptionExt as _, bail, eyre};
use std::{
    array,
    borrow::Cow,
    cell::Cell,
    ffi::{CStr, c_char, c_int},
    slice,
    sync::LazyLock,
};

//use crate::{Color, ComponentID, EntityID, GameEffectEnum, Obj, PhysicsBodyID};

thread_local! {
    static CURRENT_LUA_STATE: Cell<Option<LuaState>> = Cell::default();
}

pub static LUA: LazyLock<Lua51> =
    LazyLock::new(|| unsafe { Lua51::new("lua51.dll").expect("library to be lua") });

#[derive(Clone, Copy)]
pub struct LuaState {
    lua: *mut lua_State,
}

impl LuaState {
    #[inline]
    pub fn new(lua: *mut lua_State) -> Self {
        Self { lua }
    }

    /// Returns a lua state that is considered "current". Usually set when we get called from noita.
    #[inline]
    pub fn current() -> eyre::Result<Self> {
        CURRENT_LUA_STATE
            .get()
            .ok_or_eyre("No current lua state available")
    }

    #[inline]
    pub fn make_current(self) {
        CURRENT_LUA_STATE.set(Some(self));
    }

    #[must_use]
    #[inline]
    pub fn raw(&self) -> *mut lua_State {
        self.lua
    }

    #[must_use]
    #[inline]
    pub fn to_integer(&self, index: i32) -> isize {
        unsafe { (LUA.lua_tointeger)(self.lua, index) }
    }

    #[must_use]
    #[inline]
    pub fn to_integer_array(
        &self,
        index: i32,
        len: usize,
    ) -> impl DoubleEndedIterator<Item = isize> {
        (1..=len).map(move |i| unsafe {
            (LUA.lua_pushinteger)(self.lua, i.cast_signed());
            (LUA.lua_gettable)(self.lua, index);
            (LUA.lua_tointeger)(self.lua, -1)
        })
    }

    #[must_use]
    #[inline]
    pub fn to_number(&self, index: i32) -> f64 {
        unsafe { (LUA.lua_tonumber)(self.lua, index) }
    }

    #[must_use]
    #[inline]
    pub fn to_bool(&self, index: i32) -> bool {
        unsafe { (LUA.lua_toboolean)(self.lua, index) > 0 }
    }

    #[inline]
    pub fn to_string(&self, index: i32) -> eyre::Result<String> {
        let mut size = 0;
        let buf = unsafe { (LUA.lua_tolstring)(self.lua, index, &raw mut size) };
        if buf.is_null() {
            bail!("Expected a string, but got a null pointer");
        }
        let slice = unsafe { slice::from_raw_parts(buf.cast::<u8>(), size) };

        String::from_utf8(slice.to_owned())
            .wrap_err("Attempting to get lua string, expecting it to be utf-8")
    }

    #[inline]
    pub fn to_str<'a>(&self, index: i32) -> eyre::Result<&'a str> {
        let mut size = 0;
        let buf = unsafe { (LUA.lua_tolstring)(self.lua, index, &raw mut size) };
        if buf.is_null() {
            bail!("Expected a string, but got a null pointer");
        }
        let slice = unsafe { slice::from_raw_parts(buf.cast::<u8>(), size) };

        str::from_utf8(slice).wrap_err("Attempting to get lua string, expecting it to be utf-8")
    }

    #[inline]
    pub fn to_raw_string(&self, index: i32) -> eyre::Result<Vec<u8>> {
        let mut size = 0;
        let buf = unsafe { (LUA.lua_tolstring)(self.lua, index, &raw mut size) };
        if buf.is_null() {
            bail!("Expected a string, but got a null pointer");
        }
        let slice = unsafe { slice::from_raw_parts(buf.cast::<u8>(), size) };

        Ok(slice.to_owned())
    }

    #[must_use]
    #[inline]
    pub fn to_cfunction(&self, index: i32) -> lua_CFunction {
        unsafe { (LUA.lua_tocfunction)(self.lua, index) }
    }

    #[inline]
    pub fn push_number(&self, val: f64) {
        unsafe { (LUA.lua_pushnumber)(self.lua, val) };
    }

    #[inline]
    pub fn push_integer(&self, val: isize) {
        unsafe { (LUA.lua_pushinteger)(self.lua, val) };
    }

    #[inline]
    pub fn push_bool(&self, val: bool) {
        unsafe { (LUA.lua_pushboolean)(self.lua, i32::from(val)) };
    }

    #[inline]
    pub fn push_string(&self, s: &str) {
        unsafe {
            (LUA.lua_pushlstring)(self.lua, s.as_bytes().as_ptr().cast::<c_char>(), s.len());
        }
    }

    #[inline]
    pub fn push_raw_string(&self, s: &[u8]) {
        unsafe {
            (LUA.lua_pushlstring)(self.lua, s.as_ptr().cast::<c_char>(), s.len());
        }
    }

    #[inline]
    pub fn push_nil(&self) {
        unsafe { (LUA.lua_pushnil)(self.lua) }
    }

    #[inline]
    pub fn call(&self, nargs: i32, nresults: i32) -> eyre::Result<()> {
        let ret = unsafe { (LUA.lua_pcall)(self.lua, nargs, nresults, 0) };
        if ret == 0 {
            Ok(())
        } else {
            let msg = self
                .to_string(-1)
                .wrap_err("Failed to get error message string")?;
            bail!("Error while calling lua function: {}", msg)
        }
    }

    #[inline]
    pub fn get_global(&self, name: &CStr) {
        unsafe { (LUA.lua_getfield)(self.lua, LUA_GLOBALSINDEX, name.as_ptr()) };
    }

    #[must_use]
    #[inline]
    pub fn objlen(&self, index: i32) -> usize {
        unsafe { (LUA.lua_objlen)(self.lua, index) }
    }

    #[inline]
    pub fn index_table(&self, table_index: i32, index_in_table: usize) {
        self.push_integer(index_in_table.cast_signed());
        if table_index < 0 {
            unsafe { (LUA.lua_gettable)(self.lua, table_index - 1) };
        } else {
            unsafe { (LUA.lua_gettable)(self.lua, table_index) };
        }
    }

    #[inline]
    pub fn pop_last(&self) {
        unsafe { (LUA.lua_settop)(self.lua, -2) };
    }
    #[inline]
    pub fn pop_last_n(&self, n: i32) {
        unsafe { (LUA.lua_settop)(self.lua, -1 - (n)) };
    }

    /// Raise an error with message `s`
    ///
    /// This takes String so that it gets deallocated properly, as this functions doesn't return.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn raise_error(&self, s: String) -> ! {
        self.push_string(&s);
        drop(s);
        unsafe { (LUA.lua_error)(self.lua) };
        // lua_error does not return.
        unreachable!()
    }

    #[must_use]
    #[inline]
    pub fn is_nil_or_none(&self, index: i32) -> bool {
        (unsafe { (LUA.lua_type)(self.lua, index) }) <= 0
    }

    #[inline]
    pub fn create_table(&self, narr: c_int, nrec: c_int) {
        unsafe { (LUA.lua_createtable)(self.lua, narr, nrec) };
    }

    #[inline]
    pub fn rawset_table(&self, table_index: i32, index_in_table: i32) {
        unsafe { (LUA.lua_rawseti)(self.lua, table_index, index_in_table) };
    }

    #[must_use]
    #[inline]
    pub fn checkstack(&self, sz: i32) -> bool {
        unsafe { (LUA.lua_checkstack)(self.lua, sz) > 0 }
    }
}

pub struct RawString(Vec<u8>);

impl From<Vec<u8>> for RawString {
    #[inline]
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

/// Used for types that can be returned from functions that were defined in rust to lua.
pub trait LuaFnRet {
    fn do_return(self, lua: LuaState) -> c_int;
}

/// Function intends to return several values that it has on stack.
pub struct ValuesOnStack(pub c_int);

impl LuaFnRet for ValuesOnStack {
    #[inline]
    fn do_return(self, _lua: LuaState) -> c_int {
        self.0
    }
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
impl LuaFnRet for usize {
    #[inline]
    fn do_return(self, lua: LuaState) -> c_int {
        lua.push_integer(self.cast_signed());
        1
    }
}

/*impl LuaFnRet for EntityID {
    fn do_return(self, lua: LuaState) -> c_int {
        lua.push_integer(self.0.into());
        1
    }
}*/

impl<R: LuaFnRet> LuaFnRet for eyre::Result<R> {
    #[inline]
    fn do_return(self, lua: LuaState) -> c_int {
        match self {
            Ok(ok) => ok.do_return(lua),
            Err(err) => unsafe {
                lua.raise_error(format!("Error in rust call: {err:?}"));
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

impl LuaFnRet for RawString {
    #[inline]
    fn do_return(self, lua: LuaState) -> c_int {
        lua.push_raw_string(&self.0);
        1
    }
}

/// Trait for arguments that can be put on lua stack.
pub trait LuaPutValue {
    const SIZE_ON_STACK: u32 = 1;
    fn put(&self, lua: LuaState);
    #[inline]
    fn is_non_empty(&self) -> bool {
        true
    }
}

impl LuaPutValue for isize {
    #[inline]
    fn put(&self, lua: LuaState) {
        lua.push_integer(*self);
    }
}

impl LuaPutValue for usize {
    #[inline]
    fn put(&self, lua: LuaState) {
        lua.push_integer(self.cast_signed());
    }
}

impl LuaPutValue for f64 {
    #[inline]
    fn put(&self, lua: LuaState) {
        lua.push_number(*self);
    }
}

impl LuaPutValue for bool {
    #[inline]
    fn put(&self, lua: LuaState) {
        lua.push_bool(*self);
    }
}

impl LuaPutValue for str {
    #[inline]
    fn put(&self, lua: LuaState) {
        lua.push_string(self);
    }
}

/*impl LuaPutValue for EntityID {
    fn put(&self, lua: LuaState) {
        isize::from(self.0).put(lua);
    }
}

impl LuaPutValue for ComponentID {
    fn put(&self, lua: LuaState) {
        isize::from(self.0).put(lua);
    }
}

impl LuaPutValue for Color {
    fn put(&self, _lua: LuaState) {
        todo!()
    }
}

impl LuaPutValue for Obj {
    fn put(&self, _lua: LuaState) {
        todo!()
    }
}

impl LuaPutValue for PhysicsBodyID {
    fn put(&self, lua: LuaState) {
        lua.push_integer(self.0 as isize);
    }
}*/

impl<T: LuaPutValue> LuaPutValue for Option<T> {
    #[inline]
    fn put(&self, lua: LuaState) {
        const { assert!(T::SIZE_ON_STACK == 1) }
        match self {
            Some(val) => val.put(lua),
            None => lua.push_nil(),
        }
    }

    #[inline]
    fn is_non_empty(&self) -> bool {
        match self {
            Some(val) => val.is_non_empty(),
            None => false,
        }
    }
}

// A.k.a. vec2
impl LuaPutValue for (f32, f32) {
    const SIZE_ON_STACK: u32 = 2;
    #[inline]
    fn put(&self, lua: LuaState) {
        lua.push_number(f64::from(self.0));
        lua.push_number(f64::from(self.1));
    }
}

/*impl LuaPutValue for GameEffectEnum {
    fn put(&self, lua: LuaState) {
        lua.push_string(self.into());
    }
}*/

/// Trait for arguments that can be retrieved from the lua stack.
pub trait LuaGetValue {
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self>
    where
        Self: Sized;
    #[must_use]
    #[inline]
    fn size_on_stack() -> i32 {
        1
    }
}

impl LuaGetValue for isize {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        Ok(lua.to_integer(index))
    }
}

impl LuaGetValue for usize {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        Ok(lua.to_integer(index).cast_unsigned())
    }
}

impl LuaGetValue for f64 {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        Ok(lua.to_number(index))
    }
}

impl LuaGetValue for bool {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        Ok(lua.to_bool(index))
    }
}

/*impl LuaGetValue for Option<EntityID> {
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        let ent = lua.to_integer(index);
        Ok(if ent == 0 {
            None
        } else {
            Some(EntityID(ent.try_into()?))
        })
    }
}

impl LuaGetValue for Option<ComponentID> {
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        let com = lua.to_integer(index);
        Ok(if com == 0 {
            None
        } else {
            Some(ComponentID(com.try_into()?))
        })
    }
}*/

impl LuaGetValue for Cow<'_, str> {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        Ok(lua.to_str(index)?.into())
    }
}

impl LuaGetValue for &str {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        lua.to_str(index)
    }
}

impl LuaGetValue for () {
    #[inline]
    fn get(_lua: LuaState, _index: i32) -> eyre::Result<Self> {
        Ok(())
    }
}

/*impl LuaGetValue for Obj {
    fn get(_lua: LuaState, _index: i32) -> eyre::Result<Self> {
        todo!()
    }
}*/

/*impl LuaGetValue for Color {
    fn get(_lua: LuaState, _index: i32) -> eyre::Result<Self> {
        todo!()
    }
}*/

/*impl LuaGetValue for PhysicsBodyID {
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        Ok(PhysicsBodyID(lua.to_number(index) as i32))
    }
}*/

impl<T: LuaGetValue> LuaGetValue for Option<T> {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        Ok(if lua.is_nil_or_none(index) {
            None
        } else {
            Some(T::get(lua, index)?)
        })
    }

    #[inline]
    fn size_on_stack() -> i32 {
        1
    }
}

impl<T: LuaGetValue> LuaGetValue for Vec<T> {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        if T::size_on_stack() != 1 {
            bail!(
                "Encountered Vec<T> where T needs more than 1 slot on the stack. This isn't supported"
            );
        }
        let len = lua.objlen(index);
        let mut res = Vec::with_capacity(len);
        for i in 1..=len {
            lua.index_table(index, i);
            let get = T::get(lua, -1);
            lua.pop_last();
            res.push(get?);
        }
        Ok(res)
    }
}

impl<T: LuaGetValue, const N: usize> LuaGetValue for [T; N] {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        if T::size_on_stack() != 1 {
            bail!(
                "Encountered [T; N] where T needs more than 1 slot on the stack. This isn't supported"
            );
        }
        let len = lua.objlen(index);
        if len != N {
            return Err(eyre!("mis matched length {}", len));
        }
        let mut res: [Option<T>; N] = array::from_fn(|_| None);
        for (i, res) in res.iter_mut().enumerate() {
            lua.index_table(index, i);
            let get = T::get(lua, -1);
            lua.pop_last();
            *res = Some(get?);
        }
        let mut res_iter = res.into_iter();
        let ret: [T; N] = array::from_fn(|_| res_iter.next().unwrap().unwrap());
        Ok(ret)
    }
}

/*impl LuaGetValue for GameEffectEnum {
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        Ok(GameEffectEnum::from_str(&lua.to_string(index)?)?)
    }
}*/

impl<T0: LuaGetValue, T1: LuaGetValue> LuaGetValue for (T0, T1) {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        Ok((
            T0::get(lua, index - T1::size_on_stack())?,
            T1::get(lua, index)?,
        ))
    }

    #[inline]
    fn size_on_stack() -> i32 {
        T0::size_on_stack() + T1::size_on_stack()
    }
}

impl<T0: LuaGetValue, T1: LuaGetValue, T2: LuaGetValue> LuaGetValue for (T0, T1, T2) {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        Ok((
            T0::get(lua, index - T1::size_on_stack() - T2::size_on_stack())?,
            T1::get(lua, index - T2::size_on_stack())?,
            T2::get(lua, index)?,
        ))
    }

    #[inline]
    fn size_on_stack() -> i32 {
        T0::size_on_stack() + T1::size_on_stack() + T2::size_on_stack()
    }
}

impl<T0: LuaGetValue, T1: LuaGetValue, T2: LuaGetValue, T3: LuaGetValue> LuaGetValue
    for (T0, T1, T2, T3)
{
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        Ok((
            T0::get(
                lua,
                index - T1::size_on_stack() - T2::size_on_stack() - T3::size_on_stack(),
            )?,
            T1::get(lua, index - T2::size_on_stack() - T3::size_on_stack())?,
            T2::get(lua, index - T3::size_on_stack())?,
            T3::get(lua, index)?,
        ))
    }

    #[inline]
    fn size_on_stack() -> i32 {
        T0::size_on_stack() + T1::size_on_stack() + T2::size_on_stack() + T3::size_on_stack()
    }
}

impl<T0: LuaGetValue, T1: LuaGetValue, T2: LuaGetValue, T3: LuaGetValue, T4: LuaGetValue>
    LuaGetValue for (T0, T1, T2, T3, T4)
{
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        let prev = <(T0, T1, T2, T3)>::get(lua, index - T4::size_on_stack())?;
        Ok((prev.0, prev.1, prev.2, prev.3, T4::get(lua, index)?))
    }

    #[inline]
    fn size_on_stack() -> i32 {
        <(T0, T1, T2, T3)>::size_on_stack() + T4::size_on_stack()
    }
}

impl<
    T0: LuaGetValue,
    T1: LuaGetValue,
    T2: LuaGetValue,
    T3: LuaGetValue,
    T4: LuaGetValue,
    T5: LuaGetValue,
> LuaGetValue for (T0, T1, T2, T3, T4, T5)
{
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        let prev = <(T0, T1, T2, T3, T4)>::get(lua, index - T5::size_on_stack())?;
        Ok((prev.0, prev.1, prev.2, prev.3, prev.4, T5::get(lua, index)?))
    }

    #[inline]
    fn size_on_stack() -> i32 {
        <(T0, T1, T2, T3, T4)>::size_on_stack() + T5::size_on_stack()
    }
}

impl<
    T0: LuaGetValue,
    T1: LuaGetValue,
    T2: LuaGetValue,
    T3: LuaGetValue,
    T4: LuaGetValue,
    T5: LuaGetValue,
    T6: LuaGetValue,
> LuaGetValue for (T0, T1, T2, T3, T4, T5, T6)
{
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        let prev = <(T0, T1, T2, T3, T4, T5)>::get(lua, index - T6::size_on_stack())?;
        Ok((
            prev.0,
            prev.1,
            prev.2,
            prev.3,
            prev.4,
            prev.5,
            T6::get(lua, index)?,
        ))
    }

    #[inline]
    fn size_on_stack() -> i32 {
        <(T0, T1, T2, T3, T4, T5)>::size_on_stack() + T6::size_on_stack()
    }
}

impl LuaGetValue for (bool, bool, bool, f64, f64, f64, f64, f64, f64, f64, f64) {
    #[inline]
    fn get(lua: LuaState, index: i32) -> eyre::Result<Self> {
        Ok((
            bool::get(lua, index - 10)?,
            bool::get(lua, index - 9)?,
            bool::get(lua, index - 8)?,
            f64::get(lua, index - 7)?,
            f64::get(lua, index - 6)?,
            f64::get(lua, index - 5)?,
            f64::get(lua, index - 4)?,
            f64::get(lua, index - 3)?,
            f64::get(lua, index - 2)?,
            f64::get(lua, index - 1)?,
            f64::get(lua, index)?,
        ))
    }

    #[inline]
    fn size_on_stack() -> i32 {
        11
    }
}
