// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RegisterPublisher`](crate::client::fluent_builders::RegisterPublisher) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`accept_terms_and_conditions(bool)`](crate::client::fluent_builders::RegisterPublisher::accept_terms_and_conditions) / [`set_accept_terms_and_conditions(Option<bool>)`](crate::client::fluent_builders::RegisterPublisher::set_accept_terms_and_conditions): <p>Whether you accept the <a href="https://cloudformation-registry-documents.s3.amazonaws.com/Terms_and_Conditions_for_AWS_CloudFormation_Registry_Publishers.pdf">Terms and Conditions</a> for publishing extensions in the CloudFormation registry. You must accept the terms and conditions in order to register to publish public extensions to the CloudFormation registry.</p>  <p>The default is <code>false</code>.</p>
    ///   - [`connection_arn(impl Into<String>)`](crate::client::fluent_builders::RegisterPublisher::connection_arn) / [`set_connection_arn(Option<String>)`](crate::client::fluent_builders::RegisterPublisher::set_connection_arn): <p>If you are using a Bitbucket or GitHub account for identity verification, the Amazon Resource Name (ARN) for your connection to that account.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/publish-extension.html#publish-extension-prereqs">Registering your account to publish CloudFormation extensions</a> in the <i>CloudFormation CLI User Guide</i>.</p>
                            /// - On success, responds with [`RegisterPublisherOutput`](crate::output::RegisterPublisherOutput) with field(s):
    ///   - [`publisher_id(Option<String>)`](crate::output::RegisterPublisherOutput::publisher_id): <p>The ID assigned this account by CloudFormation for publishing extensions.</p>
                            /// - On failure, responds with [`SdkError<RegisterPublisherError>`](crate::error::RegisterPublisherError)
    pub fn register_publisher(&self) -> crate::client::fluent_builders::RegisterPublisher {
                                crate::client::fluent_builders::RegisterPublisher::new(self.handle.clone())
                            }
}

