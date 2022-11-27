use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Point, RgbColor},
    text::Alignment,
};
use esp_idf_hal::prelude::Peripherals;
use m5_go::Note;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut m5 = m5_go::M5Go::new(peripherals).unwrap();

    let ode_to_joy = vec![
        (
            vec![
                Note::E,
                Note::E,
                Note::F,
                Note::G,
                Note::G,
                Note::F,
                Note::E,
                Note::D,
                Note::C,
                Note::C,
                Note::D,
                Note::E,
            ],
            1.,
            4,
        ),
        (vec![Note::E], 1.5, 4),
        (vec![Note::D], 0.5, 4),
        (vec![Note::D], 2., 4),
        (
            vec![
                Note::E,
                Note::E,
                Note::F,
                Note::G,
                Note::G,
                Note::F,
                Note::E,
                Note::D,
                Note::C,
                Note::C,
                Note::D,
                Note::E,
            ],
            1.,
            4,
        ),
        (vec![Note::D], 1.5, 4),
        (vec![Note::C], 0.5, 4),
        (vec![Note::C], 2., 4),
        (vec![Note::D, Note::D, Note::E, Note::C, Note::D], 1., 4),
        (vec![Note::E, Note::F], 0.5, 4),
        (vec![Note::E, Note::C, Note::D], 1., 4),
        (vec![Note::E, Note::F], 0.5, 4),
        (vec![Note::E, Note::D, Note::C, Note::D], 1., 4),
        (vec![Note::G], 1., 3),
        (vec![Note::E], 2., 4),
        (
            vec![
                Note::E,
                Note::F,
                Note::G,
                Note::G,
                Note::F,
                Note::E,
                Note::D,
                Note::C,
                Note::C,
                Note::D,
                Note::E,
            ],
            1.,
            4,
        ),
        (vec![Note::D], 1.5, 4),
        (vec![Note::C], 0.5, 4),
        (vec![Note::C], 2., 4),
    ];

    m5.screen.turn_on();

    m5.screen.fill_background(Rgb565::BLACK);

    m5.screen.draw_text(
        "Here is a piece of music :",
        Point::new(0, 15),
        Alignment::Left,
        Rgb565::WHITE,
    );
    m5.screen.draw_text(
        "Ode to Joy",
        Point::new(0, 30),
        Alignment::Left,
        Rgb565::WHITE,
    );
    m5.screen.draw_text(
        "Press button A to stop it",
        Point::new(0, 45),
        Alignment::Left,
        Rgb565::WHITE,
    );

    let _ = 'block: {
        for (notes, speed, octave) in ode_to_joy {
            for note in notes {
                if m5.button_a.is_low() {
                    break 'block;
                }
                m5.speaker
                    .do_sound(note.octave(octave), (500f32 * speed) as u32, None);
            }
        }
    };

    m5.screen.turn_off();
}
