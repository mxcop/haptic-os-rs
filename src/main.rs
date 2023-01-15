#![no_std]
#![no_main]

use bl602_hal as hal;
use embedded_time::duration::Milliseconds;
use core::fmt::Write;
use embedded_hal::{delay::blocking::DelayMs, pwm::blocking::Pwm};
use embedded_hal::digital::blocking::OutputPin;
use hal::{
  clock::{Strict, SysclkFreq, UART_PLL_FREQ},
  pac,
  prelude::*,
  serial::*, pwm,
};
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
  let dp = pac::Peripherals::take().unwrap();
  let mut parts = dp.GLB.split();

  // Set up all the clocks we need
  let clocks = Strict::new()
    .use_pll(40_000_000u32.Hz())
    .sys_clk(SysclkFreq::Pll160Mhz)
    .uart_clk(UART_PLL_FREQ.Hz())
    .freeze(&mut parts.clk_cfg);

  // Set up uart output. Since this microcontroller has a pin matrix,
  // we need to set up both the pins and the muxs
  let pin16 = parts.pin16.into_uart_sig0();
  let pin7 = parts.pin7.into_uart_sig7();
  let mux0 = parts.uart_mux0.into_uart0_tx();
  let mux7 = parts.uart_mux7.into_uart0_rx();

  // Configure our UART to 2MBaud, and use the pins we configured above
  let mut serial = Serial::uart0(
    dp.UART,
    Config::default().baudrate(2_000_000.Bd()),
    ((pin16, mux0), (pin7, mux7)),
    clocks,
  );
  // Also set up a pin as GPIO, to blink an LED
  //let mut gpio11 = parts.pin11.into_pull_down_output();
  /* let mut gpio14= */ 

  let mut channels = pwm::Channels::from((dp.PWM, clocks));
  channels.channel4.enable(&()).unwrap();
  channels.channel4.set_period(Milliseconds::new(400u32)).unwrap();
  let duty = channels.channel4.get_max_duty().unwrap() / 2;
  channels.channel4.set_duty(&(), duty).unwrap();
  parts.pin14.into_pull_down_pwm();

  // Create a blocking delay function based on the current cpu frequency
  let mut d = bl602_hal::delay::McycleDelay::new(clocks.sysclk().0);

  loop {
    // Toggle the LED on and off once a second. Report LED status over UART
    //gpio11.set_high().unwrap();
    //gpio14.set_low().unwrap();
    //channels.channel4.enable(&()).unwrap();
    serial.write_str("LEDs on\r\n").ok();
    d.delay_ms(1000).unwrap();

    //gpio11.set_low().unwrap();
    //gpio14.set_high().unwrap();
    //channels.channel4.disable(&()).unwrap();
    serial.write_str("LEDs off\r\n").ok();
    d.delay_ms(1000).unwrap();
  }
}
