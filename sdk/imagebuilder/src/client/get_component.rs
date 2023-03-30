// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetComponent`](crate::client::fluent_builders::GetComponent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`component_build_version_arn(impl Into<String>)`](crate::client::fluent_builders::GetComponent::component_build_version_arn) / [`set_component_build_version_arn(Option<String>)`](crate::client::fluent_builders::GetComponent::set_component_build_version_arn): <p>The Amazon Resource Name (ARN) of the component that you want to retrieve. Regex requires "/\d+$" suffix.</p>
                            /// - On success, responds with [`GetComponentOutput`](crate::output::GetComponentOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::output::GetComponentOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`component(Option<Component>)`](crate::output::GetComponentOutput::component): <p>The component object associated with the specified ARN.</p>
                            /// - On failure, responds with [`SdkError<GetComponentError>`](crate::error::GetComponentError)
    pub fn get_component(&self) -> crate::client::fluent_builders::GetComponent {
                                crate::client::fluent_builders::GetComponent::new(self.handle.clone())
                            }
}

