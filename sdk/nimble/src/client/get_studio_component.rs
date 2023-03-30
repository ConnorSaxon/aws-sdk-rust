// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetStudioComponent`](crate::client::fluent_builders::GetStudioComponent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`studio_component_id(impl Into<String>)`](crate::client::fluent_builders::GetStudioComponent::studio_component_id) / [`set_studio_component_id(Option<String>)`](crate::client::fluent_builders::GetStudioComponent::set_studio_component_id): <p>The studio component ID.</p>
    ///   - [`studio_id(impl Into<String>)`](crate::client::fluent_builders::GetStudioComponent::studio_id) / [`set_studio_id(Option<String>)`](crate::client::fluent_builders::GetStudioComponent::set_studio_id): <p>The studio ID. </p>
                            /// - On success, responds with [`GetStudioComponentOutput`](crate::output::GetStudioComponentOutput) with field(s):
    ///   - [`studio_component(Option<StudioComponent>)`](crate::output::GetStudioComponentOutput::studio_component): <p>Information about the studio component.</p>
                            /// - On failure, responds with [`SdkError<GetStudioComponentError>`](crate::error::GetStudioComponentError)
    pub fn get_studio_component(&self) -> crate::client::fluent_builders::GetStudioComponent {
                                crate::client::fluent_builders::GetStudioComponent::new(self.handle.clone())
                            }
}

