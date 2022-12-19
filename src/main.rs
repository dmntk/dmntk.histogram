extern crate hdrhistogram;
extern crate lazy_static;
extern crate regex;

use hdrhistogram::Histogram;
use lazy_static::lazy_static;
use regex::Regex;

/// Regular expression pattern for parsing single report line.
const LINE_PATTERN: &str = r#"test\s+(?P<name>[^\s+]+)[^:]+:\s+(?P<duration>[0-9,]+)\s+(?P<unit>[^/]+)"#;

lazy_static! {
  static ref RE_LINE: Regex = Regex::new(format!("^{}", LINE_PATTERN).as_str()).unwrap();
}

/// Main entrypoint of benchmarks tool.
fn main() {
  let file_name = "/home/dmntk/Work/dmntk/dmntk.metrics/performance/2022-12-16/benchmarks.txt";
  let input = std::fs::read_to_string(file_name).expect("reading input file failed");
  let mut histogram = Histogram::<u64>::new(3).unwrap();
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
  println!("  Total count: {:>10} [ns]", histogram.len());
  println!("          Min: {:>10} [ns]", histogram.min());
  println!("          Max: {:>10} [ns]", histogram.max());
  println!("         Mean: {:>10.0} [ns]", histogram.mean());
  println!("      Std Dev: {:>10.0} [ns]", histogram.stdev());
  println!("=============================================================================");
  for v in histogram.iter_recorded() {
    println!(
      "{:>6.1}'th percentile of data is <= {:>5} [ns]  with {:>5} sample(s)",
      v.percentile(),
      v.value_iterated_to(),
      v.count_at_value()
    );
  }
  println!("=============================================================================");
  println!("\n");

  println!("{:>10}   {:>10}    {:>10}    {:>10}\n", "Value", "Percentile", "TotalCount", "1/(1-Percentile)");
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
  }
}
