use std::env;

mod day01_21;
mod day02_21;
mod day03_21;
mod day04_21;
mod day05_21;
mod day05_21;
mod tools;
mod chars;

use plotly::common::{ErrorData, ErrorType, Marker, Title, Font};
use plotly::layout::{Axis, BarMode, Layout, };
use plotly::{Bar, NamedColor, Plot};

fn create_graph(data: &Vec<(usize, (f32,f32,f32))>, errors: &Vec<(usize, (f64,f64,f64))>) {
    let xlabels: Vec<String> = data.iter().map(|n| format!("day {}", n.0+1)).collect();

    // let init_vec: Vec<f32> = data.iter().map(|n| n.1.0).collect();
    let part1_vec: Vec<f32> = data.iter().map(|n| n.1.1).collect();
    let part2_vec: Vec<f32> = data.iter().map(|n| n.1.2).collect();

    // let init_error: Vec<f64> = errors.iter().map(|n| n.1.0).collect();
    let part1_error: Vec<f64> = errors.iter().map(|n| n.1.1).collect();
    let part2_error: Vec<f64> = errors.iter().map(|n| n.1.2).collect();

    // let trace1 = Bar::new(xlabels.clone(), init_vec).name("Initialization")
    //     .marker(Marker::new().color(NamedColor::Red))
    //     .error_y(ErrorData::new(ErrorType::Data).array(init_error));
    let trace2 = Bar::new(xlabels.clone(), part1_vec).name("Part 1")
        .marker(Marker::new().color(NamedColor::Blue))
        .error_y(ErrorData::new(ErrorType::Data).array(part1_error));
    let trace3 = Bar::new(xlabels.clone(), part2_vec).name("Part 2")
        .marker(Marker::new().color(NamedColor::Green))
        .error_y(ErrorData::new(ErrorType::Data).array(part2_error));

    let layout = Layout::new().bar_mode(BarMode::Group)
        .title(Title::new("Runtimes in ms for Advent of Code 2021").font(Font::new().color(NamedColor::Black).size(24).family("Droid Serif")))
        .x_axis(Axis::new().title(Title::new("Day").font(Font::new().color(NamedColor::Black).size(12).family("Droid Serif"))))
        .y_axis(Axis::new().title(Title::new("Runtime in ms").font(Font::new().color(NamedColor::Black).size(12).family("Droid Serif"))).range(vec![0, 100]));

    let mut plot = Plot::new();
    // plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.set_layout(layout);

    // let mut file = File::create("images/runtimes.html")?;
    plot.to_html("images/runtimes.html");
    // let html = plot.to_inline_html(Some("runtimes"));
    // file.write_all(html.as_bytes());
}

