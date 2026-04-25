local mod = {}
local gui = GuiCreate()
local text = ""
local in_chat = false
local function set_controls(player)
    local controls = EntityGetFirstComponentIncludingDisabled(player, "ControlsComponent")
    local gui_component = EntityGetFirstComponentIncludingDisabled(player, "InventoryGuiComponent")
    if in_chat then
        EntitySetComponentIsEnabled(player, gui_component, false)
        ComponentSetValue2(controls, "enabled", false)
        ComponentSetValue2(controls, "mButtonDownFire", false)
        ComponentSetValue2(controls, "mButtonDownFire2", false)
        ComponentSetValue2(controls, "mButtonDownLeft", false)
        ComponentSetValue2(controls, "mButtonDownDown", false)
        ComponentSetValue2(controls, "mButtonDownRight", false)
        ComponentSetValue2(controls, "mButtonDownUp", false)
        ComponentSetValue2(controls, "mButtonDownJump", false)
        ComponentSetValue2(controls, "mButtonDownFly", false)
        ComponentSetValue2(controls, "mButtonDownKick", false)
        ComponentSetValue2(controls, "mButtonDownEat", false)
    else
        EntitySetComponentIsEnabled(player, gui_component, true)
        ComponentSetValue2(controls, "enabled", true)
    end
end
function mod.update(helix, player)
    if InputIsKeyJustDown(40) then
        if text ~= "" then
            helix.text_msg(text)
            text = ""
        end
        in_chat = not in_chat
        set_controls(player)
    end
    if in_chat then
        GuiStartFrame(gui)
        local x, y = InputGetMousePosOnScreen()
        x, y = x / 2, y / 2
        local new = GuiTextInput(gui, 421, x - 4, y - 6, " ", 8, 16)
        if new == "" then
            text = string.sub(text, 0, -2)
        elseif new ~= " " then
            text = text..string.sub(new, 2, -1)
        end
        GuiText(gui, 32, 256, text)
    end
end
return mod