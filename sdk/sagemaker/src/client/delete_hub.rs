// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteHub`](crate::client::fluent_builders::DeleteHub) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`hub_name(impl Into<String>)`](crate::client::fluent_builders::DeleteHub::hub_name) / [`set_hub_name(Option<String>)`](crate::client::fluent_builders::DeleteHub::set_hub_name): <p>The name of the hub to delete.</p>
                            /// - On success, responds with [`DeleteHubOutput`](crate::output::DeleteHubOutput)
                            /// - On failure, responds with [`SdkError<DeleteHubError>`](crate::error::DeleteHubError)
    pub fn delete_hub(&self) -> crate::client::fluent_builders::DeleteHub {
                                crate::client::fluent_builders::DeleteHub::new(self.handle.clone())
                            }
}

