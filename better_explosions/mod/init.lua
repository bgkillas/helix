package.loadlib("mods/better_explosions/better_explosions.dll", "luaopen")()
function OnWorldPreUpdate()
    better_explosions.update()
end