#![feature(slice_split_once)]
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
#[derive(Debug)]
struct FunGroup {
    ident: Ident,
    funs: Vec<Function>,
}
fn parse_group(tokens: TokenStream) -> (Vec<Function>, Vec<FunGroup>) {
    let mut function = Function::default();
    let mut ret: Vec<TokenTree> = Vec::new();
    let mut funs: Vec<Function> = Vec::new();
    let mut punct = false;
    let mut is_fun = false;
    let mut impl_name = None;
    let mut is_impl = false;
    let mut is_ret = 0;
    let mut groups = Vec::new();
    for token in tokens {
        match token.clone() {
            TokenTree::Group(_) if is_ret != 0 => {
                is_ret = 0;
                if !ret.is_empty() {
                    function.ret = Some(TokenStream::from_iter(mem::take(&mut ret)));
                }
                funs.push(mem::take(&mut function));
            }
            TokenTree::Group(g) if is_impl && let Some(name) = impl_name.take() => {
                let (mut funs, _) = parse_group(g.stream());
                for f in &mut funs {
                    f.args.remove(0);
                }
                groups.push(FunGroup { ident: name, funs })
            }
            TokenTree::Group(g) if is_fun && g.delimiter() == Delimiter::Parenthesis => {
                let mut arg = Vec::new();
                let mut start = false;
                for token in g.stream() {
                    if start {
                        arg.push(token.clone());
                    }
                    match token {
                        TokenTree::Ident(i) if i == "self" => {
                            arg.push(TokenTree::Ident(i));
                        }
                        TokenTree::Punct(p) if p.as_char() == ':' => {
                            start = true;
                        }
                        TokenTree::Punct(p) if p.as_char() == ',' => {
                            arg.pop();
                            start = false;
                            function
                                .args
                                .push(TokenStream::from_iter(mem::take(&mut arg)));
                        }
                        _ => {}
                    }
                }
                if !arg.is_empty() {
                    function.args.push(TokenStream::from_iter(arg));
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
            TokenTree::Ident(i) if i == "impl" => {
                is_impl = true;
            }
            TokenTree::Ident(i) if i == "for" || i == "fn" => {
                is_impl = false;
            }
            TokenTree::Ident(i) => {
                impl_name = Some(i);
            }
            _ if is_ret == 3 => {
                is_impl = false;
                punct = false;
                ret.push(token.clone());
            }
            _ => {
                is_impl = false;
                punct = false;
                is_ret = 0;
            }
        }
    }
    (funs, groups)
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
    let mut inner_tokens = tokens.clone();
    let (funs, groups) = parse_group(tokens);
    let luaopen = luaopen(funs, groups, dont_unload);
    inner_tokens.extend(quote! {use noita_api::lua_function;});
    inner_tokens.extend(luaopen);
    let mut group = Group::new(Delimiter::Brace, TokenStream::from_iter(inner_tokens));
    group.set_span(span.unwrap());
    ret_tokens.push(TokenTree::Group(group));
    TokenStream::from_iter(ret_tokens)
}
fn make_group(group: FunGroup) -> TokenStream {
    let ident = group.ident;
    let name = format_ident!("GLOBAL_{}", ident.to_string().to_ascii_uppercase());
    let funs = make_inner_funs(group.funs, Some(ident.clone()));
    quote! {
        static mut #name: std::sync::LazyLock<#ident> = std::sync::LazyLock::new(#ident::default);
        #(#funs)*
    }
}
fn luaopen(funs: Vec<Function>, groups: Vec<FunGroup>, dont_unload: bool) -> TokenStream {
    let inner_funs = make_inner_funs(funs, None);
    let keep_loaded = if dont_unload {
        quote! {
            static KEEP_SELF_LOADED: std::sync::OnceLock<noita_api::libloading::Library>
                = std::sync::OnceLock::new();
            KEEP_SELF_LOADED.get_or_init(|| unsafe { noita_api::libloading::Library::new(format!("{}.dll", env!("CARGO_PKG_NAME"))).unwrap() });
        }
    } else {
        quote! {}
    };
    let groups = groups.into_iter().map(make_group);
    quote! {
        #[unsafe(no_mangle)]
        unsafe extern "C" fn luaopen(lua: *mut noita_api::lua_bindings::lua_State) -> std::ffi::c_int {
            std::panic::set_hook(Box::new(|panic| noita_api::log_println!("{panic}")));
            #keep_loaded
            unsafe {
                noita_api::lua::LUA.lua_createtable(lua, 0, 0);
                #(#inner_funs)*
                #(#groups)*
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
#[proc_macro_attribute]
pub fn assert_size(
    arg: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let arg: TokenStream = arg.into();
    let tokens: TokenStream = tokens.into();
    let mut struct_name = None;
    let mut expect_name = false;
    for token in tokens.clone().into_iter() {
        match token {
            TokenTree::Ident(ident)
                if matches!(ident.to_string().as_str(), "struct" | "enum" | "union") =>
            {
                expect_name = true
            }
            TokenTree::Ident(ident) if expect_name => {
                struct_name = Some(ident);
                break;
            }
            _ => {}
        }
    }
    let struct_name = struct_name.unwrap();
    let assert = quote! {
        #[cfg(target_arch = "x86")]
        const _: () = assert!(size_of::<#struct_name>() == #arg);
        #[cfg(target_arch = "x86_64")]
        const _: () = assert!(size_of::<#struct_name>() >= #arg);
    };
    quote! {
        #tokens
        #assert
    }
    .into()
}
#[proc_macro_attribute]
pub fn assert_size_with(
    arg: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let arg: TokenStream = arg.into();
    let arg = arg.into_iter().collect::<Vec<TokenTree>>();
    let (arg, t) = arg
        .rsplit_once(|t| {
            if let TokenTree::Punct(p) = t {
                p.as_char() == ','
            } else {
                false
            }
        })
        .unwrap();
    let t = TokenStream::from_iter(t.iter().cloned());
    let arg = TokenStream::from_iter(arg.iter().cloned());
    let tokens: TokenStream = tokens.into();
    let mut struct_name = None;
    let mut expect_name = false;
    for token in tokens.clone().into_iter() {
        match token {
            TokenTree::Ident(ident)
                if matches!(ident.to_string().as_str(), "struct" | "enum" | "union") =>
            {
                expect_name = true
            }
            TokenTree::Ident(ident) if expect_name => {
                struct_name = Some(ident);
                break;
            }
            _ => {}
        }
    }
    let struct_name = struct_name.unwrap();
    let assert = quote! {
        #[cfg(target_arch = "x86")]
        const _: () = assert!(size_of::<#struct_name::<#t>>() == #arg);
        #[cfg(target_arch = "x86_64")]
        const _: () = assert!(size_of::<#struct_name::<#t>>() >= #arg);
    };
    quote! {
        #tokens
        #assert
    }
    .into()
}
#[proc_macro_attribute]
pub fn generate_global(
    _: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tokens: TokenStream = tokens.into();
    let mut is_ptr = false;
    let mut is_ptr_ptr = false;
    let mut type_name = Vec::new();
    let mut global_const = None;
    let mut n = 0;
    for token in tokens.clone().into_iter() {
        match token {
            TokenTree::Ident(ident) if global_const.is_none() && ident != "const" => {
                global_const = Some(ident.clone());
            }
            TokenTree::Ident(ident) if ident == "StdPtr" => {
                if !is_ptr {
                    is_ptr = true;
                } else if !is_ptr_ptr {
                    is_ptr_ptr = true;
                }
                n = 0;
            }
            TokenTree::Ident(ident) if is_ptr || is_ptr_ptr => {
                type_name.push(TokenTree::Ident(ident));
            }
            TokenTree::Punct(p) if (is_ptr || is_ptr_ptr) && p.as_char() == ':' => {
                type_name.push(TokenTree::Punct(p));
            }
            TokenTree::Punct(p) if (is_ptr || is_ptr_ptr) && p.as_char() == '<' => {
                if n != 0 {
                    type_name.push(TokenTree::Punct(p));
                }
                n += 1;
            }
            TokenTree::Punct(p) if (is_ptr || is_ptr_ptr) && p.as_char() == '>' && n > 1 => {
                type_name.push(TokenTree::Punct(p));
                n -= 1;
            }
            TokenTree::Punct(p) if (is_ptr || is_ptr_ptr) && p.as_char() == '>' => break,
            _ => {}
        }
    }
    let type_name = TokenStream::from_iter(type_name);
    let global_type = get_global_type(global_const.unwrap(), type_name, is_ptr_ptr);
    quote! {
        #tokens
        #global_type
    }
    .into()
}
fn get_global_type(global_const: Ident, type_name: TokenStream, is_ptr_ptr: bool) -> TokenStream {
    let ptr_read = if is_ptr_ptr {
        quote! {unsafe{#global_const.read()}}
    } else {
        quote! {#global_const}
    };
    quote! {
        impl #type_name {
            pub fn global() -> StdBox<Self> {
                StdBox::from(#ptr_read)
            }
        }
    }
}
fn add_lua_fn(fun: Function, struct_ident: Option<Ident>) -> TokenStream {
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
            let index = if i == fun.args.len() - 1 {
                quote! {}
            } else {
                quote! {index += <#ts as noita_api::lua::LuaGetValue>::size_on_stack();}
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
    let ret = if let Some(struct_ident) = struct_ident {
        let name = format_ident!("GLOBAL_{}", struct_ident.to_string().to_ascii_uppercase());
        quote! {
            let ret = unsafe{#struct_ident::#ident(&mut #name, #(#args,)*)};
        }
    } else {
        quote! {
            let ret = #ident(#(#args,)*);
        }
    };
    quote! {
        unsafe extern "C" fn #bridge_fn_name(lua: *mut noita_api::lua_bindings::lua_State) -> std::ffi::c_int {
            let lua_state = noita_api::lua::LuaState::new(lua);
            lua_state.make_current();
            #index
            #(#vars)*
            #ret
            let ret = noita_api::lua::LuaFnRet::do_return(ret, lua_state);
            ret
        }
        noita_api::lua::LUA.lua_pushcclosure(lua, Some(#bridge_fn_name), 0);
        noita_api::lua::LUA.lua_setfield(lua, -2, #fn_name_c.as_ptr());
    }
}
fn name_to_c_literal(name: &str) -> Literal {
    Literal::c_string(CString::new(name).unwrap().as_c_str())
}
fn make_inner_funs(idents: Vec<Function>, ident: Option<Ident>) -> Vec<TokenStream> {
    let mut inner_funs = Vec::new();
    for fun in idents {
        inner_funs.push(add_lua_fn(fun, ident.clone()));
    }
    inner_funs
}
