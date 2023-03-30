// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyClusterIamRoles`](crate::client::fluent_builders::ModifyClusterIamRoles) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::ModifyClusterIamRoles::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::client::fluent_builders::ModifyClusterIamRoles::set_cluster_identifier): <p>The unique identifier of the cluster for which you want to associate or disassociate IAM roles.</p>
    ///   - [`add_iam_roles(Vec<String>)`](crate::client::fluent_builders::ModifyClusterIamRoles::add_iam_roles) / [`set_add_iam_roles(Option<Vec<String>>)`](crate::client::fluent_builders::ModifyClusterIamRoles::set_add_iam_roles): <p>Zero or more IAM roles to associate with the cluster. The roles must be in their Amazon Resource Name (ARN) format. </p>
    ///   - [`remove_iam_roles(Vec<String>)`](crate::client::fluent_builders::ModifyClusterIamRoles::remove_iam_roles) / [`set_remove_iam_roles(Option<Vec<String>>)`](crate::client::fluent_builders::ModifyClusterIamRoles::set_remove_iam_roles): <p>Zero or more IAM roles in ARN format to disassociate from the cluster. </p>
    ///   - [`default_iam_role_arn(impl Into<String>)`](crate::client::fluent_builders::ModifyClusterIamRoles::default_iam_role_arn) / [`set_default_iam_role_arn(Option<String>)`](crate::client::fluent_builders::ModifyClusterIamRoles::set_default_iam_role_arn): <p>The Amazon Resource Name (ARN) for the IAM role that was set as default for the cluster when the cluster was last modified.</p>
                            /// - On success, responds with [`ModifyClusterIamRolesOutput`](crate::output::ModifyClusterIamRolesOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::output::ModifyClusterIamRolesOutput::cluster): <p>Describes a cluster.</p>
                            /// - On failure, responds with [`SdkError<ModifyClusterIamRolesError>`](crate::error::ModifyClusterIamRolesError)
    pub fn modify_cluster_iam_roles(&self) -> crate::client::fluent_builders::ModifyClusterIamRoles {
                                crate::client::fluent_builders::ModifyClusterIamRoles::new(self.handle.clone())
                            }
}

