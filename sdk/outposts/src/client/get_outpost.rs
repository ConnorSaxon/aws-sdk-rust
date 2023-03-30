// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetOutpost`](crate::client::fluent_builders::GetOutpost) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`outpost_id(impl Into<String>)`](crate::client::fluent_builders::GetOutpost::outpost_id) / [`set_outpost_id(Option<String>)`](crate::client::fluent_builders::GetOutpost::set_outpost_id): <p> The ID or the Amazon Resource Name (ARN) of the Outpost. </p>
                            /// - On success, responds with [`GetOutpostOutput`](crate::output::GetOutpostOutput) with field(s):
    ///   - [`outpost(Option<Outpost>)`](crate::output::GetOutpostOutput::outpost): <p>Information about an Outpost.</p>
                            /// - On failure, responds with [`SdkError<GetOutpostError>`](crate::error::GetOutpostError)
    pub fn get_outpost(&self) -> crate::client::fluent_builders::GetOutpost {
                                crate::client::fluent_builders::GetOutpost::new(self.handle.clone())
                            }
}

