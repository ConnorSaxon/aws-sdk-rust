// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetAccessor`](crate::client::fluent_builders::GetAccessor) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`accessor_id(impl Into<String>)`](crate::client::fluent_builders::GetAccessor::accessor_id) / [`set_accessor_id(Option<String>)`](crate::client::fluent_builders::GetAccessor::set_accessor_id): <p>The unique identifier of the accessor.</p>
                            /// - On success, responds with [`GetAccessorOutput`](crate::output::GetAccessorOutput) with field(s):
    ///   - [`accessor(Option<Accessor>)`](crate::output::GetAccessorOutput::accessor): <p>The properties of the accessor.</p>
                            /// - On failure, responds with [`SdkError<GetAccessorError>`](crate::error::GetAccessorError)
    pub fn get_accessor(&self) -> crate::client::fluent_builders::GetAccessor {
                                crate::client::fluent_builders::GetAccessor::new(self.handle.clone())
                            }
}

