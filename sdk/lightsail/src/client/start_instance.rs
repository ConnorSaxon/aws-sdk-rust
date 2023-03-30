// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartInstance`](crate::client::fluent_builders::StartInstance) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_name(impl Into<String>)`](crate::client::fluent_builders::StartInstance::instance_name) / [`set_instance_name(Option<String>)`](crate::client::fluent_builders::StartInstance::set_instance_name): <p>The name of the instance (a virtual private server) to start.</p>
                            /// - On success, responds with [`StartInstanceOutput`](crate::output::StartInstanceOutput) with field(s):
    ///   - [`operations(Option<Vec<Operation>>)`](crate::output::StartInstanceOutput::operations): <p>An array of objects that describe the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
                            /// - On failure, responds with [`SdkError<StartInstanceError>`](crate::error::StartInstanceError)
    pub fn start_instance(&self) -> crate::client::fluent_builders::StartInstance {
                                crate::client::fluent_builders::StartInstance::new(self.handle.clone())
                            }
}

