#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;

use esp32c3_hal::{
    clock::ClockControl,
    pac::Peripherals,
    prelude::*,
    system::SystemExt,
    timer::TimerGroup,
    gpio::IO,
    Rtc, Delay,
};
use smart_leds::{RGB, RGB8, colors::{ORANGE, BLUE}, SmartLedsWrite};
use ws2812_spi::Ws2812;

#[entry]
fn main() -> ! {
    const NUM_LEDS: usize = 10;
    const DELAY_SECONDS: u32 = 30;

    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let mut delay: Delay = Delay::new(&clocks);

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT and the RTC WDT.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let mut timer0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut timer1 = TimerGroup::new(peripherals.TIMG1, &clocks);

    timer0.wdt.disable();
    timer1.wdt.disable();
    rtc.swd.disable();
    rtc.rwdt.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mosi = io.pins.gpio7;

    let spi = esp32c3_hal::spi::Spi::new_mosi_only(
        peripherals.SPI2, 
        mosi, 
        fugit::HertzU32::Hz(3_333_333), 
        esp32c3_hal::spi::SpiMode::Mode0, 
        &mut system.peripheral_clock_control, 
        &clocks
    );

    let mut ws = Ws2812::new(spi);

    loop{
        let first_rgb: RGB8 = ORANGE;
        let second_rgb: RGB8 = BLUE;

        let mut data: [RGB<u8>; 10] = [first_rgb; NUM_LEDS];

        ws.write(data.iter().cloned()).unwrap();

        for i in 0..NUM_LEDS {
            delay.delay_ms(DELAY_SECONDS);
            data[i] = second_rgb;
            ws.write(data.iter().cloned()).unwrap();
        }
        for i in 0..NUM_LEDS {
            delay.delay_ms(DELAY_SECONDS);
            data[i] = first_rgb;
            ws.write(data.iter().cloned()).unwrap();
        }
    }

}