pub fn line_y_value(
    y_rise: f32,
    x_run: f32,
    x_value: f32,
) -> f32 {
    let slope = y_rise / x_run;
    let y_value = slope * x_value;
    y_value
}

pub fn line_y_value_with_y_offset(
    y_rise: f32,
    x_run: f32,
    x_value: f32,
    y_offset: f32,
) -> f32 {
    let y_value = line_y_value(y_rise, x_run, x_value);
    y_value + y_offset
}
