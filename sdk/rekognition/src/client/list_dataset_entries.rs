// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDatasetEntries`](crate::client::fluent_builders::ListDatasetEntries) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListDatasetEntries::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dataset_arn(impl Into<String>)`](crate::client::fluent_builders::ListDatasetEntries::dataset_arn) / [`set_dataset_arn(Option<String>)`](crate::client::fluent_builders::ListDatasetEntries::set_dataset_arn): <p> The Amazon Resource Name (ARN) for the dataset that you want to use. </p>
    ///   - [`contains_labels(Vec<String>)`](crate::client::fluent_builders::ListDatasetEntries::contains_labels) / [`set_contains_labels(Option<Vec<String>>)`](crate::client::fluent_builders::ListDatasetEntries::set_contains_labels): <p>Specifies a label filter for the response. The response includes an entry only if one or more of the labels in <code>ContainsLabels</code> exist in the entry. </p>
    ///   - [`labeled(bool)`](crate::client::fluent_builders::ListDatasetEntries::labeled) / [`set_labeled(Option<bool>)`](crate::client::fluent_builders::ListDatasetEntries::set_labeled): <p> Specify <code>true</code> to get only the JSON Lines where the image is labeled. Specify <code>false</code> to get only the JSON Lines where the image isn't labeled. If you don't specify <code>Labeled</code>, <code>ListDatasetEntries</code> returns JSON Lines for labeled and unlabeled images. </p>
    ///   - [`source_ref_contains(impl Into<String>)`](crate::client::fluent_builders::ListDatasetEntries::source_ref_contains) / [`set_source_ref_contains(Option<String>)`](crate::client::fluent_builders::ListDatasetEntries::set_source_ref_contains): <p>If specified, <code>ListDatasetEntries</code> only returns JSON Lines where the value of <code>SourceRefContains</code> is part of the <code>source-ref</code> field. The <code>source-ref</code> field contains the Amazon S3 location of the image. You can use <code>SouceRefContains</code> for tasks such as getting the JSON Line for a single image, or gettting JSON Lines for all images within a specific folder.</p>
    ///   - [`has_errors(bool)`](crate::client::fluent_builders::ListDatasetEntries::has_errors) / [`set_has_errors(Option<bool>)`](crate::client::fluent_builders::ListDatasetEntries::set_has_errors): <p>Specifies an error filter for the response. Specify <code>True</code> to only include entries that have errors. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListDatasetEntries::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListDatasetEntries::set_next_token): <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListDatasetEntries::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListDatasetEntries::set_max_results): <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100. </p>
                            /// - On success, responds with [`ListDatasetEntriesOutput`](crate::output::ListDatasetEntriesOutput) with field(s):
    ///   - [`dataset_entries(Option<Vec<String>>)`](crate::output::ListDatasetEntriesOutput::dataset_entries): <p> A list of entries (images) in the dataset. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListDatasetEntriesOutput::next_token): <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
                            /// - On failure, responds with [`SdkError<ListDatasetEntriesError>`](crate::error::ListDatasetEntriesError)
    pub fn list_dataset_entries(&self) -> crate::client::fluent_builders::ListDatasetEntries {
                                crate::client::fluent_builders::ListDatasetEntries::new(self.handle.clone())
                            }
}

