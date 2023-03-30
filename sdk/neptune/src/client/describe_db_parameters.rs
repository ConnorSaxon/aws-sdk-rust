// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDBParameters`](crate::client::fluent_builders::DescribeDBParameters) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeDBParameters::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_parameter_group_name(impl Into<String>)`](crate::client::fluent_builders::DescribeDBParameters::db_parameter_group_name) / [`set_db_parameter_group_name(Option<String>)`](crate::client::fluent_builders::DescribeDBParameters::set_db_parameter_group_name): <p>The name of a specific DB parameter group to return details for.</p>  <p>Constraints:</p>  <ul>   <li> <p>If supplied, must match the name of an existing DBParameterGroup.</p> </li>  </ul>
    ///   - [`source(impl Into<String>)`](crate::client::fluent_builders::DescribeDBParameters::source) / [`set_source(Option<String>)`](crate::client::fluent_builders::DescribeDBParameters::set_source): <p>The parameter types to return.</p>  <p>Default: All parameter types returned</p>  <p>Valid Values: <code>user | system | engine-default</code> </p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeDBParameters::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeDBParameters::set_filters): <p>This parameter is not currently supported.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeDBParameters::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeDBParameters::set_max_records): <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>  <p>Default: 100</p>  <p>Constraints: Minimum 20, maximum 100.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeDBParameters::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeDBParameters::set_marker): <p>An optional pagination token provided by a previous <code>DescribeDBParameters</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
                            /// - On success, responds with [`DescribeDbParametersOutput`](crate::output::DescribeDbParametersOutput) with field(s):
    ///   - [`parameters(Option<Vec<Parameter>>)`](crate::output::DescribeDbParametersOutput::parameters): <p>A list of <code>Parameter</code> values.</p>
    ///   - [`marker(Option<String>)`](crate::output::DescribeDbParametersOutput::marker): <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
                            /// - On failure, responds with [`SdkError<DescribeDBParametersError>`](crate::error::DescribeDBParametersError)
    pub fn describe_db_parameters(&self) -> crate::client::fluent_builders::DescribeDBParameters {
                                crate::client::fluent_builders::DescribeDBParameters::new(self.handle.clone())
                            }
}

