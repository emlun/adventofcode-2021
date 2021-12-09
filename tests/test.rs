use adventofcode_2021::common::day_input_filename;
use adventofcode_2021::common::get_file_lines;
use adventofcode_2021::days;

fn test_day(day: u8, correct_a: &str, correct_b: &str) -> Result<(), std::io::Error> {
    let solve = days::get_solver(day).unwrap();
    let input_lines = get_file_lines(&day_input_filename(day))?;
    let (solution_a, solution_b) = solve(&input_lines);
    assert_eq!(
        solution_a.as_str(),
        correct_a,
        "Incorrect solution for day {}a",
        day
    );
    assert_eq!(
        solution_b.as_str(),
        correct_b,
        "Incorrect solution for day {}b",
        day
    );

    Ok(())
}

macro_rules! test_day {
    ($name: ident, $sol_a: literal, $sol_b: literal) => {
        #[test]
        fn $name() -> Result<(), std::io::Error> {
            let day_name = stringify!($name);
            let day_num: u8 = day_name[3..].parse().unwrap();
            test_day(day_num, $sol_a, $sol_b)
        }
    };
}

test_day!(day01, "1715", "1739");
test_day!(day02, "1635930", "1781819478");
test_day!(day03, "1071734", "6124992");
test_day!(day04, "28082", "8224");
test_day!(day05, "4993", "21101");
test_day!(day06, "386755", "1732731810807");
test_day!(day07, "355150", "98368490");
test_day!(day08, "245", "983026");
test_day!(day09, "537", "1142757");
