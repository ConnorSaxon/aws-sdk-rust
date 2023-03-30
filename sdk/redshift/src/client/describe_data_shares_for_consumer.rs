// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDataSharesForConsumer`](crate::client::fluent_builders::DescribeDataSharesForConsumer) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeDataSharesForConsumer::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`consumer_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeDataSharesForConsumer::consumer_arn) / [`set_consumer_arn(Option<String>)`](crate::client::fluent_builders::DescribeDataSharesForConsumer::set_consumer_arn): <p>The Amazon Resource Name (ARN) of the consumer that returns in the list of datashares.</p>
    ///   - [`status(DataShareStatusForConsumer)`](crate::client::fluent_builders::DescribeDataSharesForConsumer::status) / [`set_status(Option<DataShareStatusForConsumer>)`](crate::client::fluent_builders::DescribeDataSharesForConsumer::set_status): <p>An identifier giving the status of a datashare in the consumer cluster. If this field is specified, Amazon Redshift returns the list of datashares that have the specified status.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeDataSharesForConsumer::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeDataSharesForConsumer::set_max_records): <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeDataSharesForConsumer::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeDataSharesForConsumer::set_marker): <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeDataSharesForConsumer</code> request exceed the value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
                            /// - On success, responds with [`DescribeDataSharesForConsumerOutput`](crate::output::DescribeDataSharesForConsumerOutput) with field(s):
    ///   - [`data_shares(Option<Vec<DataShare>>)`](crate::output::DescribeDataSharesForConsumerOutput::data_shares): <p>Shows the results of datashares available for consumers.</p>
    ///   - [`marker(Option<String>)`](crate::output::DescribeDataSharesForConsumerOutput::marker): <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeDataSharesForConsumer</code> request exceed the value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
                            /// - On failure, responds with [`SdkError<DescribeDataSharesForConsumerError>`](crate::error::DescribeDataSharesForConsumerError)
    pub fn describe_data_shares_for_consumer(&self) -> crate::client::fluent_builders::DescribeDataSharesForConsumer {
                                crate::client::fluent_builders::DescribeDataSharesForConsumer::new(self.handle.clone())
                            }
}

