// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeValidDBInstanceModifications`](crate::client::fluent_builders::DescribeValidDBInstanceModifications) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_instance_identifier(impl Into<String>)`](crate::client::fluent_builders::DescribeValidDBInstanceModifications::db_instance_identifier) / [`set_db_instance_identifier(Option<String>)`](crate::client::fluent_builders::DescribeValidDBInstanceModifications::set_db_instance_identifier): <p>The customer identifier or the ARN of your DB instance.</p>
                            /// - On success, responds with [`DescribeValidDbInstanceModificationsOutput`](crate::output::DescribeValidDbInstanceModificationsOutput) with field(s):
    ///   - [`valid_db_instance_modifications_message(Option<ValidDbInstanceModificationsMessage>)`](crate::output::DescribeValidDbInstanceModificationsOutput::valid_db_instance_modifications_message): <p>Information about valid modifications that you can make to your DB instance. Contains the result of a successful call to the <code>DescribeValidDBInstanceModifications</code> action. You can use this information when you call <code>ModifyDBInstance</code>.</p>
                            /// - On failure, responds with [`SdkError<DescribeValidDBInstanceModificationsError>`](crate::error::DescribeValidDBInstanceModificationsError)
    pub fn describe_valid_db_instance_modifications(&self) -> crate::client::fluent_builders::DescribeValidDBInstanceModifications {
                                crate::client::fluent_builders::DescribeValidDBInstanceModifications::new(self.handle.clone())
                            }
}

