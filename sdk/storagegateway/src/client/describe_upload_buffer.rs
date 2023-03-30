// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeUploadBuffer`](crate::client::fluent_builders::DescribeUploadBuffer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeUploadBuffer::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::client::fluent_builders::DescribeUploadBuffer::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
                            /// - On success, responds with [`DescribeUploadBufferOutput`](crate::output::DescribeUploadBufferOutput) with field(s):
    ///   - [`gateway_arn(Option<String>)`](crate::output::DescribeUploadBufferOutput::gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    ///   - [`disk_ids(Option<Vec<String>>)`](crate::output::DescribeUploadBufferOutput::disk_ids): <p>An array of the gateway's local disk IDs that are configured as working storage. Each local disk ID is specified as a string (minimum length of 1 and maximum length of 300). If no local disks are configured as working storage, then the DiskIds array is empty.</p>
    ///   - [`upload_buffer_used_in_bytes(i64)`](crate::output::DescribeUploadBufferOutput::upload_buffer_used_in_bytes): <p>The total number of bytes being used in the gateway's upload buffer.</p>
    ///   - [`upload_buffer_allocated_in_bytes(i64)`](crate::output::DescribeUploadBufferOutput::upload_buffer_allocated_in_bytes): <p>The total number of bytes allocated in the gateway's as upload buffer.</p>
                            /// - On failure, responds with [`SdkError<DescribeUploadBufferError>`](crate::error::DescribeUploadBufferError)
    pub fn describe_upload_buffer(&self) -> crate::client::fluent_builders::DescribeUploadBuffer {
                                crate::client::fluent_builders::DescribeUploadBuffer::new(self.handle.clone())
                            }
}

