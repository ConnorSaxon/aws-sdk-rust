// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeFileSystems`](crate::client::fluent_builders::DescribeFileSystems) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeFileSystems::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`file_system_ids(Vec<String>)`](crate::client::fluent_builders::DescribeFileSystems::file_system_ids) / [`set_file_system_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeFileSystems::set_file_system_ids): <p>IDs of the file systems whose descriptions you want to retrieve (String).</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeFileSystems::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeFileSystems::set_max_results): <p>Maximum number of file systems to return in the response (integer). This parameter value must be greater than 0. The number of items that Amazon FSx returns is the minimum of the <code>MaxResults</code> parameter specified in the request and the service's internal maximum number of items per page.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeFileSystems::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeFileSystems::set_next_token): <p>Opaque pagination token returned from a previous <code>DescribeFileSystems</code> operation (String). If a token present, the operation continues the list from where the returning call left off.</p>
                            /// - On success, responds with [`DescribeFileSystemsOutput`](crate::output::DescribeFileSystemsOutput) with field(s):
    ///   - [`file_systems(Option<Vec<FileSystem>>)`](crate::output::DescribeFileSystemsOutput::file_systems): <p>An array of file system descriptions.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeFileSystemsOutput::next_token): <p>Present if there are more file systems than returned in the response (String). You can use the <code>NextToken</code> value in the later request to fetch the descriptions. </p>
                            /// - On failure, responds with [`SdkError<DescribeFileSystemsError>`](crate::error::DescribeFileSystemsError)
    pub fn describe_file_systems(&self) -> crate::client::fluent_builders::DescribeFileSystems {
                                crate::client::fluent_builders::DescribeFileSystems::new(self.handle.clone())
                            }
}

