use crate::mem::{PhysAddr, phys_to_virt};
use core::ptr::NonNull;
use kspin::SpinNoIrq;
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

pub const BAUD_RATE: u32 = 115200;
pub const CLOCK_HZ: u32 = 100_000_000;

// UART2 base address
pub const UART_BASE: PhysAddr = pa!(0x2800_E000);

pub static UART: SpinNoIrq<Uart> = SpinNoIrq::new(Uart::new(phys_to_virt(UART_BASE).as_mut_ptr()));

register_structs! {
    UartRegs {
        (0x000 => uartdr: ReadWrite<u32, UARTDR::Register>), // 数据寄存器
        (0x004 => uartrsr: ReadWrite<u32, UARTRSR::Register>), // 接收状态寄存器
        (0x008 => _reserved1: [u8; 0x10]),
        (0x018 => uartfr: ReadWrite<u32, UARTFR::Register>), // 标志寄存器
        (0x01c => _reserved2: [u8; 0x4]),
        (0x020 => uartilpr: ReadWrite<u32, UARTILPR::Register>), // 低功耗计数寄存器
        (0x024 => uartibrd: ReadWrite<u32, UARTIBRD::Register>), // 波特率整数值配置寄存器
        (0x028 => uartfbrd: ReadWrite<u32, UARTFBRD::Register>), // 波特率小数值配置寄存器
        (0x02C => uartlcr_h: ReadWrite<u32, UARTLCR_H::Register>), // 线控寄存器
        (0x030 => uartcr: ReadWrite<u32, UARTCR::Register>), // 控制寄存器
        (0x034 => uartifls: ReadWrite<u32, UARTIFLS::Register>), // FIFO阈值选择寄存器
        (0x038 => uartimsc: ReadWrite<u32, UARTIMSC::Register>), // 中断屏蔽选择
        (0x03C => uartris: ReadWrite<u32, UARTRIS::Register>), // 中断状态寄存器
        (0x040 => uartmis: ReadWrite<u32, UARTMIS::Register>), // 中断屏蔽状态寄存器
        (0x044 => uarticr: ReadWrite<u32, UARTICR::Register>), // 中断清除寄存器
        (0x048 => uartdmacr: ReadWrite<u32, UARTDMACR::Register>), // DMA控制寄存器
        (0x04C => @END),
    }
}

