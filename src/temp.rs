use nrf51::TEMP;

/// The measurement unit to return a temperature in.
pub enum Degrees {
    Celsius,
    Fahrenheit,
}

/// The `measure_temp_rational` function takes a `TEMP` peripheral and returns
/// an integer temperature measured in quarters of a degree Celsius.
///
/// For example, if this function returns `88`, the temperature recorded is
/// actually 88 divided by four; that is, 22 degrees Celsius.
pub fn measure_temp_rational(p: &mut TEMP) -> i32 {
    p.tasks_start.write(|w| unsafe { w.bits(1) });
    while p.events_datardy.read().bits() == 0 {}
    p.events_datardy.write(|w| unsafe { w.bits(0) });
    p.temp.read().bits() as i32
}

/// The `measure_temp_float` function takes a `nrf51::TEMP` peripheral and a
/// `Degrees` unit and returns a float temperature.
pub fn measure_temp_float(p: &mut TEMP, mode: &Degrees) -> f32 {
    match mode {
        Degrees::Celsius => (measure_temp_rational(p) as f32) / 4.0,
        Degrees::Fahrenheit => (measure_temp_rational(p) as f32) * 0.45 + 32.0,
    }
}
