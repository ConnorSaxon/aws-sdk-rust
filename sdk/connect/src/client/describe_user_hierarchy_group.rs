// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeUserHierarchyGroup`](crate::client::fluent_builders::DescribeUserHierarchyGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`hierarchy_group_id(impl Into<String>)`](crate::client::fluent_builders::DescribeUserHierarchyGroup::hierarchy_group_id) / [`set_hierarchy_group_id(Option<String>)`](crate::client::fluent_builders::DescribeUserHierarchyGroup::set_hierarchy_group_id): <p>The identifier of the hierarchy group.</p>
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::DescribeUserHierarchyGroup::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::DescribeUserHierarchyGroup::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
                            /// - On success, responds with [`DescribeUserHierarchyGroupOutput`](crate::output::DescribeUserHierarchyGroupOutput) with field(s):
    ///   - [`hierarchy_group(Option<HierarchyGroup>)`](crate::output::DescribeUserHierarchyGroupOutput::hierarchy_group): <p>Information about the hierarchy group.</p>
                            /// - On failure, responds with [`SdkError<DescribeUserHierarchyGroupError>`](crate::error::DescribeUserHierarchyGroupError)
    pub fn describe_user_hierarchy_group(&self) -> crate::client::fluent_builders::DescribeUserHierarchyGroup {
                                crate::client::fluent_builders::DescribeUserHierarchyGroup::new(self.handle.clone())
                            }
}

