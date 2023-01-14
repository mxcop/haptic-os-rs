use bl602_hal as hal;
use hal::{
  prelude::*,
  gpio::{Pin16, Uart, UartMux0, Uart0Tx, Pin7, UartMux7, Uart0Rx}, 
  serial::{Serial, Config}, 
  pac::{UART, Peripherals}, 
  clock::Clocks
};
use panic_halt as _;

type SerialCom = Serial<UART, ((Pin16<Uart>, UartMux0<Uart0Tx>), (Pin7<Uart>, UartMux7<Uart0Rx>))>;

pub(crate) fn init_serial_com(clocks: &Clocks) -> SerialCom {
  let dp = Peripherals::take().unwrap();
  let parts = dp.GLB.split();

  // Set up uart output. Since this microcontroller has a pin matrix,
  // we need to set up both the pins and the muxs
  let pin16 = parts.pin16.into_uart_sig0();
  let pin7 = parts.pin7.into_uart_sig7();
  let mux0 = parts.uart_mux0.into_uart0_tx();
  let mux7 = parts.uart_mux7.into_uart0_rx();

  // Configure our UART to 2MBaud, and use the pins we configured above
  Serial::uart0(
    dp.UART,
    Config::default().baudrate(2_000_000.Bd()),
    ((pin16, mux0), (pin7, mux7)),
    *clocks,
  )
}