// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopCanary`](crate::client::fluent_builders::StopCanary) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::StopCanary::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::StopCanary::set_name): <p>The name of the canary that you want to stop. To find the names of your canaries, use <a href="https://docs.aws.amazon.com/AmazonSynthetics/latest/APIReference/API_DescribeCanaries.html">ListCanaries</a>.</p>
                            /// - On success, responds with [`StopCanaryOutput`](crate::output::StopCanaryOutput)
                            /// - On failure, responds with [`SdkError<StopCanaryError>`](crate::error::StopCanaryError)
    pub fn stop_canary(&self) -> crate::client::fluent_builders::StopCanary {
                                crate::client::fluent_builders::StopCanary::new(self.handle.clone())
                            }
}

