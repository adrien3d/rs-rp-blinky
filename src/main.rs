//! This example test the Pimoroni Pico Plus 2 on board LED.
//!
//! It does not work with the RP Pico 2 board. See `blinky.rs`.

#![no_std]
#![no_main]

use core::net::Ipv4Addr;

use cyw43::JoinOptions;
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Config, Ipv4Cidr, StackResources};
use embassy_rp::clocks::RoscRng;
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, gpio};
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use heapless::Vec;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const WIFI_NETWORK: &str = "ssid"; // change to your network SSID
const WIFI_PASSWORD: &str = "pwd"; // change to your network password
// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico on board LED, connected to gpio 25"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Placeholder1");
    let p = embassy_rp::init(Default::default());
    let mut rng = RoscRng;
    let fw = include_bytes!("cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("cyw43-firmware/43439A0_clm.bin");

    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs download ../../cyw43-firmware/43439A0.bin --binary-format bin --chip RP235x --base-address 0x10100000
    //     probe-rs download ../../cyw43-firmware/43439A0_clm.bin --binary-format bin --chip RP235x --base-address 0x10140000
    //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
    //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    info!("Placeholder2");
    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );
    info!("Placeholder3");
    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    info!("Placeholder31");
    let state = STATE.init(cyw43::State::new());
    info!("Placeholder32");
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    info!("Placeholder33");
    unwrap!(spawner.spawn(cyw43_task(runner)));

    info!("Placeholder4");


    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    info!("Placeholder5");
    // let config = Config::dhcpv4(Default::default());
    // // Use static IP configuration instead of DHCP
    // let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
    //    address: Ipv4Cidr::new(Ipv4Addr::new(192, 168, 1, 22), 24),
    //    dns_servers: Vec::new(),
    //    gateway: Some(Ipv4Addr::new(192, 168, 69, 1)),
    // });

    // // Generate random seed
    // let seed = rng.next_u64();

    // // Init network stack
    // static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
    // let (stack, runner) = embassy_net::new(net_device, config, RESOURCES.init(StackResources::new()), seed);

    // unwrap!(spawner.spawn(net_task(runner)));

    // loop {
    //     match control
    //         .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
    //         .await
    //     {
    //         Ok(_) => break,
    //         Err(err) => {
    //             info!("join failed with status={}", err.status);
    //         }
    //     }
    // }

    // Wait for DHCP, not necessary when using static IP
    // info!("waiting for DHCP...");


    // control.init(clm).await;
    // control
    //     .set_power_management(cyw43::PowerManagementMode::PowerSave)
    //     .await;

    let delay = Duration::from_millis(2000);
    loop {
        info!("led on!");
        control.gpio_set(0, true).await;
        Timer::after(delay).await;

        info!("led off!");
        control.gpio_set(0, false).await;
        Timer::after(delay).await;
    }
}



//     info!("Placeholder3");

//     static STATE: StaticCell<cyw43::State> = StaticCell::new();
//     info!("Placeholder31");
//     let state = STATE.init(cyw43::State::new());
//     info!("Placeholder32");
//     let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    
//     info!("Placeholder33");
//     unwrap!(spawner.spawn(cyw43_task(runner)));

//     info!("Placeholder4");
//     control.init(clm).await;
//     control
//         .set_power_management(cyw43::PowerManagementMode::PowerSave)
//         .await;

//     let config = Config::dhcpv4(Default::default());

//     info!("Placeholder5");
//     // Use static IP configuration instead of DHCP
//     //let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
//     //    address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 69, 2), 24),
//     //    dns_servers: Vec::new(),
//     //    gateway: Some(Ipv4Address::new(192, 168, 69, 1)),
//     //});

//     // Generate random seed
//     let seed = 123456789;//rng.next_u32(); //next_u64

//     // Init network stack
//     static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
//     let (stack, runner) = embassy_net::new(net_device, config, RESOURCES.init(StackResources::new()), seed);

//     info!("Placeholder6");
//     unwrap!(spawner.spawn(net_task(runner)));

//     info!("Placeholder7");
//     loop {
//         match control
//             .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
//             .await
//         {
//             Ok(_) => break,
//             Err(err) => {
//                 info!("join failed with status={}", err.status);
//             }
//         }
//     }
//     info!("Placeholder8");

//     // Wait for DHCP, not necessary when using static IP
//     info!("waiting for DHCP...");
//     while !stack.is_config_up() {
//         Timer::after_millis(100).await;
//         info!("waiting for DHCP in progress...");
//     }
//     info!("DHCP is now up!");



//     info!("Placeholder9");

//     loop {
//         info!("led on!");
//         led.set_high();
//         Timer::after_millis(250).await;

//         info!("led off!");
//         led.set_low();
//         Timer::after_millis(250).await;
//     }
// }
