trait Printer {
    fn print(&self, fizzbuzz: FizzBuzzAnswer);
}

trait Comparator {
    fn divisible(&self, number: u32) -> FizzBuzzAnswer;
}

enum FizzBuzzAnswer {
    Fizz,
    Buzz,
    FizzBuzz,
    None,
}

struct PrettyPrinter {}

struct FizzBuzzSolver<P: Printer, CMP: Comparator> {
    printer: P,
    comparator: CMP,
}

impl<P: Printer, CMP: Comparator> FizzBuzzSolver<P, CMP> {
    fn new(printer: P, comparator: CMP) -> Self {
        Self {
            printer,
            comparator,
        }
    }

    fn solver(&self, number: u32) {
        let res = self.comparator.divisible(number);
        self.printer.print(res);
    }
}

struct BrokenPrinter {}

impl Printer for PrettyPrinter {
    fn print(&self, fizzbuzz: FizzBuzzAnswer) {
        match fizzbuzz {
            FizzBuzzAnswer::Fizz => println!("fizz"),
            FizzBuzzAnswer::Buzz => println!("buzz"),
            FizzBuzzAnswer::FizzBuzz => println!("fizzbuzz"),
            FizzBuzzAnswer::None => {}
        }
    }
}

impl Printer for BrokenPrinter {
    fn print(&self, fizzbuzz: FizzBuzzAnswer) {
        match fizzbuzz {
            FizzBuzzAnswer::Fizz => println!("???"),
            FizzBuzzAnswer::Buzz => println!("???"),
            FizzBuzzAnswer::FizzBuzz => println!("???"),
            FizzBuzzAnswer::None => println!("i'm broken : ("),
        }
    }
}

struct GoodComparator {}

impl Comparator for GoodComparator {
    fn divisible(&self, number: u32) -> FizzBuzzAnswer {
        if number % 3 == 0 && number % 5 == 0 {
            FizzBuzzAnswer::FizzBuzz
        } else if number % 5 == 0 {
            FizzBuzzAnswer::Buzz
        } else if number % 3 == 0 {
            FizzBuzzAnswer::Fizz
        } else {
            FizzBuzzAnswer::None
        }
    }
}

struct BadComparator {}

impl Comparator for BadComparator {
    fn divisible(&self, number: u32) -> FizzBuzzAnswer {
        FizzBuzzAnswer::None
    }
}

fn main() {
    let ok_solver = FizzBuzzSolver::new(PrettyPrinter {}, GoodComparator {});
    ok_solver.solver(9);

    let not_ok_solver = FizzBuzzSolver::new(BrokenPrinter {}, GoodComparator {});
    not_ok_solver.solver(9);

    let not_ok_solver = FizzBuzzSolver::new(PrettyPrinter {}, BadComparator {});
    not_ok_solver.solver(9);
}
