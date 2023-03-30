// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartImport`](crate::client::fluent_builders::StartImport) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`payload(Blob)`](crate::client::fluent_builders::StartImport::payload) / [`set_payload(Option<Blob>)`](crate::client::fluent_builders::StartImport::set_payload): <p>A zip archive in binary format. The archive should contain one file, a JSON file containing the resource to import. The resource should match the type specified in the <code>resourceType</code> field.</p>
    ///   - [`resource_type(ResourceType)`](crate::client::fluent_builders::StartImport::resource_type) / [`set_resource_type(Option<ResourceType>)`](crate::client::fluent_builders::StartImport::set_resource_type): <p>Specifies the type of resource to export. Each resource also exports any resources that it depends on. </p>  <ul>   <li> <p>A bot exports dependent intents.</p> </li>   <li> <p>An intent exports dependent slot types.</p> </li>  </ul>
    ///   - [`merge_strategy(MergeStrategy)`](crate::client::fluent_builders::StartImport::merge_strategy) / [`set_merge_strategy(Option<MergeStrategy>)`](crate::client::fluent_builders::StartImport::set_merge_strategy): <p>Specifies the action that the <code>StartImport</code> operation should take when there is an existing resource with the same name.</p>  <ul>   <li> <p>FAIL_ON_CONFLICT - The import operation is stopped on the first conflict between a resource in the import file and an existing resource. The name of the resource causing the conflict is in the <code>failureReason</code> field of the response to the <code>GetImport</code> operation.</p> <p>OVERWRITE_LATEST - The import operation proceeds even if there is a conflict with an existing resource. The $LASTEST version of the existing resource is overwritten with the data from the import file.</p> </li>  </ul>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::StartImport::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::StartImport::set_tags): <p>A list of tags to add to the imported bot. You can only add tags when you import a bot, you can't add tags to an intent or slot type.</p>
                            /// - On success, responds with [`StartImportOutput`](crate::output::StartImportOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::StartImportOutput::name): <p>The name given to the import job.</p>
    ///   - [`resource_type(Option<ResourceType>)`](crate::output::StartImportOutput::resource_type): <p>The type of resource to import.</p>
    ///   - [`merge_strategy(Option<MergeStrategy>)`](crate::output::StartImportOutput::merge_strategy): <p>The action to take when there is a merge conflict.</p>
    ///   - [`import_id(Option<String>)`](crate::output::StartImportOutput::import_id): <p>The identifier for the specific import job.</p>
    ///   - [`import_status(Option<ImportStatus>)`](crate::output::StartImportOutput::import_status): <p>The status of the import job. If the status is <code>FAILED</code>, you can get the reason for the failure using the <code>GetImport</code> operation.</p>
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::StartImportOutput::tags): <p>A list of tags added to the imported bot.</p>
    ///   - [`created_date(Option<DateTime>)`](crate::output::StartImportOutput::created_date): <p>A timestamp for the date and time that the import job was requested.</p>
                            /// - On failure, responds with [`SdkError<StartImportError>`](crate::error::StartImportError)
    pub fn start_import(&self) -> crate::client::fluent_builders::StartImport {
                                crate::client::fluent_builders::StartImport::new(self.handle.clone())
                            }
}

