// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAnalysis`](crate::client::fluent_builders::CreateAnalysis) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::CreateAnalysis::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::CreateAnalysis::set_aws_account_id): <p>The ID of the Amazon Web Services account where you are creating an analysis.</p>
    ///   - [`analysis_id(impl Into<String>)`](crate::client::fluent_builders::CreateAnalysis::analysis_id) / [`set_analysis_id(Option<String>)`](crate::client::fluent_builders::CreateAnalysis::set_analysis_id): <p>The ID for the analysis that you're creating. This ID displays in the URL of the analysis.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateAnalysis::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateAnalysis::set_name): <p>A descriptive name for the analysis that you're creating. This name displays for the analysis in the Amazon QuickSight console. </p>
    ///   - [`parameters(Parameters)`](crate::client::fluent_builders::CreateAnalysis::parameters) / [`set_parameters(Option<Parameters>)`](crate::client::fluent_builders::CreateAnalysis::set_parameters): <p>The parameter names and override values that you want to use. An analysis can have any parameter type, and some parameters might accept multiple values. </p>
    ///   - [`permissions(Vec<ResourcePermission>)`](crate::client::fluent_builders::CreateAnalysis::permissions) / [`set_permissions(Option<Vec<ResourcePermission>>)`](crate::client::fluent_builders::CreateAnalysis::set_permissions): <p>A structure that describes the principals and the resource-level permissions on an analysis. You can use the <code>Permissions</code> structure to grant permissions by providing a list of Identity and Access Management (IAM) action information for each principal listed by Amazon Resource Name (ARN). </p>  <p>To specify no permissions, omit <code>Permissions</code>.</p>
    ///   - [`source_entity(AnalysisSourceEntity)`](crate::client::fluent_builders::CreateAnalysis::source_entity) / [`set_source_entity(Option<AnalysisSourceEntity>)`](crate::client::fluent_builders::CreateAnalysis::set_source_entity): <p>A source entity to use for the analysis that you're creating. This metadata structure contains details that describe a source template and one or more datasets.</p>
    ///   - [`theme_arn(impl Into<String>)`](crate::client::fluent_builders::CreateAnalysis::theme_arn) / [`set_theme_arn(Option<String>)`](crate::client::fluent_builders::CreateAnalysis::set_theme_arn): <p>The ARN for the theme to apply to the analysis that you're creating. To see the theme in the Amazon QuickSight console, make sure that you have access to it.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateAnalysis::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateAnalysis::set_tags): <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the analysis.</p>
    ///   - [`definition(AnalysisDefinition)`](crate::client::fluent_builders::CreateAnalysis::definition) / [`set_definition(Option<AnalysisDefinition>)`](crate::client::fluent_builders::CreateAnalysis::set_definition): <p>The definition of an analysis.</p>  <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p>
                            /// - On success, responds with [`CreateAnalysisOutput`](crate::output::CreateAnalysisOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::CreateAnalysisOutput::arn): <p>The ARN for the analysis.</p>
    ///   - [`analysis_id(Option<String>)`](crate::output::CreateAnalysisOutput::analysis_id): <p>The ID of the analysis.</p>
    ///   - [`creation_status(Option<ResourceStatus>)`](crate::output::CreateAnalysisOutput::creation_status): <p>The status of the creation of the analysis. </p>
    ///   - [`status(i32)`](crate::output::CreateAnalysisOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`request_id(Option<String>)`](crate::output::CreateAnalysisOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
                            /// - On failure, responds with [`SdkError<CreateAnalysisError>`](crate::error::CreateAnalysisError)
    pub fn create_analysis(&self) -> crate::client::fluent_builders::CreateAnalysis {
                                crate::client::fluent_builders::CreateAnalysis::new(self.handle.clone())
                            }
}

