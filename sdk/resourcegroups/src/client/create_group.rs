// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateGroup`](crate::client::fluent_builders::CreateGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateGroup::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateGroup::set_name): <p>The name of the group, which is the identifier of the group in other operations. You can't change the name of a resource group after you create it. A resource group name can consist of letters, numbers, hyphens, periods, and underscores. The name cannot start with <code>AWS</code> or <code>aws</code>; these are reserved. A resource group name must be unique within each Amazon Web Services Region in your Amazon Web Services account.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateGroup::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateGroup::set_description): <p>The description of the resource group. Descriptions can consist of letters, numbers, hyphens, underscores, periods, and spaces.</p>
    ///   - [`resource_query(ResourceQuery)`](crate::client::fluent_builders::CreateGroup::resource_query) / [`set_resource_query(Option<ResourceQuery>)`](crate::client::fluent_builders::CreateGroup::set_resource_query): <p>The resource query that determines which Amazon Web Services resources are members of this group. For more information about resource queries, see <a href="https://docs.aws.amazon.com/ARG/latest/userguide/gettingstarted-query.html#gettingstarted-query-cli-tag">Create a tag-based group in Resource Groups</a>. </p> <note>   <p>A resource group can contain either a <code>ResourceQuery</code> or a <code>Configuration</code>, but not both.</p>  </note>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateGroup::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateGroup::set_tags): <p>The tags to add to the group. A tag is key-value pair string.</p>
    ///   - [`configuration(Vec<GroupConfigurationItem>)`](crate::client::fluent_builders::CreateGroup::configuration) / [`set_configuration(Option<Vec<GroupConfigurationItem>>)`](crate::client::fluent_builders::CreateGroup::set_configuration): <p>A configuration associates the resource group with an Amazon Web Services service and specifies how the service can interact with the resources in the group. A configuration is an array of <code>GroupConfigurationItem</code> elements. For details about the syntax of service configurations, see <a href="https://docs.aws.amazon.com/ARG/latest/APIReference/about-slg.html">Service configurations for Resource Groups</a>.</p> <note>   <p>A resource group can contain either a <code>Configuration</code> or a <code>ResourceQuery</code>, but not both.</p>  </note>
                            /// - On success, responds with [`CreateGroupOutput`](crate::output::CreateGroupOutput) with field(s):
    ///   - [`group(Option<Group>)`](crate::output::CreateGroupOutput::group): <p>The description of the resource group.</p>
    ///   - [`resource_query(Option<ResourceQuery>)`](crate::output::CreateGroupOutput::resource_query): <p>The resource query associated with the group. For more information about resource queries, see <a href="https://docs.aws.amazon.com/ARG/latest/userguide/gettingstarted-query.html#gettingstarted-query-cli-tag">Create a tag-based group in Resource Groups</a>. </p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::CreateGroupOutput::tags): <p>The tags associated with the group.</p>
    ///   - [`group_configuration(Option<GroupConfiguration>)`](crate::output::CreateGroupOutput::group_configuration): <p>The service configuration associated with the resource group. For details about the syntax of a service configuration, see <a href="https://docs.aws.amazon.com/ARG/latest/APIReference/about-slg.html">Service configurations for Resource Groups</a>.</p>
                            /// - On failure, responds with [`SdkError<CreateGroupError>`](crate::error::CreateGroupError)
    pub fn create_group(&self) -> crate::client::fluent_builders::CreateGroup {
                                crate::client::fluent_builders::CreateGroup::new(self.handle.clone())
                            }
}

