extern crate rppal;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use rppal::gpio::{Gpio, Level, Mode};
use rppal::system::DeviceInfo;

const GPIO_PIN_CLK: u8 = 26;
const GPIO_PIN_DAT: u8 = 19;

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Error: {:?}", e);
        }
    }
}

fn run() -> Result<(), Box<::rppal::gpio::Error>> {
    let device_info = DeviceInfo::new().unwrap();
    println!(
        "Model: {} (SoC: {})",
        device_info.model(),
        device_info.soc()
    );

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut gpio = Gpio::new().unwrap();
        gpio.set_mode(GPIO_PIN_CLK, Mode::Input);
        gpio.set_mode(GPIO_PIN_DAT, Mode::Input);

        let mut last_clk_state = Level::High;

        loop {
            match gpio.read(GPIO_PIN_CLK) {
                Ok(Level::High) => if last_clk_state == Level::Low {
                    if let Ok(Level::Low) = gpio.read(GPIO_PIN_DAT) {
                        tx.send(1).unwrap();
                    } else {
                        tx.send(-1).unwrap();
                    }

                    last_clk_state = Level::High;
                },
                Ok(state) => {
                    last_clk_state = state;
                }
                Err(_) => {}
            }

            thread::sleep(Duration::from_millis(1));
        }
    });

    let mut count = 0;
    for received in rx {
        count += received;
        println!("Got: {} for a count of {}", received, count);
    }

    Ok(())
}
