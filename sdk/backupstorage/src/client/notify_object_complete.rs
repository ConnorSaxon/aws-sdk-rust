// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`NotifyObjectComplete`](crate::client::fluent_builders::NotifyObjectComplete) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`backup_job_id(impl Into<String>)`](crate::client::fluent_builders::NotifyObjectComplete::backup_job_id) / [`set_backup_job_id(Option<String>)`](crate::client::fluent_builders::NotifyObjectComplete::set_backup_job_id): Backup job Id for the in-progress backup
    ///   - [`upload_id(impl Into<String>)`](crate::client::fluent_builders::NotifyObjectComplete::upload_id) / [`set_upload_id(Option<String>)`](crate::client::fluent_builders::NotifyObjectComplete::set_upload_id): Upload Id for the in-progress upload
    ///   - [`object_checksum(impl Into<String>)`](crate::client::fluent_builders::NotifyObjectComplete::object_checksum) / [`set_object_checksum(Option<String>)`](crate::client::fluent_builders::NotifyObjectComplete::set_object_checksum): Object checksum
    ///   - [`object_checksum_algorithm(SummaryChecksumAlgorithm)`](crate::client::fluent_builders::NotifyObjectComplete::object_checksum_algorithm) / [`set_object_checksum_algorithm(Option<SummaryChecksumAlgorithm>)`](crate::client::fluent_builders::NotifyObjectComplete::set_object_checksum_algorithm): Checksum algorithm
    ///   - [`metadata_string(impl Into<String>)`](crate::client::fluent_builders::NotifyObjectComplete::metadata_string) / [`set_metadata_string(Option<String>)`](crate::client::fluent_builders::NotifyObjectComplete::set_metadata_string): Optional metadata associated with an Object. Maximum string length is 256 bytes.
    ///   - [`metadata_blob(ByteStream)`](crate::client::fluent_builders::NotifyObjectComplete::metadata_blob) / [`set_metadata_blob(ByteStream)`](crate::client::fluent_builders::NotifyObjectComplete::set_metadata_blob): Optional metadata associated with an Object. Maximum length is 4MB.
    ///   - [`metadata_blob_length(i64)`](crate::client::fluent_builders::NotifyObjectComplete::metadata_blob_length) / [`set_metadata_blob_length(i64)`](crate::client::fluent_builders::NotifyObjectComplete::set_metadata_blob_length): The size of MetadataBlob.
    ///   - [`metadata_blob_checksum(impl Into<String>)`](crate::client::fluent_builders::NotifyObjectComplete::metadata_blob_checksum) / [`set_metadata_blob_checksum(Option<String>)`](crate::client::fluent_builders::NotifyObjectComplete::set_metadata_blob_checksum): Checksum of MetadataBlob.
    ///   - [`metadata_blob_checksum_algorithm(DataChecksumAlgorithm)`](crate::client::fluent_builders::NotifyObjectComplete::metadata_blob_checksum_algorithm) / [`set_metadata_blob_checksum_algorithm(Option<DataChecksumAlgorithm>)`](crate::client::fluent_builders::NotifyObjectComplete::set_metadata_blob_checksum_algorithm): Checksum algorithm.
                            /// - On success, responds with [`NotifyObjectCompleteOutput`](crate::output::NotifyObjectCompleteOutput) with field(s):
    ///   - [`object_checksum(Option<String>)`](crate::output::NotifyObjectCompleteOutput::object_checksum): Object checksum
    ///   - [`object_checksum_algorithm(Option<SummaryChecksumAlgorithm>)`](crate::output::NotifyObjectCompleteOutput::object_checksum_algorithm): Checksum algorithm
                            /// - On failure, responds with [`SdkError<NotifyObjectCompleteError>`](crate::error::NotifyObjectCompleteError)
    pub fn notify_object_complete(&self) -> crate::client::fluent_builders::NotifyObjectComplete {
                                crate::client::fluent_builders::NotifyObjectComplete::new(self.handle.clone())
                            }
}

