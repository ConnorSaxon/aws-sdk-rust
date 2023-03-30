// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateQuerySuggestionsBlockList`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`index_id(impl Into<String>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::index_id) / [`set_index_id(Option<String>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::set_index_id): <p>The identifier of the index you want to create a query suggestions block list for.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::set_name): <p>A user friendly name for the block list.</p>  <p>For example, the block list named 'offensive-words' includes all offensive words that could appear in user queries and need to be blocked from suggestions.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::set_description): <p>A user-friendly description for the block list.</p>  <p>For example, the description "List of all offensive words that can appear in user queries and need to be blocked from suggestions."</p>
    ///   - [`source_s3_path(S3Path)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::source_s3_path) / [`set_source_s3_path(Option<S3Path>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::set_source_s3_path): <p>The S3 path to your block list text file in your S3 bucket.</p>  <p>Each block word or phrase should be on a separate line in a text file.</p>  <p>For information on the current quota limits for block lists, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/quotas.html">Quotas for Amazon Kendra</a>.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::set_client_token): <p>A token that you provide to identify the request to create a query suggestions block list.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::set_role_arn): <p>The IAM (Identity and Access Management) role used by Amazon Kendra to access the block list text file in your S3 bucket.</p>  <p>You need permissions to the role ARN (Amazon Web Services Resource Name). The role needs S3 read permissions to your file in S3 and needs to give STS (Security Token Service) assume role permissions to Amazon Kendra.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateQuerySuggestionsBlockList::set_tags): <p>A tag that you can assign to a block list that categorizes the block list.</p>
                            /// - On success, responds with [`CreateQuerySuggestionsBlockListOutput`](crate::output::CreateQuerySuggestionsBlockListOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::CreateQuerySuggestionsBlockListOutput::id): <p>The identifier of the created block list.</p>
                            /// - On failure, responds with [`SdkError<CreateQuerySuggestionsBlockListError>`](crate::error::CreateQuerySuggestionsBlockListError)
    pub fn create_query_suggestions_block_list(&self) -> crate::client::fluent_builders::CreateQuerySuggestionsBlockList {
                                crate::client::fluent_builders::CreateQuerySuggestionsBlockList::new(self.handle.clone())
                            }
}

