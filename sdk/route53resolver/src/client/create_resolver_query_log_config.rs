// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateResolverQueryLogConfig`](crate::client::fluent_builders::CreateResolverQueryLogConfig) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateResolverQueryLogConfig::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateResolverQueryLogConfig::set_name): <p>The name that you want to give the query logging configuration.</p>
    ///   - [`destination_arn(impl Into<String>)`](crate::client::fluent_builders::CreateResolverQueryLogConfig::destination_arn) / [`set_destination_arn(Option<String>)`](crate::client::fluent_builders::CreateResolverQueryLogConfig::set_destination_arn): <p>The ARN of the resource that you want Resolver to send query logs. You can send query logs to an S3 bucket, a CloudWatch Logs log group, or a Kinesis Data Firehose delivery stream. Examples of valid values include the following:</p>  <ul>   <li> <p> <b>S3 bucket</b>: </p> <p> <code>arn:aws:s3:::examplebucket</code> </p> <p>You can optionally append a file prefix to the end of the ARN.</p> <p> <code>arn:aws:s3:::examplebucket/development/</code> </p> </li>   <li> <p> <b>CloudWatch Logs log group</b>: </p> <p> <code>arn:aws:logs:us-west-1:123456789012:log-group:/mystack-testgroup-12ABC1AB12A1:*</code> </p> </li>   <li> <p> <b>Kinesis Data Firehose delivery stream</b>:</p> <p> <code>arn:aws:kinesis:us-east-2:0123456789:stream/my_stream_name</code> </p> </li>  </ul>
    ///   - [`creator_request_id(impl Into<String>)`](crate::client::fluent_builders::CreateResolverQueryLogConfig::creator_request_id) / [`set_creator_request_id(Option<String>)`](crate::client::fluent_builders::CreateResolverQueryLogConfig::set_creator_request_id): <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp. </p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateResolverQueryLogConfig::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateResolverQueryLogConfig::set_tags): <p>A list of the tag keys and values that you want to associate with the query logging configuration.</p>
                            /// - On success, responds with [`CreateResolverQueryLogConfigOutput`](crate::output::CreateResolverQueryLogConfigOutput) with field(s):
    ///   - [`resolver_query_log_config(Option<ResolverQueryLogConfig>)`](crate::output::CreateResolverQueryLogConfigOutput::resolver_query_log_config): <p>Information about the <code>CreateResolverQueryLogConfig</code> request, including the status of the request.</p>
                            /// - On failure, responds with [`SdkError<CreateResolverQueryLogConfigError>`](crate::error::CreateResolverQueryLogConfigError)
    pub fn create_resolver_query_log_config(&self) -> crate::client::fluent_builders::CreateResolverQueryLogConfig {
                                crate::client::fluent_builders::CreateResolverQueryLogConfig::new(self.handle.clone())
                            }
}

