// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetPolicy`](crate::client::fluent_builders::GetPolicy) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::GetPolicy::send) it.
                            /// - On success, responds with [`GetPolicyOutput`](crate::output::GetPolicyOutput) with field(s):
    ///   - [`policy(Option<Policy>)`](crate::output::GetPolicyOutput::policy): A policy configures behavior that you allow or disallow for your account. For information about MediaConvert policies, see the user guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html
                            /// - On failure, responds with [`SdkError<GetPolicyError>`](crate::error::GetPolicyError)
    pub fn get_policy(&self) -> crate::client::fluent_builders::GetPolicy {
                                crate::client::fluent_builders::GetPolicy::new(self.handle.clone())
                            }
}

