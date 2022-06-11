#![deny(unsafe_code)]
#![no_main]
#![no_std]

use volatile::Volatile;
use aux5::{Delay, DelayMs, LedArray, OutputSwitch, entry};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut half_period = 100_u16;
    let ms = 50_u8;
    let v_half_period = Volatile::new(&mut half_period);

    loop {
        // for elem in 0..8 {
        //     leds[elem].on().ok();
        //     delay.delay_ms(v_half_period.read());
        // }
        // for elem2 in 0..8 {
        //     leds[elem2].off().ok();
        //     delay.delay_ms(v_half_period.read());
        // }
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().ok();
            delay.delay_ms(ms);

            leds[curr].off().ok();
            delay.delay_ms(ms);
        }
    }
}

