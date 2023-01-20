extern crate charts;
extern crate hdrhistogram;
extern crate lazy_static;
extern crate regex;

use charts::{BarDatum, Chart, ScaleBand, ScaleLinear, VerticalBarView};
use hdrhistogram::Histogram;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Write;
use std::fs;

/// Name of the input file with measured benchmarks.
const INPUT_FILE_NAME: &str = "benchmarks.txt";

/// Name of the output file with histogram data.
const OUTPUT_HGRM_FILE_NAME: &str = "benchmarks.hgrm";

/// Name of the output file with SVG histogram.
const OUTPUT_SVG_FILE_NAME: &str = "benchmarks.svg";

/// Regular expression pattern for parsing single report line.
const LINE_PATTERN: &str = r#"test\s+(?P<name>[^\s+]+)[^:]+:\s+(?P<duration>[0-9,]+)\s+(?P<unit>[^/]+)"#;

lazy_static! {
  static ref RE_LINE: Regex = Regex::new(format!("^{}", LINE_PATTERN).as_str()).unwrap();
}

struct Data(String, f32);

impl BarDatum for Data {
  fn get_category(&self) -> String {
    self.0.clone()
  }
  fn get_value(&self) -> f32 {
    self.1
  }
  fn get_key(&self) -> String {
    "".to_string()
  }
}

/// Generates the histogram chart in SVG format.
fn generate_histogram_chart(data: Vec<Data>) {
  let width = 2000;
  let height = 600;
  let (top, right, bottom, left) = (90, 40, 50, 60);
  let labels = data.iter().map(|item| item.0.clone()).collect::<Vec<String>>();
  let x = ScaleBand::new()
    .set_domain(labels)
    .set_range(vec![0, width - left - right])
    .set_inner_padding(0.1)
    .set_outer_padding(0.1);
  let y = ScaleLinear::new().set_domain(vec![0.0, 800.0]).set_range(vec![height - top - bottom, 0]);
  let view = VerticalBarView::new().set_x_scale(&x).set_y_scale(&y).load_data(&data).unwrap();
  Chart::new()
    .set_width(width)
    .set_height(height)
    .set_margins(top, right, bottom, left)
    .add_title(String::from("DMNTK v0.0.54 benchmarks"))
    .add_view(&view)
    .add_axis_bottom(&x)
    .add_axis_left(&y)
    .add_left_axis_label("Microseconds")
    .add_bottom_axis_label("Percentile")
    .save(OUTPUT_SVG_FILE_NAME)
    .unwrap();
}

/// Main entrypoint of benchmarks tool.
fn main() {
  let input = fs::read_to_string(INPUT_FILE_NAME).expect("reading input file failed");
  let mut histogram = Histogram::<u64>::new(1).unwrap();
  for line in input.lines() {
    if let Some(captures) = RE_LINE.captures(line) {
      if let Some(_name) = captures.name("name") {
        if let Some(duration) = captures.name("duration") {
          if let Some(unit) = captures.name("unit") {
            let d = duration.as_str().replace(",", "").parse::<u64>().unwrap();
            let value = match unit.as_str() {
              "ns" => d,
              other => panic!("unexpected unit: '{}'", other),
            };
            histogram += value / 1000;
          }
        }
      }
    }
  }
  println!("=============================================================================");
  println!("  Total count: {:>10} [µs]", histogram.len());
  println!("          Min: {:>10} [µs]", histogram.min());
  println!("          Max: {:>10} [µs]", histogram.max());
  println!("         Mean: {:>10.0} [µs]", histogram.mean());
  println!("      Std Dev: {:>10.0} [µs]", histogram.stdev());
  println!("=============================================================================");
  for v in histogram.iter_recorded() {
    println!(
      "{:>6.1}'th percentile of data is <= {:>5} [µs]  with {:>5} sample(s)",
      v.percentile(),
      v.value_iterated_to(),
      v.count_at_value()
    );
  }
  println!("=============================================================================");
  println!("\n");

  let mut buffer = String::new();

  println!("{:>10}   {:>10}    {:>10}    {:>10}\n", "Value", "Percentile", "TotalCount", "1/(1-Percentile)");
  let _ = writeln!(
    &mut buffer,
    "{:>10}   {:>10}    {:>10}    {:>10}\n",
    "Value", "Percentile", "TotalCount", "1/(1-Percentile)"
  );
  let mut total = 0_u64;
  for v in histogram.iter_recorded() {
    total += v.count_since_last_iteration();
    let perc = v.percentile() / 100.0;
    println!(
      "{:>10}   {:>10.6}    {:>10}    {:>10.2}",
      v.value_iterated_to(),
      perc,
      total,
      1.0 / (1.0 - perc)
    );
    let _ = writeln!(
      &mut buffer,
      "{:>10}   {:>10.6}    {:>10}    {:>10.2}",
      v.value_iterated_to(),
      perc,
      total,
      1.0 / (1.0 - perc)
    );
  }

  fs::write(OUTPUT_HGRM_FILE_NAME, buffer).expect("writing hgrm file failed");

  println!("\n=============================================================================\n");

  println!("{}   {}", "Percentile", "Microseconds");

  let mut data = vec![];
  for v in histogram.iter_recorded() {
    let label = format!("{:.2}", v.percentile());
    let value = v.value_iterated_to() as f32;
    println!("{:>9}{:>15}", label, v.value_iterated_to());
    data.push(Data(label, value));
  }

  println!("\n=============================================================================\n");

  generate_histogram_chart(data);
}
