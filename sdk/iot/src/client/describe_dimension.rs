// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDimension`](crate::client::fluent_builders::DescribeDimension) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DescribeDimension::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DescribeDimension::set_name): <p>The unique identifier for the dimension.</p>
                            /// - On success, responds with [`DescribeDimensionOutput`](crate::output::DescribeDimensionOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::DescribeDimensionOutput::name): <p>The unique identifier for the dimension.</p>
    ///   - [`arn(Option<String>)`](crate::output::DescribeDimensionOutput::arn): <p>The Amazon Resource Name (ARN) for the dimension.</p>
    ///   - [`r#type(Option<DimensionType>)`](crate::output::DescribeDimensionOutput::type): <p>The type of the dimension.</p>
    ///   - [`string_values(Option<Vec<String>>)`](crate::output::DescribeDimensionOutput::string_values): <p>The value or list of values used to scope the dimension. For example, for topic filters, this is the pattern used to match the MQTT topic name.</p>
    ///   - [`creation_date(Option<DateTime>)`](crate::output::DescribeDimensionOutput::creation_date): <p>The date the dimension was created.</p>
    ///   - [`last_modified_date(Option<DateTime>)`](crate::output::DescribeDimensionOutput::last_modified_date): <p>The date the dimension was last modified.</p>
                            /// - On failure, responds with [`SdkError<DescribeDimensionError>`](crate::error::DescribeDimensionError)
    pub fn describe_dimension(&self) -> crate::client::fluent_builders::DescribeDimension {
                                crate::client::fluent_builders::DescribeDimension::new(self.handle.clone())
                            }
}

