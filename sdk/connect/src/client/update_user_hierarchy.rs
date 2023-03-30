// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateUserHierarchy`](crate::client::fluent_builders::UpdateUserHierarchy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`hierarchy_group_id(impl Into<String>)`](crate::client::fluent_builders::UpdateUserHierarchy::hierarchy_group_id) / [`set_hierarchy_group_id(Option<String>)`](crate::client::fluent_builders::UpdateUserHierarchy::set_hierarchy_group_id): <p>The identifier of the hierarchy group.</p>
    ///   - [`user_id(impl Into<String>)`](crate::client::fluent_builders::UpdateUserHierarchy::user_id) / [`set_user_id(Option<String>)`](crate::client::fluent_builders::UpdateUserHierarchy::set_user_id): <p>The identifier of the user account.</p>
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::UpdateUserHierarchy::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::UpdateUserHierarchy::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
                            /// - On success, responds with [`UpdateUserHierarchyOutput`](crate::output::UpdateUserHierarchyOutput)
                            /// - On failure, responds with [`SdkError<UpdateUserHierarchyError>`](crate::error::UpdateUserHierarchyError)
    pub fn update_user_hierarchy(&self) -> crate::client::fluent_builders::UpdateUserHierarchy {
                                crate::client::fluent_builders::UpdateUserHierarchy::new(self.handle.clone())
                            }
}

