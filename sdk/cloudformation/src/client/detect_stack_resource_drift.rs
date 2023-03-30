// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DetectStackResourceDrift`](crate::client::fluent_builders::DetectStackResourceDrift) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`stack_name(impl Into<String>)`](crate::client::fluent_builders::DetectStackResourceDrift::stack_name) / [`set_stack_name(Option<String>)`](crate::client::fluent_builders::DetectStackResourceDrift::set_stack_name): <p>The name of the stack to which the resource belongs.</p>
    ///   - [`logical_resource_id(impl Into<String>)`](crate::client::fluent_builders::DetectStackResourceDrift::logical_resource_id) / [`set_logical_resource_id(Option<String>)`](crate::client::fluent_builders::DetectStackResourceDrift::set_logical_resource_id): <p>The logical name of the resource for which to return drift information.</p>
                            /// - On success, responds with [`DetectStackResourceDriftOutput`](crate::output::DetectStackResourceDriftOutput) with field(s):
    ///   - [`stack_resource_drift(Option<StackResourceDrift>)`](crate::output::DetectStackResourceDriftOutput::stack_resource_drift): <p>Information about whether the resource's actual configuration has drifted from its expected template configuration, including actual and expected property values and any differences detected.</p>
                            /// - On failure, responds with [`SdkError<DetectStackResourceDriftError>`](crate::error::DetectStackResourceDriftError)
    pub fn detect_stack_resource_drift(&self) -> crate::client::fluent_builders::DetectStackResourceDrift {
                                crate::client::fluent_builders::DetectStackResourceDrift::new(self.handle.clone())
                            }
}

