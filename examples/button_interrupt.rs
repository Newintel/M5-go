use std::cell::RefCell;

use critical_section::Mutex;
use esp_idf_hal::{delay::FreeRtos, gpio::InterruptType, prelude::Peripherals};
use smart_leds::colors::{HOT_PINK, PURPLE, WHITE, YELLOW_GREEN};

static LIGHTS_ON: Mutex<RefCell<Option<bool>>> = Mutex::new(RefCell::new(None));
static COLOR_INDEX: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut m5 = m5_go::M5Go::new(peripherals)?;

    m5.button_a.set_interrupt_type(InterruptType::NegEdge)?;
    m5.button_b.set_interrupt_type(InterruptType::NegEdge)?;
    m5.button_c.set_interrupt_type(InterruptType::NegEdge)?;

    unsafe {
        m5.button_a.subscribe(on_button_a_pushed)?;
        m5.button_b.subscribe(on_button_b_pushed)?;
        m5.button_c.subscribe(on_button_c_pushed)?;
    }

    let colors = [WHITE, HOT_PINK, PURPLE, YELLOW_GREEN];

    loop {
        m5.leds.off();
        if critical_section::with(|cs| LIGHTS_ON.borrow_ref(cs).unwrap_or_default()) {
            let index = critical_section::with(|cs| *COLOR_INDEX.borrow_ref(cs));
            m5.leds.fill(colors[index as usize % colors.len()]);
            m5.leds.display();
        }

        FreeRtos::delay_ms(100);
    }
}

fn on_button_a_pushed() {
    esp_println::println!("button a pushed");
    critical_section::with(|cs| {
        LIGHTS_ON.replace_with(cs, |old| Some(!old.unwrap_or(false)));
    })
}

fn on_button_b_pushed() {
    esp_println::println!("button b pushed");
    critical_section::with(|cs| {
        COLOR_INDEX.replace_with(cs, |old| if *old == 0 { 0 } else { *old - 1 });
    })
}

fn on_button_c_pushed() {
    esp_println::println!("button c pushed");
    critical_section::with(|cs| {
        COLOR_INDEX.replace_with(cs, |old| if *old == u32::MAX { *old } else { *old + 1 });
    })
}
