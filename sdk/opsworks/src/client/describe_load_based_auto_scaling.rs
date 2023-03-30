// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeLoadBasedAutoScaling`](crate::client::fluent_builders::DescribeLoadBasedAutoScaling) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`layer_ids(Vec<String>)`](crate::client::fluent_builders::DescribeLoadBasedAutoScaling::layer_ids) / [`set_layer_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeLoadBasedAutoScaling::set_layer_ids): <p>An array of layer IDs.</p>
                            /// - On success, responds with [`DescribeLoadBasedAutoScalingOutput`](crate::output::DescribeLoadBasedAutoScalingOutput) with field(s):
    ///   - [`load_based_auto_scaling_configurations(Option<Vec<LoadBasedAutoScalingConfiguration>>)`](crate::output::DescribeLoadBasedAutoScalingOutput::load_based_auto_scaling_configurations): <p>An array of <code>LoadBasedAutoScalingConfiguration</code> objects that describe each layer's configuration.</p>
                            /// - On failure, responds with [`SdkError<DescribeLoadBasedAutoScalingError>`](crate::error::DescribeLoadBasedAutoScalingError)
    pub fn describe_load_based_auto_scaling(&self) -> crate::client::fluent_builders::DescribeLoadBasedAutoScaling {
                                crate::client::fluent_builders::DescribeLoadBasedAutoScaling::new(self.handle.clone())
                            }
}

