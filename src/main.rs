#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, gpio::{GpioPin, Output, IO}, ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        LSGlobalClkSource,
        LowSpeed,
        LEDC,
    }, peripherals::Peripherals, prelude::*, Delay
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let gpio0 = io.pins.gpio0.into_push_pull_output();
    let gpio1 = io.pins.gpio1.into_push_pull_output();
    let gpio2 = io.pins.gpio2.into_push_pull_output();
    let gpio3 = io.pins.gpio3.into_push_pull_output();
    let gpio4 = io.pins.gpio4.into_push_pull_output();
    let gpio5 = io.pins.gpio5.into_push_pull_output();

    let mut ledc = LEDC::new(peripherals.LEDC, &clocks);

    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);

    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty5Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 24u32.kHz(),
        })
        .unwrap();

    let mut channel0: esp_hal::ledc::channel::Channel<'_, LowSpeed, GpioPin<Output<esp_hal::gpio::PushPull>, 0>> = ledc.get_channel(channel::Number::Channel0, gpio0);
    let mut channel1: esp_hal::ledc::channel::Channel<'_, LowSpeed, GpioPin<Output<esp_hal::gpio::PushPull>, 1>> = ledc.get_channel(channel::Number::Channel1, gpio1);
    let mut channel2: esp_hal::ledc::channel::Channel<'_, LowSpeed, GpioPin<Output<esp_hal::gpio::PushPull>, 2>> = ledc.get_channel(channel::Number::Channel2, gpio2);
    let mut channel3: esp_hal::ledc::channel::Channel<'_, LowSpeed, GpioPin<Output<esp_hal::gpio::PushPull>, 3>> = ledc.get_channel(channel::Number::Channel3, gpio3);
    let mut channel4: esp_hal::ledc::channel::Channel<'_, LowSpeed, GpioPin<Output<esp_hal::gpio::PushPull>, 4>> = ledc.get_channel(channel::Number::Channel4, gpio4);
    let mut channel5: esp_hal::ledc::channel::Channel<'_, LowSpeed, GpioPin<Output<esp_hal::gpio::PushPull>, 5>> = ledc.get_channel(channel::Number::Channel5, gpio5);

    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
 
    channel1
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

     channel2
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    channel3
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    channel4
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
   
    channel5
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    
    let duty_neg90 = 2; // Example value for ~1ms pulse width, adjust based on calculation
    let duty_90 = 4; // Example value for ~2ms pulse width, adjust based on calculation

    loop {
        // Pivot to "-90" degrees
        channel0.set_duty(duty_neg90).unwrap();
        delay.delay_ms(1000 as u32);

        // Pivot to "90" degrees
        channel0.set_duty(duty_90).unwrap();
        delay.delay_ms(1000 as u32);

        // Pivot to "-90" degrees
        channel1.set_duty(duty_neg90).unwrap();
        delay.delay_ms(1000 as u32);

        // Pivot to "90" degrees
        channel1.set_duty(duty_90).unwrap();
        delay.delay_ms(1000 as u32);
        // Pivot to "-90" degrees
        channel2.set_duty(duty_neg90).unwrap();
        delay.delay_ms(1000 as u32);

        // Pivot to "90" degrees
        channel2.set_duty(duty_90).unwrap();
        delay.delay_ms(1000 as u32);

        // Pivot to "-90" degrees
        channel3.set_duty(duty_neg90).unwrap();
        delay.delay_ms(1000 as u32);

        // Pivot to "90" degrees
        channel3.set_duty(duty_90).unwrap();
        delay.delay_ms(1000 as u32);
        // Pivot to "-90" degrees
        channel4.set_duty(duty_neg90).unwrap();
        delay.delay_ms(1000 as u32);

        // Pivot to "90" degrees
        channel4.set_duty(duty_90).unwrap();
        delay.delay_ms(1000 as u32);

        // Pivot to "-90" degrees
        channel5.set_duty(duty_neg90).unwrap();
        delay.delay_ms(1000 as u32);

        // Pivot to "90" degrees
        channel5.set_duty(duty_90).unwrap();
        delay.delay_ms(1000 as u32);
    }
}