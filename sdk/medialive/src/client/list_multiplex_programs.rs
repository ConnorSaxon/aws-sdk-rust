// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListMultiplexPrograms`](crate::client::fluent_builders::ListMultiplexPrograms) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListMultiplexPrograms::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListMultiplexPrograms::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListMultiplexPrograms::set_max_results): The maximum number of items to return.
    ///   - [`multiplex_id(impl Into<String>)`](crate::client::fluent_builders::ListMultiplexPrograms::multiplex_id) / [`set_multiplex_id(Option<String>)`](crate::client::fluent_builders::ListMultiplexPrograms::set_multiplex_id): The ID of the multiplex that the programs belong to.
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListMultiplexPrograms::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListMultiplexPrograms::set_next_token): The token to retrieve the next page of results.
                            /// - On success, responds with [`ListMultiplexProgramsOutput`](crate::output::ListMultiplexProgramsOutput) with field(s):
    ///   - [`multiplex_programs(Option<Vec<MultiplexProgramSummary>>)`](crate::output::ListMultiplexProgramsOutput::multiplex_programs): List of multiplex programs.
    ///   - [`next_token(Option<String>)`](crate::output::ListMultiplexProgramsOutput::next_token): Token for the next ListMultiplexProgram request.
                            /// - On failure, responds with [`SdkError<ListMultiplexProgramsError>`](crate::error::ListMultiplexProgramsError)
    pub fn list_multiplex_programs(&self) -> crate::client::fluent_builders::ListMultiplexPrograms {
                                crate::client::fluent_builders::ListMultiplexPrograms::new(self.handle.clone())
                            }
}

