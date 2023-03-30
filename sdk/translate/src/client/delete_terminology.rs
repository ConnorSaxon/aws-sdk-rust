// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteTerminology`](crate::client::fluent_builders::DeleteTerminology) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeleteTerminology::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeleteTerminology::set_name): <p>The name of the custom terminology being deleted. </p>
                            /// - On success, responds with [`DeleteTerminologyOutput`](crate::output::DeleteTerminologyOutput)
                            /// - On failure, responds with [`SdkError<DeleteTerminologyError>`](crate::error::DeleteTerminologyError)
    pub fn delete_terminology(&self) -> crate::client::fluent_builders::DeleteTerminology {
                                crate::client::fluent_builders::DeleteTerminology::new(self.handle.clone())
                            }
}

