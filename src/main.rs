#![deny(unsafe_code)]
#![no_main]
#![no_std]


//use cortex_m_rt::entry;
//use cortex_m_semihosting::heprintln;
//use panic_halt as _;
// Print panic message to probe console
use panic_probe as _;

mod domain;



#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {
    use stm32f4xx_hal::{
        prelude::*,
    };

    

    #[shared]
    struct Shared {}

    // Local resources go here
    #[local]
    struct Local {}

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        (
            Shared {
               // Initialization of shared resources go here
            },
            Local {
                // Initialization of local resources go here
            },
            init::Monotonics()
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue;
        }
    }

    fn main() {
        use crate::domain::kv_store::{KVStore, Database, Key, Value};

        let mut db = Database::new();

        let key = Key::parse("new").unwrap();
        let value = Value::parse("value").unwrap();

        let _ = db.set(key.clone(), value);

        let answer = db.get(&key);
        cortex_m_semihosting::hprintln!("value: {:?}", answer);

        let _answer = db.delete(&key);
    }

}

    
    