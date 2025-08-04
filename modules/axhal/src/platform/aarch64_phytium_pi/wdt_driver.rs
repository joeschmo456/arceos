use super::watchdog::*;
use crate::platform::aarch64_common::gic::{WATCHDOG_IRQ_NUM, register_handler};
use ::memory_addr::PhysAddr;
use kspin::SpinNoIrq;

const WDT_BASE: PhysAddr = PhysAddr::from_usize(0x000_2804_0000);
static WATCHDOG: SpinNoIrq<WatchDog> =
    SpinNoIrq::new(WatchDog::new(WDT_BASE.as_usize() as *mut u8));

pub fn watchdog_example() {
    info!("Watchdog example");
    crate::irq::set_enable(WATCHDOG_IRQ_NUM, true);
    register_handler(WATCHDOG_IRQ_NUM, handle_wdt_irq);

    info!("Watchdog initializing");
    WATCHDOG.lock().init_watchdog();

    info!("Watchdog started");
    WATCHDOG.lock().enable_watchdog();

    for _i in 0..10 {
        info!("Watchdog feeding");
        WATCHDOG.lock().feed_watchdog();
        info!("Watchdog fed");
        core::hint::spin_loop();
    }
}

pub fn handle_wdt_irq() {
    debug!("WatchDog IRQ triggered");
}
