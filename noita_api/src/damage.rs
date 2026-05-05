use crate::{Entity, StdBox, Vec2, fast_call, get_fast_call, log_println};
use noita_api_macros::{search, search_fun};
#[repr(C)]
#[derive(Debug)]
pub struct DamageModel {}
#[repr(C)]
#[derive(Debug)]
struct DamageThing {
    pub entity: Option<StdBox<Entity>>,
    pub damage_by_type: [u8; 0x40],
    pub impulse: Vec2,
    pub world_pos: Vec2,
    pub knockback_force: f32,
    pub entity_thats_responsible: usize,
    pub entity_type_id: usize,
    unknown1: usize,
    pub projectile_thats_responsible: usize,
    pub ragdoll_fx: usize,
    unknown2: usize,
    /*pub vs13::string ragdoll_entity_file,
    pub vs13::string str_tags_q,
    unknown3: usize,
    unknown4: usize,
    pub float blood_multiplier,*/
}
pub type DamageFun = fast_call!(
    fn(Option<StdBox<Entity>>, Option<StdBox<DamageModel>>, *const u8, usize, StdBox<DamageThing>)
);
static RAW: std::sync::atomic::AtomicPtr<retour::RawDetour> = std::sync::atomic::AtomicPtr::null();
#[inline]
pub fn install_damage_function() {
    let ptr = search!("TakeDamage_Impl() - DamageModelComponent couldn't be found");
    let fun_addr = search_fun!(0x68, ptr);
    unsafe {
        let fun = get_fast_call!(
            fun_addr as usize,
            fn(
                Option<StdBox<Entity>>,
                Option<StdBox<DamageModel>>,
                *const u8,
                usize,
                StdBox<DamageThing>,
            )
        );
        let raw = retour::RawDetour::new(fun as *const (), hook as *const ()).unwrap();
        raw.enable().unwrap();
        RAW.store(
            Box::leak(Box::new(raw)),
            std::sync::atomic::Ordering::Relaxed,
        );
    }
}
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
fn get_ptr() -> *const () {
    unsafe {
        RAW.load(std::sync::atomic::Ordering::Relaxed)
            .as_ref()
            .unwrap()
    }
    .trampoline() as *const ()
}
pub fn fun(
    orig: DamageFun,
    entity: Option<StdBox<Entity>>,
    damage_model: Option<StdBox<DamageModel>>,
    description: *const u8,
    damage_types: usize,
    damage_args: StdBox<DamageThing>,
) {
    log_println!(
        "pre: {:?} {:?} {:?} {:?} {:?}",
        entity,
        damage_model,
        description,
        damage_types,
        damage_args
    );
    orig(entity, damage_model, description, damage_types, damage_args);
    log_println!(
        "post: {:?} {:?} {:?} {:?} {:?}",
        entity,
        damage_model,
        description,
        damage_types,
        damage_args
    );
}
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
#[allow(clippy::too_many_arguments)]
extern "fastcall" fn on_fire_inner(
    entity: Option<StdBox<Entity>>,
    damage_model: Option<StdBox<DamageModel>>,
    description: *const u8,
    damage_types: usize,
    damage_args: StdBox<DamageThing>,
) {
    fun(
        crate::call_orig_damage,
        entity,
        damage_model,
        description,
        damage_types,
        damage_args,
    );
}
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
#[unsafe(naked)]
pub extern "fastcall" fn call_orig_damage(
    _entity: Option<StdBox<Entity>>,
    _damage_model: Option<StdBox<DamageModel>>,
    _description: *const u8,
    _damage_types: usize,
    _damage_args: StdBox<DamageThing>,
) {
    std::arch::naked_asm!(
        "push ebp",
        "mov ebp,esp",
        "push [ebp+0x10]",
        "push [ebp+0x0c]",
        "push [ebp+0x08]",
        "call {get_ptr}",
        "call eax",
        "mov esp,ebp",
        "pop ebp",
        "ret 0x0c",
        get_ptr = sym get_ptr,
    )
}
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
#[unsafe(naked)]
pub extern "fastcall" fn hook(
    _entity: Option<StdBox<Entity>>,
    _damage_model: Option<StdBox<DamageModel>>,
    _description: *const u8,
    _damage_types: usize,
    _damage_args: StdBox<DamageThing>,
) {
    std::arch::naked_asm!(
        "push ebp",
        "mov ebp,esp",
        "push [ebp+0x10]",
        "push [ebp+0x0c]",
        "push [ebp+0x08]",
        "call {on_fire_inner}",
        "mov esp,ebp",
        "pop ebp",
        "ret",
        on_fire_inner = sym on_fire_inner,
    )
}
