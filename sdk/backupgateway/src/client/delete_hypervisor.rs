// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteHypervisor`](crate::client::fluent_builders::DeleteHypervisor) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`hypervisor_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteHypervisor::hypervisor_arn) / [`set_hypervisor_arn(Option<String>)`](crate::client::fluent_builders::DeleteHypervisor::set_hypervisor_arn): <p>The Amazon Resource Name (ARN) of the hypervisor to delete.</p>
                            /// - On success, responds with [`DeleteHypervisorOutput`](crate::output::DeleteHypervisorOutput) with field(s):
    ///   - [`hypervisor_arn(Option<String>)`](crate::output::DeleteHypervisorOutput::hypervisor_arn): <p>The Amazon Resource Name (ARN) of the hypervisor you deleted.</p>
                            /// - On failure, responds with [`SdkError<DeleteHypervisorError>`](crate::error::DeleteHypervisorError)
    pub fn delete_hypervisor(&self) -> crate::client::fluent_builders::DeleteHypervisor {
                                crate::client::fluent_builders::DeleteHypervisor::new(self.handle.clone())
                            }
}

