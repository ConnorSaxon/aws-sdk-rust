// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListBranches`](crate::client::fluent_builders::ListBranches) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::client::fluent_builders::ListBranches::app_id) / [`set_app_id(Option<String>)`](crate::client::fluent_builders::ListBranches::set_app_id): <p> The unique ID for an Amplify app. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListBranches::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListBranches::set_next_token): <p> A pagination token. Set to null to start listing branches from the start. If a non-null pagination token is returned in a result, pass its value in here to list more branches. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListBranches::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListBranches::set_max_results): <p> The maximum number of records to list in a single response. </p>
                            /// - On success, responds with [`ListBranchesOutput`](crate::output::ListBranchesOutput) with field(s):
    ///   - [`branches(Option<Vec<Branch>>)`](crate::output::ListBranchesOutput::branches): <p> A list of branches for an Amplify app. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListBranchesOutput::next_token): <p> A pagination token. If a non-null pagination token is returned in a result, pass its value in another request to retrieve more entries. </p>
                            /// - On failure, responds with [`SdkError<ListBranchesError>`](crate::error::ListBranchesError)
    pub fn list_branches(&self) -> crate::client::fluent_builders::ListBranches {
                                crate::client::fluent_builders::ListBranches::new(self.handle.clone())
                            }
}

