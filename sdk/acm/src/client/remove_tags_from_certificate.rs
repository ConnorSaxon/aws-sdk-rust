// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RemoveTagsFromCertificate`](crate::client::fluent_builders::RemoveTagsFromCertificate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`certificate_arn(impl Into<String>)`](crate::client::fluent_builders::RemoveTagsFromCertificate::certificate_arn) / [`set_certificate_arn(Option<String>)`](crate::client::fluent_builders::RemoveTagsFromCertificate::set_certificate_arn): <p>String that contains the ARN of the ACM Certificate with one or more tags that you want to remove. This must be of the form:</p>  <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p>  <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a>.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::RemoveTagsFromCertificate::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::RemoveTagsFromCertificate::set_tags): <p>The key-value pair that defines the tag to remove.</p>
                            /// - On success, responds with [`RemoveTagsFromCertificateOutput`](crate::output::RemoveTagsFromCertificateOutput)
                            /// - On failure, responds with [`SdkError<RemoveTagsFromCertificateError>`](crate::error::RemoveTagsFromCertificateError)
    pub fn remove_tags_from_certificate(&self) -> crate::client::fluent_builders::RemoveTagsFromCertificate {
                                crate::client::fluent_builders::RemoveTagsFromCertificate::new(self.handle.clone())
                            }
}

