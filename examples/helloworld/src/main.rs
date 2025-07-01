#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
#[cfg(feature = "axstd")]
use axstd::thread::sleep;
#[cfg(feature = "axstd")]
use axstd::time;

#[cfg_attr(feature = "axstd", unsafe(no_mangle))]
fn main() {
    println!("Hello, world!");
    loop {
        sleep(time::Duration::from_secs(1));
        println!("in loop");
        let wcs = 0xffff_0000_2804_1000 as *mut u32;
        println!("sleep 1s, get wcs {:x}", unsafe { *wcs });
    }
}
