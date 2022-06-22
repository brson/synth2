#![allow(unused)]

use anyhow::{Result, anyhow};
use std::path::Path;

pub fn write_image(buf: &[f64], outdir: &Path, file_stem: &str) -> Result<()> {
    use charts::{Chart, ScaleLinear, MarkerType, PointLabelPosition, LineSeriesView};

    let filepath = outdir.join(file_stem).with_extension("svg");

    let width = 1280;
    let height = 720;
    let (top, right, bottom, left) = (100, 100, 100, 100);

    let available_width = width - left - right;
    let available_height = height - top - bottom;

    let x = ScaleLinear::new()
        .set_domain(vec![0_f32, buf.len() as f32])
        .set_range(vec![0, available_width]);

    let y = ScaleLinear::new()
        .set_domain(vec![-1_f32, 1_f32])
        .set_range(vec![available_height, 0]);

    let line_data = (0..).zip(buf.iter().copied());
    let line_data = line_data.map(|(x, y)| (x as f32, y as f32));
    let line_data: Vec<_> = line_data.collect();

    let line_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
    //.set_label_position(PointLabelPosition::N)
        .set_label_visibility(false)
        .load_data(&line_data)
        .map_err(|e| anyhow!("{}", e))?;

    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        //.add_title(String::from("Line Chart"))
        .add_view(&line_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        //.add_left_axis_label("Custom Y Axis Label")
        //.add_bottom_axis_label("Custom X Axis Label")
        .save(filepath)
        .map_err(|e| anyhow!("{}", e))?;

    Ok(())
}
