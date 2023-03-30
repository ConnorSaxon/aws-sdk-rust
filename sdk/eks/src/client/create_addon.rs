// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAddon`](crate::client::fluent_builders::CreateAddon) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl Into<String>)`](crate::client::fluent_builders::CreateAddon::cluster_name) / [`set_cluster_name(Option<String>)`](crate::client::fluent_builders::CreateAddon::set_cluster_name): <p>The name of the cluster to create the add-on for.</p>
    ///   - [`addon_name(impl Into<String>)`](crate::client::fluent_builders::CreateAddon::addon_name) / [`set_addon_name(Option<String>)`](crate::client::fluent_builders::CreateAddon::set_addon_name): <p>The name of the add-on. The name must match one of the names that <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_DescribeAddonVersions.html"> <code>DescribeAddonVersions</code> </a> returns.</p>
    ///   - [`addon_version(impl Into<String>)`](crate::client::fluent_builders::CreateAddon::addon_version) / [`set_addon_version(Option<String>)`](crate::client::fluent_builders::CreateAddon::set_addon_version): <p>The version of the add-on. The version must match one of the versions returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_DescribeAddonVersions.html"> <code>DescribeAddonVersions</code> </a>.</p>
    ///   - [`service_account_role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateAddon::service_account_role_arn) / [`set_service_account_role_arn(Option<String>)`](crate::client::fluent_builders::CreateAddon::set_service_account_role_arn): <p>The Amazon Resource Name (ARN) of an existing IAM role to bind to the add-on's service account. The role must be assigned the IAM permissions required by the add-on. If you don't specify an existing IAM role, then the add-on uses the permissions assigned to the node IAM role. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/create-node-role.html">Amazon EKS node IAM role</a> in the <i>Amazon EKS User Guide</i>.</p> <note>   <p>To specify an existing IAM role, you must have an IAM OpenID Connect (OIDC) provider created for your cluster. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/enable-iam-roles-for-service-accounts.html">Enabling IAM roles for service accounts on your cluster</a> in the <i>Amazon EKS User Guide</i>.</p>  </note>
    ///   - [`resolve_conflicts(ResolveConflicts)`](crate::client::fluent_builders::CreateAddon::resolve_conflicts) / [`set_resolve_conflicts(Option<ResolveConflicts>)`](crate::client::fluent_builders::CreateAddon::set_resolve_conflicts): <p>How to resolve field value conflicts for an Amazon EKS add-on. Conflicts are handled based on the value you choose:</p>  <ul>   <li> <p> <b>None</b> – If the self-managed version of the add-on is installed on your cluster, Amazon EKS doesn't change the value. Creation of the add-on might fail.</p> </li>   <li> <p> <b>Overwrite</b> – If the self-managed version of the add-on is installed on your cluster and the Amazon EKS default value is different than the existing value, Amazon EKS changes the value to the Amazon EKS default value.</p> </li>   <li> <p> <b>Preserve</b> – Not supported. You can set this value when updating an add-on though. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_UpdateAddon.html">UpdateAddon</a>.</p> </li>  </ul>  <p>If you don't currently have the self-managed version of the add-on installed on your cluster, the Amazon EKS add-on is installed. Amazon EKS sets all values to default values, regardless of the option that you specify.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::CreateAddon::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::CreateAddon::set_client_request_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateAddon::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateAddon::set_tags): <p>The metadata to apply to the cluster to assist with categorization and organization. Each tag consists of a key and an optional value. You define both.</p>
    ///   - [`configuration_values(impl Into<String>)`](crate::client::fluent_builders::CreateAddon::configuration_values) / [`set_configuration_values(Option<String>)`](crate::client::fluent_builders::CreateAddon::set_configuration_values): <p>The set of configuration values for the add-on that's created. The values that you provide are validated against the schema in <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_DescribeAddonConfiguration.html"> <code>DescribeAddonConfiguration</code> </a>.</p>
                            /// - On success, responds with [`CreateAddonOutput`](crate::output::CreateAddonOutput) with field(s):
    ///   - [`addon(Option<Addon>)`](crate::output::CreateAddonOutput::addon): <p>An Amazon EKS add-on. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-add-ons.html">Amazon EKS add-ons</a> in the <i>Amazon EKS User Guide</i>.</p>
                            /// - On failure, responds with [`SdkError<CreateAddonError>`](crate::error::CreateAddonError)
    pub fn create_addon(&self) -> crate::client::fluent_builders::CreateAddon {
                                crate::client::fluent_builders::CreateAddon::new(self.handle.clone())
                            }
}

