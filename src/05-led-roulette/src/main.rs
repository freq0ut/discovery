#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();
    let period = 50_u16;

    loop {  
        for curr in 0..8 {
            let next = (curr+1) % 8;

            leds[next].on();
            delay.delay_ms(period);

            leds[curr].off();
            delay.delay_ms(period);
        }
    }
}