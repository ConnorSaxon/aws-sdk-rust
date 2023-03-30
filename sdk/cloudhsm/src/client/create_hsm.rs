// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateHsm`](crate::client::fluent_builders::CreateHsm) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`subnet_id(impl Into<String>)`](crate::client::fluent_builders::CreateHsm::subnet_id) / [`set_subnet_id(Option<String>)`](crate::client::fluent_builders::CreateHsm::set_subnet_id): <p>The identifier of the subnet in your VPC in which to place the HSM.</p>
    ///   - [`ssh_key(impl Into<String>)`](crate::client::fluent_builders::CreateHsm::ssh_key) / [`set_ssh_key(Option<String>)`](crate::client::fluent_builders::CreateHsm::set_ssh_key): <p>The SSH public key to install on the HSM.</p>
    ///   - [`eni_ip(impl Into<String>)`](crate::client::fluent_builders::CreateHsm::eni_ip) / [`set_eni_ip(Option<String>)`](crate::client::fluent_builders::CreateHsm::set_eni_ip): <p>The IP address to assign to the HSM's ENI.</p>  <p>If an IP address is not specified, an IP address will be randomly chosen from the CIDR range of the subnet.</p>
    ///   - [`iam_role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateHsm::iam_role_arn) / [`set_iam_role_arn(Option<String>)`](crate::client::fluent_builders::CreateHsm::set_iam_role_arn): <p>The ARN of an IAM role to enable the AWS CloudHSM service to allocate an ENI on your behalf.</p>
    ///   - [`external_id(impl Into<String>)`](crate::client::fluent_builders::CreateHsm::external_id) / [`set_external_id(Option<String>)`](crate::client::fluent_builders::CreateHsm::set_external_id): <p>The external ID from <code>IamRoleArn</code>, if present.</p>
    ///   - [`subscription_type(SubscriptionType)`](crate::client::fluent_builders::CreateHsm::subscription_type) / [`set_subscription_type(Option<SubscriptionType>)`](crate::client::fluent_builders::CreateHsm::set_subscription_type): <p>Specifies the type of subscription for the HSM.</p>  <ul>   <li> <p> <b>PRODUCTION</b> - The HSM is being used in a production environment.</p> </li>   <li> <p> <b>TRIAL</b> - The HSM is being used in a product trial.</p> </li>  </ul>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateHsm::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateHsm::set_client_token): <p>A user-defined token to ensure idempotence. Subsequent calls to this operation with the same token will be ignored.</p>
    ///   - [`syslog_ip(impl Into<String>)`](crate::client::fluent_builders::CreateHsm::syslog_ip) / [`set_syslog_ip(Option<String>)`](crate::client::fluent_builders::CreateHsm::set_syslog_ip): <p>The IP address for the syslog monitoring server. The AWS CloudHSM service only supports one syslog monitoring server.</p>
                            /// - On success, responds with [`CreateHsmOutput`](crate::output::CreateHsmOutput) with field(s):
    ///   - [`hsm_arn(Option<String>)`](crate::output::CreateHsmOutput::hsm_arn): <p>The ARN of the HSM.</p>
                            /// - On failure, responds with [`SdkError<CreateHsmError>`](crate::error::CreateHsmError)
    pub fn create_hsm(&self) -> crate::client::fluent_builders::CreateHsm {
                                crate::client::fluent_builders::CreateHsm::new(self.handle.clone())
                            }
}

