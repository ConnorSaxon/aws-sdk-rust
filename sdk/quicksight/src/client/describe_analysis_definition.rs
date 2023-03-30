// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAnalysisDefinition`](crate::client::fluent_builders::DescribeAnalysisDefinition) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::DescribeAnalysisDefinition::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::DescribeAnalysisDefinition::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the analysis. You must be using the Amazon Web Services account that the analysis is in.</p>
    ///   - [`analysis_id(impl Into<String>)`](crate::client::fluent_builders::DescribeAnalysisDefinition::analysis_id) / [`set_analysis_id(Option<String>)`](crate::client::fluent_builders::DescribeAnalysisDefinition::set_analysis_id): <p>The ID of the analysis that you're describing. The ID is part of the URL of the analysis.</p>
                            /// - On success, responds with [`DescribeAnalysisDefinitionOutput`](crate::output::DescribeAnalysisDefinitionOutput) with field(s):
    ///   - [`analysis_id(Option<String>)`](crate::output::DescribeAnalysisDefinitionOutput::analysis_id): <p>The ID of the analysis described.</p>
    ///   - [`name(Option<String>)`](crate::output::DescribeAnalysisDefinitionOutput::name): <p>The descriptive name of the analysis.</p>
    ///   - [`errors(Option<Vec<AnalysisError>>)`](crate::output::DescribeAnalysisDefinitionOutput::errors): <p>Errors associated with the analysis.</p>
    ///   - [`resource_status(Option<ResourceStatus>)`](crate::output::DescribeAnalysisDefinitionOutput::resource_status): <p>Status associated with the analysis.</p>  <ul>   <li> <p> <code>CREATION_IN_PROGRESS</code> </p> </li>   <li> <p> <code>CREATION_SUCCESSFUL</code> </p> </li>   <li> <p> <code>CREATION_FAILED</code> </p> </li>   <li> <p> <code>UPDATE_IN_PROGRESS</code> </p> </li>   <li> <p> <code>UPDATE_SUCCESSFUL</code> </p> </li>   <li> <p> <code>UPDATE_FAILED</code> </p> </li>   <li> <p> <code>DELETED</code> </p> </li>  </ul>
    ///   - [`theme_arn(Option<String>)`](crate::output::DescribeAnalysisDefinitionOutput::theme_arn): <p>The ARN of the theme of the analysis.</p>
    ///   - [`definition(Option<AnalysisDefinition>)`](crate::output::DescribeAnalysisDefinitionOutput::definition): <p>The definition of an analysis.</p>  <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p>
    ///   - [`status(i32)`](crate::output::DescribeAnalysisDefinitionOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`request_id(Option<String>)`](crate::output::DescribeAnalysisDefinitionOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
                            /// - On failure, responds with [`SdkError<DescribeAnalysisDefinitionError>`](crate::error::DescribeAnalysisDefinitionError)
    pub fn describe_analysis_definition(&self) -> crate::client::fluent_builders::DescribeAnalysisDefinition {
                                crate::client::fluent_builders::DescribeAnalysisDefinition::new(self.handle.clone())
                            }
}

