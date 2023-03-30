// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetPreset`](crate::client::fluent_builders::GetPreset) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::GetPreset::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::GetPreset::set_name): The name of the preset.
                            /// - On success, responds with [`GetPresetOutput`](crate::output::GetPresetOutput) with field(s):
    ///   - [`preset(Option<Preset>)`](crate::output::GetPresetOutput::preset): A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process.
                            /// - On failure, responds with [`SdkError<GetPresetError>`](crate::error::GetPresetError)
    pub fn get_preset(&self) -> crate::client::fluent_builders::GetPreset {
                                crate::client::fluent_builders::GetPreset::new(self.handle.clone())
                            }
}

