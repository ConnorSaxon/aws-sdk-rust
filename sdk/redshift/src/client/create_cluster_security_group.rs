// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateClusterSecurityGroup`](crate::client::fluent_builders::CreateClusterSecurityGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_security_group_name(impl Into<String>)`](crate::client::fluent_builders::CreateClusterSecurityGroup::cluster_security_group_name) / [`set_cluster_security_group_name(Option<String>)`](crate::client::fluent_builders::CreateClusterSecurityGroup::set_cluster_security_group_name): <p>The name for the security group. Amazon Redshift stores the value as a lowercase string.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must contain no more than 255 alphanumeric characters or hyphens.</p> </li>   <li> <p>Must not be "Default".</p> </li>   <li> <p>Must be unique for all security groups that are created by your Amazon Web Services account.</p> </li>  </ul>  <p>Example: <code>examplesecuritygroup</code> </p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateClusterSecurityGroup::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateClusterSecurityGroup::set_description): <p>A description for the security group.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateClusterSecurityGroup::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateClusterSecurityGroup::set_tags): <p>A list of tag instances.</p>
                            /// - On success, responds with [`CreateClusterSecurityGroupOutput`](crate::output::CreateClusterSecurityGroupOutput) with field(s):
    ///   - [`cluster_security_group(Option<ClusterSecurityGroup>)`](crate::output::CreateClusterSecurityGroupOutput::cluster_security_group): <p>Describes a security group.</p>
                            /// - On failure, responds with [`SdkError<CreateClusterSecurityGroupError>`](crate::error::CreateClusterSecurityGroupError)
    pub fn create_cluster_security_group(&self) -> crate::client::fluent_builders::CreateClusterSecurityGroup {
                                crate::client::fluent_builders::CreateClusterSecurityGroup::new(self.handle.clone())
                            }
}

