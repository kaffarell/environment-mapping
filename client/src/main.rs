use rppal::gpio::{Gpio, Level};
use rppal::system::DeviceInfo;

const GPIO_PIN_CLK: u8 = 15;
const GPIO_PIN_DAT: u8 = 14;

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Error: {:?}", e);
        }
    }
}


#[tokio::main]
async fn post_request(data: f32) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post")
        .body(data.to_string())
        .send()
        .await?;

    println!("Response: {:?}", res);
    return Ok(());
}

fn run() -> Result<(), Box<::rppal::gpio::Error>> {
    let device_info = DeviceInfo::new().unwrap();
    println!(
        "Model: {} (SoC: {})",
        device_info.model(),
        device_info.soc()
    );


    let pin_clk = Gpio::new()?.get(GPIO_PIN_CLK)?.into_input();
    let pin_dat = Gpio::new()?.get(GPIO_PIN_DAT)?.into_input();

    let mut last_clk_state = Level::High;
    // Stores the current state of rotation
    // 0 - 20
    let mut current_state = 0;

    loop {
        match pin_clk.read() {
            Level::High => if last_clk_state == Level::Low {
                if let Level::Low = pin_dat.read() {
                    current_state += 1;
                    println!("{}", current_state);
                } else {
                    current_state -= 1;
                    println!("{}", current_state);
                }

                last_clk_state = Level::High;
            },
            state => {
                last_clk_state = state;
            }
        };
        if current_state > 20 {
            current_state = 0;
        }else if current_state < 0 {
            current_state = 20;
        }
        // Get degree
        let degree: f32 = (360.0/20.0) * current_state as f32;
        // Send POST request
        post_request(degree);        
    }
}
