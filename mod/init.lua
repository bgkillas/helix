local helix = package.loadlib("mods/helix/helix.dll", "luaopen")()
local text = dofile_once("mods/helix/text.lua")
local player = 0
function OnWorldPreUpdate()
    text.update(helix, player)
    helix.update()
end
function OnModPreInit()
    helix.init()
end
function OnWorldPostUpdate()
    helix.post_update()
end
function OnWorldInitialized()
    helix.world_init()
end
function OnMagicNumbersAndWorldSeedInitialized()
    helix.world_seed_init()
end
function OnPausePreUpdate()
    helix.on_pause()
end
function OnPlayerSpawned(entity)
    player = entity
end
