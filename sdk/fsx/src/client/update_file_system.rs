// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateFileSystem`](crate::client::fluent_builders::UpdateFileSystem) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`file_system_id(impl Into<String>)`](crate::client::fluent_builders::UpdateFileSystem::file_system_id) / [`set_file_system_id(Option<String>)`](crate::client::fluent_builders::UpdateFileSystem::set_file_system_id): <p>The ID of the file system that you are updating.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::UpdateFileSystem::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::UpdateFileSystem::set_client_request_token): <p>A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent updates. This string is automatically filled on your behalf when you use the Command Line Interface (CLI) or an Amazon Web Services SDK.</p>
    ///   - [`storage_capacity(i32)`](crate::client::fluent_builders::UpdateFileSystem::storage_capacity) / [`set_storage_capacity(Option<i32>)`](crate::client::fluent_builders::UpdateFileSystem::set_storage_capacity): <p>Use this parameter to increase the storage capacity of an Amazon FSx for Windows File Server, Amazon FSx for Lustre, or Amazon FSx for NetApp ONTAP file system. Specifies the storage capacity target value, in GiB, to increase the storage capacity for the file system that you're updating. </p> <note>   <p>You can't make a storage capacity increase request if there is an existing storage capacity increase request in progress.</p>  </note>  <p>For Windows file systems, the storage capacity target value must be at least 10 percent greater than the current storage capacity value. To increase storage capacity, the file system must have at least 16 MBps of throughput capacity. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-storage-capacity.html">Managing storage capacity</a> in the <i>Amazon FSx for Windows File Server User Guide</i>.</p>  <p>For Lustre file systems, the storage capacity target value can be the following:</p>  <ul>   <li> <p>For <code>SCRATCH_2</code>, <code>PERSISTENT_1</code>, and <code>PERSISTENT_2 SSD</code> deployment types, valid values are in multiples of 2400 GiB. The value must be greater than the current storage capacity.</p> </li>   <li> <p>For <code>PERSISTENT HDD</code> file systems, valid values are multiples of 6000 GiB for 12-MBps throughput per TiB file systems and multiples of 1800 GiB for 40-MBps throughput per TiB file systems. The values must be greater than the current storage capacity.</p> </li>   <li> <p>For <code>SCRATCH_1</code> file systems, you can't increase the storage capacity.</p> </li>  </ul>  <p>For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/managing-storage-capacity.html">Managing storage and throughput capacity</a> in the <i>Amazon FSx for Lustre User Guide</i>.</p>  <p>For ONTAP file systems, the storage capacity target value must be at least 10 percent greater than the current storage capacity value. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/managing-storage-capacity.html">Managing storage capacity and provisioned IOPS</a> in the <i>Amazon FSx for NetApp ONTAP User Guide</i>.</p>
    ///   - [`windows_configuration(UpdateFileSystemWindowsConfiguration)`](crate::client::fluent_builders::UpdateFileSystem::windows_configuration) / [`set_windows_configuration(Option<UpdateFileSystemWindowsConfiguration>)`](crate::client::fluent_builders::UpdateFileSystem::set_windows_configuration): <p>The configuration updates for an Amazon FSx for Windows File Server file system.</p>
    ///   - [`lustre_configuration(UpdateFileSystemLustreConfiguration)`](crate::client::fluent_builders::UpdateFileSystem::lustre_configuration) / [`set_lustre_configuration(Option<UpdateFileSystemLustreConfiguration>)`](crate::client::fluent_builders::UpdateFileSystem::set_lustre_configuration): <p>The configuration object for Amazon FSx for Lustre file systems used in the <code>UpdateFileSystem</code> operation.</p>
    ///   - [`ontap_configuration(UpdateFileSystemOntapConfiguration)`](crate::client::fluent_builders::UpdateFileSystem::ontap_configuration) / [`set_ontap_configuration(Option<UpdateFileSystemOntapConfiguration>)`](crate::client::fluent_builders::UpdateFileSystem::set_ontap_configuration): <p>The configuration updates for an Amazon FSx for NetApp ONTAP file system.</p>
    ///   - [`open_zfs_configuration(UpdateFileSystemOpenZfsConfiguration)`](crate::client::fluent_builders::UpdateFileSystem::open_zfs_configuration) / [`set_open_zfs_configuration(Option<UpdateFileSystemOpenZfsConfiguration>)`](crate::client::fluent_builders::UpdateFileSystem::set_open_zfs_configuration): <p>The configuration updates for an Amazon FSx for OpenZFS file system.</p>
                            /// - On success, responds with [`UpdateFileSystemOutput`](crate::output::UpdateFileSystemOutput) with field(s):
    ///   - [`file_system(Option<FileSystem>)`](crate::output::UpdateFileSystemOutput::file_system): <p>A description of the file system that was updated.</p>
                            /// - On failure, responds with [`SdkError<UpdateFileSystemError>`](crate::error::UpdateFileSystemError)
    pub fn update_file_system(&self) -> crate::client::fluent_builders::UpdateFileSystem {
                                crate::client::fluent_builders::UpdateFileSystem::new(self.handle.clone())
                            }
}

