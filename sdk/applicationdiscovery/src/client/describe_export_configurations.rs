// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeExportConfigurations`](crate::client::fluent_builders::DescribeExportConfigurations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`export_ids(Vec<String>)`](crate::client::fluent_builders::DescribeExportConfigurations::export_ids) / [`set_export_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeExportConfigurations::set_export_ids): <p>A list of continuous export IDs to search for.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeExportConfigurations::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::DescribeExportConfigurations::set_max_results): <p>A number between 1 and 100 specifying the maximum number of continuous export descriptions returned.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeExportConfigurations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeExportConfigurations::set_next_token): <p>The token from the previous call to describe-export-tasks.</p>
                            /// - On success, responds with [`DescribeExportConfigurationsOutput`](crate::output::DescribeExportConfigurationsOutput) with field(s):
    ///   - [`exports_info(Option<Vec<ExportInfo>>)`](crate::output::DescribeExportConfigurationsOutput::exports_info): <p></p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeExportConfigurationsOutput::next_token): <p>The token from the previous call to describe-export-tasks.</p>
                            /// - On failure, responds with [`SdkError<DescribeExportConfigurationsError>`](crate::error::DescribeExportConfigurationsError)
    #[deprecated]
    pub fn describe_export_configurations(&self) -> crate::client::fluent_builders::DescribeExportConfigurations {
                                crate::client::fluent_builders::DescribeExportConfigurations::new(self.handle.clone())
                            }
}

