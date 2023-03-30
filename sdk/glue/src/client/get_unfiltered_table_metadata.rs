// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetUnfilteredTableMetadata`](crate::client::fluent_builders::GetUnfilteredTableMetadata) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::client::fluent_builders::GetUnfilteredTableMetadata::catalog_id) / [`set_catalog_id(Option<String>)`](crate::client::fluent_builders::GetUnfilteredTableMetadata::set_catalog_id): (undocumented)
    ///   - [`database_name(impl Into<String>)`](crate::client::fluent_builders::GetUnfilteredTableMetadata::database_name) / [`set_database_name(Option<String>)`](crate::client::fluent_builders::GetUnfilteredTableMetadata::set_database_name): (undocumented)
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::GetUnfilteredTableMetadata::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::GetUnfilteredTableMetadata::set_name): (undocumented)
    ///   - [`audit_context(AuditContext)`](crate::client::fluent_builders::GetUnfilteredTableMetadata::audit_context) / [`set_audit_context(Option<AuditContext>)`](crate::client::fluent_builders::GetUnfilteredTableMetadata::set_audit_context): <p>A structure containing information for audit.</p>
    ///   - [`supported_permission_types(Vec<PermissionType>)`](crate::client::fluent_builders::GetUnfilteredTableMetadata::supported_permission_types) / [`set_supported_permission_types(Option<Vec<PermissionType>>)`](crate::client::fluent_builders::GetUnfilteredTableMetadata::set_supported_permission_types): (undocumented)
                            /// - On success, responds with [`GetUnfilteredTableMetadataOutput`](crate::output::GetUnfilteredTableMetadataOutput) with field(s):
    ///   - [`table(Option<Table>)`](crate::output::GetUnfilteredTableMetadataOutput::table): <p>Represents a collection of related data organized in columns and rows.</p>
    ///   - [`authorized_columns(Option<Vec<String>>)`](crate::output::GetUnfilteredTableMetadataOutput::authorized_columns): (undocumented)
    ///   - [`is_registered_with_lake_formation(bool)`](crate::output::GetUnfilteredTableMetadataOutput::is_registered_with_lake_formation): (undocumented)
    ///   - [`cell_filters(Option<Vec<ColumnRowFilter>>)`](crate::output::GetUnfilteredTableMetadataOutput::cell_filters): (undocumented)
                            /// - On failure, responds with [`SdkError<GetUnfilteredTableMetadataError>`](crate::error::GetUnfilteredTableMetadataError)
    pub fn get_unfiltered_table_metadata(&self) -> crate::client::fluent_builders::GetUnfilteredTableMetadata {
                                crate::client::fluent_builders::GetUnfilteredTableMetadata::new(self.handle.clone())
                            }
}

