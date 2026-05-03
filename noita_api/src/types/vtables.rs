use std::marker::PhantomData;
#[repr(C)]
#[derive(Debug)]
pub struct ComponentBufferVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct CellVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct ConfigExplosionVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct ConfigDamageCriticalVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct ConfigGridCosmeticParticleVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct EntityManagerVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct EventManagerVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct BiomeModifiersVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct GridWorldVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct GridWorldThreadedVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct PlatformVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct AppConfigVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct ComponentVTable<T> {
    phantom: PhantomData<T>,
}
#[repr(C)]
#[derive(Debug)]
pub struct GlobalStatsVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct GameStatsVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct TranslationManagerVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct ComponentBufferInitVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct ComponentUpdaterVTable {}
#[repr(C)]
#[derive(Debug)]
pub struct ComponentSystemVTable {}
#[derive(Debug)]
#[repr(C)]
pub struct ModVTable {}
