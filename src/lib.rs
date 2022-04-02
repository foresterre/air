/// This module implements generic often used instances.
mod reporter;

pub use reporter::GenericReporter;
use std::sync::mpsc;

/// Trait which handle reporting an event, to be implemented by a specific reporter.
///
/// Importantly, local instances don't want to deal with wiring up instances like mpsc queues,
/// instead the instance implementing `Report` may deal with the wiring-up.
///
/// This allows event reporting in user code to focus on the reporting of events themselves.
/// ```no_run
/// # use air::{Report, ReportType};
///
/// fn run_program<T: ReportType, R: Report<T>>(reporter: &mut R) {
///     let event = MyEvent {
///         name: "Christopher".into(),
///     };
///
///     reporter.report_event(event)
/// }
///  
/// ```  
pub trait Report<T> {
    fn report_event<E>(&mut self, event: E)
    where
        T: ReportType,
        E: Event<T>;
}

// #[marker_trait]
pub trait ReportType {}

// todo: possibly not the best of names, acts more like an 'envelope'.
pub trait Event<T: ReportType> {
    type Message: Send;

    // ... If we use an Id here, we can let `prepare_event` send a formatted
    // message instead
    //
    // ... However we're closing in on a system where the message itself, instead
    // implements Format<Json> (|Message| -> String) and Format<Human> (|Message| -> Vec<u8>)
    //
    // ... Then, a reporter constructs a Message, sends it to some inbox (a reporter),
    // , in the inbox its transformed from the Message type to some type which a writer can
    // deal with (String, Vec<u8>, Json-like but in Rust types, ...),
    //
    fn id() -> &'static str; // Can also make a generic type `Id` in trait signature.

    /// Send an event by constructing a message, and pushing it onto
    /// the queue.
    ///
    /// ```no_run
    /// use std::sync::mpsc::Sender;
    /// use air::{Event, ReportType};
    ///
    /// // Assume we have the following report type
    /// struct MyJson;
    /// impl ReportType for MyJson {}
    ///
    /// // Now create an event
    /// struct HelloWorld;
    ///
    /// impl Event<MyJson> for HelloWorld {
    ///     type Message = String;
    ///
    ///     fn prepare_event(&self, sender: Sender<Self::Message>) {
    ///         let _ = sender.send("Hello World".into());
    ///     }
    /// }
    ///
    /// ```
    // TODO: error handling
    //
    // TODO this method has to decide between being a formatter, ie producing
    //  a formatted message and sending the formatted message on the bus
    //  ..
    //  or being a glorified message sender, which hides complexities from the
    //  actual Report::report_event caller, but let's others deal with formatting;
    //      in this case, the impl Event<FormatType> pattern may not work, as Message is not
    //      of FormatType but still the message type.
    fn prepare_event(&self, sender: mpsc::Sender<Self::Message>) {
        sender.send()
    }
}
