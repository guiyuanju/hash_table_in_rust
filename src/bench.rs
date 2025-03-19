use std::time::{Duration, Instant};

use plotters::prelude::*;

pub fn draw(name: &str, data: &Vec<(f32, f32)>) -> Result<(), Box<dyn std::error::Error>> {
    let mut max_y = 0.0f32;
    for (_, y) in data {
        if y > &max_y {
            max_y = *y;
        }
    }
    let file_name = format!("plotters-doc-data/{:}.png", name);
    let root = BitMapBackend::new(&file_name, (640, 480)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption(name, ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(50)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..(data.len() as f32), 0f32..max_y)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(7)
        .y_labels(7)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.6}", x))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(data.clone(), &RED))?;

    root.present()?;

    Ok(())
}

pub fn measure<F>(f: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    start.elapsed()
}
