// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetClusterCredentialsWithIAM`](crate::client::fluent_builders::GetClusterCredentialsWithIAM) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_name(impl Into<String>)`](crate::client::fluent_builders::GetClusterCredentialsWithIAM::db_name) / [`set_db_name(Option<String>)`](crate::client::fluent_builders::GetClusterCredentialsWithIAM::set_db_name): <p>The name of the database for which you are requesting credentials. If the database name is specified, the IAM policy must allow access to the resource <code>dbname</code> for the specified database name. If the database name is not specified, access to all databases is allowed.</p>
    ///   - [`cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::GetClusterCredentialsWithIAM::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::client::fluent_builders::GetClusterCredentialsWithIAM::set_cluster_identifier): <p>The unique identifier of the cluster that contains the database for which you are requesting credentials. </p>
    ///   - [`duration_seconds(i32)`](crate::client::fluent_builders::GetClusterCredentialsWithIAM::duration_seconds) / [`set_duration_seconds(Option<i32>)`](crate::client::fluent_builders::GetClusterCredentialsWithIAM::set_duration_seconds): <p>The number of seconds until the returned temporary password expires.</p>  <p>Range: 900-3600. Default: 900.</p>
                            /// - On success, responds with [`GetClusterCredentialsWithIamOutput`](crate::output::GetClusterCredentialsWithIamOutput) with field(s):
    ///   - [`db_user(Option<String>)`](crate::output::GetClusterCredentialsWithIamOutput::db_user): <p>A database user name that you provide when you connect to a database. The database user is mapped 1:1 to the source IAM identity. </p>
    ///   - [`db_password(Option<String>)`](crate::output::GetClusterCredentialsWithIamOutput::db_password): <p>A temporary password that you provide when you connect to a database.</p>
    ///   - [`expiration(Option<DateTime>)`](crate::output::GetClusterCredentialsWithIamOutput::expiration): <p>The time (UTC) when the temporary password expires. After this timestamp, a log in with the temporary password fails.</p>
    ///   - [`next_refresh_time(Option<DateTime>)`](crate::output::GetClusterCredentialsWithIamOutput::next_refresh_time): <p>Reserved for future use.</p>
                            /// - On failure, responds with [`SdkError<GetClusterCredentialsWithIAMError>`](crate::error::GetClusterCredentialsWithIAMError)
    pub fn get_cluster_credentials_with_iam(&self) -> crate::client::fluent_builders::GetClusterCredentialsWithIAM {
                                crate::client::fluent_builders::GetClusterCredentialsWithIAM::new(self.handle.clone())
                            }
}

