// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeConformancePacks`](crate::client::fluent_builders::DescribeConformancePacks) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeConformancePacks::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`conformance_pack_names(Vec<String>)`](crate::client::fluent_builders::DescribeConformancePacks::conformance_pack_names) / [`set_conformance_pack_names(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeConformancePacks::set_conformance_pack_names): <p>Comma-separated list of conformance pack names for which you want details. If you do not specify any names, Config returns details for all your conformance packs. </p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::DescribeConformancePacks::limit) / [`set_limit(i32)`](crate::client::fluent_builders::DescribeConformancePacks::set_limit): <p>The maximum number of conformance packs returned on each page.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeConformancePacks::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeConformancePacks::set_next_token): <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
                            /// - On success, responds with [`DescribeConformancePacksOutput`](crate::output::DescribeConformancePacksOutput) with field(s):
    ///   - [`conformance_pack_details(Option<Vec<ConformancePackDetail>>)`](crate::output::DescribeConformancePacksOutput::conformance_pack_details): <p>Returns a list of <code>ConformancePackDetail</code> objects.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeConformancePacksOutput::next_token): <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
                            /// - On failure, responds with [`SdkError<DescribeConformancePacksError>`](crate::error::DescribeConformancePacksError)
    pub fn describe_conformance_packs(&self) -> crate::client::fluent_builders::DescribeConformancePacks {
                                crate::client::fluent_builders::DescribeConformancePacks::new(self.handle.clone())
                            }
}

