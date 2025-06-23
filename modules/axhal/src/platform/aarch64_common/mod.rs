mod boot;
pub mod generic_timer;
pub mod pl061;
#[cfg(not(platform_family = "aarch64-raspi"))]
pub mod psci;

#[cfg(feature = "irq")]
pub mod gic;

#[cfg(not(platform_family = "aarch64-bsta1000b"))]
pub mod pl011;
