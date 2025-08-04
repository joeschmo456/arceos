use core::ptr::NonNull;
use tock_registers::{interfaces::Writeable, register_structs, registers::ReadWrite};

register_structs! {
    pub WatchDogRegs {
        (0x0000 => pub wdt_wrr: ReadWrite<u32>), // 看门狗更新寄存器
        (0x0004 => _reserved1),
        (0x0fcc => pub wdt_w_iidr: ReadWrite<u32>), // 看门狗接口身份识别寄存器
        (0x0fd0 => _reserved2),
        (0x1000 => pub wdt_wcs: ReadWrite<u32>), // 看门狗控制和状态寄存器
        (0x1004 => _reserved3),
        (0x1008 => pub wdt_wor: ReadWrite<u32>), // 看门狗清除寄存器
        (0x100c => _reserved4),
        (0x1010 => pub wdt_wcvl: ReadWrite<u32>), // 看门狗比较值低 32 位寄存器
        (0x1014 => pub wdt_wcvh: ReadWrite<u32>), // 看门狗比较值高 32 位寄存器
        (0x1018 => @END),
    }
}

pub struct WatchDog {
    base: NonNull<WatchDogRegs>,
}

unsafe impl Send for WatchDog {}
unsafe impl Sync for WatchDog {}

impl WatchDog {
    pub const fn new(base: *mut u8) -> Self {
        Self {
            base: NonNull::new(base).unwrap().cast(),
        }
    }

    const fn regs(&self) -> &WatchDogRegs {
        unsafe { self.base.as_ref() }
    }

    pub fn init_watchdog(&mut self) {
        info!("Initializing WatchDog...");
        self.regs().wdt_wcs.set(0x1);
    }

    pub fn set_watchdog_timeout(&self, timeout: u32) {
        info!(
            "Setting WatchDog timeout to: 0x{:08X} ({} cycles)",
            timeout, timeout
        );
        self.regs().wdt_wor.set(timeout);
    }
    pub fn enable_watchdog(&self) {
        info!("Enabling WatchDog...");
        self.regs().wdt_wcs.set(0x1);
    }

    pub fn disable_watchdog(&self) {
        info!("Disabling WatchDog...");
        self.regs().wdt_wcs.set(0x0);
    }
    pub fn feed_watchdog(&self) {
        info!("Feeding watchdog");
        self.regs().wdt_wrr.set(0x1);
    }
}
