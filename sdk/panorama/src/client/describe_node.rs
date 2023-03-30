// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeNode`](crate::client::fluent_builders::DescribeNode) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`node_id(impl Into<String>)`](crate::client::fluent_builders::DescribeNode::node_id) / [`set_node_id(Option<String>)`](crate::client::fluent_builders::DescribeNode::set_node_id): <p>The node's ID.</p>
    ///   - [`owner_account(impl Into<String>)`](crate::client::fluent_builders::DescribeNode::owner_account) / [`set_owner_account(Option<String>)`](crate::client::fluent_builders::DescribeNode::set_owner_account): <p>The account ID of the node's owner.</p>
                            /// - On success, responds with [`DescribeNodeOutput`](crate::output::DescribeNodeOutput) with field(s):
    ///   - [`node_id(Option<String>)`](crate::output::DescribeNodeOutput::node_id): <p>The node's ID.</p>
    ///   - [`name(Option<String>)`](crate::output::DescribeNodeOutput::name): <p>The node's name.</p>
    ///   - [`category(Option<NodeCategory>)`](crate::output::DescribeNodeOutput::category): <p>The node's category.</p>
    ///   - [`owner_account(Option<String>)`](crate::output::DescribeNodeOutput::owner_account): <p>The account ID of the node's owner.</p>
    ///   - [`package_name(Option<String>)`](crate::output::DescribeNodeOutput::package_name): <p>The node's package name.</p>
    ///   - [`package_id(Option<String>)`](crate::output::DescribeNodeOutput::package_id): <p>The node's package ID.</p>
    ///   - [`package_arn(Option<String>)`](crate::output::DescribeNodeOutput::package_arn): <p>The node's ARN.</p>
    ///   - [`package_version(Option<String>)`](crate::output::DescribeNodeOutput::package_version): <p>The node's package version.</p>
    ///   - [`patch_version(Option<String>)`](crate::output::DescribeNodeOutput::patch_version): <p>The node's patch version.</p>
    ///   - [`node_interface(Option<NodeInterface>)`](crate::output::DescribeNodeOutput::node_interface): <p>The node's interface.</p>
    ///   - [`asset_name(Option<String>)`](crate::output::DescribeNodeOutput::asset_name): <p>The node's asset name.</p>
    ///   - [`description(Option<String>)`](crate::output::DescribeNodeOutput::description): <p>The node's description.</p>
    ///   - [`created_time(Option<DateTime>)`](crate::output::DescribeNodeOutput::created_time): <p>When the node was created.</p>
    ///   - [`last_updated_time(Option<DateTime>)`](crate::output::DescribeNodeOutput::last_updated_time): <p>When the node was updated.</p>
                            /// - On failure, responds with [`SdkError<DescribeNodeError>`](crate::error::DescribeNodeError)
    pub fn describe_node(&self) -> crate::client::fluent_builders::DescribeNode {
                                crate::client::fluent_builders::DescribeNode::new(self.handle.clone())
                            }
}

