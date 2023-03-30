// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeletePreset`](crate::client::fluent_builders::DeletePreset) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeletePreset::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeletePreset::set_name): The name of the preset to be deleted.
                            /// - On success, responds with [`DeletePresetOutput`](crate::output::DeletePresetOutput)
                            /// - On failure, responds with [`SdkError<DeletePresetError>`](crate::error::DeletePresetError)
    pub fn delete_preset(&self) -> crate::client::fluent_builders::DeletePreset {
                                crate::client::fluent_builders::DeletePreset::new(self.handle.clone())
                            }
}

