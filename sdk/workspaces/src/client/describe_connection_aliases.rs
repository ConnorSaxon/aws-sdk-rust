// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeConnectionAliases`](crate::client::fluent_builders::DescribeConnectionAliases) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`alias_ids(Vec<String>)`](crate::client::fluent_builders::DescribeConnectionAliases::alias_ids) / [`set_alias_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeConnectionAliases::set_alias_ids): <p>The identifiers of the connection aliases to describe.</p>
    ///   - [`resource_id(impl Into<String>)`](crate::client::fluent_builders::DescribeConnectionAliases::resource_id) / [`set_resource_id(Option<String>)`](crate::client::fluent_builders::DescribeConnectionAliases::set_resource_id): <p>The identifier of the directory associated with the connection alias.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::DescribeConnectionAliases::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::DescribeConnectionAliases::set_limit): <p>The maximum number of connection aliases to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeConnectionAliases::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeConnectionAliases::set_next_token): <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results. </p>
                            /// - On success, responds with [`DescribeConnectionAliasesOutput`](crate::output::DescribeConnectionAliasesOutput) with field(s):
    ///   - [`connection_aliases(Option<Vec<ConnectionAlias>>)`](crate::output::DescribeConnectionAliasesOutput::connection_aliases): <p>Information about the specified connection aliases.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeConnectionAliasesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return. </p>
                            /// - On failure, responds with [`SdkError<DescribeConnectionAliasesError>`](crate::error::DescribeConnectionAliasesError)
    pub fn describe_connection_aliases(&self) -> crate::client::fluent_builders::DescribeConnectionAliases {
                                crate::client::fluent_builders::DescribeConnectionAliases::new(self.handle.clone())
                            }
}

