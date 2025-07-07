pub mod mem;
pub mod uart;
//pub mod test_uart;

#[cfg(feature = "smp")]
pub mod mp;

#[cfg(feature = "irq")]
pub mod irq {
    pub use crate::platform::aarch64_common::gic::*;
}

pub mod console {
    pub use crate::platform::aarch64_common::pl011::*;
}

pub mod time {
    pub use crate::platform::aarch64_common::generic_timer::*;
}

pub mod misc {
    pub fn terminate() -> ! {
        info!("Shutting down...");
        loop {
            crate::arch::halt();
        }
    }

    //pub use super::test_uart::*;
}

unsafe extern "C" {
    fn rust_main(cpu_id: usize, dtb: usize);
    #[cfg(feature = "smp")]
    fn rust_main_secondary(cpu_id: usize);
}

pub(crate) unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    crate::mem::clear_bss();
    let cpu_id = cpu_hard_id_to_logic_id(cpu_id);
    crate::arch::write_page_table_root0(0.into()); // disable low address access
    crate::cpu::init_primary(cpu_id);
    super::aarch64_common::pl011::init_early();
    super::aarch64_common::generic_timer::init_early();
    rust_main(cpu_id, dtb);
}

#[cfg(feature = "smp")]
pub(crate) unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    let cpu_id = cpu_hard_id_to_logic_id(cpu_id);
    crate::arch::write_page_table_root0(0.into()); // disable low address access
    crate::cpu::init_secondary(cpu_id);
    rust_main_secondary(cpu_id);
}

/// Initializes the platform devices for the primary CPU.
///
/// For example, the interrupt controller and the timer.
pub fn platform_init() {
    #[cfg(feature = "irq")]
    super::aarch64_common::gic::init_primary();
    super::aarch64_common::generic_timer::init_percpu();
    super::aarch64_common::pl011::init();
    
    crate::platform::uart::init();
    use crate::platform::uart::UART2;
    let mut uart = UART2.lock();
    let mut data:u8 = 0;
    loop {
        uart.putchar(data);
        info!("no 8 end data {data}");
        let read = uart.getchar();
        info!("read data {read}");
        info!("sleep 1s");
        data = data.wrapping_add(1);
    }


/* 
    use crate::time::Duration;
    use crate::misc::UART2;

    let mut uart = UART2.lock();
    uart.init_no_irq(100_000_000, 115200);
    let mut data:u8 = 0;
    loop {
        uart.put_byte_poll(data);
        info!("use  bitrate send data {data}");
        let read = uart.read_byte_poll();
        info!("read data {read}");
        info!("sleep 1s");
        data = data.wrapping_add(1);
    }
*/

}

/// Initializes the platform devices for secondary CPUs.
#[cfg(feature = "smp")]
pub fn platform_init_secondary() {
    #[cfg(feature = "irq")]
    super::aarch64_common::gic::init_secondary();
    super::aarch64_common::generic_timer::init_percpu();


}

fn cpu_hard_id_to_logic_id(hard_id: usize) -> usize {
    axconfig::devices::CPU_ID_LIST
        .iter()
        .position(|&x| x == hard_id)
        .unwrap()
}

use crate::mem::phys_to_virt;
pub fn wdt_init() {
    
    let base_addr = pa!(axconfig::devices::WDT0_PADDR); 
     info!("addr0   {:#x}", axconfig::devices::WDT0_PADDR);
     info!("addr1   {:#x}", axconfig::devices::WDT1_PADDR);
    let base_addr = phys_to_virt(base_addr).as_mut_ptr();
    // 获取看门狗基地址
    #[cfg(feature = "irq")]
    {
        use super::irq;
        use super::irq::register_handler;
        const GPIO_IRQ: usize = 196;
        
        info!("init irq ?  {:#x}", GPIO_IRQ);
        register_handler(GPIO_IRQ, handle_wdt_irq);
        irq::set_enable(GPIO_IRQ, true);
    }
    info!("set irq");
    // 使能中断

    unsafe{    
        let reg = base_addr.add(0x008) as *mut u32;
        core::ptr::write_volatile(reg, 0x30000000);
        info!("set timeout 0x10000");

        let enable_reg = base_addr.add(0x000) as *mut u32;
        let val = core::ptr::read_volatile(enable_reg);
        info!("0x000{:#x}", val);
        core::ptr::write_volatile(enable_reg, val | (1 << 0));

    }
    


}

pub fn handle_wdt_irq() {
    use core::arch::asm;
    let base_addr = pa!(axconfig::devices::WDT0_PADDR); // 用 let
    let base_addr = phys_to_virt(base_addr).as_mut_ptr();

    info!("wdt time out ");
/* 
    unsafe {

        asm!("mov w0, #0x18");
        asm!("hlt #0xF000");
    }
    */
}