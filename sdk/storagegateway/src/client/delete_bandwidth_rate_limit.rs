// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteBandwidthRateLimit`](crate::client::fluent_builders::DeleteBandwidthRateLimit) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteBandwidthRateLimit::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::client::fluent_builders::DeleteBandwidthRateLimit::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    ///   - [`bandwidth_type(impl Into<String>)`](crate::client::fluent_builders::DeleteBandwidthRateLimit::bandwidth_type) / [`set_bandwidth_type(Option<String>)`](crate::client::fluent_builders::DeleteBandwidthRateLimit::set_bandwidth_type): <p>One of the BandwidthType values that indicates the gateway bandwidth rate limit to delete.</p>  <p>Valid Values: <code>UPLOAD</code> | <code>DOWNLOAD</code> | <code>ALL</code> </p>
                            /// - On success, responds with [`DeleteBandwidthRateLimitOutput`](crate::output::DeleteBandwidthRateLimitOutput) with field(s):
    ///   - [`gateway_arn(Option<String>)`](crate::output::DeleteBandwidthRateLimitOutput::gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
                            /// - On failure, responds with [`SdkError<DeleteBandwidthRateLimitError>`](crate::error::DeleteBandwidthRateLimitError)
    pub fn delete_bandwidth_rate_limit(&self) -> crate::client::fluent_builders::DeleteBandwidthRateLimit {
                                crate::client::fluent_builders::DeleteBandwidthRateLimit::new(self.handle.clone())
                            }
}

