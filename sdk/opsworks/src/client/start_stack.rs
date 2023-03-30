// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartStack`](crate::client::fluent_builders::StartStack) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`stack_id(impl Into<String>)`](crate::client::fluent_builders::StartStack::stack_id) / [`set_stack_id(Option<String>)`](crate::client::fluent_builders::StartStack::set_stack_id): <p>The stack ID.</p>
                            /// - On success, responds with [`StartStackOutput`](crate::output::StartStackOutput)
                            /// - On failure, responds with [`SdkError<StartStackError>`](crate::error::StartStackError)
    pub fn start_stack(&self) -> crate::client::fluent_builders::StartStack {
                                crate::client::fluent_builders::StartStack::new(self.handle.clone())
                            }
}

