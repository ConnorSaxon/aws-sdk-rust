// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetResourcePermission`](crate::client::fluent_builders::GetResourcePermission) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`action_type(PermissionActionType)`](crate::client::fluent_builders::GetResourcePermission::action_type) / [`set_action_type(Option<PermissionActionType>)`](crate::client::fluent_builders::GetResourcePermission::set_action_type): <p></p>
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::GetResourcePermission::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::GetResourcePermission::set_resource_arn): <p>The Amazon Resource Name (ARN) of the resource.</p>
                            /// - On success, responds with [`GetResourcePermissionOutput`](crate::output::GetResourcePermissionOutput) with field(s):
    ///   - [`policy(Option<String>)`](crate::output::GetResourcePermissionOutput::policy): <p></p>
                            /// - On failure, responds with [`SdkError<GetResourcePermissionError>`](crate::error::GetResourcePermissionError)
    pub fn get_resource_permission(&self) -> crate::client::fluent_builders::GetResourcePermission {
                                crate::client::fluent_builders::GetResourcePermission::new(self.handle.clone())
                            }
}

