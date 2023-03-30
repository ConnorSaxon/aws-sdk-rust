// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteComponent`](crate::client::fluent_builders::DeleteComponent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`component_build_version_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteComponent::component_build_version_arn) / [`set_component_build_version_arn(Option<String>)`](crate::client::fluent_builders::DeleteComponent::set_component_build_version_arn): <p>The Amazon Resource Name (ARN) of the component build version to delete.</p>
                            /// - On success, responds with [`DeleteComponentOutput`](crate::output::DeleteComponentOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::output::DeleteComponentOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`component_build_version_arn(Option<String>)`](crate::output::DeleteComponentOutput::component_build_version_arn): <p>The Amazon Resource Name (ARN) of the component build version that was deleted.</p>
                            /// - On failure, responds with [`SdkError<DeleteComponentError>`](crate::error::DeleteComponentError)
    pub fn delete_component(&self) -> crate::client::fluent_builders::DeleteComponent {
                                crate::client::fluent_builders::DeleteComponent::new(self.handle.clone())
                            }
}

