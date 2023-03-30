// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateProvisioningArtifact`](crate::client::fluent_builders::CreateProvisioningArtifact) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`accept_language(impl Into<String>)`](crate::client::fluent_builders::CreateProvisioningArtifact::accept_language) / [`set_accept_language(Option<String>)`](crate::client::fluent_builders::CreateProvisioningArtifact::set_accept_language): <p>The language code.</p>  <ul>   <li> <p> <code>en</code> - English (default)</p> </li>   <li> <p> <code>jp</code> - Japanese</p> </li>   <li> <p> <code>zh</code> - Chinese</p> </li>  </ul>
    ///   - [`product_id(impl Into<String>)`](crate::client::fluent_builders::CreateProvisioningArtifact::product_id) / [`set_product_id(Option<String>)`](crate::client::fluent_builders::CreateProvisioningArtifact::set_product_id): <p>The product identifier.</p>
    ///   - [`parameters(ProvisioningArtifactProperties)`](crate::client::fluent_builders::CreateProvisioningArtifact::parameters) / [`set_parameters(Option<ProvisioningArtifactProperties>)`](crate::client::fluent_builders::CreateProvisioningArtifact::set_parameters): <p>The configuration for the provisioning artifact.</p>
    ///   - [`idempotency_token(impl Into<String>)`](crate::client::fluent_builders::CreateProvisioningArtifact::idempotency_token) / [`set_idempotency_token(Option<String>)`](crate::client::fluent_builders::CreateProvisioningArtifact::set_idempotency_token): <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
                            /// - On success, responds with [`CreateProvisioningArtifactOutput`](crate::output::CreateProvisioningArtifactOutput) with field(s):
    ///   - [`provisioning_artifact_detail(Option<ProvisioningArtifactDetail>)`](crate::output::CreateProvisioningArtifactOutput::provisioning_artifact_detail): <p>Information about the provisioning artifact.</p>
    ///   - [`info(Option<HashMap<String, String>>)`](crate::output::CreateProvisioningArtifactOutput::info): <p>Specify the template source with one of the following options, but not both. Keys accepted: [ <code>LoadTemplateFromURL</code>, <code>ImportFromPhysicalId</code> ].</p>  <p>Use the URL of the CloudFormation template in Amazon S3 or GitHub in JSON format. </p>  <p> <code>LoadTemplateFromURL</code> </p>  <p>Use the URL of the CloudFormation template in Amazon S3 or GitHub in JSON format.</p>  <p> <code>ImportFromPhysicalId</code> </p>  <p>Use the physical id of the resource that contains the template; currently supports CloudFormation stack ARN.</p>
    ///   - [`status(Option<Status>)`](crate::output::CreateProvisioningArtifactOutput::status): <p>The status of the current request.</p>
                            /// - On failure, responds with [`SdkError<CreateProvisioningArtifactError>`](crate::error::CreateProvisioningArtifactError)
    pub fn create_provisioning_artifact(&self) -> crate::client::fluent_builders::CreateProvisioningArtifact {
                                crate::client::fluent_builders::CreateProvisioningArtifact::new(self.handle.clone())
                            }
}

