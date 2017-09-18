#![feature(test)]

extern crate stopwatch;
extern crate test;
extern crate time;

use std::env;
use std::fmt;
use time::Duration;
use stopwatch::Stopwatch;

pub struct Measure {
    runs: usize,
    mean: i64,
    percentiles: Vec<(f32, i64)>,
}

impl fmt::Display for Measure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "took mean {}ns", self.mean / (self.runs as i64))?;

        for &(p, n) in &self.percentiles {
            writeln!(f, "Percentile {}%: {}ns", p * 100f32, n)?;
        }

        Ok(())
    }
}

pub fn parse_runs_from_env() -> usize {
    let default_runs = 200;

    let mut args = env::args();

    // Get the command name
    let _ = args.next().unwrap();

    // Get the first argument as a usize
    if args.len() >= 1 {
        args.next()
            .unwrap()
            .parse::<usize>()
            .unwrap_or(default_runs)
    } else {
        default_runs
    }
}

pub fn run<F, FOut>(runs: usize, mut f: F) -> Measure
where
    F: FnMut() -> FOut,
{
    let mut results = Vec::<i64>::with_capacity(runs as usize);
    for _ in 0..runs {
        let mut sw = Stopwatch::start_new();
        let res = f();
        sw.stop();

        test::black_box(res);

        let elapsed = Duration::from_std(sw.elapsed()).unwrap();
        results.push(elapsed.num_nanoseconds().unwrap());
    }

    results.sort();

    let mean: i64 = results.iter().sum();
    let pv = percentiles(&results, runs as f32);

    Measure {
        runs: runs,
        mean: mean,
        percentiles: pv,
    }
}

fn percentiles(data: &Vec<i64>, runs: f32) -> Vec<(f32, i64)> {
    vec![0.50, 0.66, 0.75, 0.80, 0.90, 0.95, 0.98, 0.99, 1.00]
        .iter()
        .map(|p| {
            let p: f32 = *p;
            let i: usize = (p * runs) as usize;
            (p, data.get(i - 1).unwrap().to_owned())
        })
        .collect()
}
