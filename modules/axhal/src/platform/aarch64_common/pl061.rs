use memory_addr::PhysAddr;
use tock_registers::{
    registers::{ReadOnly, ReadWrite, WriteOnly},
    register_bitfields, register_structs,
};
use tock_registers::interfaces::{Readable, Writeable};
use crate::platform::aarch64_common::psci::system_off as terminate;

use crate::mem::phys_to_virt;
use core::arch::asm;

const GPIO_BASE: PhysAddr = pa!(axconfig::devices::GPIO_PADDR);
pub const pl061_regs: *mut PL061Regs = phys_to_virt(GPIO_BASE).as_usize() as *mut PL061Regs;

register_bitfields! [
    u32,

    GPIODIR [
        Pin0 OFFSET(0) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
    ],

    GPIOIS [
        Pin0 OFFSET(0) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
    ],

    GPIOIBE [
        Pin0 OFFSET(0) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
    ],

    GPIOIEV [
        Pin0 OFFSET(0) NUMBITS(1) [
            FallingLow = 0,  
            RisingHigh = 1,
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
    ],

    GPIOIE [
        Pin0 OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
    ],

    GPIORIS [
        Pin0 OFFSET(0) NUMBITS(1) [],
        Pin1 OFFSET(1) NUMBITS(1) [],
        Pin2 OFFSET(2) NUMBITS(1) [],
        Pin3 OFFSET(3) NUMBITS(1) [],
        Pin4 OFFSET(4) NUMBITS(1) [],
        Pin5 OFFSET(5) NUMBITS(1) [],
        Pin6 OFFSET(6) NUMBITS(1) [],
        Pin7 OFFSET(7) NUMBITS(1) [],
    ],

    GPIOMIS [
        Pin0 OFFSET(0) NUMBITS(1) [],
        Pin1 OFFSET(1) NUMBITS(1) [],
        Pin2 OFFSET(2) NUMBITS(1) [],
        Pin3 OFFSET(3) NUMBITS(1) [],
        Pin4 OFFSET(4) NUMBITS(1) [],
        Pin5 OFFSET(5) NUMBITS(1) [],
        Pin6 OFFSET(6) NUMBITS(1) [],
        Pin7 OFFSET(7) NUMBITS(1) [],
    ],

    GPIOIC [
        Pin0 OFFSET(0) NUMBITS(1) [],
        Pin1 OFFSET(1) NUMBITS(1) [],
        Pin2 OFFSET(2) NUMBITS(1) [],
        Pin3 OFFSET(3) NUMBITS(1) [],
        Pin4 OFFSET(4) NUMBITS(1) [],
        Pin5 OFFSET(5) NUMBITS(1) [],
        Pin6 OFFSET(6) NUMBITS(1) [],
        Pin7 OFFSET(7) NUMBITS(1) [],
    ],

    GPIOAFSEL [
        Pin0 OFFSET(0) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
    ],
];


register_structs! {
    pub PL061Regs {
        (0x000 => data: [ReadWrite<u32>; 256]),
        (0x400 => dir: ReadWrite<u32, GPIODIR::Register>),
        (0x404 => is: ReadWrite<u32, GPIOIS::Register>),
        (0x408 => ibe: ReadWrite<u32, GPIOIBE::Register>),
        (0x40c => iev: ReadWrite<u32, GPIOIEV::Register>),
        (0x410 => ie: ReadWrite<u32, GPIOIE::Register>),
        (0x414 => ris: ReadOnly<u32, GPIORIS::Register>),
        (0x418 => mis: ReadOnly<u32, GPIOIS::Register>),
        (0x41c => ic: WriteOnly<u32, GPIOIC::Register>),
        (0x420 => afsel: ReadWrite<u32, GPIOAFSEL::Register>),
        (0x424 => @END),
    }
}

pub fn init() {
    #[cfg(feature = "irq")]
    const GPIO_IRQ: usize = axconfig::devices::GPIO_IRQ;
    // 使能GPIO中断
    info!("Enabling GPIO interrupt");
    crate::irq::set_enable(GPIO_IRQ, true);
    // 注册GPIO中断处理函数
    info!("Registering GPIO interrupt handler");
    crate::irq::register_handler(GPIO_IRQ, handle);

    // 设置GPIO引脚为输出模式
    info!("Setting GPIO pin 3 to output mode");
    let gpio_regs = unsafe { &mut *(pl061_regs) };
    gpio_regs.ie.write(GPIOIE::Pin3::SET);
    let status = gpio_regs.ie.get();
}

fn handle() {
    let gpio_regs = unsafe { &mut *(pl061_regs) };

    gpio_regs.ie.write(GPIOIE::Pin3::CLEAR);
    gpio_regs.ic.set(gpio_regs.ie.get());
    
    // 关机
    info!("GPIO poweroff");
    unsafe {
        asm!("mov w0, #0x18");
        asm!("hlt #0xF000");
    }
}