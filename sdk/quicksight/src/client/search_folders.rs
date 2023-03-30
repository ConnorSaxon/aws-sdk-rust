// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SearchFolders`](crate::client::fluent_builders::SearchFolders) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::SearchFolders::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::SearchFolders::set_aws_account_id): <p>The ID for the Amazon Web Services account that contains the folder.</p>
    ///   - [`filters(Vec<FolderSearchFilter>)`](crate::client::fluent_builders::SearchFolders::filters) / [`set_filters(Option<Vec<FolderSearchFilter>>)`](crate::client::fluent_builders::SearchFolders::set_filters): <p>The filters to apply to the search. Currently, you can search only by the parent folder ARN. For example, <code>"Filters": [ { "Name": "PARENT_FOLDER_ARN", "Operator": "StringEquals", "Value": "arn:aws:quicksight:us-east-1:1:folder/folderId" } ]</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::SearchFolders::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::SearchFolders::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::SearchFolders::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::SearchFolders::set_max_results): <p>The maximum number of results to be returned per request.</p>
                            /// - On success, responds with [`SearchFoldersOutput`](crate::output::SearchFoldersOutput) with field(s):
    ///   - [`status(i32)`](crate::output::SearchFoldersOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`folder_summary_list(Option<Vec<FolderSummary>>)`](crate::output::SearchFoldersOutput::folder_summary_list): <p>A structure that contains all of the folders in the Amazon Web Services account. This structure provides basic information about the folders.</p>
    ///   - [`next_token(Option<String>)`](crate::output::SearchFoldersOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`request_id(Option<String>)`](crate::output::SearchFoldersOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
                            /// - On failure, responds with [`SdkError<SearchFoldersError>`](crate::error::SearchFoldersError)
    pub fn search_folders(&self) -> crate::client::fluent_builders::SearchFolders {
                                crate::client::fluent_builders::SearchFolders::new(self.handle.clone())
                            }
}

