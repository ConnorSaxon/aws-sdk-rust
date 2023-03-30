// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteCustomActionType`](crate::client::fluent_builders::DeleteCustomActionType) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`category(ActionCategory)`](crate::client::fluent_builders::DeleteCustomActionType::category) / [`set_category(Option<ActionCategory>)`](crate::client::fluent_builders::DeleteCustomActionType::set_category): <p>The category of the custom action that you want to delete, such as source or deploy.</p>
    ///   - [`provider(impl Into<String>)`](crate::client::fluent_builders::DeleteCustomActionType::provider) / [`set_provider(Option<String>)`](crate::client::fluent_builders::DeleteCustomActionType::set_provider): <p>The provider of the service used in the custom action, such as AWS CodeDeploy.</p>
    ///   - [`version(impl Into<String>)`](crate::client::fluent_builders::DeleteCustomActionType::version) / [`set_version(Option<String>)`](crate::client::fluent_builders::DeleteCustomActionType::set_version): <p>The version of the custom action to delete.</p>
                            /// - On success, responds with [`DeleteCustomActionTypeOutput`](crate::output::DeleteCustomActionTypeOutput)
                            /// - On failure, responds with [`SdkError<DeleteCustomActionTypeError>`](crate::error::DeleteCustomActionTypeError)
    pub fn delete_custom_action_type(&self) -> crate::client::fluent_builders::DeleteCustomActionType {
                                crate::client::fluent_builders::DeleteCustomActionType::new(self.handle.clone())
                            }
}

