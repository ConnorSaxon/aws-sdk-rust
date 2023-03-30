// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeImport`](crate::client::fluent_builders::DescribeImport) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`import_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeImport::import_arn) / [`set_import_arn(Option<String>)`](crate::client::fluent_builders::DescribeImport::set_import_arn): <p> The Amazon Resource Name (ARN) associated with the table you're importing to. </p>
                            /// - On success, responds with [`DescribeImportOutput`](crate::output::DescribeImportOutput) with field(s):
    ///   - [`import_table_description(Option<ImportTableDescription>)`](crate::output::DescribeImportOutput::import_table_description): <p> Represents the properties of the table created for the import, and parameters of the import. The import parameters include import status, how many items were processed, and how many errors were encountered. </p>
                            /// - On failure, responds with [`SdkError<DescribeImportError>`](crate::error::DescribeImportError)
    pub fn describe_import(&self) -> crate::client::fluent_builders::DescribeImport {
                                crate::client::fluent_builders::DescribeImport::new(self.handle.clone())
                            }
}

