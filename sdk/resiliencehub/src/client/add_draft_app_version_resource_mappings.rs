// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AddDraftAppVersionResourceMappings`](crate::client::fluent_builders::AddDraftAppVersionResourceMappings) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_arn(impl Into<String>)`](crate::client::fluent_builders::AddDraftAppVersionResourceMappings::app_arn) / [`set_app_arn(Option<String>)`](crate::client::fluent_builders::AddDraftAppVersionResourceMappings::set_app_arn): <p>The Amazon Resource Name (ARN) of the application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    ///   - [`resource_mappings(Vec<ResourceMapping>)`](crate::client::fluent_builders::AddDraftAppVersionResourceMappings::resource_mappings) / [`set_resource_mappings(Option<Vec<ResourceMapping>>)`](crate::client::fluent_builders::AddDraftAppVersionResourceMappings::set_resource_mappings): <p> Mappings used to map logical resources from the template to physical resources. You can use the mapping type <code>CFN_STACK</code> if the application template uses a logical stack name. Or you can map individual resources by using the mapping type <code>RESOURCE</code>. We recommend using the mapping type <code>CFN_STACK</code> if the application is backed by a CloudFormation stack.</p>
                            /// - On success, responds with [`AddDraftAppVersionResourceMappingsOutput`](crate::output::AddDraftAppVersionResourceMappingsOutput) with field(s):
    ///   - [`app_arn(Option<String>)`](crate::output::AddDraftAppVersionResourceMappingsOutput::app_arn): <p> The Amazon Resource Name (ARN) of the application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    ///   - [`app_version(Option<String>)`](crate::output::AddDraftAppVersionResourceMappingsOutput::app_version): <p>The version of the application.</p>
    ///   - [`resource_mappings(Option<Vec<ResourceMapping>>)`](crate::output::AddDraftAppVersionResourceMappingsOutput::resource_mappings): <p>Mappings used to map logical resources from the template to physical resources. You can use the mapping type <code>CFN_STACK</code> if the application template uses a logical stack name. Or you can map individual resources by using the mapping type <code>RESOURCE</code>. We recommend using the mapping type <code>CFN_STACK</code> if the application is backed by a CloudFormation stack.</p>
                            /// - On failure, responds with [`SdkError<AddDraftAppVersionResourceMappingsError>`](crate::error::AddDraftAppVersionResourceMappingsError)
    pub fn add_draft_app_version_resource_mappings(&self) -> crate::client::fluent_builders::AddDraftAppVersionResourceMappings {
                                crate::client::fluent_builders::AddDraftAppVersionResourceMappings::new(self.handle.clone())
                            }
}

