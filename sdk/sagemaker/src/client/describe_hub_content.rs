// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeHubContent`](crate::client::fluent_builders::DescribeHubContent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`hub_name(impl Into<String>)`](crate::client::fluent_builders::DescribeHubContent::hub_name) / [`set_hub_name(Option<String>)`](crate::client::fluent_builders::DescribeHubContent::set_hub_name): <p>The name of the hub that contains the content to describe.</p>
    ///   - [`hub_content_type(HubContentType)`](crate::client::fluent_builders::DescribeHubContent::hub_content_type) / [`set_hub_content_type(Option<HubContentType>)`](crate::client::fluent_builders::DescribeHubContent::set_hub_content_type): <p>The type of content in the hub.</p>
    ///   - [`hub_content_name(impl Into<String>)`](crate::client::fluent_builders::DescribeHubContent::hub_content_name) / [`set_hub_content_name(Option<String>)`](crate::client::fluent_builders::DescribeHubContent::set_hub_content_name): <p>The name of the content to describe.</p>
    ///   - [`hub_content_version(impl Into<String>)`](crate::client::fluent_builders::DescribeHubContent::hub_content_version) / [`set_hub_content_version(Option<String>)`](crate::client::fluent_builders::DescribeHubContent::set_hub_content_version): <p>The version of the content to describe.</p>
                            /// - On success, responds with [`DescribeHubContentOutput`](crate::output::DescribeHubContentOutput) with field(s):
    ///   - [`hub_content_name(Option<String>)`](crate::output::DescribeHubContentOutput::hub_content_name): <p>The name of the hub content.</p>
    ///   - [`hub_content_arn(Option<String>)`](crate::output::DescribeHubContentOutput::hub_content_arn): <p>The Amazon Resource Name (ARN) of the hub content.</p>
    ///   - [`hub_content_version(Option<String>)`](crate::output::DescribeHubContentOutput::hub_content_version): <p>The version of the hub content.</p>
    ///   - [`hub_content_type(Option<HubContentType>)`](crate::output::DescribeHubContentOutput::hub_content_type): <p>The type of hub content.</p>
    ///   - [`document_schema_version(Option<String>)`](crate::output::DescribeHubContentOutput::document_schema_version): <p>The document schema version for the hub content.</p>
    ///   - [`hub_name(Option<String>)`](crate::output::DescribeHubContentOutput::hub_name): <p>The name of the hub that contains the content.</p>
    ///   - [`hub_arn(Option<String>)`](crate::output::DescribeHubContentOutput::hub_arn): <p>The Amazon Resource Name (ARN) of the hub that contains the content. </p>
    ///   - [`hub_content_display_name(Option<String>)`](crate::output::DescribeHubContentOutput::hub_content_display_name): <p>The display name of the hub content.</p>
    ///   - [`hub_content_description(Option<String>)`](crate::output::DescribeHubContentOutput::hub_content_description): <p>A description of the hub content.</p>
    ///   - [`hub_content_markdown(Option<String>)`](crate::output::DescribeHubContentOutput::hub_content_markdown): <p>Markdown files associated with the hub content to import.</p>
    ///   - [`hub_content_document(Option<String>)`](crate::output::DescribeHubContentOutput::hub_content_document): <p>The hub content document that describes information about the hub content such as type, associated containers, scripts, and more.</p>
    ///   - [`hub_content_search_keywords(Option<Vec<String>>)`](crate::output::DescribeHubContentOutput::hub_content_search_keywords): <p>The searchable keywords for the hub content.</p>
    ///   - [`hub_content_dependencies(Option<Vec<HubContentDependency>>)`](crate::output::DescribeHubContentOutput::hub_content_dependencies): <p>The location of any dependencies that the hub content has, such as scripts, model artifacts, datasets, or notebooks.</p>
    ///   - [`hub_content_status(Option<HubContentStatus>)`](crate::output::DescribeHubContentOutput::hub_content_status): <p>The status of the hub content.</p>
    ///   - [`failure_reason(Option<String>)`](crate::output::DescribeHubContentOutput::failure_reason): <p>The failure reason if importing hub content failed.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribeHubContentOutput::creation_time): <p>The date and time that hub content was created.</p>
                            /// - On failure, responds with [`SdkError<DescribeHubContentError>`](crate::error::DescribeHubContentError)
    pub fn describe_hub_content(&self) -> crate::client::fluent_builders::DescribeHubContent {
                                crate::client::fluent_builders::DescribeHubContent::new(self.handle.clone())
                            }
}

