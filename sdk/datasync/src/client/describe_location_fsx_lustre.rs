// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeLocationFsxLustre`](crate::client::fluent_builders::DescribeLocationFsxLustre) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`location_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeLocationFsxLustre::location_arn) / [`set_location_arn(Option<String>)`](crate::client::fluent_builders::DescribeLocationFsxLustre::set_location_arn): <p>The Amazon Resource Name (ARN) of the FSx for Lustre location to describe. </p>
                            /// - On success, responds with [`DescribeLocationFsxLustreOutput`](crate::output::DescribeLocationFsxLustreOutput) with field(s):
    ///   - [`location_arn(Option<String>)`](crate::output::DescribeLocationFsxLustreOutput::location_arn): <p>The Amazon Resource Name (ARN) of the FSx for Lustre location that was described.</p>
    ///   - [`location_uri(Option<String>)`](crate::output::DescribeLocationFsxLustreOutput::location_uri): <p>The URI of the FSx for Lustre location that was described.</p>
    ///   - [`security_group_arns(Option<Vec<String>>)`](crate::output::DescribeLocationFsxLustreOutput::security_group_arns): <p>The Amazon Resource Names (ARNs) of the security groups that are configured for the FSx for Lustre file system.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribeLocationFsxLustreOutput::creation_time): <p>The time that the FSx for Lustre location was created.</p>
                            /// - On failure, responds with [`SdkError<DescribeLocationFsxLustreError>`](crate::error::DescribeLocationFsxLustreError)
    pub fn describe_location_fsx_lustre(&self) -> crate::client::fluent_builders::DescribeLocationFsxLustre {
                                crate::client::fluent_builders::DescribeLocationFsxLustre::new(self.handle.clone())
                            }
}

