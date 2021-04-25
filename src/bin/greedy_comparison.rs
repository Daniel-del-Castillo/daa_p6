use std::fs::File;
use std::io::Result;
use std::io::Write;
use std::time::Instant;

use clap::{App, Arg, ArgMatches};

use daap7::{
    problem_solver::{
        grasp::{
            local_search::NoSearch,
            stop_criterion::{IterationsWithoutChange, TotalIterations},
        },
        FastGreedySolver, GreedySolver, ProblemSolver, RandomizedGreedySolver, GRASP,
    },
    ProblemInstance,
};

const FILE_EXPLANATION: &'static str =
    "The file with the problem instance. It should have the following format:
(You should substitute the {} with the correct values and use a tab as separator):
n:\t{number of tasks}
m:\t{number of machines}
{whatever but without have tabs}\t{list of task times separated by tabs}
{a line, you can put here whatever you want}
{list of setup times to go from inactive to each task}
{list of setup times to go from task 1 to each task}
{list of setup times to go from task 2 to each task}
Continues...\n
- The first column and row of the matrix represent the inactive state
- The matrix must be MxM, being M equal to th number of tasks + 1
- The task times list must have an element for each task";

fn main() -> std::io::Result<()> {
    let matches = get_args();
    let instance = match ProblemInstance::from_file(&matches.value_of("problem_file").unwrap()) {
        Ok(instance) => instance,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };
    let mut output_file = File::create(&matches.value_of("output_file").unwrap())?;
    print_headers(&mut output_file)?;
    for iterations in vec![100, 500, 1000, 5000, 10000] {
        let solver_list: Vec<(String, Box<dyn ProblemSolver>)> = get_solver_list(iterations);
        print_results(&instance, &mut output_file, solver_list)?;
    }
    Ok(())
}

fn get_args() -> ArgMatches<'static> {
    App::new("parallel-machine-scheduling-problem-with-dependent-setup-times")
        .arg(
            Arg::with_name("problem_file")
                .required(true)
                .help(FILE_EXPLANATION)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_file")
                .required(true)
                .help("The file in which the CSV output will be written")
                .takes_value(true),
        )
        .get_matches()
}

fn get_solver_list(iterations: usize) -> Vec<(String, Box<dyn ProblemSolver>)> {
    let mut list: Vec<(String, Box<dyn ProblemSolver>)> = vec![
        (
            format!("{},Greedy solver,", iterations),
            Box::new(GreedySolver::new()),
        ),
        (
            format!("{},Fast Greedy solver,", iterations),
            Box::new(FastGreedySolver::new()),
        ),
    ];
    for k in vec![2, 3, 4] {
        list.push((
            format!("{},Fast Greedy solver k={},", iterations, k),
            Box::new(RandomizedGreedySolver::new(k)),
        ));
        list.push((
            format!("{},GRASP k={} total iterations,", iterations, k),
            Box::new(GRASP::new(
                k,
                NoSearch::new(),
                TotalIterations::new(iterations),
            )),
        ));
        list.push((
            format!("{},GRASP k={} iterations without change,", iterations, k),
            Box::new(GRASP::new(
                k,
                NoSearch::new(),
                IterationsWithoutChange::new(iterations / 10),
            )),
        ));
    }
    list
}

fn print_results(
    instance: &ProblemInstance,
    output_file: &mut File,
    details: Vec<(String, Box<dyn ProblemSolver>)>,
) -> Result<()> {
    for (title, mut solver) in details.into_iter() {
        write!(output_file, "{}", title)?;
        print_result(instance, output_file, &mut solver)?;
    }
    Ok(())
}

fn print_result(
    instance: &ProblemInstance,
    output_file: &mut File,
    solver: &mut Box<dyn ProblemSolver>,
) -> Result<()> {
    let instant = Instant::now();
    let tct = solver.solve(&instance).get_total_completion_time();
    let duration = instant.elapsed();
    write!(output_file, "{},{}\n", tct, duration.as_millis())
}

fn print_headers(file: &mut File) -> Result<()> {
    write!(file, "iterations")?;
    write!(file, "algorithm,")?;
    write!(file, "tct,")?;
    write!(file, "time\n")
}
