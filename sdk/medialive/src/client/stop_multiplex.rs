// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopMultiplex`](crate::client::fluent_builders::StopMultiplex) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`multiplex_id(impl Into<String>)`](crate::client::fluent_builders::StopMultiplex::multiplex_id) / [`set_multiplex_id(Option<String>)`](crate::client::fluent_builders::StopMultiplex::set_multiplex_id): The ID of the multiplex.
                            /// - On success, responds with [`StopMultiplexOutput`](crate::output::StopMultiplexOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::StopMultiplexOutput::arn): The unique arn of the multiplex.
    ///   - [`availability_zones(Option<Vec<String>>)`](crate::output::StopMultiplexOutput::availability_zones): A list of availability zones for the multiplex.
    ///   - [`destinations(Option<Vec<MultiplexOutputDestination>>)`](crate::output::StopMultiplexOutput::destinations): A list of the multiplex output destinations.
    ///   - [`id(Option<String>)`](crate::output::StopMultiplexOutput::id): The unique id of the multiplex.
    ///   - [`multiplex_settings(Option<MultiplexSettings>)`](crate::output::StopMultiplexOutput::multiplex_settings): Configuration for a multiplex event.
    ///   - [`name(Option<String>)`](crate::output::StopMultiplexOutput::name): The name of the multiplex.
    ///   - [`pipelines_running_count(i32)`](crate::output::StopMultiplexOutput::pipelines_running_count): The number of currently healthy pipelines.
    ///   - [`program_count(i32)`](crate::output::StopMultiplexOutput::program_count): The number of programs in the multiplex.
    ///   - [`state(Option<MultiplexState>)`](crate::output::StopMultiplexOutput::state): The current state of the multiplex.
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::StopMultiplexOutput::tags): A collection of key-value pairs.
                            /// - On failure, responds with [`SdkError<StopMultiplexError>`](crate::error::StopMultiplexError)
    pub fn stop_multiplex(&self) -> crate::client::fluent_builders::StopMultiplex {
                                crate::client::fluent_builders::StopMultiplex::new(self.handle.clone())
                            }
}

