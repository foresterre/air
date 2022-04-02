use crate::{Event, Report, ReportType};
use std::io::Write;
use std::marker::PhantomData;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

/// A generic instance of a reporter.
///
/// It takes events `e`, formats these into their reportable format `f(e)`, and
/// then sends the formatted events to a writer `send(f(e))`.
///
/// The reporter is bound on a `ReportType`, `T`, such that only events of the correct format are
/// ingested.
///
/// It's also bound on a message `Message`, as events must at some point leave their ephemeral state
/// and take shape (i.e. be instantiated).
#[derive(Debug)]
pub struct GenericReporter<T: ReportType, Message> {
    output_type: PhantomData<T>,
    sender: Sender<Message>,
}

impl<T: ReportType, Message> GenericReporter<T, Message> {
    /// Create a new instance of a generic reporter, bound to a specific type of reports `T`.
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            output_type: PhantomData,
            sender,
        }
    }
}

impl<T, W> Report<T> for GenericReporter<T, W>
where
    T: ReportType,
    W: std::io::Write,
{
    fn report_event<E>(&mut self, event: E)
    where
        E: Event<T>,
    {
        event.prepare_event(self.sender);
    }
}

/// A generic instance of a writer.
///
/// It receives events through a mpsc queue, and writes said events to `W`.
pub struct GenericWriter<W: Write, Message> {
    receiver: Receiver<Message>,
    writer: W,
}

impl<W: Write, Message> GenericWriter<W, Message> {
    fn new(receiver: Receiver<Message>, writer: W) -> Self {
        Self { receiver, writer }
    }
}
