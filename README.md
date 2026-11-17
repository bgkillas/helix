this was meant to be the successor to noita entangled worlds, but i never finished it or made documentation due to crashes which i could not determine the cause of.

noita_api_macros main macro turns code like

```rust
#[noita_api::lua_module]
mod lua {
    #[lua_function]
    fn foo(...) -> ... {...}
}
```

into code that when loaded via lua, the returned object after loading can call this function foo with its arguments and return whatever type it specifies,
this supports most reasonable types that have a lua counterpart.

in a lua_module you also have certain hooks like instead of using #[lua_function] you can have #[fire_hook], which hooks into the games function call for shooting a wand with your own function which calls the corrosponding hooked function https://github.com/bgkillas/helix/blob/master/helix/src/lib.rs#L105, so you can do arbitrary operations with this data

if you specify a hook or lua_function in a impl of a struct this acts on a global which initiates the Default of the struct and calls with the global the corrosponding hooks/lua functions. https://github.com/bgkillas/helix/blob/master/helix/src/lib.rs#L44

to hook functions i take a section of the assembly found from ghidra and create a pattern which is used to find the location at runtime to prevent losing data between updates/versions so easily, then when we arrive at the pattern we find the containing function by looking for function call asm, so the macro search_fun![0x80, 0xbf, ???2, 0x00] searches through allocated blocks of memory for this pattern of bytes then finds the address of the containing function, ???N means N bytes that are of any value, you can also search for strings and put pointers to search for, also let (ptr, ret) = search!(0x80, !var) can put the byte in the location of var in ret 

there are also some hacks for calling conventions that dont exist being made from the compiler so some naked_asm is used for that