fn main() {
    let days: Vec<(&str, fn(bool, bool) -> (u128, u128, u128), usize)> = vec![
        ("Day 01 of 2021", day01_21::run, 500),
        ("Day 02 of 2021", day02_21::run, 500),
        ("Day 03 of 2021", day03_21::run, 500),
        ("Day 04 of 2021", day04_21::run, 500),
        ("Day 05 of 2021", day05_21::run, 50),
        ("Day 06 of 2021", day06_21::run, 50),
        // ("Day 07 of 2021", day07_21::run, 50),
        // ("Day 08 of 2021", day08_21::run, 50),
        // ("Day 09 of 2021", day09_21::run, 50),
        // ("Day 10 of 2021", day10_21::run, 50),
        // ("Day 11 of 2021", day11_21::run, 50),
        // ("Day 12 of 2021", day12_21::run, 50),
        // ("Day 13 of 2021", day13_21::run, 50),
        // ("Day 14 of 2021", day14_21::run, 10),
        // ("Day 15 of 2021", day15_21::run, 10),
        // ("Day 16 of 2021", day16_21::run, 50),
        // ("Day 17 of 2021", day17_21::run, 10),
        // ("Day 18 of 2021", day18_21::run, 50),
        // ("Day 19 of 2021", day19_21::run, 10),
        // ("Day 20 of 2021", day20_21::run, 50),
        // ("Day 21 of 2021", day21_21::run, 50),
        // ("Day 22 of 2021", day22_21::run, 50),
        // ("Day 23 of 2021", day23_21::run, 50),
        // ("Day 24 of 2021", day24_21::run, 50),
        // ("Day 25 of 2021", day25_21::run, 50),
    ];

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Please specify the day to run!");
        return;
    }

    if let Ok(real) = args[2].parse::<bool>() {
        println!("-----------------------------------------------------------------------------------------------------------------------");
        if args[1] == "performance" {
            println!("Running performance check");
            println!("");
            let mut data: Vec<(usize,(f32,f32,f32))> = vec![];
            let mut errors: Vec<(usize,(f64,f64,f64))> = vec![];
            let mut totals: (f64,f64,f64) = (0.0f64,0.0f64,0.0f64);
            for (i, (name, day, runs)) in days.iter().enumerate() {
                let mut timings: Vec<(u128, u128, u128)> = vec![];
                for _ in 0..*runs {
                    timings.push(day(real, false));
                }
                let total = timings.iter().fold((0, 0, 0), |acc, val| {
                    (acc.0 + val.0, acc.1 + val.1, acc.2 + val.2)
                });

                let mean = (
                    total.0 as f64 / *runs as f64,
                    total.1 as f64 / *runs as f64,
                    total.2 as f64 / *runs as f64,
                );
                totals.0 += mean.0; totals.1 += mean.1; totals.2 += mean.2;

                let variance = timings
                    .iter()
                    .map(|(v1, v2, v3)| {
                        let diff = (
                            mean.0 - (*v1 as f64),
                            mean.1 - (*v2 as f64),
                            mean.2 - (*v3 as f64),
                        );

                        (diff.0 * diff.0, diff.1 * diff.1, diff.2 * diff.2)
                    })
                    .fold((0.0, 0.0, 0.0), |acc, val| {
                        (
                            acc.0 + val.0 / (*runs as f64),
                            acc.1 + val.1 / (*runs as f64),
                            acc.2 + val.2 / (*runs as f64),
                        )
                    });

                println!("{} - init: {:>10.4} +/- {:>10.4} ms, p1: {:>10.4} +/- {:>10.4} ms, p2: {:>10.4} +/- {:>10.4} ms", 
                    name, 
                    mean.0 / 1_000_000f64, variance.0.sqrt() / 1_000_000f64, 
                    mean.1 / 1_000_000f64, variance.1.sqrt() / 1_000_000f64, 
                    mean.2 / 1_000_000f64, variance.2.sqrt() / 1_000_000f64
                );

                data.push( (i, ((mean.0/1_000_000f64) as f32,(mean.1/1_000_000f64) as f32,(mean.2/1_000_000f64) as f32 ) ) );
                errors.push( (i, ( variance.0.sqrt() / 1_000_000f64, variance.1.sqrt() / 1_000_000f64, variance.2.sqrt() / 1_000_000f64 )) );
            }

            let grand_total = (totals.0 + totals.1 + totals.2)/1_000_000_000f64;
            println!("{} in {:>4.2} - init: {:>10.4} +/- {:>10.4}  s, p1: {:>10.4} +/- {:>10.4}  s, p2: {:>10.4} +/- {:>10.4}  s", 
                "total:", grand_total, 
                totals.0 as f64 / 1_000_000_000f64, 0, 
                totals.1 as f64 / 1_000_000_000f64, 0, 
                totals.2 as f64 / 1_000_000_000f64, 0
            );
            create_graph(&data, &errors);
        } else if args[1] == "all" {
            for (name, day, _) in days.iter() {
                let timing = day(real, false);
                println!(
                    "{} - init: {:10} ns, p1: {:10} ns, p2: {:10} ns",
                    name, timing.0, timing.1, timing.2
                );
            }
        } else {
            if let Ok(i) = args[1].parse::<usize>() {
                println!("{}", days[i - 1].0);
                println!("");
                let timing = days[i - 1].1(real, true);
                println!("");
                println!(
                    "Timing - init: {:10} ns, p1: {:10} ns, p2: {:10} ns",
                    timing.0, timing.1, timing.2
                );
            }
        }
        println!("-----------------------------------------------------------------------------------------------------------------------");
    }
}