register_bitfields![u32,
    pub UARTDR [
        data OFFSET(0) NUMBITS(8) [], // 数据
        fe OFFSET(8) NUMBITS(1) [],   // 奇偶校验错误
        pe OFFSET(9) NUMBITS(1) [],  // 奇偶校验错误
        be OFFSET(10) NUMBITS(1) [],  // 帧错误
        oe OFFSET(11) NUMBITS(1) []    // 溢出错误
    ],
    pub UARTRSR [
        fe OFFSET(0) NUMBITS(1) [], // 帧错误
        pe OFFSET(1) NUMBITS(1) [], // 奇偶校验错误
        be OFFSET(2) NUMBITS(1) [], // 突发错误
        oe OFFSET(3) NUMBITS(1) [] // 溢出错误
    ],
    pub UARTFR [
        cts OFFSET(0) NUMBITS(1) [],   // CTS信号
        dsr OFFSET(1) NUMBITS(1) [], // DSR信号
        dcd OFFSET(2) NUMBITS(1) [], // DCD信号
        busy OFFSET(3) NUMBITS(1) [], // 发送忙
        rxfe OFFSET(4) NUMBITS(1) [],  // 接收FIFO为空
        txff OFFSET(5) NUMBITS(1) [],  // 发送FIFO满
        rxff OFFSET(6) NUMBITS(1) [],  // 接收FIFO满
        txfe OFFSET(7) NUMBITS(1) [],  // 发送FIFO为空
        ri OFFSET(8) NUMBITS(1) []     // RI信号
    ],
    pub UARTILPR [
        ilpdvsr OFFSET(0) NUMBITS(18) [] // 8 位低功耗计数器
    ],
    pub UARTCR [
        uarten OFFSET(0) NUMBITS(1) [], // UART使能
        txe OFFSET(8) NUMBITS(1) [],    // 发送使能
        rxe OFFSET(9) NUMBITS(1) [],    // 接收使能
        dtr OFFSET(10) NUMBITS(1) [],   // DTR信号
        rts OFFSET(11) NUMBITS(1) []     // RTS信号
    ],
    pub UARTIBRD [
        integer OFFSET(0) NUMBITS(16) [] // 整数部分
    ],
    pub UARTFBRD [
        integer OFFSET(0) NUMBITS(8) [], // 整数部分
        fraction OFFSET(8) NUMBITS(6) [] // 小数部分
    ],
    pub UARTLCR_H [
        wlen OFFSET(0) NUMBITS(2) [], // 字长
        stp2 OFFSET(2) NUMBITS(1) [], // 双停止位
        pen OFFSET(3) NUMBITS(1) [],  // 奇偶校验使能
        eps OFFSET(4) NUMBITS(1) [],  // 奇偶校验选择
        brk OFFSET(5) NUMBITS(1) [],  // 发送中断
        fpen OFFSET(6) NUMBITS(1) []   // 硬件流控制使能
    ],
    pub UARTIMSC [
        rxim OFFSET(4) NUMBITS(1) [], // 接收中断使能
        txim OFFSET(5) NUMBITS(1) [], // 发送中断使能
        rtim OFFSET(6) NUMBITS(1) [], // 接收超时中断使能
        feim OFFSET(7) NUMBITS(1) [], // 奇偶校验错误中断使能
        peim OFFSET(8) NUMBITS(1) [], // 奇偶校验错误中断使能
        beim OFFSET(9) NUMBITS(1) [], // 帧错误中断使能
        oem OFFSET(10) NUMBITS(1) []  // 溢出错误中断使能
    ],
    pub UARTIFLS [
        txif OFFSET(0) NUMBITS(3) [], // 发送FIFO阈值
        rxif OFFSET(3) NUMBITS(3) []  // 接收FIFO阈值
    ],
    pub UARTRIS [
        rxis OFFSET(4) NUMBITS(1) [], // 接收中断状态
        txis OFFSET(5) NUMBITS(1) [], // 发送中断状态
        rtis OFFSET(6) NUMBITS(1) [], // 接收超时中断状态
        feis OFFSET(7) NUMBITS(1) [], // 奇偶校验错误中断状态
        peis OFFSET(8) NUMBITS(1) [], // 奇偶校验错误中断状态
        beis OFFSET(9) NUMBITS(1) [], // 帧错误中断状态
        oem OFFSET(10) NUMBITS(1) []  // 溢出错误中断状态
    ],
    pub UARTMIS [
        rimmis OFFSET(0) NUMBITS(1) [], // 接收中断掩码
        ctsmis OFFSET(1) NUMBITS(1) [], // CTS 中断掩码
        dcdmis OFFSET(2) NUMBITS(1) [], // DCD 中断掩码
        dsrmmis OFFSET(3) NUMBITS(1) [], // DSR 中断掩码
        rxmis OFFSET(4) NUMBITS(1) [], // 接收中断掩码
        txmis OFFSET(5) NUMBITS(1) [], // 发送中断掩码
        rtmis OFFSET(6) NUMBITS(1) [], // 接收超时中断掩码
        femis OFFSET(7) NUMBITS(1) [], // 奇偶校验错误中断掩码
        peimis OFFSET(8) NUMBITS(1) [], // 奇偶校验错误中断掩码
        bemis OFFSET(9) NUMBITS(1) [], // 帧错误中断掩码
        oemis OFFSET(10) NUMBITS(1) []  // 溢出错误中断掩码
    ],

    pub UARTICR [
        rxim OFFSET(4) NUMBITS(1) [], // 清除接收中断
        txim OFFSET(5) NUMBITS(1) [], // 清除发送中断
        rtim OFFSET(6) NUMBITS(1) [], // 清除接收超时中断
        feim OFFSET(7) NUMBITS(1) [], // 清除奇偶校验错误中断
        peim OFFSET(8) NUMBITS(1) [], // 清除奇偶校验错误中断
        beim OFFSET(9) NUMBITS(1) [], // 清除帧错误中断
        oem OFFSET(10) NUMBITS(1) []  // 清除溢出错误中断
    ],
    pub UARTDMACR [
        rxdmae OFFSET(0) NUMBITS(1) [], // 接收 DMA 使能位
        txdmae OFFSET(1) NUMBITS(1) [], // 发送 DMA 使能位
        dmaonerr OFFSET(2) NUMBITS(1) []  // DMA 错误
    ]
];

pub struct Uart {
    base: NonNull<UartRegs>,
}

unsafe impl Sync for Uart {}
unsafe impl Send for Uart {}

impl Uart {
    pub const fn new(base: *mut u8) -> Self {
        Self {
            base: NonNull::new(base).unwrap().cast(),
        }
    }

    const fn regs(&self) -> &UartRegs {
        unsafe { self.base.as_ref() }
    }

    pub fn init(&self) {
        info!("UART initing...");
        // 关闭 UART
        self.regs().uartcr.write(UARTCR::uarten::CLEAR);

        // 配置波特率
        // let interger = CLOCK_HZ / BAUD_RATE;
        let integer = CLOCK_HZ / (16 * BAUD_RATE) as u32;
        let fraction =
            (((CLOCK_HZ % (16 * BAUD_RATE)) * 64 + (16 * BAUD_RATE) / 2) / (16 * BAUD_RATE)) as u32;
        // 写入波特率寄存器
        self.regs().uartibrd.set(integer);
        self.regs().uartfbrd.set(fraction);

        // 设置位宽为8位、停止位为1位、无校验，使能FIFO
        self.regs().uartlcr_h.set(0x70);

        // 关闭中断
        self.regs().uartimsc.set(0x0);

        // 使能 UART，使能发送和接受数据
        self.regs().uartcr.write(
            UARTCR::uarten::SET
                + UARTCR::txe::SET
                + UARTCR::rxe::SET
                + UARTCR::dtr::SET
                + UARTCR::rts::SET,
        );
    }

    // 发送数据
    pub fn send(&self, data: u8) {
        // 等待 TxFIFO 不满
        if self.regs().uartfr.read(UARTFR::txff) == 1 {
            info!("Tx FIFO is full!");
            return;
        }
        // 写入数据
        self.regs().uartdr.write(UARTDR::data.val(data as u32));
    }

    // 接收数据
    pub fn recv(&self) -> u8 {
        // 等待 Rx FIFO 不空
        if self.regs().uartfr.read(UARTFR::rxfe) == 1 {
            info!("Rx FIFO is empty!");
            // return 0;
        }
        // 读取数据
        self.regs().uartdr.read(UARTDR::data) as u8
    }
}
