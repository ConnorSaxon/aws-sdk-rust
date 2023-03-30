// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAdjustmentTypes`](crate::client::fluent_builders::DescribeAdjustmentTypes) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DescribeAdjustmentTypes::send) it.
                            /// - On success, responds with [`DescribeAdjustmentTypesOutput`](crate::output::DescribeAdjustmentTypesOutput) with field(s):
    ///   - [`adjustment_types(Option<Vec<AdjustmentType>>)`](crate::output::DescribeAdjustmentTypesOutput::adjustment_types): <p>The policy adjustment types.</p>
                            /// - On failure, responds with [`SdkError<DescribeAdjustmentTypesError>`](crate::error::DescribeAdjustmentTypesError)
    pub fn describe_adjustment_types(&self) -> crate::client::fluent_builders::DescribeAdjustmentTypes {
                                crate::client::fluent_builders::DescribeAdjustmentTypes::new(self.handle.clone())
                            }
}

