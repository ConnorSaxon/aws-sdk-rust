// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartCanary`](crate::client::fluent_builders::StartCanary) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::StartCanary::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::StartCanary::set_name): <p>The name of the canary that you want to run. To find canary names, use <a href="https://docs.aws.amazon.com/AmazonSynthetics/latest/APIReference/API_DescribeCanaries.html">DescribeCanaries</a>.</p>
                            /// - On success, responds with [`StartCanaryOutput`](crate::output::StartCanaryOutput)
                            /// - On failure, responds with [`SdkError<StartCanaryError>`](crate::error::StartCanaryError)
    pub fn start_canary(&self) -> crate::client::fluent_builders::StartCanary {
                                crate::client::fluent_builders::StartCanary::new(self.handle.clone())
                            }
}

