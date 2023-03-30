// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAcceleratorAttributes`](crate::client::fluent_builders::DescribeAcceleratorAttributes) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`accelerator_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeAcceleratorAttributes::accelerator_arn) / [`set_accelerator_arn(Option<String>)`](crate::client::fluent_builders::DescribeAcceleratorAttributes::set_accelerator_arn): <p>The Amazon Resource Name (ARN) of the accelerator with the attributes that you want to describe.</p>
                            /// - On success, responds with [`DescribeAcceleratorAttributesOutput`](crate::output::DescribeAcceleratorAttributesOutput) with field(s):
    ///   - [`accelerator_attributes(Option<AcceleratorAttributes>)`](crate::output::DescribeAcceleratorAttributesOutput::accelerator_attributes): <p>The attributes of the accelerator.</p>
                            /// - On failure, responds with [`SdkError<DescribeAcceleratorAttributesError>`](crate::error::DescribeAcceleratorAttributesError)
    pub fn describe_accelerator_attributes(&self) -> crate::client::fluent_builders::DescribeAcceleratorAttributes {
                                crate::client::fluent_builders::DescribeAcceleratorAttributes::new(self.handle.clone())
                            }
}

