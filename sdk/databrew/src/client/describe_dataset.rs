// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDataset`](crate::client::fluent_builders::DescribeDataset) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DescribeDataset::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DescribeDataset::set_name): <p>The name of the dataset to be described.</p>
                            /// - On success, responds with [`DescribeDatasetOutput`](crate::output::DescribeDatasetOutput) with field(s):
    ///   - [`created_by(Option<String>)`](crate::output::DescribeDatasetOutput::created_by): <p>The identifier (user name) of the user who created the dataset.</p>
    ///   - [`create_date(Option<DateTime>)`](crate::output::DescribeDatasetOutput::create_date): <p>The date and time that the dataset was created.</p>
    ///   - [`name(Option<String>)`](crate::output::DescribeDatasetOutput::name): <p>The name of the dataset.</p>
    ///   - [`format(Option<InputFormat>)`](crate::output::DescribeDatasetOutput::format): <p>The file format of a dataset that is created from an Amazon S3 file or folder.</p>
    ///   - [`format_options(Option<FormatOptions>)`](crate::output::DescribeDatasetOutput::format_options): <p>Represents a set of options that define the structure of either comma-separated value (CSV), Excel, or JSON input.</p>
    ///   - [`input(Option<Input>)`](crate::output::DescribeDatasetOutput::input): <p>Represents information on how DataBrew can find data, in either the Glue Data Catalog or Amazon S3.</p>
    ///   - [`last_modified_date(Option<DateTime>)`](crate::output::DescribeDatasetOutput::last_modified_date): <p>The date and time that the dataset was last modified.</p>
    ///   - [`last_modified_by(Option<String>)`](crate::output::DescribeDatasetOutput::last_modified_by): <p>The identifier (user name) of the user who last modified the dataset.</p>
    ///   - [`source(Option<Source>)`](crate::output::DescribeDatasetOutput::source): <p>The location of the data for this dataset, Amazon S3 or the Glue Data Catalog.</p>
    ///   - [`path_options(Option<PathOptions>)`](crate::output::DescribeDatasetOutput::path_options): <p>A set of options that defines how DataBrew interprets an Amazon S3 path of the dataset.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::DescribeDatasetOutput::tags): <p>Metadata tags associated with this dataset.</p>
    ///   - [`resource_arn(Option<String>)`](crate::output::DescribeDatasetOutput::resource_arn): <p>The Amazon Resource Name (ARN) of the dataset.</p>
                            /// - On failure, responds with [`SdkError<DescribeDatasetError>`](crate::error::DescribeDatasetError)
    pub fn describe_dataset(&self) -> crate::client::fluent_builders::DescribeDataset {
                                crate::client::fluent_builders::DescribeDataset::new(self.handle.clone())
                            }
}

