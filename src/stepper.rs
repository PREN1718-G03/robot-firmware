use hifive::gpio;
use hifive::e310x::GPIO0;

type DirectionPin = gpio::Pin18;
type StepPin = gpio::Pin23;

pub enum Direction {
    Forward,
    Backward
}

pub fn init(gpio: &GPIO0) {
    DirectionPin::init(gpio, gpio::PinConfig::Output);
}

pub fn set_direction(gpio: &GPIO0, dir: Direction) {
    match dir {
        Direction::Forward => {
            DirectionPin::high(gpio);
        },
        Direction::Backward => {
            DirectionPin::low(gpio);
        }
    }
}

pub fn half_step(gpio: &GPIO0) {
    StepPin::toggle(gpio);
}
