// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateUserHierarchyGroupName`](crate::client::fluent_builders::UpdateUserHierarchyGroupName) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateUserHierarchyGroupName::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateUserHierarchyGroupName::set_name): <p>The name of the hierarchy group. Must not be more than 100 characters.</p>
    ///   - [`hierarchy_group_id(impl Into<String>)`](crate::client::fluent_builders::UpdateUserHierarchyGroupName::hierarchy_group_id) / [`set_hierarchy_group_id(Option<String>)`](crate::client::fluent_builders::UpdateUserHierarchyGroupName::set_hierarchy_group_id): <p>The identifier of the hierarchy group.</p>
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::UpdateUserHierarchyGroupName::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::UpdateUserHierarchyGroupName::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
                            /// - On success, responds with [`UpdateUserHierarchyGroupNameOutput`](crate::output::UpdateUserHierarchyGroupNameOutput)
                            /// - On failure, responds with [`SdkError<UpdateUserHierarchyGroupNameError>`](crate::error::UpdateUserHierarchyGroupNameError)
    pub fn update_user_hierarchy_group_name(&self) -> crate::client::fluent_builders::UpdateUserHierarchyGroupName {
                                crate::client::fluent_builders::UpdateUserHierarchyGroupName::new(self.handle.clone())
                            }
}

