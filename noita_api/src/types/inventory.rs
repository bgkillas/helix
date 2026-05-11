use crate::{Component, Entity, StdVec};
#[repr(C)]
#[derive(Debug)]
pub struct Inventory {
    pub entity: Option<Entity>,
    pub inventory_quick: Option<Entity>,
    pub inventory_full: Option<Entity>,
    pub held_item_id: usize,
    pub switch_item_id: isize,
    pub inventory_component: Option<Component<()>>,
    unk7b1: bool,
    pub item_placed: bool,
    unk7b3: bool,
    padding: u8,
    pub item_in_pickup_range: bool,
    padding2: u8,
    padding3: u8,
    padding4: u8,
    pub is_in_inventory: bool,
    unk9b2: bool,
    pub is_dragging: bool,
    padding5: u8,
    unk10: StdVec<isize>,
    pub pickup_state: usize,
    pub wand_pickup: Option<Entity>,
    pub animation_state: usize,
    unk15: StdVec<[isize; 18]>,
}
impl Default for Inventory {
    #[inline]
    fn default() -> Self {
        todo!()
    }
}
