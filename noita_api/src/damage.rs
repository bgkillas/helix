use crate::{Entity, StdBox, StdString, Vec2, fast_call};
use noita_api_macros::{search, search_fun};
use retour::{Function as _, RawDetour};
use std::sync::OnceLock;
#[repr(C)]
#[derive(Debug)]
pub struct DamageModel {}
#[repr(C)]
#[derive(Debug)]
pub struct DamageThing {
    pub entity: Option<Entity>,
    pub damage_by_type: [u8; 0x40],
    pub impulse: Vec2<f32>,
    pub world_pos: Vec2<f32>,
    pub knockback_force: f32,
    pub entity_thats_responsible: usize,
    pub entity_type_id: usize,
    unknown1: usize,
    pub projectile_thats_responsible: usize,
    pub ragdoll_fx: usize,
    unknown2: usize,
    pub ragdoll_entity_file: StdString,
    pub str_tags_q: StdString,
    unknown3: usize,
    unknown4: usize,
    pub blood_multiplier: f32,
}
pub type DamageFun = fast_call!(
    fn(
        Option<Entity>,
        Option<StdBox<DamageModel>>,
        StdBox<StdString>,
        usize,
        StdBox<DamageThing>,
        f32,
    )
);
static RAW: OnceLock<RawDetour> = OnceLock::new();
#[inline]
#[allow(clippy::as_conversions)]
pub fn install_damage_function_manual(
    damage_fun_hook: fast_call!(
        fn(
            Option<Entity>,
            Option<StdBox<DamageModel>>,
            StdBox<StdString>,
            usize,
            StdBox<DamageThing>,
        )
    ),
) {
    if RAW.get().is_some() {
        return;
    }
    let ptr = search!("TakeDamage_Impl() - DamageModelComponent couldn't be found");
    // 0x0103_4ad0
    let fun_addr = search_fun!(0x68, ptr);
    unsafe {
        let raw = RawDetour::new(fun_addr.cast(), damage_fun_hook.to_ptr()).unwrap();
        raw.enable().unwrap();
        RAW.set(raw).unwrap();
    }
}
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
fn get_ptr() -> *const () {
    std::ptr::from_ref(RAW.get().unwrap().trampoline())
}
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
#[unsafe(naked)]
pub extern "fastcall" fn damage_fun(
    _entity: Option<Entity>,
    _damage_model: Option<StdBox<DamageModel>>,
    _description: StdBox<StdString>,
    _damage_types: usize,
    _damage_args: StdBox<DamageThing>,
    _damage: f32,
) {
    std::arch::naked_asm!(
        "push ebp",
        "mov ebp,esp",
        "movss xmm2,[ebp+0x14]",
        "push [ebp+0x10]",
        "push [ebp+0x0c]",
        "push [ebp+0x08]",
        "call {get_ptr}",
        "call eax",
        "mov esp,ebp",
        "pop ebp",
        "ret 0x10",
        get_ptr = sym get_ptr,
    )
}
#[macro_export]
macro_rules! install_damage_function {
    ($fun:path) => {
        #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
        #[allow(clippy::too_many_arguments)]
        extern "fastcall" fn on_damage_inner(
            entity: Option<$crate::Entity>,
            damage_model: Option<$crate::StdBox<$crate::DamageModel>>,
            description: $crate::StdBox<$crate::StdString>,
            damage_types: usize,
            damage_args: $crate::StdBox<$crate::DamageThing>,
            damage: f32,
        ) {
            $fun(
                $crate::damage_fun,
                entity,
                damage_model,
                description,
                damage_types,
                damage_args,
                damage,
            );
        }
        #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
        #[unsafe(naked)]
        pub extern "fastcall" fn damage_fun_hook(
            _entity: Option<$crate::Entity>,
            _damage_model: Option<$crate::StdBox<$crate::DamageModel>>,
            _description: $crate::StdBox<$crate::StdString>,
            _damage_types: usize,
            _damage_args: $crate::StdBox<$crate::DamageThing>,
        ) {
            std::arch::naked_asm!(
                "push ebp",
                "mov ebp,esp",
                "sub esp,0x04",
                "movss [esp],xmm2",
                "push [ebp+0x10]",
                "push [ebp+0x0c]",
                "push [ebp+0x08]",
                "call {on_damage_inner}",
                "mov esp,ebp",
                "pop ebp",
                "ret",
                on_damage_inner = sym on_damage_inner,
            )
        }
        #[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
        {
            _ = $fun;
        }
        #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
        $crate::install_damage_function_manual(damage_fun_hook)
    }
}
