// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDefaultCreditSpecification`](crate::client::fluent_builders::GetDefaultCreditSpecification) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::GetDefaultCreditSpecification::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::GetDefaultCreditSpecification::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`instance_family(UnlimitedSupportedInstanceFamily)`](crate::client::fluent_builders::GetDefaultCreditSpecification::instance_family) / [`set_instance_family(Option<UnlimitedSupportedInstanceFamily>)`](crate::client::fluent_builders::GetDefaultCreditSpecification::set_instance_family): <p>The instance family.</p>
                            /// - On success, responds with [`GetDefaultCreditSpecificationOutput`](crate::output::GetDefaultCreditSpecificationOutput) with field(s):
    ///   - [`instance_family_credit_specification(Option<InstanceFamilyCreditSpecification>)`](crate::output::GetDefaultCreditSpecificationOutput::instance_family_credit_specification): <p>The default credit option for CPU usage of the instance family.</p>
                            /// - On failure, responds with [`SdkError<GetDefaultCreditSpecificationError>`](crate::error::GetDefaultCreditSpecificationError)
    pub fn get_default_credit_specification(&self) -> crate::client::fluent_builders::GetDefaultCreditSpecification {
                                crate::client::fluent_builders::GetDefaultCreditSpecification::new(self.handle.clone())
                            }
}

