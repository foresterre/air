use air::{Event, Report, ReportType, Reporter};
use std::io::{BufRead, Stdout, Write};

struct MyEvent {
    name: String,
}

pub struct HumanOutput;
pub struct JsonOutput;
pub struct OutputDisabled;

impl ReportType for HumanOutput {}
impl ReportType for JsonOutput {}
impl ReportType for OutputDisabled {}

impl Event<HumanOutput> for MyEvent {
    fn write_fmt<W>(&self, _writer: &mut W)
    where
        W: Write,
    {
        _writer.write_fmt(format_args!("Hi {} 👋", &self.name));
    }
}
impl Event<JsonOutput> for MyEvent {
    fn write_fmt<W>(&self, _writer: &mut W)
    where
        W: Write,
    {
        let object = json::object! {
            "name": self.name.as_str()
        };

        _writer.write_fmt(format_args!("{}", object));
    }
}

fn main() {
    let mut choice = String::with_capacity(32);
    {
        std::io::stdin().lock().read_line(&mut choice);
    }

    match choice.as_str().trim() {
        "json" => {
            let mut r = Reporter::<JsonOutput, Stdout>::new(std::io::stdout());
            run_program(&mut r);
        }
        _ => {
            let mut r = Reporter::<HumanOutput, Stdout>::new(std::io::stdout());
            run_program(&mut r);
        }
    }
}

fn run_program<T: ReportType, R: Report<T>>(reporter: &mut R)
where
    MyEvent: Event<T>,
{
    let event = MyEvent {
        name: "Christopher".into(),
    };

    reporter.report_event(event);
}
