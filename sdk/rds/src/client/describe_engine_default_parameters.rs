// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeEngineDefaultParameters`](crate::client::fluent_builders::DescribeEngineDefaultParameters) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeEngineDefaultParameters::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_parameter_group_family(impl Into<String>)`](crate::client::fluent_builders::DescribeEngineDefaultParameters::db_parameter_group_family) / [`set_db_parameter_group_family(Option<String>)`](crate::client::fluent_builders::DescribeEngineDefaultParameters::set_db_parameter_group_family): <p>The name of the DB parameter group family.</p>  <p>Valid Values:</p>  <ul>   <li> <p> <code>aurora5.6</code> </p> </li>   <li> <p> <code>aurora-mysql5.7</code> </p> </li>   <li> <p> <code>aurora-mysql8.0</code> </p> </li>   <li> <p> <code>aurora-postgresql10</code> </p> </li>   <li> <p> <code>aurora-postgresql11</code> </p> </li>   <li> <p> <code>aurora-postgresql12</code> </p> </li>   <li> <p> <code>aurora-postgresql13</code> </p> </li>   <li> <p> <code>aurora-postgresql14</code> </p> </li>   <li> <p> <code>custom-oracle-ee-19</code> </p> </li>   <li> <p> <code>mariadb10.2</code> </p> </li>   <li> <p> <code>mariadb10.3</code> </p> </li>   <li> <p> <code>mariadb10.4</code> </p> </li>   <li> <p> <code>mariadb10.5</code> </p> </li>   <li> <p> <code>mariadb10.6</code> </p> </li>   <li> <p> <code>mysql5.7</code> </p> </li>   <li> <p> <code>mysql8.0</code> </p> </li>   <li> <p> <code>oracle-ee-19</code> </p> </li>   <li> <p> <code>oracle-ee-cdb-19</code> </p> </li>   <li> <p> <code>oracle-ee-cdb-21</code> </p> </li>   <li> <p> <code>oracle-se2-19</code> </p> </li>   <li> <p> <code>oracle-se2-cdb-19</code> </p> </li>   <li> <p> <code>oracle-se2-cdb-21</code> </p> </li>   <li> <p> <code>postgres10</code> </p> </li>   <li> <p> <code>postgres11</code> </p> </li>   <li> <p> <code>postgres12</code> </p> </li>   <li> <p> <code>postgres13</code> </p> </li>   <li> <p> <code>postgres14</code> </p> </li>   <li> <p> <code>sqlserver-ee-11.0</code> </p> </li>   <li> <p> <code>sqlserver-ee-12.0</code> </p> </li>   <li> <p> <code>sqlserver-ee-13.0</code> </p> </li>   <li> <p> <code>sqlserver-ee-14.0</code> </p> </li>   <li> <p> <code>sqlserver-ee-15.0</code> </p> </li>   <li> <p> <code>sqlserver-ex-11.0</code> </p> </li>   <li> <p> <code>sqlserver-ex-12.0</code> </p> </li>   <li> <p> <code>sqlserver-ex-13.0</code> </p> </li>   <li> <p> <code>sqlserver-ex-14.0</code> </p> </li>   <li> <p> <code>sqlserver-ex-15.0</code> </p> </li>   <li> <p> <code>sqlserver-se-11.0</code> </p> </li>   <li> <p> <code>sqlserver-se-12.0</code> </p> </li>   <li> <p> <code>sqlserver-se-13.0</code> </p> </li>   <li> <p> <code>sqlserver-se-14.0</code> </p> </li>   <li> <p> <code>sqlserver-se-15.0</code> </p> </li>   <li> <p> <code>sqlserver-web-11.0</code> </p> </li>   <li> <p> <code>sqlserver-web-12.0</code> </p> </li>   <li> <p> <code>sqlserver-web-13.0</code> </p> </li>   <li> <p> <code>sqlserver-web-14.0</code> </p> </li>   <li> <p> <code>sqlserver-web-15.0</code> </p> </li>  </ul>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeEngineDefaultParameters::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeEngineDefaultParameters::set_filters): <p>This parameter isn't currently supported.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeEngineDefaultParameters::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeEngineDefaultParameters::set_max_records): <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so you can retrieve the remaining results.</p>  <p>Default: 100</p>  <p>Constraints: Minimum 20, maximum 100.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeEngineDefaultParameters::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeEngineDefaultParameters::set_marker): <p>An optional pagination token provided by a previous <code>DescribeEngineDefaultParameters</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
                            /// - On success, responds with [`DescribeEngineDefaultParametersOutput`](crate::output::DescribeEngineDefaultParametersOutput) with field(s):
    ///   - [`engine_defaults(Option<EngineDefaults>)`](crate::output::DescribeEngineDefaultParametersOutput::engine_defaults): <p>Contains the result of a successful invocation of the <code>DescribeEngineDefaultParameters</code> action.</p>
                            /// - On failure, responds with [`SdkError<DescribeEngineDefaultParametersError>`](crate::error::DescribeEngineDefaultParametersError)
    pub fn describe_engine_default_parameters(&self) -> crate::client::fluent_builders::DescribeEngineDefaultParameters {
                                crate::client::fluent_builders::DescribeEngineDefaultParameters::new(self.handle.clone())
                            }
}

