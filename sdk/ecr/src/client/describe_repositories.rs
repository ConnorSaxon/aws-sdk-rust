// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeRepositories`](crate::client::fluent_builders::DescribeRepositories) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeRepositories::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`registry_id(impl Into<String>)`](crate::client::fluent_builders::DescribeRepositories::registry_id) / [`set_registry_id(Option<String>)`](crate::client::fluent_builders::DescribeRepositories::set_registry_id): <p>The Amazon Web Services account ID associated with the registry that contains the repositories to be described. If you do not specify a registry, the default registry is assumed.</p>
    ///   - [`repository_names(Vec<String>)`](crate::client::fluent_builders::DescribeRepositories::repository_names) / [`set_repository_names(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeRepositories::set_repository_names): <p>A list of repositories to describe. If this parameter is omitted, then all repositories in a registry are described.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeRepositories::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeRepositories::set_next_token): <p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeRepositories</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return. This option cannot be used when you specify repositories with <code>repositoryNames</code>.</p> <note>   <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>  </note>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeRepositories::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeRepositories::set_max_results): <p>The maximum number of repository results returned by <code>DescribeRepositories</code> in paginated output. When this parameter is used, <code>DescribeRepositories</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeRepositories</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 1000. If this parameter is not used, then <code>DescribeRepositories</code> returns up to 100 results and a <code>nextToken</code> value, if applicable. This option cannot be used when you specify repositories with <code>repositoryNames</code>.</p>
                            /// - On success, responds with [`DescribeRepositoriesOutput`](crate::output::DescribeRepositoriesOutput) with field(s):
    ///   - [`repositories(Option<Vec<Repository>>)`](crate::output::DescribeRepositoriesOutput::repositories): <p>A list of repository objects corresponding to valid repositories.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeRepositoriesOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>DescribeRepositories</code> request. When the results of a <code>DescribeRepositories</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeRepositoriesError>`](crate::error::DescribeRepositoriesError)
    pub fn describe_repositories(&self) -> crate::client::fluent_builders::DescribeRepositories {
                                crate::client::fluent_builders::DescribeRepositories::new(self.handle.clone())
                            }
}

