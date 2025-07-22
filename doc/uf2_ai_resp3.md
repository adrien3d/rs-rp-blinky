# rust rp2350 program with drag and drop a uf2 file from a raspberry pi

Here’s a step-by-step recipe for building a “hello-world” (e.g. LED-blink) Rust firmware for the RP2040 on a Raspberry Pi, converting it to UF2, and flashing it by drag-and-drop.

1. Set up your Pi toolchain  
   ```bash
   # Debian-based (Raspberry Pi OS)
   sudo apt update
   sudo apt install -y build-essential gcc-arm-none-eabi cmake libssl-dev pkg-config git
   # Install Rust (if you haven’t already)
   curl https://sh.rustup.rs -sSf | sh   # follow prompts
   source $HOME/.cargo/env
   rustup target add thumbv6m-none-eabi
   ```

2. Create a new Rust project  
   ```bash
   cargo new --bin pico-blink
   cd pico-blink
   ```

3. Add rp2040-hal and cortex-m crates  
   Edit Cargo.toml, under `[dependencies]` add:
   ```toml
   rp2040-hal = { version = "0.6.0", features = ["rt"] }
   cortex-m = "0.7"
   cortex-m-rt = "0.7"
   panic-halt = "0.2"   # or panic-persist
   ```

4. Provide a linker script  
   In the project root create `memory.x`:
   ```ld
   /* memory.x for RP2040: 2 MB flash @ 0x10000000, 264 KB SRAM @ 0x20000000 */
   MEMORY
   {
     FLASH : ORIGIN = 0x10000000, LENGTH = 2048K
     RAM   : ORIGIN = 0x20000000, LENGTH = 264K
   }
   _stack_start = ORIGIN(RAM) + LENGTH(RAM);
   ```
   And tell cargo to use it. Create (or edit) `.cargo/config.toml`:
   ```toml
   [build]
   target = "thumbv6m-none-eabi"

   [target.thumbv6m-none-eabi]
   rustflags = [
     "-C", "link-arg=-Tmemory.x",
     "-C", "link-arg=-nostartfiles"
   ]
   ```

5. Write your firmware  
   In `src/main.rs` put a simple blink example:
   ```rust
   #![no_std]
   #![no_main]

   use cortex_m_rt::entry;
   use panic_halt as _;
   use rp2040_hal::{clocks::init_clocks_and_plls, sio::Sio, watchdog::Watchdog, pac, gpio::Pins};

   #[entry]
   fn main() -> ! {
       let mut pac = pac::Peripherals::take().unwrap();
       let mut watchdog = Watchdog::new(pac.WATCHDOG);
       let clocks = init_clocks_and_plls(
           pac.XOSC, pac.CLOCKS, pac.PLL_SYS, pac.PLL_USB, &mut pac.RESETS, &mut watchdog
       ).ok().unwrap();
       let sio = Sio::new(pac.SIO);
       let pins = Pins::new(
           pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS
       );
       let mut led = pins.gpio25.into_push_pull_output();

       loop {
           led.set_high().unwrap();
           cortex_m::asm::delay(clocks.system_clock.freq().0 / 4);
           led.set_low().unwrap();
           cortex_m::asm::delay(clocks.system_clock.freq().0 / 4);
       }
   }
   ```

6. Build the ELF  
   ```bash
   cargo build --release
   ```
   You’ll get `target/thumbv6m-none-eabi/release/pico-blink`

7. Install the UF2 converter  
   ```bash
   cargo install elf2uf2-rs
   # or: cargo install uf2conv
   ```

8. Convert ELF → UF2  
   ```bash
   elf2uf2-rs target/thumbv6m-none-eabi/release/pico-blink \
     --base 0x10000000 \
     -o pico-blink.uf2
   ```

9. Flash by drag-and-drop  
   • Plug your Pico into the Pi’s USB port while holding BOOTSEL → it mounts as a “RPI-RP2” drive.  
   • In PCManFM (or via `cp`), drag or copy `pico-blink.uf2` onto that drive. The Pico will reboot and start blinking.

That’s it! You now have a fully Rust-based RP2040 workflow on your Raspberry Pi with UF2 drag-and-drop flashing.