#![no_std]
#![no_main]

use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use panic_probe as _;

use rp235x_hal::{
    self as hal,
    clocks::init_clocks_and_plls,
    entry,
    pac,
    Clock,
};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef =
    hal::block::ImageDef::secure_exe();

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(peripherals.WATCHDOG);

    let clocks = init_clocks_and_plls(
        12_000_000,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(peripherals.SIO);

    let pins = hal::gpio::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let mut led = pins.gpio25.into_push_pull_output();

    let mut delay = cortex_m::delay::Delay::new(
        core.SYST,
        clocks.system_clock.freq().to_Hz(),
    );

    defmt::info!("RTOS bring-up: core 0 started");

    let priority = kernel::highest_ready((1 << 2) | (1 << 5) | (1 << 9))
    .unwrap();

    defmt::info!("Scheduler smoke test: priority {}", priority);

    let mut ticks = 0_u32;

    loop {
        led.set_high().unwrap();
        delay.delay_ms(250);

        led.set_low().unwrap();
        delay.delay_ms(250);

        ticks = ticks.wrapping_add(1);
        defmt::debug!("Heartbeat {}", ticks);
    }
}