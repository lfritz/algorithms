extern crate algorithms;
extern crate docopt;
extern crate rand;
extern crate rustc_serialize;
extern crate time;

use docopt::Docopt;
use rand::Rng;
use rand::ThreadRng;

static USAGE: &'static str = "
Usage: sort_compare [-n N] [-t T] <algorithm1> <algorithm2>
       sort_compare --help

Options:
    -h --help   Show this message
    -n N        Set input size [default: 1000]
    -t T        Set number of repetitions [default: 100]
";

type SortFunction = fn(&mut Vec<i64>) -> ();

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_algorithm1: String,
    arg_algorithm2: String,
    flag_n: Option<usize>,
    flag_t: Option<usize>,
}

fn time_sort(rng: &mut ThreadRng,
             sort: SortFunction,
             n: usize,
             t: usize) -> i64 {
    let mut total = 0;
    for _ in 0..t {
        let mut input: Vec<i64> = Vec::new();
        for _ in 0..n {
            input.push(rng.gen());
        }
        let start_time = time::get_time();
        sort(&mut input);
        let end_time = time::get_time();
        total += (end_time - start_time).num_microseconds().unwrap();
        assert!(algorithms::sorting::is_sorted(&input));
    }
    total
}

fn get_sort(name: &String) -> SortFunction {
    match name.as_ref() {
        "heap"      => algorithms::sorting::heap::sort::sort,
        "quick"     => algorithms::sorting::quick::sort,
        "merge_bu"  => algorithms::sorting::merge::bottom_up::sort,
        "merge_td"  => algorithms::sorting::merge::top_down::sort,
        "shell"     => algorithms::sorting::shell::sort,
        "insertion" => algorithms::sorting::insertion::sort,
        _           => algorithms::sorting::selection::sort,
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let n = args.flag_n.unwrap_or(1000);
    let t = args.flag_n.unwrap_or(100);

    let mut rng = rand::thread_rng();
    let t1 = time_sort(&mut rng, get_sort(&args.arg_algorithm1), n, t);
    let t2 = time_sort(&mut rng, get_sort(&args.arg_algorithm2), n, t);

    println!("For {} random ints", n);
    println!("    {} sort is {:.2} times faster than {} sort",
             args.arg_algorithm1,
             (t2 as f64) / (t1 as f64),
             args.arg_algorithm2);
}
