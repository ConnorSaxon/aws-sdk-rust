// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchGetDataQualityResult`](crate::client::fluent_builders::BatchGetDataQualityResult) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`result_ids(Vec<String>)`](crate::client::fluent_builders::BatchGetDataQualityResult::result_ids) / [`set_result_ids(Option<Vec<String>>)`](crate::client::fluent_builders::BatchGetDataQualityResult::set_result_ids): <p>A list of unique result IDs for the data quality results.</p>
                            /// - On success, responds with [`BatchGetDataQualityResultOutput`](crate::output::BatchGetDataQualityResultOutput) with field(s):
    ///   - [`results(Option<Vec<DataQualityResult>>)`](crate::output::BatchGetDataQualityResultOutput::results): <p>A list of <code>DataQualityResult</code> objects representing the data quality results.</p>
    ///   - [`results_not_found(Option<Vec<String>>)`](crate::output::BatchGetDataQualityResultOutput::results_not_found): <p>A list of result IDs for which results were not found.</p>
                            /// - On failure, responds with [`SdkError<BatchGetDataQualityResultError>`](crate::error::BatchGetDataQualityResultError)
    pub fn batch_get_data_quality_result(&self) -> crate::client::fluent_builders::BatchGetDataQualityResult {
                                crate::client::fluent_builders::BatchGetDataQualityResult::new(self.handle.clone())
                            }
}

