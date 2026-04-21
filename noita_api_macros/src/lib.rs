use proc_macro2::{Delimiter, Group, Ident, Literal, TokenStream, TokenTree};
use quote::{format_ident, quote};
use std::ffi::CString;
use std::mem;
#[derive(Default, Debug)]
struct Function {
    name: Option<Ident>,
    args: Vec<TokenStream>,
    ret: Option<TokenStream>,
}
fn parse_attribute(mut tokens: TokenStream, dont_unload: bool) -> TokenStream {
    let mut ret_tokens = Vec::new();
    let mut span = None;
    for token in tokens.clone() {
        if let TokenTree::Group(group) = token {
            span = Some(group.span());
            tokens = group.stream();
            break;
        }
        ret_tokens.push(token);
    }
    let mut function = Function::default();
    let mut ret: Vec<TokenTree> = Vec::new();
    let mut inner_tokens = Vec::new();
    inner_tokens.extend(quote! {use noita_api::lua_function;});
    let mut funs: Vec<Function> = Vec::new();
    let mut punct = false;
    let mut is_fun = false;
    let mut is_ret = 0;
    for token in tokens {
        match token.clone() {
            TokenTree::Group(_) if is_ret != 0 => {
                is_ret = 0;
                if !ret.is_empty() {
                    function.ret = Some(TokenStream::from_iter(mem::take(&mut ret)));
                }
                funs.push(mem::take(&mut function));
            }
            TokenTree::Group(g) if is_fun && g.delimiter() == Delimiter::Parenthesis => {
                let mut arg = Vec::new();
                let mut start = false;
                for token in g.stream() {
                    if start {
                        arg.push(token.clone());
                    }
                    match token {
                        TokenTree::Punct(p) if p.as_char() == ':' => {
                            start = true;
                        }
                        TokenTree::Punct(p) if p.as_char() == ',' => {
                            arg.pop();
                            start = false;
                            function
                                .args
                                .push(TokenStream::from_iter(mem::take(&mut arg)))
                        }
                        _ => {}
                    }
                }
                if !arg.is_empty() {
                    function.args.push(TokenStream::from_iter(arg))
                }
                is_fun = false;
                is_ret = 1;
            }
            TokenTree::Group(g)
                if punct
                    && g.delimiter() == Delimiter::Bracket
                    && let Some(TokenTree::Ident(i)) = g.stream().into_iter().next()
                    && i == "lua_function" =>
            {
                is_fun = true;
                punct = false;
            }
            TokenTree::Punct(p) if p.as_char() == '#' => {
                punct = true;
                is_fun = false;
            }
            TokenTree::Punct(p) if is_ret == 1 && p.as_char() == '-' => {
                is_ret += 1;
            }
            TokenTree::Punct(p) if is_ret == 2 && p.as_char() == '>' => {
                is_ret += 1;
            }
            TokenTree::Ident(i) if is_fun => {
                function.name = Some(i);
                punct = false;
                is_ret = 0;
            }
            _ if is_ret == 3 => {
                punct = false;
                ret.push(token.clone());
            }
            _ => {
                punct = false;
                is_ret = 0;
            }
        }
        inner_tokens.push(token);
    }
    let luaopen = luaopen(funs, dont_unload);
    inner_tokens.extend(luaopen);
    let mut group = Group::new(Delimiter::Brace, TokenStream::from_iter(inner_tokens));
    group.set_span(span.unwrap());
    ret_tokens.push(TokenTree::Group(group));
    TokenStream::from_iter(ret_tokens)
}
fn luaopen(funs: Vec<Function>, dont_unload: bool) -> TokenStream {
    let inner_funs = make_inner_funs(funs);
    let dll = Literal::string(&format!("{}.dll", env!("CARGO_PKG_NAME")));
    let keep_loaded = if dont_unload {
        quote! {
            static KEEP_SELF_LOADED: std::sync::LazyLock<Result<noita_api::libloading::Library, noita_api::libloading::Error>>
                = std::sync::LazyLock::new(|| unsafe { noita_api::libloading::Library::new(#dll) });
            let _ = std::hint::black_box(KEEP_SELF_LOADED.as_ref());
        }
    } else {
        quote! {}
    };
    quote! {
        #[unsafe(no_mangle)]
        unsafe extern "C" fn luaopen(lua: *mut noita_api::lua_bindings::lua_State) -> std::ffi::c_int {
            #keep_loaded
            unsafe {
                noita_api::lua::LUA.lua_createtable(lua, 0, 0);
                #(#inner_funs)*
            }
            1
        }
    }
}
#[proc_macro_attribute]
pub fn lua_function(
    _: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    tokens
}
#[proc_macro_attribute]
pub fn lua_module(
    arg: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let arg: TokenStream = arg.into();
    let dont_unload = if let Some(TokenTree::Ident(ident)) = arg.into_iter().next() {
        ident == "true"
    } else {
        false
    };
    parse_attribute(tokens.into(), dont_unload).into()
}
fn add_lua_fn(fun: Function) -> TokenStream {
    let ident = fun.name.unwrap();
    let bridge_fn_name = format_ident!("{ident}_lua_bridge");
    let fn_name_c = name_to_c_literal(&ident.to_string());
    let vars: Vec<_> = fun
        .args
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, ts)| {
            let ident = format_ident!("a{}", i);
            let index = if i != fun.args.len() - 1 {
                quote! {index += <#ts as noita_api::lua::LuaGetValue>::size_on_stack();}
            } else {
                quote! {}
            };
            quote! {
                let val: eyre::Result<#ts> = noita_api::lua::LuaGetValue::get(lua_state, index);
                let #ident = match val {
                    Ok(v) => v,
                    Err(err) => lua_state.raise_error(format!("Error in rust call: {err:?}")),
                };
                #index
            }
        })
        .collect();
    let index = if fun.args.is_empty() {
        quote! {}
    } else {
        quote! {let mut index = 1;}
    };
    let args: Vec<_> = fun
        .args
        .into_iter()
        .enumerate()
        .map(|(i, _)| {
            let ident = format_ident!("a{}", i);
            quote! {
                #ident
            }
        })
        .collect();
    quote! {
        unsafe extern "C" fn #bridge_fn_name(lua: *mut noita_api::lua_bindings::lua_State) -> std::ffi::c_int {
            let lua_state = noita_api::lua::LuaState::new(lua);
            lua_state.make_current();
            #index
            #(#vars)*
            let ret = noita_api::lua::LuaFnRet::do_return(#ident(#(#args,)*), lua_state);
            ret
        }
        noita_api::lua::LUA.lua_pushcclosure(lua, Some(#bridge_fn_name), 0);
        noita_api::lua::LUA.lua_setfield(lua, -2, #fn_name_c.as_ptr());
    }
}
fn name_to_c_literal(name: &str) -> Literal {
    Literal::c_string(CString::new(name).unwrap().as_c_str())
}
fn make_inner_funs(idents: Vec<Function>) -> Vec<TokenStream> {
    let mut inner_funs = Vec::new();
    for fun in idents {
        inner_funs.push(add_lua_fn(fun));
    }
    inner_funs
}
