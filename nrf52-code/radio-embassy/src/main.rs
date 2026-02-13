#![no_std]
#![no_main]

use bsp::hal::radio::ieee802154::Packet;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut board = bsp::init().unwrap();
    defmt::println!("-- Radio Test firmware --");

    board.radio.set_channel(20);
    board.radio.set_transmission_power(8);

    let mut tx_packet = Packet::new();
    let mut rx_packet = Packet::new();
    tx_packet.copy_from_slice(&[0x01, 0x02, 0x03, 0x04]);
    loop {
        board.radio.try_send(&mut tx_packet).await.unwrap();
        //board.radio.receive(&mut rx_packet).await.unwrap();
        board.leds.ld1_green.toggle();
        Timer::after_millis(300).await;
    }
}
