// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteBillingGroup`](crate::client::fluent_builders::DeleteBillingGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::DeleteBillingGroup::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::DeleteBillingGroup::set_arn): <p>The Amazon Resource Name (ARN) of the billing group that you're deleting.</p>
                            /// - On success, responds with [`DeleteBillingGroupOutput`](crate::output::DeleteBillingGroupOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::DeleteBillingGroupOutput::arn): <p>The Amazon Resource Name (ARN) of the deleted billing group.</p>
                            /// - On failure, responds with [`SdkError<DeleteBillingGroupError>`](crate::error::DeleteBillingGroupError)
    pub fn delete_billing_group(&self) -> crate::client::fluent_builders::DeleteBillingGroup {
                                crate::client::fluent_builders::DeleteBillingGroup::new(self.handle.clone())
                            }
}

