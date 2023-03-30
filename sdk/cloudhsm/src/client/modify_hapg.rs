// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyHapg`](crate::client::fluent_builders::ModifyHapg) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`hapg_arn(impl Into<String>)`](crate::client::fluent_builders::ModifyHapg::hapg_arn) / [`set_hapg_arn(Option<String>)`](crate::client::fluent_builders::ModifyHapg::set_hapg_arn): <p>The ARN of the high-availability partition group to modify.</p>
    ///   - [`label(impl Into<String>)`](crate::client::fluent_builders::ModifyHapg::label) / [`set_label(Option<String>)`](crate::client::fluent_builders::ModifyHapg::set_label): <p>The new label for the high-availability partition group.</p>
    ///   - [`partition_serial_list(Vec<String>)`](crate::client::fluent_builders::ModifyHapg::partition_serial_list) / [`set_partition_serial_list(Option<Vec<String>>)`](crate::client::fluent_builders::ModifyHapg::set_partition_serial_list): <p>The list of partition serial numbers to make members of the high-availability partition group.</p>
                            /// - On success, responds with [`ModifyHapgOutput`](crate::output::ModifyHapgOutput) with field(s):
    ///   - [`hapg_arn(Option<String>)`](crate::output::ModifyHapgOutput::hapg_arn): <p>The ARN of the high-availability partition group.</p>
                            /// - On failure, responds with [`SdkError<ModifyHapgError>`](crate::error::ModifyHapgError)
    pub fn modify_hapg(&self) -> crate::client::fluent_builders::ModifyHapg {
                                crate::client::fluent_builders::ModifyHapg::new(self.handle.clone())
                            }
}

