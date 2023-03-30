// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateDataset`](crate::client::fluent_builders::CreateDataset) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateDataset::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateDataset::set_name): <p>The name of the dataset to be created. Valid characters are alphanumeric (A-Z, a-z, 0-9), hyphen (-), period (.), and space.</p>
    ///   - [`format(InputFormat)`](crate::client::fluent_builders::CreateDataset::format) / [`set_format(Option<InputFormat>)`](crate::client::fluent_builders::CreateDataset::set_format): <p>The file format of a dataset that is created from an Amazon S3 file or folder.</p>
    ///   - [`format_options(FormatOptions)`](crate::client::fluent_builders::CreateDataset::format_options) / [`set_format_options(Option<FormatOptions>)`](crate::client::fluent_builders::CreateDataset::set_format_options): <p>Represents a set of options that define the structure of either comma-separated value (CSV), Excel, or JSON input.</p>
    ///   - [`input(Input)`](crate::client::fluent_builders::CreateDataset::input) / [`set_input(Option<Input>)`](crate::client::fluent_builders::CreateDataset::set_input): <p>Represents information on how DataBrew can find data, in either the Glue Data Catalog or Amazon S3.</p>
    ///   - [`path_options(PathOptions)`](crate::client::fluent_builders::CreateDataset::path_options) / [`set_path_options(Option<PathOptions>)`](crate::client::fluent_builders::CreateDataset::set_path_options): <p>A set of options that defines how DataBrew interprets an Amazon S3 path of the dataset.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateDataset::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateDataset::set_tags): <p>Metadata tags to apply to this dataset.</p>
                            /// - On success, responds with [`CreateDatasetOutput`](crate::output::CreateDatasetOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::CreateDatasetOutput::name): <p>The name of the dataset that you created.</p>
                            /// - On failure, responds with [`SdkError<CreateDatasetError>`](crate::error::CreateDatasetError)
    pub fn create_dataset(&self) -> crate::client::fluent_builders::CreateDataset {
                                crate::client::fluent_builders::CreateDataset::new(self.handle.clone())
                            }
}

