// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchStart`](crate::client::fluent_builders::BatchStart) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`channel_ids(Vec<String>)`](crate::client::fluent_builders::BatchStart::channel_ids) / [`set_channel_ids(Option<Vec<String>>)`](crate::client::fluent_builders::BatchStart::set_channel_ids): List of channel IDs
    ///   - [`multiplex_ids(Vec<String>)`](crate::client::fluent_builders::BatchStart::multiplex_ids) / [`set_multiplex_ids(Option<Vec<String>>)`](crate::client::fluent_builders::BatchStart::set_multiplex_ids): List of multiplex IDs
                            /// - On success, responds with [`BatchStartOutput`](crate::output::BatchStartOutput) with field(s):
    ///   - [`failed(Option<Vec<BatchFailedResultModel>>)`](crate::output::BatchStartOutput::failed): List of failed operations
    ///   - [`successful(Option<Vec<BatchSuccessfulResultModel>>)`](crate::output::BatchStartOutput::successful): List of successful operations
                            /// - On failure, responds with [`SdkError<BatchStartError>`](crate::error::BatchStartError)
    pub fn batch_start(&self) -> crate::client::fluent_builders::BatchStart {
                                crate::client::fluent_builders::BatchStart::new(self.handle.clone())
                            }
}

