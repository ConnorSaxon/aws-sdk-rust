// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteFileSystem`](crate::client::fluent_builders::DeleteFileSystem) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`file_system_id(impl Into<String>)`](crate::client::fluent_builders::DeleteFileSystem::file_system_id) / [`set_file_system_id(Option<String>)`](crate::client::fluent_builders::DeleteFileSystem::set_file_system_id): <p>The ID of the file system that you want to delete.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::DeleteFileSystem::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::DeleteFileSystem::set_client_request_token): <p>A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent deletion. This token is automatically filled on your behalf when using the Command Line Interface (CLI) or an Amazon Web Services SDK.</p>
    ///   - [`windows_configuration(DeleteFileSystemWindowsConfiguration)`](crate::client::fluent_builders::DeleteFileSystem::windows_configuration) / [`set_windows_configuration(Option<DeleteFileSystemWindowsConfiguration>)`](crate::client::fluent_builders::DeleteFileSystem::set_windows_configuration): <p>The configuration object for the Microsoft Windows file system used in the <code>DeleteFileSystem</code> operation.</p>
    ///   - [`lustre_configuration(DeleteFileSystemLustreConfiguration)`](crate::client::fluent_builders::DeleteFileSystem::lustre_configuration) / [`set_lustre_configuration(Option<DeleteFileSystemLustreConfiguration>)`](crate::client::fluent_builders::DeleteFileSystem::set_lustre_configuration): <p>The configuration object for the Amazon FSx for Lustre file system being deleted in the <code>DeleteFileSystem</code> operation.</p>
    ///   - [`open_zfs_configuration(DeleteFileSystemOpenZfsConfiguration)`](crate::client::fluent_builders::DeleteFileSystem::open_zfs_configuration) / [`set_open_zfs_configuration(Option<DeleteFileSystemOpenZfsConfiguration>)`](crate::client::fluent_builders::DeleteFileSystem::set_open_zfs_configuration): <p>The configuration object for the OpenZFS file system used in the <code>DeleteFileSystem</code> operation.</p>
                            /// - On success, responds with [`DeleteFileSystemOutput`](crate::output::DeleteFileSystemOutput) with field(s):
    ///   - [`file_system_id(Option<String>)`](crate::output::DeleteFileSystemOutput::file_system_id): <p>The ID of the file system that's being deleted.</p>
    ///   - [`lifecycle(Option<FileSystemLifecycle>)`](crate::output::DeleteFileSystemOutput::lifecycle): <p>The file system lifecycle for the deletion request. If the <code>DeleteFileSystem</code> operation is successful, this status is <code>DELETING</code>.</p>
    ///   - [`windows_response(Option<DeleteFileSystemWindowsResponse>)`](crate::output::DeleteFileSystemOutput::windows_response): <p>The response object for the Microsoft Windows file system used in the <code>DeleteFileSystem</code> operation.</p>
    ///   - [`lustre_response(Option<DeleteFileSystemLustreResponse>)`](crate::output::DeleteFileSystemOutput::lustre_response): <p>The response object for the Amazon FSx for Lustre file system being deleted in the <code>DeleteFileSystem</code> operation.</p>
    ///   - [`open_zfs_response(Option<DeleteFileSystemOpenZfsResponse>)`](crate::output::DeleteFileSystemOutput::open_zfs_response): <p>The response object for the OpenZFS file system that's being deleted in the <code>DeleteFileSystem</code> operation.</p>
                            /// - On failure, responds with [`SdkError<DeleteFileSystemError>`](crate::error::DeleteFileSystemError)
    pub fn delete_file_system(&self) -> crate::client::fluent_builders::DeleteFileSystem {
                                crate::client::fluent_builders::DeleteFileSystem::new(self.handle.clone())
                            }
}

