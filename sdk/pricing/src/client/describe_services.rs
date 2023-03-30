// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeServices`](crate::client::fluent_builders::DescribeServices) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeServices::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`service_code(impl Into<String>)`](crate::client::fluent_builders::DescribeServices::service_code) / [`set_service_code(Option<String>)`](crate::client::fluent_builders::DescribeServices::set_service_code): <p>The code for the service whose information you want to retrieve, such as <code>AmazonEC2</code>. You can use the <code>ServiceCode</code> to filter the results in a <code>GetProducts</code> call. To retrieve a list of all services, leave this blank.</p>
    ///   - [`format_version(impl Into<String>)`](crate::client::fluent_builders::DescribeServices::format_version) / [`set_format_version(Option<String>)`](crate::client::fluent_builders::DescribeServices::set_format_version): <p>The format version that you want the response to be in.</p>  <p>Valid values are: <code>aws_v1</code> </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeServices::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeServices::set_next_token): <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeServices::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeServices::set_max_results): <p>The maximum number of results that you want returned in the response.</p>
                            /// - On success, responds with [`DescribeServicesOutput`](crate::output::DescribeServicesOutput) with field(s):
    ///   - [`services(Option<Vec<Service>>)`](crate::output::DescribeServicesOutput::services): <p>The service metadata for the service or services in the response.</p>
    ///   - [`format_version(Option<String>)`](crate::output::DescribeServicesOutput::format_version): <p>The format version of the response. For example, <code>aws_v1</code>.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeServicesOutput::next_token): <p>The pagination token for the next set of retrievable results.</p>
                            /// - On failure, responds with [`SdkError<DescribeServicesError>`](crate::error::DescribeServicesError)
    pub fn describe_services(&self) -> crate::client::fluent_builders::DescribeServices {
                                crate::client::fluent_builders::DescribeServices::new(self.handle.clone())
                            }
}

