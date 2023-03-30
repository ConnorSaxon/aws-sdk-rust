// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateFpgaImage`](crate::client::fluent_builders::CreateFpgaImage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateFpgaImage::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CreateFpgaImage::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`input_storage_location(StorageLocation)`](crate::client::fluent_builders::CreateFpgaImage::input_storage_location) / [`set_input_storage_location(Option<StorageLocation>)`](crate::client::fluent_builders::CreateFpgaImage::set_input_storage_location): <p>The location of the encrypted design checkpoint in Amazon S3. The input must be a tarball.</p>
    ///   - [`logs_storage_location(StorageLocation)`](crate::client::fluent_builders::CreateFpgaImage::logs_storage_location) / [`set_logs_storage_location(Option<StorageLocation>)`](crate::client::fluent_builders::CreateFpgaImage::set_logs_storage_location): <p>The location in Amazon S3 for the output logs.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateFpgaImage::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateFpgaImage::set_description): <p>A description for the AFI.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateFpgaImage::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateFpgaImage::set_name): <p>A name for the AFI.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateFpgaImage::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateFpgaImage::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::CreateFpgaImage::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::CreateFpgaImage::set_tag_specifications): <p>The tags to apply to the FPGA image during creation.</p>
                            /// - On success, responds with [`CreateFpgaImageOutput`](crate::output::CreateFpgaImageOutput) with field(s):
    ///   - [`fpga_image_id(Option<String>)`](crate::output::CreateFpgaImageOutput::fpga_image_id): <p>The FPGA image identifier (AFI ID).</p>
    ///   - [`fpga_image_global_id(Option<String>)`](crate::output::CreateFpgaImageOutput::fpga_image_global_id): <p>The global FPGA image identifier (AGFI ID).</p>
                            /// - On failure, responds with [`SdkError<CreateFpgaImageError>`](crate::error::CreateFpgaImageError)
    pub fn create_fpga_image(&self) -> crate::client::fluent_builders::CreateFpgaImage {
                                crate::client::fluent_builders::CreateFpgaImage::new(self.handle.clone())
                            }
}

