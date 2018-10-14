//! Command-line interface and command-line argument parsing. Uses [clap] under
//! the hood.

use clap::{AppSettings, Arg};

use algorithms;
use algorithms::Algorithm;

/// [Internal name](clap::Arg::with_name) of the
/// [algorithm](Options::algorithm) argument which is used to
/// [get its value](clap::ArgMatches::value_of).
const ALGORITHM_ARG: &str = "ALGORITHM";
/// [Internal name](clap::Arg::with_name) of the
/// [`--min`/`-a`](Options::min) option which is used to
/// [get its value](clap::ArgMatches::value_of).
const MIN_OPT: &str = "MIN";
/// [Internal name](clap::Arg::with_name) of the
/// [`--max`/`-b`](Options::max) option which is used to
/// [get its value](clap::ArgMatches::value_of).
const MAX_OPT: &str = "MAX";
/// [Internal name](clap::Arg::with_name) of the
/// [`--order`/`-o`](Options::order) option which is used to
/// [get its value](clap::ArgMatches::value_of).
const ORDER_OPT: &str = "ORDER";

/// Contains all options that can be provided by a user using the CLI.
pub struct Options {
  /// Instance of a sorting [algorithm](Algorithm) struct.
  pub algorithm: Box<dyn Algorithm + Send>,
  /// Min value in the [array](::array::Array).
  pub min: u32,
  /// Max values in the [array](::array::Array).
  pub max: u32,
  /// Order of elements in the [array](::array::Array).
  pub order: Order,
}

/// Order of elements in the [array](::array::Array).
pub enum Order {
  /// Sorted in the ascending order.
  Sorted,
  /// Sorted in the descending order.
  Reversed,
  /// [Shuffled](::rand::Rng::shuffle).
  Shuffled,
}

/// Parses command-line arguments into [`Options`]. **If the help/version is
/// printed or an error occurs, it will be displayed to the user and the process
/// will exit.**
///
/// _See_ [`clap::App.get_matches`](clap::App::get_matches)
pub fn parse_options() -> Options {
  let parser = app_from_crate!()
    .setting(AppSettings::NextLineHelp)
    .setting(AppSettings::ColoredHelp)
    .arg(
      Arg::with_name(MIN_OPT)
        .short("a")
        .long("min")
        .help("Sets min value in the array")
        .default_value("1"),
    ).arg(
      Arg::with_name(MAX_OPT)
        .short("b")
        .long("max")
        .help("Sets max value in the array")
        .default_value("100"),
    ).arg(
      Arg::with_name(ORDER_OPT)
        .short("o")
        .long("order")
        .help("Sets order of elements in the array")
        .possible_values(&["sorted", "reversed", "shuffled"])
        .case_insensitive(true)
        .default_value("shuffled"),
    ).arg(
      Arg::with_name(ALGORITHM_ARG)
        .help("Sets sorting algorithm")
        .possible_values(&[
          "bubble",
          "cycle",
          "gnome",
          "insertion",
          "quicksort",
          "selection",
        ]).case_insensitive(true)
        .required(true),
    );

  let matches = parser.get_matches();

  Options {
    algorithm: match matches.value_of(ALGORITHM_ARG).unwrap() {
      "bubble" => Box::new(algorithms::BubbleSort),
      "cycle" => Box::new(algorithms::CycleSort),
      "gnome" => Box::new(algorithms::GnomeSort),
      "insertion" => Box::new(algorithms::InsertionSort),
      "quicksort" => Box::new(algorithms::Quicksort),
      "selection" => Box::new(algorithms::SelectionSort),
      _ => unreachable!(),
    },

    min: value_t_or_exit!(matches, MIN_OPT, u32),
    max: value_t_or_exit!(matches, MAX_OPT, u32),

    order: match matches.value_of(ORDER_OPT).unwrap() {
      "sorted" => Order::Sorted,
      "reversed" => Order::Reversed,
      "shuffled" => Order::Shuffled,
      _ => unreachable!(),
    },
  }
}
