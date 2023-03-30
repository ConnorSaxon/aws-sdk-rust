// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SignalResource`](crate::client::fluent_builders::SignalResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`stack_name(impl Into<String>)`](crate::client::fluent_builders::SignalResource::stack_name) / [`set_stack_name(Option<String>)`](crate::client::fluent_builders::SignalResource::set_stack_name): <p>The stack name or unique stack ID that includes the resource that you want to signal.</p>
    ///   - [`logical_resource_id(impl Into<String>)`](crate::client::fluent_builders::SignalResource::logical_resource_id) / [`set_logical_resource_id(Option<String>)`](crate::client::fluent_builders::SignalResource::set_logical_resource_id): <p>The logical ID of the resource that you want to signal. The logical ID is the name of the resource that given in the template.</p>
    ///   - [`unique_id(impl Into<String>)`](crate::client::fluent_builders::SignalResource::unique_id) / [`set_unique_id(Option<String>)`](crate::client::fluent_builders::SignalResource::set_unique_id): <p>A unique ID of the signal. When you signal Amazon EC2 instances or Auto Scaling groups, specify the instance ID that you are signaling as the unique ID. If you send multiple signals to a single resource (such as signaling a wait condition), each signal requires a different unique ID.</p>
    ///   - [`status(ResourceSignalStatus)`](crate::client::fluent_builders::SignalResource::status) / [`set_status(Option<ResourceSignalStatus>)`](crate::client::fluent_builders::SignalResource::set_status): <p>The status of the signal, which is either success or failure. A failure signal causes CloudFormation to immediately fail the stack creation or update.</p>
                            /// - On success, responds with [`SignalResourceOutput`](crate::output::SignalResourceOutput)
                            /// - On failure, responds with [`SdkError<SignalResourceError>`](crate::error::SignalResourceError)
    pub fn signal_resource(&self) -> crate::client::fluent_builders::SignalResource {
                                crate::client::fluent_builders::SignalResource::new(self.handle.clone())
                            }
}

