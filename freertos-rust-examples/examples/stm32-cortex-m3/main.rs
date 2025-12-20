#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::{ExceptionFrame, entry, exception};
use freertos_next::*;
use stm32f1_hal::{cortex_m::asm, gpio::PinState, pac, prelude::*, rcc};

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

#[entry]
fn main() -> ! {
    Task::new()
        .name("default")
        .stack_size(1000)
        .start(move |_| {
            app_main();
        })
        .unwrap();
    FreeRtosUtils::start_scheduler();
}

fn app_main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.init();
    let sysclk = 72.MHz();
    let mut rcc = dp
        .RCC
        .init()
        .freeze(rcc::Config::hse(8.MHz()).sysclk(sysclk), &mut flash.acr);
    assert_eq!(rcc.clocks().sysclk(), sysclk);

    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob
        .pb0
        .into_open_drain_output_with_state(&mut gpiob.crl, PinState::High);

    loop {
        CurrentTask::delay(Duration::ms(500));
        led.set_low();
        CurrentTask::delay(Duration::ms(500));
        led.set_high();
    }
}

#[allow(non_snake_case)]
#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    // custom default handler
    // irqn is negative for Cortex-M exceptions
    // irqn is positive for device specific (line IRQ)
    // set_led(true);(true);
    // panic!("Exception: {}", irqn);
    asm::bkpt();
    loop {}
}

#[allow(non_snake_case)]
#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();
    loop {}
}

// We no longer need to use #[alloc_error_handler] since v1.68.
// It will automatically call the panic handler.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    asm::bkpt();
    loop {}
}
