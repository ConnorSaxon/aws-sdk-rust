// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutImagePolicy`](crate::client::fluent_builders::PutImagePolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`image_arn(impl Into<String>)`](crate::client::fluent_builders::PutImagePolicy::image_arn) / [`set_image_arn(Option<String>)`](crate::client::fluent_builders::PutImagePolicy::set_image_arn): <p>The Amazon Resource Name (ARN) of the image that this policy should be applied to.</p>
    ///   - [`policy(impl Into<String>)`](crate::client::fluent_builders::PutImagePolicy::policy) / [`set_policy(Option<String>)`](crate::client::fluent_builders::PutImagePolicy::set_policy): <p>The policy to apply.</p>
                            /// - On success, responds with [`PutImagePolicyOutput`](crate::output::PutImagePolicyOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::output::PutImagePolicyOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`image_arn(Option<String>)`](crate::output::PutImagePolicyOutput::image_arn): <p>The Amazon Resource Name (ARN) of the image that this policy was applied to.</p>
                            /// - On failure, responds with [`SdkError<PutImagePolicyError>`](crate::error::PutImagePolicyError)
    pub fn put_image_policy(&self) -> crate::client::fluent_builders::PutImagePolicy {
                                crate::client::fluent_builders::PutImagePolicy::new(self.handle.clone())
                            }
}

