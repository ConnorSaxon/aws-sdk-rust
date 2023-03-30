// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateStudio`](crate::client::fluent_builders::CreateStudio) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateStudio::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateStudio::set_name): <p>A descriptive name for the Amazon EMR Studio.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateStudio::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateStudio::set_description): <p>A detailed description of the Amazon EMR Studio.</p>
    ///   - [`auth_mode(AuthMode)`](crate::client::fluent_builders::CreateStudio::auth_mode) / [`set_auth_mode(Option<AuthMode>)`](crate::client::fluent_builders::CreateStudio::set_auth_mode): <p>Specifies whether the Studio authenticates users using IAM or IAM Identity Center.</p>
    ///   - [`vpc_id(impl Into<String>)`](crate::client::fluent_builders::CreateStudio::vpc_id) / [`set_vpc_id(Option<String>)`](crate::client::fluent_builders::CreateStudio::set_vpc_id): <p>The ID of the Amazon Virtual Private Cloud (Amazon VPC) to associate with the Studio.</p>
    ///   - [`subnet_ids(Vec<String>)`](crate::client::fluent_builders::CreateStudio::subnet_ids) / [`set_subnet_ids(Option<Vec<String>>)`](crate::client::fluent_builders::CreateStudio::set_subnet_ids): <p>A list of subnet IDs to associate with the Amazon EMR Studio. A Studio can have a maximum of 5 subnets. The subnets must belong to the VPC specified by <code>VpcId</code>. Studio users can create a Workspace in any of the specified subnets.</p>
    ///   - [`service_role(impl Into<String>)`](crate::client::fluent_builders::CreateStudio::service_role) / [`set_service_role(Option<String>)`](crate::client::fluent_builders::CreateStudio::set_service_role): <p>The IAM role that the Amazon EMR Studio assumes. The service role provides a way for Amazon EMR Studio to interoperate with other Amazon Web Services services.</p>
    ///   - [`user_role(impl Into<String>)`](crate::client::fluent_builders::CreateStudio::user_role) / [`set_user_role(Option<String>)`](crate::client::fluent_builders::CreateStudio::set_user_role): <p>The IAM user role that users and groups assume when logged in to an Amazon EMR Studio. Only specify a <code>UserRole</code> when you use IAM Identity Center authentication. The permissions attached to the <code>UserRole</code> can be scoped down for each user or group using session policies.</p>
    ///   - [`workspace_security_group_id(impl Into<String>)`](crate::client::fluent_builders::CreateStudio::workspace_security_group_id) / [`set_workspace_security_group_id(Option<String>)`](crate::client::fluent_builders::CreateStudio::set_workspace_security_group_id): <p>The ID of the Amazon EMR Studio Workspace security group. The Workspace security group allows outbound network traffic to resources in the Engine security group, and it must be in the same VPC specified by <code>VpcId</code>.</p>
    ///   - [`engine_security_group_id(impl Into<String>)`](crate::client::fluent_builders::CreateStudio::engine_security_group_id) / [`set_engine_security_group_id(Option<String>)`](crate::client::fluent_builders::CreateStudio::set_engine_security_group_id): <p>The ID of the Amazon EMR Studio Engine security group. The Engine security group allows inbound network traffic from the Workspace security group, and it must be in the same VPC specified by <code>VpcId</code>.</p>
    ///   - [`default_s3_location(impl Into<String>)`](crate::client::fluent_builders::CreateStudio::default_s3_location) / [`set_default_s3_location(Option<String>)`](crate::client::fluent_builders::CreateStudio::set_default_s3_location): <p>The Amazon S3 location to back up Amazon EMR Studio Workspaces and notebook files.</p>
    ///   - [`idp_auth_url(impl Into<String>)`](crate::client::fluent_builders::CreateStudio::idp_auth_url) / [`set_idp_auth_url(Option<String>)`](crate::client::fluent_builders::CreateStudio::set_idp_auth_url): <p>The authentication endpoint of your identity provider (IdP). Specify this value when you use IAM authentication and want to let federated users log in to a Studio with the Studio URL and credentials from your IdP. Amazon EMR Studio redirects users to this endpoint to enter credentials.</p>
    ///   - [`idp_relay_state_parameter_name(impl Into<String>)`](crate::client::fluent_builders::CreateStudio::idp_relay_state_parameter_name) / [`set_idp_relay_state_parameter_name(Option<String>)`](crate::client::fluent_builders::CreateStudio::set_idp_relay_state_parameter_name): <p>The name that your identity provider (IdP) uses for its <code>RelayState</code> parameter. For example, <code>RelayState</code> or <code>TargetSource</code>. Specify this value when you use IAM authentication and want to let federated users log in to a Studio using the Studio URL. The <code>RelayState</code> parameter differs by IdP.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateStudio::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateStudio::set_tags): <p>A list of tags to associate with the Amazon EMR Studio. Tags are user-defined key-value pairs that consist of a required key string with a maximum of 128 characters, and an optional value string with a maximum of 256 characters.</p>
                            /// - On success, responds with [`CreateStudioOutput`](crate::output::CreateStudioOutput) with field(s):
    ///   - [`studio_id(Option<String>)`](crate::output::CreateStudioOutput::studio_id): <p>The ID of the Amazon EMR Studio.</p>
    ///   - [`url(Option<String>)`](crate::output::CreateStudioOutput::url): <p>The unique Studio access URL.</p>
                            /// - On failure, responds with [`SdkError<CreateStudioError>`](crate::error::CreateStudioError)
    pub fn create_studio(&self) -> crate::client::fluent_builders::CreateStudio {
                                crate::client::fluent_builders::CreateStudio::new(self.handle.clone())
                            }
}

