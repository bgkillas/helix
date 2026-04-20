use proc_macro2::{Ident, Literal, TokenStream, TokenTree};
use quote::{format_ident, quote};
use std::ffi::CString;
#[proc_macro]
pub fn register_lua_functions_dont_unload(
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tokens: TokenStream = tokens.into();
    let mut tokens = tokens.into_iter();
    let TokenTree::Ident(name) = tokens.next().unwrap() else {
        unreachable!()
    };
    let mut funs = Vec::new();
    while let Some(token) = tokens.nth(1) {
        let TokenTree::Ident(token) = token else {
            unreachable!()
        };
        funs.push(token);
    }
    let (make_inner_funs, inner_funs) = make_inner_funs(funs);
    let dll = Literal::string(&format!("{}.dll", name));
    quote! {
        #[unsafe(no_mangle)]
        unsafe extern "C" fn luaopen(lua: *mut noita_api::noita_api::lua::lua_bindings::lua_State) -> std::ffi::c_int {
            static KEEP_SELF_LOADED: LazyLock<Result<noita_api::libloading::Library, noita_api::libloading::Error>> = LazyLock::new(|| unsafe { noita_api::libloading::Library::new(#dll) });
            let _ = std::hint::black_box(KEEP_SELF_LOADED.as_ref());
            #(#make_inner_funs)*
            use noita_api::noita_api::lua::{LUA, lua_bindings::LUA_REGISTRYINDEX};
            unsafe {
                LUA.lua_createtable(lua, 0, 0);
                #(#inner_funs)*
            }
            1
        }
    }
    .into()
}
#[proc_macro]
pub fn register_lua_functions(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: TokenStream = tokens.into();
    let mut tokens = tokens.into_iter();
    let mut funs = Vec::new();
    while let Some(token) = tokens.next() {
        let TokenTree::Ident(token) = token else {
            unreachable!()
        };
        funs.push(token);
        tokens.next();
    }
    let (make_inner_funs, inner_funs) = make_inner_funs(funs);
    quote! {
        #[unsafe(no_mangle)]
        unsafe extern "C" fn luaopen(lua: *mut noita_api::noita_api::lua::lua_bindings::lua_State) -> std::ffi::c_int {
            #(#make_inner_funs)*
            use noita_api::noita_api::lua::{LUA, lua_bindings::LUA_REGISTRYINDEX};
            unsafe {
                LUA.lua_createtable(lua, 0, 0);
                #(#inner_funs)*
            }
            1
        }
    }
    .into()
}
fn add_lua_fn(fn_name_ident: Ident, ident: Ident) -> TokenStream {
    let bridge_fn_name = format_ident!("{fn_name_ident}_lua_bridge");
    let fn_name_c = name_to_c_literal(&ident.to_string());
    quote! {
        unsafe extern "C" fn #bridge_fn_name(lua: *mut noita_api::noita_api::lua::lua_bindings::lua_State) -> std::ffi::c_int {
            let lua_state = noita_api::noita_api::lua::LuaState::new(lua);
            lua_state.make_current();
            let ret = noita_api::noita_api::lua::LuaFnRet::do_return(#fn_name_ident(lua_state), lua_state);
            ret
        }
        LUA.lua_pushcclosure(lua, Some(#bridge_fn_name), 0);
        LUA.lua_setfield(lua, -2, #fn_name_c.as_ptr());
    }
}
fn name_to_c_literal(name: &str) -> Literal {
    Literal::c_string(CString::new(name).unwrap().as_c_str())
}
fn make_inner_funs(idents: Vec<Ident>) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut make_inner_funs = Vec::new();
    let mut inner_funs = Vec::new();
    for ident in idents {
        let inner = format_ident!("inner_{}", ident);
        inner_funs.push(add_lua_fn(inner.clone(), ident.clone()));
        make_inner_funs.push(quote! {
            fn #inner(_: noita_api::noita_api::lua::LuaState) -> eyre::Result<()> {
                #ident()
            }
        });
    }
    (make_inner_funs, inner_funs)
}
