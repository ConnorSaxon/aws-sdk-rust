// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateRdsDbInstance`](crate::client::fluent_builders::UpdateRdsDbInstance) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rds_db_instance_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateRdsDbInstance::rds_db_instance_arn) / [`set_rds_db_instance_arn(Option<String>)`](crate::client::fluent_builders::UpdateRdsDbInstance::set_rds_db_instance_arn): <p>The Amazon RDS instance's ARN.</p>
    ///   - [`db_user(impl Into<String>)`](crate::client::fluent_builders::UpdateRdsDbInstance::db_user) / [`set_db_user(Option<String>)`](crate::client::fluent_builders::UpdateRdsDbInstance::set_db_user): <p>The master user name.</p>
    ///   - [`db_password(impl Into<String>)`](crate::client::fluent_builders::UpdateRdsDbInstance::db_password) / [`set_db_password(Option<String>)`](crate::client::fluent_builders::UpdateRdsDbInstance::set_db_password): <p>The database password.</p>
                            /// - On success, responds with [`UpdateRdsDbInstanceOutput`](crate::output::UpdateRdsDbInstanceOutput)
                            /// - On failure, responds with [`SdkError<UpdateRdsDbInstanceError>`](crate::error::UpdateRdsDbInstanceError)
    pub fn update_rds_db_instance(&self) -> crate::client::fluent_builders::UpdateRdsDbInstance {
                                crate::client::fluent_builders::UpdateRdsDbInstance::new(self.handle.clone())
                            }
}

