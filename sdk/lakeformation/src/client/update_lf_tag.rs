// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateLFTag`](crate::client::fluent_builders::UpdateLFTag) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::client::fluent_builders::UpdateLFTag::catalog_id) / [`set_catalog_id(Option<String>)`](crate::client::fluent_builders::UpdateLFTag::set_catalog_id): <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    ///   - [`tag_key(impl Into<String>)`](crate::client::fluent_builders::UpdateLFTag::tag_key) / [`set_tag_key(Option<String>)`](crate::client::fluent_builders::UpdateLFTag::set_tag_key): <p>The key-name for the LF-tag for which to add or delete values.</p>
    ///   - [`tag_values_to_delete(Vec<String>)`](crate::client::fluent_builders::UpdateLFTag::tag_values_to_delete) / [`set_tag_values_to_delete(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateLFTag::set_tag_values_to_delete): <p>A list of LF-tag values to delete from the LF-tag.</p>
    ///   - [`tag_values_to_add(Vec<String>)`](crate::client::fluent_builders::UpdateLFTag::tag_values_to_add) / [`set_tag_values_to_add(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateLFTag::set_tag_values_to_add): <p>A list of LF-tag values to add from the LF-tag.</p>
                            /// - On success, responds with [`UpdateLfTagOutput`](crate::output::UpdateLfTagOutput)
                            /// - On failure, responds with [`SdkError<UpdateLFTagError>`](crate::error::UpdateLFTagError)
    pub fn update_lf_tag(&self) -> crate::client::fluent_builders::UpdateLFTag {
                                crate::client::fluent_builders::UpdateLFTag::new(self.handle.clone())
                            }
}

