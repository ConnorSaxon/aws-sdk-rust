// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeImport`](crate::client::fluent_builders::DescribeImport) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`import_id(impl Into<String>)`](crate::client::fluent_builders::DescribeImport::import_id) / [`set_import_id(Option<String>)`](crate::client::fluent_builders::DescribeImport::set_import_id): <p>The unique identifier of the import to describe.</p>
                            /// - On success, responds with [`DescribeImportOutput`](crate::output::DescribeImportOutput) with field(s):
    ///   - [`import_id(Option<String>)`](crate::output::DescribeImportOutput::import_id): <p>The unique identifier of the described import.</p>
    ///   - [`resource_specification(Option<ImportResourceSpecification>)`](crate::output::DescribeImportOutput::resource_specification): <p>The specifications of the imported bot, bot locale, or custom vocabulary.</p>
    ///   - [`imported_resource_id(Option<String>)`](crate::output::DescribeImportOutput::imported_resource_id): <p>The unique identifier that Amazon Lex assigned to the resource created by the import.</p>
    ///   - [`imported_resource_name(Option<String>)`](crate::output::DescribeImportOutput::imported_resource_name): <p>The name of the imported resource.</p>
    ///   - [`merge_strategy(Option<MergeStrategy>)`](crate::output::DescribeImportOutput::merge_strategy): <p>The strategy used when there was a name conflict between the imported resource and an existing resource. When the merge strategy is <code>FailOnConflict</code> existing resources are not overwritten and the import fails.</p>
    ///   - [`import_status(Option<ImportStatus>)`](crate::output::DescribeImportOutput::import_status): <p>The status of the import process. When the status is <code>Completed</code> the resource is imported and ready for use.</p>
    ///   - [`failure_reasons(Option<Vec<String>>)`](crate::output::DescribeImportOutput::failure_reasons): <p>If the <code>importStatus</code> field is <code>Failed</code>, this provides one or more reasons for the failure.</p>
    ///   - [`creation_date_time(Option<DateTime>)`](crate::output::DescribeImportOutput::creation_date_time): <p>The date and time that the import was created.</p>
    ///   - [`last_updated_date_time(Option<DateTime>)`](crate::output::DescribeImportOutput::last_updated_date_time): <p>The date and time that the import was last updated.</p>
                            /// - On failure, responds with [`SdkError<DescribeImportError>`](crate::error::DescribeImportError)
    pub fn describe_import(&self) -> crate::client::fluent_builders::DescribeImport {
                                crate::client::fluent_builders::DescribeImport::new(self.handle.clone())
                            }
}

