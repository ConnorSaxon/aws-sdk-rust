// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRelationalDatabaseMasterUserPassword`](crate::client::fluent_builders::GetRelationalDatabaseMasterUserPassword) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`relational_database_name(impl Into<String>)`](crate::client::fluent_builders::GetRelationalDatabaseMasterUserPassword::relational_database_name) / [`set_relational_database_name(Option<String>)`](crate::client::fluent_builders::GetRelationalDatabaseMasterUserPassword::set_relational_database_name): <p>The name of your database for which to get the master user password.</p>
    ///   - [`password_version(RelationalDatabasePasswordVersion)`](crate::client::fluent_builders::GetRelationalDatabaseMasterUserPassword::password_version) / [`set_password_version(Option<RelationalDatabasePasswordVersion>)`](crate::client::fluent_builders::GetRelationalDatabaseMasterUserPassword::set_password_version): <p>The password version to return.</p>  <p>Specifying <code>CURRENT</code> or <code>PREVIOUS</code> returns the current or previous passwords respectively. Specifying <code>PENDING</code> returns the newest version of the password that will rotate to <code>CURRENT</code>. After the <code>PENDING</code> password rotates to <code>CURRENT</code>, the <code>PENDING</code> password is no longer available.</p>  <p>Default: <code>CURRENT</code> </p>
                            /// - On success, responds with [`GetRelationalDatabaseMasterUserPasswordOutput`](crate::output::GetRelationalDatabaseMasterUserPasswordOutput) with field(s):
    ///   - [`master_user_password(Option<String>)`](crate::output::GetRelationalDatabaseMasterUserPasswordOutput::master_user_password): <p>The master user password for the <code>password version</code> specified.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::output::GetRelationalDatabaseMasterUserPasswordOutput::created_at): <p>The timestamp when the specified version of the master user password was created.</p>
                            /// - On failure, responds with [`SdkError<GetRelationalDatabaseMasterUserPasswordError>`](crate::error::GetRelationalDatabaseMasterUserPasswordError)
    pub fn get_relational_database_master_user_password(&self) -> crate::client::fluent_builders::GetRelationalDatabaseMasterUserPassword {
                                crate::client::fluent_builders::GetRelationalDatabaseMasterUserPassword::new(self.handle.clone())
                            }
}

