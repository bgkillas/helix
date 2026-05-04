#![feature(slice_split_once)]
#![allow(clippy::shadow_reuse)]
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt as _, format_ident, quote};
use std::ffi::CString;
use std::fmt::{Display, Formatter, Write as _};
use std::fs::OpenOptions;
use std::io::Write as _;
use std::{iter, mem};
#[derive(Default, Debug)]
struct Function {
    name: Option<Ident>,
    args: Vec<Type>,
    arg_names: Vec<String>,
    ret: Option<Type>,
}
#[derive(Debug, Clone)]
enum Type {
    Bool,
    Isize,
    F64,
    Str,
    RawStr,
    Parent,
    NilOr(Box<Type>),
    Tuple(Vec<Type>),
    Vec(Box<Type>),
    Slice(Box<Type>),
    Array(Box<Type>, usize),
    LuaState,
    Empty,
}
impl Type {
    fn make_ref(&self) -> bool {
        matches!(self, Self::Slice(_))
    }
    fn put_in_lua(&self) -> bool {
        !matches!(self, Self::Parent | Self::Empty | Self::LuaState)
    }
}
impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Bool => write!(f, "boolean"),
            Type::Isize => write!(f, "integer"),
            Type::F64 => write!(f, "number"),
            Type::Str | Type::RawStr => write!(f, "string"),
            Type::NilOr(ty) => write!(f, "{ty}?"),
            Type::Tuple(tys) => write!(
                f,
                "({})",
                tys.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Type::Vec(ty) => write!(f, "Vec<{ty}>"),
            Type::Slice(ty) => write!(f, "[{ty}]"),
            Type::Array(ty, n) => write!(f, "[{ty}; {n}]"),
            Type::Parent | Type::Empty | Type::LuaState => unreachable!(),
        }
    }
}
impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Type::Bool => tokens.append(TokenTree::Ident(Ident::new("bool", Span::call_site()))),
            Type::Isize => tokens.append(TokenTree::Ident(Ident::new("isize", Span::call_site()))),
            Type::F64 => tokens.append(TokenTree::Ident(Ident::new("f64", Span::call_site()))),
            Type::Str => {
                tokens.append(TokenTree::Punct(Punct::new('&', Spacing::Alone)));
                tokens.append(TokenTree::Ident(Ident::new("str", Span::call_site())));
            }
            Type::RawStr => {
                tokens.append(TokenTree::Punct(Punct::new('&', Spacing::Alone)));
                tokens.append(TokenTree::Ident(Ident::new("RawStr", Span::call_site())));
            }
            Type::NilOr(ty) => {
                tokens.append(TokenTree::Ident(Ident::new("Option", Span::call_site())));
                tokens.append(TokenTree::Punct(Punct::new('<', Spacing::Alone)));
                ty.to_tokens(tokens);
                tokens.append(TokenTree::Punct(Punct::new('>', Spacing::Alone)));
            }
            Type::Tuple(tys) => {
                let mut t = TokenStream::new();
                for ty in tys {
                    ty.to_tokens(&mut t);
                    t.append(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
                }
                tokens.append(Group::new(Delimiter::Parenthesis, t));
            }
            Type::Vec(ty) | Type::Slice(ty) => {
                tokens.append(TokenTree::Ident(Ident::new("Vec", Span::call_site())));
                tokens.append(TokenTree::Punct(Punct::new('<', Spacing::Alone)));
                ty.to_tokens(tokens);
                tokens.append(TokenTree::Punct(Punct::new('>', Spacing::Alone)));
            }
            Type::Array(ty, n) => {
                let mut t = TokenStream::new();
                ty.to_tokens(&mut t);
                t.append(TokenTree::Punct(Punct::new(';', Spacing::Alone)));
                n.to_tokens(&mut t);
                tokens.append(Group::new(Delimiter::Bracket, t));
            }
            Type::Parent | Type::Empty | Type::LuaState => unreachable!(),
        }
    }
}
impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "& str" => Self::Str,
            "& RawStr" => Self::RawStr,
            "LuaState" => Self::LuaState,
            "bool" => Self::Bool,
            "isize" => Self::Isize,
            "f64" => Self::F64,
            "self" => Self::Parent,
            "" => Self::Empty,
            s if let Some(s) = s.strip_prefix("Option < ")
                && let Some(s) = s.strip_suffix(" >") =>
            {
                Self::NilOr(Box::new(Type::from(s)))
            }
            s if let Some(s) = s.strip_prefix("Vec < ")
                && let Some(s) = s.strip_suffix(" >") =>
            {
                Self::Vec(Box::new(Type::from(s)))
            }
            s if let Some(s) = s.strip_prefix("& mut [")
                && let Some(s) = s.strip_suffix("]") =>
            {
                Self::Slice(Box::new(Type::from(s)))
            }
            s if let Some(s) = s.strip_prefix("& [")
                && let Some(s) = s.strip_suffix("]") =>
            {
                Self::Slice(Box::new(Type::from(s)))
            }
            s if let Some(s) = s.strip_prefix("[")
                && let Some(s) = s.strip_suffix("]")
                && let Some((t, n)) = s.rsplit_once("; ") =>
            {
                Self::Array(Box::new(Type::from(t)), n.parse().unwrap())
            }
            s if let Some(s) = s.strip_prefix("(")
                && let Some(s) = s.strip_suffix(")") =>
            {
                Self::Tuple(s.split(", ").map(Type::from).collect())
            }
            _ => panic!("unsupported type {value:?}"),
        }
    }
}
impl From<TokenStream> for Type {
    fn from(value: TokenStream) -> Self {
        Type::from(value.to_string().as_str())
    }
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
                    function.ret = Some(TokenStream::from_iter(mem::take(&mut ret)).into());
                }
                funs.push(mem::take(&mut function));
            }
            TokenTree::Group(g) if is_impl && let Some(name) = impl_name.take() => {
                let (mut funs, _) = parse_group(g.stream());
                for f in &mut funs {
                    f.args.remove(0);
                }
                groups.push(FunGroup { ident: name, funs });
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
                                .push(TokenStream::from_iter(mem::take(&mut arg)).into());
                        }
                        TokenTree::Ident(i) if !start && i != "mut" => {
                            function.arg_names.push(i.to_string());
                        }
                        _ => {}
                    }
                }
                if !arg.is_empty() {
                    function.args.push(TokenStream::from_iter(arg).into());
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
            TokenTree::Ident(i) if is_ret != 3 => {
                impl_name = Some(i);
            }
            t if is_ret == 3 => {
                is_impl = false;
                punct = false;
                ret.push(t);
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
fn parse_attribute(
    mut tokens: TokenStream,
    dont_unload: bool,
    file_path: Option<&str>,
) -> TokenStream {
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
    let luaopen = luaopen(funs, groups, dont_unload, file_path);
    inner_tokens.extend(quote! {use noita_api::lua_function;});
    inner_tokens.extend(luaopen);
    let mut group = Group::new(Delimiter::Brace, TokenStream::from_iter(inner_tokens));
    group.set_span(span.unwrap());
    ret_tokens.push(TokenTree::Group(group));
    TokenStream::from_iter(ret_tokens)
}
fn make_group(group: FunGroup) -> (TokenStream, TokenStream) {
    let ident = group.ident;
    let name = format_ident!("GLOBAL_{}", ident.to_string().to_ascii_uppercase());
    let funs_defs = make_inner_funs(group.funs, Some(&ident));
    let funs = funs_defs.iter().map(|a| a.0.clone());
    let defs = funs_defs.iter().map(|a| a.1.clone());
    (
        quote! {
            static #name: std::cell::SyncUnsafeCell<std::sync::LazyLock<#ident>> = std::cell::SyncUnsafeCell::new(std::sync::LazyLock::new(#ident::default));
            #(#funs)*
        },
        quote! {
            #(#defs)*
        },
    )
}
fn get_str(fun: &Function, name: &str) -> String {
    let mut str = String::new();
    for (ty, name) in fun.args.iter().zip(&fun.arg_names) {
        if ty.put_in_lua() {
            writeln!(str, "---@param {name} {ty}").unwrap();
        }
    }
    if let Some(ret) = &fun.ret {
        writeln!(str, "---@return {ret}").unwrap();
    }
    writeln!(
        str,
        "function {name}.{}({}) end",
        fun.name.as_ref().unwrap(),
        fun.arg_names.join(", ")
    )
    .unwrap();
    str
}
fn create_file<'a>(funs: impl Iterator<Item = &'a Function>, file: &str) {
    let name = file.strip_suffix(".lua").unwrap();
    let (_, name) = name.rsplit_once('/').unwrap();
    let Ok(mut file) = OpenOptions::new()
        .append(false)
        .create(true)
        .truncate(true)
        .write(true)
        .read(false)
        .open(file)
    else {
        return;
    };
    file.write_all(b"helix = {}\n").unwrap();
    for fun in funs {
        let fun = get_str(fun, name);
        file.write_all(fun.as_bytes()).unwrap();
    }
}
fn luaopen(
    funs: Vec<Function>,
    groups: Vec<FunGroup>,
    dont_unload: bool,
    file_path: Option<&str>,
) -> TokenStream {
    if let Some(file) = file_path {
        create_file(
            funs.iter().chain(groups.iter().flat_map(|a| a.funs.iter())),
            file,
        );
    }
    let inner_funs_defs = make_inner_funs(funs, None);
    let inner_funs = inner_funs_defs.iter().map(|a| a.0.clone());
    let inner_defs = inner_funs_defs.iter().map(|a| a.1.clone());
    let keep_loaded = if dont_unload {
        quote! {
            static KEEP_SELF_LOADED: std::sync::OnceLock<noita_api::libloading::Library>
                = std::sync::OnceLock::new();
            KEEP_SELF_LOADED.get_or_init(|| unsafe { noita_api::libloading::Library::new(format!("{}.dll", env!("CARGO_PKG_NAME"))).unwrap() });
        }
    } else {
        quote! {}
    };
    let groups = groups.into_iter().map(make_group).collect::<Vec<_>>();
    let groups_funs = groups.iter().map(|a| a.0.clone());
    let groups_defs = groups.iter().map(|a| a.1.clone());
    let name = quote! {concat!(env!("CARGO_PKG_NAME"), "\0")};
    quote! {
        #[unsafe(no_mangle)]
        unsafe extern "C" fn luaopen(lua: *mut noita_api::lua_bindings::lua_State) -> std::ffi::c_int {
            std::panic::set_hook(Box::new(|panic| noita_api::log_println!("{panic}")));
            #keep_loaded
            #(#groups_funs)*
            #(#inner_funs)*
            fn register_functions(lua: *mut noita_api::lua_bindings::lua_State) {
                unsafe {
                    noita_api::lua_bindings::lua_createtable(lua, 0, 0);
                    #(#inner_defs)*
                    #(#groups_defs)*
                    noita_api::lua_bindings::lua_setfield(
                        lua,
                        noita_api::lua_bindings::LUA_GLOBALSINDEX,
                        #name.as_ptr().cast(),
                    );
                }
            }
            fn newstate() -> *mut noita_api::lua_bindings::lua_State {
                let lua = unsafe { noita_api::NEW_STATE.call() };
                register_functions(lua);
                lua
            }
            noita_api::install_global(newstate);
            register_functions(lua);
            0
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
    let mut arg_iter = arg.into_iter();
    let dont_unload = if let Some(TokenTree::Ident(ident)) = arg_iter.next() {
        ident == "true"
    } else {
        false
    };
    arg_iter.next();
    let file_path = if let Some(TokenTree::Literal(l)) = arg_iter.next()
        && let Some(s) = l.to_string().strip_prefix("\"")
        && let Some(s) = s.strip_suffix("\"")
    {
        Some(s.to_owned())
    } else {
        None
    };
    parse_attribute(tokens.into(), dont_unload, file_path.as_deref()).into()
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
    for token in tokens.clone() {
        match token {
            TokenTree::Ident(ident)
                if matches!(ident.to_string().as_str(), "struct" | "enum" | "union") =>
            {
                expect_name = true;
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
    let t: TokenStream = t.iter().cloned().collect();
    let arg: TokenStream = arg.iter().cloned().collect();
    let tokens: TokenStream = tokens.into();
    let mut struct_name = None;
    let mut expect_name = false;
    for token in tokens.clone() {
        match token {
            TokenTree::Ident(ident)
                if matches!(ident.to_string().as_str(), "struct" | "enum" | "union") =>
            {
                expect_name = true;
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
pub fn generate_globals(
    _: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tokens: TokenStream = tokens.into();
    let mut new = Vec::new();
    new.extend(tokens);
    let Some(TokenTree::Group(g)) = new.pop() else {
        unreachable!()
    };
    let mut group = Vec::new();
    for token in g.stream() {
        if let TokenTree::Ident(i) = &token
            && i == "const"
        {
            group.extend(quote! {
                #[noita_api_macros::generate_global]
            });
        }
        group.push(token);
    }
    new.push(TokenTree::Group(Group::new(
        Delimiter::Brace,
        group.into_iter().collect(),
    )));
    let tokens: TokenStream = new.into_iter().collect();
    quote! {
        #tokens
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
    for token in tokens.clone() {
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
            TokenTree::Group(g) if is_ptr || is_ptr_ptr => {
                type_name.push(TokenTree::Group(g));
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
    let global_type = get_global_type(&global_const.unwrap(), &type_name, is_ptr_ptr);
    quote! {
        #tokens
        #global_type
    }
    .into()
}
fn get_global_type(global_const: &Ident, type_name: &TokenStream, is_ptr_ptr: bool) -> TokenStream {
    let ptr_read = if is_ptr_ptr {
        quote! {unsafe{#global_const.read()}}
    } else {
        quote! {#global_const}
    };
    quote! {
        impl #type_name {
            #[inline]
            pub fn global() -> StdBox<Self> {
                StdBox::from(#ptr_read)
            }
        }
    }
}
fn add_lua_fn(fun: Function, struct_ident: Option<&Ident>) -> (TokenStream, TokenStream) {
    let ident = fun.name.unwrap();
    let bridge_fn_name = format_ident!("{ident}_lua_bridge");
    let fn_name_c = name_to_c_literal(&ident.to_string());
    let vars: Vec<_> = fun
        .args
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(i, ts)| {
            let ident = format_ident!("a{}", i);
            if ts.put_in_lua() {
                Some(quote! {
                    let val: Result<(i32, #ts), noita_api::lua::LuaError> = noita_api::lua::LuaGetValue::get(lua_state, index);
                    let (index, mut #ident) = match val {
                        Ok(v) => v,
                        Err(err) => lua_state.raise_error(format!("Error in rust call: {err:?}")),
                    };
                })
            } else if matches!(ts, Type::LuaState){
                Some(quote! {let #ident = noita_api::lua::LuaState::new(lua);})
            } else {
                None
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
        .filter_map(|(i, t)| {
            if t.put_in_lua() || matches!(t, Type::LuaState) {
                let ident = format_ident!("a{}", i);
                Some(if t.make_ref() {
                    quote! {
                        &mut #ident
                    }
                } else {
                    quote! {
                        #ident
                    }
                })
            } else {
                None
            }
        })
        .collect();
    let ret = if let Some(struct_ident) = struct_ident {
        let name = format_ident!("GLOBAL_{}", struct_ident.to_string().to_ascii_uppercase());
        quote! {
            let ret = unsafe{#struct_ident::#ident(#name.get().as_mut().unwrap(), #(#args,)*)};
        }
    } else {
        quote! {
            let ret = #ident(#(#args,)*);
        }
    };
    (
        quote! {
            unsafe extern "C" fn #bridge_fn_name(lua: *mut noita_api::lua_bindings::lua_State) -> std::ffi::c_int {
                let lua_state = noita_api::lua::LuaState::new(lua);
                #index
                #(#vars)*
                #ret
                let ret = noita_api::lua::LuaFnRet::do_return(ret, lua_state);
                ret
            }
        },
        quote! {
            noita_api::lua_bindings::lua_pushcclosure(lua, Some(#bridge_fn_name), 0);
            noita_api::lua_bindings::lua_setfield(lua, -2, #fn_name_c.as_ptr());
        },
    )
}
fn name_to_c_literal(name: &str) -> Literal {
    Literal::c_string(CString::new(name).unwrap().as_c_str())
}
fn make_inner_funs(
    idents: Vec<Function>,
    ident: Option<&Ident>,
) -> Vec<(TokenStream, TokenStream)> {
    let mut inner_funs = Vec::new();
    for fun in idents {
        inner_funs.push(add_lua_fn(fun, ident));
    }
    inner_funs
}
fn make_lua_get_tuple(n: usize) -> TokenStream {
    let generics = (0..n)
        .map(|i| format_ident!("T{i}"))
        .map(|i| quote! {#i: LuaGetValue});
    let tuple = (0..n).map(|i| format_ident!("T{i}")).map(|i| quote! {#i});
    let res = (0..n)
        .map(|i| (format_ident!("T{i}"), format_ident!("t{i}")))
        .map(|(ty, n)| quote! {let (index, #n) = #ty::get(lua, index)?;});
    let ret = (0..n).map(|i| format_ident!("t{i}")).map(|n| quote! {#n});
    quote! {
        impl<#(#generics,)*> LuaGetValue for (#(#tuple,)*) {
            #[inline]
            fn get(lua: LuaState, index: i32) -> Result<(i32, Self), LuaError>
            {
                #(#res)*
                Ok((index, (#(#ret,)*)))
            }
        }
    }
}
#[proc_macro]
pub fn make_lua_get_tuples(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: TokenStream = tokens.into();
    let TokenTree::Literal(n) = tokens.into_iter().next().unwrap() else {
        unreachable!()
    };
    let n: usize = n.to_string().parse().unwrap();
    let tuple = (2..=n).map(make_lua_get_tuple);
    quote! {
        #(#tuple)*
    }
    .into()
}
fn make_lua_ret_tuple(n: usize) -> TokenStream {
    let generics = (0..n)
        .map(|i| format_ident!("T{i}"))
        .map(|i| quote! {#i: LuaFnRet});
    let tuple = (0..n).map(|i| format_ident!("T{i}")).map(|i| quote! {#i});
    let res = (0..n).map(|i| (format_ident!("T{i}"), i)).map(|(ty, n)| {
        let i = Literal::usize_unsuffixed(n);
        quote! {#ty::do_return(self.#i, lua)}
    });
    quote! {
        impl<#(#generics,)*> LuaFnRet for (#(#tuple,)*) {
            #[inline]
            fn do_return(self, lua: LuaState) -> c_int {
                #(#res+)*0
            }
        }
    }
}
#[proc_macro]
pub fn make_lua_ret_tuples(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: TokenStream = tokens.into();
    let TokenTree::Literal(n) = tokens.into_iter().next().unwrap() else {
        unreachable!()
    };
    let n: usize = n.to_string().parse().unwrap();
    let tuple = (2..=n).map(make_lua_ret_tuple);
    quote! {
        #(#tuple)*
    }
    .into()
}
#[proc_macro_attribute]
pub fn gen_stubs(
    _: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tokens = TokenStream::from(tokens);
    let group = tokens
        .clone()
        .into_iter()
        .find_map(|a| {
            if let TokenTree::Group(g) = a
                && g.delimiter() == Delimiter::Brace
            {
                Some(g)
            } else {
                None
            }
        })
        .unwrap();
    let vec: Vec<TokenTree> = group.stream().into_iter().collect();
    let mut stubs: Vec<TokenStream> = vec
        .split(|t| {
            if let TokenTree::Punct(p) = t {
                p.as_char() == ';'
            } else {
                false
            }
        })
        .map(|t| {
            let t: TokenStream = t.iter().skip(1).cloned().collect();
            quote! {
                #[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
                #[allow(unused)]
                pub unsafe extern "C" #t {
                    unreachable!()
                }
            }
        })
        .collect();
    stubs.pop();
    quote! {
        #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
        #tokens
        #(#stubs)*
    }
    .into()
}
fn search_data(tokens: TokenStream) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut ignore = false;
    let mut is_var = false;
    let mut is_wildcard = false;
    let mut count = 1;
    let mut cursor = 0;
    let mut rets = Vec::new();
    let mut tokens: Vec<TokenStream> = tokens
        .into_iter()
        .filter_map(|p| match p {
            TokenTree::Punct(p) if p.as_char() == '?' && !ignore => {
                count = 1;
                is_var = false;
                is_wildcard = true;
                ignore = true;
                None
            }
            TokenTree::Punct(p) if p.as_char() == '!' && !ignore => {
                count = 1;
                is_var = true;
                is_wildcard = true;
                ignore = true;
                None
            }
            TokenTree::Literal(l) if is_wildcard => {
                count = l.to_string().parse().unwrap();
                None
            }
            TokenTree::Literal(l) => {
                cursor += 1;
                Some(quote! {crate::search::Token::Byte(#l),})
            }
            TokenTree::Punct(p) if p.as_char() == ',' => {
                ignore = false;
                if is_wildcard {
                    is_wildcard = false;
                    if is_var {
                        is_var = false;
                        rets.push((cursor, count));
                    }
                    cursor += count;
                    let any = iter::repeat_n(quote! {crate::search::Token::Any}, count);
                    Some(quote! {#(#any,)*})
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();
    if is_wildcard {
        if is_var {
            rets.push((cursor, count));
        }
        tokens.push(quote! {crate::search::Token::Any,});
    }
    let rets = rets.into_iter().map(|(cursor, size)| {
        quote! {
            unsafe{std::mem::transmute(ptr.add(#cursor).cast::<[u8; #size]>().read())}
        }
    });
    (tokens, rets.collect())
}
#[proc_macro]
pub fn search(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: TokenStream = tokens.into();
    let (tokens, rets) = search_data(tokens);
    if rets.is_empty() {
        quote! {
            {
                crate::search::search([#(#tokens)*])
            }
        }
        .into()
    } else {
        quote! {
            {
                let ptr = crate::search::search([#(#tokens)*]);
                (ptr,#(#rets,)*)
            }
        }
        .into()
    }
}
#[proc_macro]
pub fn search_fun(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: TokenStream = tokens.into();
    let (tokens, rets) = search_data(tokens);
    if rets.is_empty() {
        quote! {
            {
                crate::search::get_function(crate::search::search([#(#tokens)*]))
            }
        }
        .into()
    } else {
        quote! {
            {
                let ptr = crate::search::search([#(#tokens)*]);
                (crate::search::get_function(ptr),#(#rets,)*)
            }
        }
        .into()
    }
}
