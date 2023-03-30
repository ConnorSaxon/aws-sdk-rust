// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeClusterParameters`](crate::client::fluent_builders::DescribeClusterParameters) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeClusterParameters::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`parameter_group_name(impl Into<String>)`](crate::client::fluent_builders::DescribeClusterParameters::parameter_group_name) / [`set_parameter_group_name(Option<String>)`](crate::client::fluent_builders::DescribeClusterParameters::set_parameter_group_name): <p>The name of a cluster parameter group for which to return details.</p>
    ///   - [`source(impl Into<String>)`](crate::client::fluent_builders::DescribeClusterParameters::source) / [`set_source(Option<String>)`](crate::client::fluent_builders::DescribeClusterParameters::set_source): <p>The parameter types to return. Specify <code>user</code> to show parameters that are different form the default. Similarly, specify <code>engine-default</code> to show parameters that are the same as the default parameter group. </p>  <p>Default: All parameter types returned.</p>  <p>Valid Values: <code>user</code> | <code>engine-default</code> </p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeClusterParameters::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeClusterParameters::set_max_records): <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p>  <p>Default: <code>100</code> </p>  <p>Constraints: minimum 20, maximum 100.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeClusterParameters::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeClusterParameters::set_marker): <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeClusterParameters</code> request exceed the value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
                            /// - On success, responds with [`DescribeClusterParametersOutput`](crate::output::DescribeClusterParametersOutput) with field(s):
    ///   - [`parameters(Option<Vec<Parameter>>)`](crate::output::DescribeClusterParametersOutput::parameters): <p>A list of <code>Parameter</code> instances. Each instance lists the parameters of one cluster parameter group. </p>
    ///   - [`marker(Option<String>)`](crate::output::DescribeClusterParametersOutput::marker): <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
                            /// - On failure, responds with [`SdkError<DescribeClusterParametersError>`](crate::error::DescribeClusterParametersError)
    pub fn describe_cluster_parameters(&self) -> crate::client::fluent_builders::DescribeClusterParameters {
                                crate::client::fluent_builders::DescribeClusterParameters::new(self.handle.clone())
                            }
}

