local helix = package.loadlib("./mods/helix/helix.dll", "luaopen")()
function OnWorldPreUpdate()
    helix.update()
end
function OnWorldPostUpdate()
    helix.post_update()
end
function OnWorldInitialized()
    helix.init()
end
