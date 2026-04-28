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
function OnMagicNumbersAndWorldSeedInitialized()
    helix.world_seed_init()
end
function OnPausePreUpdate()
    helix.on_pause()
end
function OnPlayerSpawned(entity)
    player = entity
end
function OnPausedChanged(paused, is_wand_pickup)
    helix.on_paused_change(paused, is_wand_pickup)
end
