// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeOrderableDBInstanceOptions`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`engine(impl Into<String>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::engine) / [`set_engine(Option<String>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::set_engine): <p>The name of the engine to retrieve DB instance options for.</p>
    ///   - [`engine_version(impl Into<String>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::engine_version) / [`set_engine_version(Option<String>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::set_engine_version): <p>The engine version filter value. Specify this parameter to show only the available offerings matching the specified engine version.</p>
    ///   - [`db_instance_class(impl Into<String>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::db_instance_class) / [`set_db_instance_class(Option<String>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::set_db_instance_class): <p>The DB instance class filter value. Specify this parameter to show only the available offerings matching the specified DB instance class.</p>
    ///   - [`license_model(impl Into<String>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::license_model) / [`set_license_model(Option<String>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::set_license_model): <p>The license model filter value. Specify this parameter to show only the available offerings matching the specified license model.</p>
    ///   - [`vpc(bool)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::vpc) / [`set_vpc(Option<bool>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::set_vpc): <p>The VPC filter value. Specify this parameter to show only the available VPC or non-VPC offerings.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::set_filters): <p>This parameter is not currently supported.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::set_max_records): <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>  <p>Default: 100</p>  <p>Constraints: Minimum 20, maximum 100.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::set_marker): <p> An optional pagination token provided by a previous DescribeOrderableDBInstanceOptions request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> .</p>
                            /// - On success, responds with [`DescribeOrderableDbInstanceOptionsOutput`](crate::output::DescribeOrderableDbInstanceOptionsOutput) with field(s):
    ///   - [`orderable_db_instance_options(Option<Vec<OrderableDbInstanceOption>>)`](crate::output::DescribeOrderableDbInstanceOptionsOutput::orderable_db_instance_options): <p>An <code>OrderableDBInstanceOption</code> structure containing information about orderable options for the DB instance.</p>
    ///   - [`marker(Option<String>)`](crate::output::DescribeOrderableDbInstanceOptionsOutput::marker): <p> An optional pagination token provided by a previous OrderableDBInstanceOptions request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> .</p>
                            /// - On failure, responds with [`SdkError<DescribeOrderableDBInstanceOptionsError>`](crate::error::DescribeOrderableDBInstanceOptionsError)
    pub fn describe_orderable_db_instance_options(&self) -> crate::client::fluent_builders::DescribeOrderableDBInstanceOptions {
                                crate::client::fluent_builders::DescribeOrderableDBInstanceOptions::new(self.handle.clone())
                            }
}

