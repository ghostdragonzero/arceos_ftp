pub mod mem;

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
    pub use crate::platform::aarch64_common::psci::system_off as terminate;
}

unsafe extern "C" {
    fn rust_main(cpu_id: usize, dtb: usize);
    #[cfg(feature = "smp")]
    fn rust_main_secondary(cpu_id: usize);
}

pub(crate) unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    crate::mem::clear_bss();
    crate::cpu::init_primary(cpu_id);
    super::aarch64_common::pl011::init_early();
    super::aarch64_common::generic_timer::init_early();
    rust_main(cpu_id, dtb);
}

#[cfg(feature = "smp")]
pub(crate) unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
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
    gpio_init();
}

/// Initializes the platform devices for secondary CPUs.
#[cfg(feature = "smp")]
pub fn platform_init_secondary() {
    #[cfg(feature = "irq")]
    super::aarch64_common::gic::init_secondary();
    super::aarch64_common::generic_timer::init_percpu();
}

use crate::mem::phys_to_virt;
pub fn gpio_init() {
    
    let base_addr = pa!(axconfig::devices::GPIO_PADDR); 
    let base_addr = phys_to_virt(base_addr).as_mut_ptr();
    const gpio_e: usize = 0x410;
    const VALUE: u8 = (1<<3);

    info!("init GPIO Pin3");

    unsafe {
        /* 
        let data_reg = base_addr.add(0x404) as *mut u32;
        let mut data = core::ptr::read_volatile(data_reg);
        data  &= !(1 << 3);
        core::ptr::write_volatile(data_reg, data);
       
        let dir_reg = base_addr.add(0x40c) as *mut u8;
        // 设置 GPIO3 为输入
        let mut dir = core::ptr::read_volatile(dir_reg);
        dir &= !(1 << 3);
        core::ptr::write_volatile(dir_reg, dir);
        */

        //enable interrupt
        let reg = base_addr.add(gpio_e) as *mut u8;
        core::ptr::write_volatile(reg, VALUE);


    }
    #[cfg(feature = "irq")]
    {
        use super::irq;
        use super::irq::register_handler;
        const GPIO_IRQ: usize = 39;
        
        info!("init irq ?  {:#x}", GPIO_IRQ);
        register_handler(GPIO_IRQ, handle_gpio_irq);
        irq::set_enable(GPIO_IRQ, true);
    }
    info!("set irq");
}

pub fn handle_gpio_irq() {
    use core::arch::asm;
    let base_addr = pa!(axconfig::devices::GPIO_PADDR); // 用 let
    let base_addr = phys_to_virt(base_addr).as_mut_ptr();
    const OFFSET: usize = 0x41c; // GPIOE

    info!("power off by gpio");

    unsafe {
        let reg = base_addr.add(OFFSET) as *mut u32;
        let val = core::ptr::read_volatile(reg);
        info!("GPIO E value : {:#x}", val);
        core::ptr::write_volatile(reg, 1 << 3); // 清除 GPIO3 的中断

        asm!("mov w0, #0x18");
        asm!("hlt #0xF000");
    }
}