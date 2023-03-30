// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeSourceRegions`](crate::client::fluent_builders::DescribeSourceRegions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeSourceRegions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`region_name(impl Into<String>)`](crate::client::fluent_builders::DescribeSourceRegions::region_name) / [`set_region_name(Option<String>)`](crate::client::fluent_builders::DescribeSourceRegions::set_region_name): <p>The source Amazon Web Services Region name. For example, <code>us-east-1</code>.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must specify a valid Amazon Web Services Region name.</p> </li>  </ul>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeSourceRegions::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeSourceRegions::set_max_records): <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so you can retrieve the remaining results.</p>  <p>Default: 100</p>  <p>Constraints: Minimum 20, maximum 100.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeSourceRegions::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeSourceRegions::set_marker): <p>An optional pagination token provided by a previous <code>DescribeSourceRegions</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeSourceRegions::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeSourceRegions::set_filters): <p>This parameter isn't currently supported.</p>
                            /// - On success, responds with [`DescribeSourceRegionsOutput`](crate::output::DescribeSourceRegionsOutput) with field(s):
    ///   - [`marker(Option<String>)`](crate::output::DescribeSourceRegionsOutput::marker): <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    ///   - [`source_regions(Option<Vec<SourceRegion>>)`](crate::output::DescribeSourceRegionsOutput::source_regions): <p>A list of <code>SourceRegion</code> instances that contains each source Amazon Web Services Region that the current Amazon Web Services Region can get a read replica or a DB snapshot from.</p>
                            /// - On failure, responds with [`SdkError<DescribeSourceRegionsError>`](crate::error::DescribeSourceRegionsError)
    pub fn describe_source_regions(&self) -> crate::client::fluent_builders::DescribeSourceRegions {
                                crate::client::fluent_builders::DescribeSourceRegions::new(self.handle.clone())
                            }
}

