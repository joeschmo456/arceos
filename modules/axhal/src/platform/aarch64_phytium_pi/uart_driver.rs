use super::uart::*;

pub fn uart_example() {
    UART.lock().init();
    info!("UART initialized");

    for i in 0..10 {
        UART.lock().send(i as u8);
        info!("UART sent: {}", i);
        let recv = UART.lock().recv();
        info!("UART received: {}", recv);
        //core::hint::spin_loop();
    } 
    info!("UART example end");
}
