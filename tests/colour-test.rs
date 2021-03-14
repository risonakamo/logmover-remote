use colored::Colorize;
use yansi::Paint;

fn main()
{
    Paint::enable_windows_ascii();
    println!("{}","asdad".green());
    println!("{}",Paint::green("asdddd"));
}