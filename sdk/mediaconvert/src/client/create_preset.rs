// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreatePreset`](crate::client::fluent_builders::CreatePreset) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`category(impl Into<String>)`](crate::client::fluent_builders::CreatePreset::category) / [`set_category(Option<String>)`](crate::client::fluent_builders::CreatePreset::set_category): Optional. A category for the preset you are creating.
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreatePreset::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreatePreset::set_description): Optional. A description of the preset you are creating.
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreatePreset::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreatePreset::set_name): The name of the preset you are creating.
    ///   - [`settings(PresetSettings)`](crate::client::fluent_builders::CreatePreset::settings) / [`set_settings(Option<PresetSettings>)`](crate::client::fluent_builders::CreatePreset::set_settings): Settings for preset
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreatePreset::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreatePreset::set_tags): The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.
                            /// - On success, responds with [`CreatePresetOutput`](crate::output::CreatePresetOutput) with field(s):
    ///   - [`preset(Option<Preset>)`](crate::output::CreatePresetOutput::preset): A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process.
                            /// - On failure, responds with [`SdkError<CreatePresetError>`](crate::error::CreatePresetError)
    pub fn create_preset(&self) -> crate::client::fluent_builders::CreatePreset {
                                crate::client::fluent_builders::CreatePreset::new(self.handle.clone())
                            }
}

