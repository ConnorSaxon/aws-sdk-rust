// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListFolderMembers`](crate::client::fluent_builders::ListFolderMembers) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::ListFolderMembers::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::ListFolderMembers::set_aws_account_id): <p>The ID for the Amazon Web Services account that contains the folder.</p>
    ///   - [`folder_id(impl Into<String>)`](crate::client::fluent_builders::ListFolderMembers::folder_id) / [`set_folder_id(Option<String>)`](crate::client::fluent_builders::ListFolderMembers::set_folder_id): <p>The ID of the folder.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListFolderMembers::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListFolderMembers::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListFolderMembers::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListFolderMembers::set_max_results): <p>The maximum number of results to be returned per request.</p>
                            /// - On success, responds with [`ListFolderMembersOutput`](crate::output::ListFolderMembersOutput) with field(s):
    ///   - [`status(i32)`](crate::output::ListFolderMembersOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`folder_member_list(Option<Vec<MemberIdArnPair>>)`](crate::output::ListFolderMembersOutput::folder_member_list): <p>A structure that contains all of the folder members (dashboards, analyses, and datasets) in the folder.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListFolderMembersOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`request_id(Option<String>)`](crate::output::ListFolderMembersOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
                            /// - On failure, responds with [`SdkError<ListFolderMembersError>`](crate::error::ListFolderMembersError)
    pub fn list_folder_members(&self) -> crate::client::fluent_builders::ListFolderMembers {
                                crate::client::fluent_builders::ListFolderMembers::new(self.handle.clone())
                            }
}

