// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeFolderResolvedPermissions`](crate::client::fluent_builders::DescribeFolderResolvedPermissions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::DescribeFolderResolvedPermissions::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::DescribeFolderResolvedPermissions::set_aws_account_id): <p>The ID for the Amazon Web Services account that contains the folder.</p>
    ///   - [`folder_id(impl Into<String>)`](crate::client::fluent_builders::DescribeFolderResolvedPermissions::folder_id) / [`set_folder_id(Option<String>)`](crate::client::fluent_builders::DescribeFolderResolvedPermissions::set_folder_id): <p>The ID of the folder.</p>
                            /// - On success, responds with [`DescribeFolderResolvedPermissionsOutput`](crate::output::DescribeFolderResolvedPermissionsOutput) with field(s):
    ///   - [`status(i32)`](crate::output::DescribeFolderResolvedPermissionsOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`folder_id(Option<String>)`](crate::output::DescribeFolderResolvedPermissionsOutput::folder_id): <p>The ID of the folder.</p>
    ///   - [`arn(Option<String>)`](crate::output::DescribeFolderResolvedPermissionsOutput::arn): <p>The Amazon Resource Name (ARN) of the folder.</p>
    ///   - [`permissions(Option<Vec<ResourcePermission>>)`](crate::output::DescribeFolderResolvedPermissionsOutput::permissions): <p>Information about the permissions for the folder.</p>
    ///   - [`request_id(Option<String>)`](crate::output::DescribeFolderResolvedPermissionsOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
                            /// - On failure, responds with [`SdkError<DescribeFolderResolvedPermissionsError>`](crate::error::DescribeFolderResolvedPermissionsError)
    pub fn describe_folder_resolved_permissions(&self) -> crate::client::fluent_builders::DescribeFolderResolvedPermissions {
                                crate::client::fluent_builders::DescribeFolderResolvedPermissions::new(self.handle.clone())
                            }
}

