// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PublishLayerVersion`](crate::client::fluent_builders::PublishLayerVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`layer_name(impl Into<String>)`](crate::client::fluent_builders::PublishLayerVersion::layer_name) / [`set_layer_name(Option<String>)`](crate::client::fluent_builders::PublishLayerVersion::set_layer_name): <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::PublishLayerVersion::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::PublishLayerVersion::set_description): <p>The description of the version.</p>
    ///   - [`content(LayerVersionContentInput)`](crate::client::fluent_builders::PublishLayerVersion::content) / [`set_content(Option<LayerVersionContentInput>)`](crate::client::fluent_builders::PublishLayerVersion::set_content): <p>The function layer archive.</p>
    ///   - [`compatible_runtimes(Vec<Runtime>)`](crate::client::fluent_builders::PublishLayerVersion::compatible_runtimes) / [`set_compatible_runtimes(Option<Vec<Runtime>>)`](crate::client::fluent_builders::PublishLayerVersion::set_compatible_runtimes): <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <code>ListLayers</code> and <code>ListLayerVersions</code>.</p>
    ///   - [`license_info(impl Into<String>)`](crate::client::fluent_builders::PublishLayerVersion::license_info) / [`set_license_info(Option<String>)`](crate::client::fluent_builders::PublishLayerVersion::set_license_info): <p>The layer's software license. It can be any of the following:</p>  <ul>   <li> <p>An <a href="https://spdx.org/licenses/">SPDX license identifier</a>. For example, <code>MIT</code>.</p> </li>   <li> <p>The URL of a license hosted on the internet. For example, <code>https://opensource.org/licenses/MIT</code>.</p> </li>   <li> <p>The full text of the license.</p> </li>  </ul>
    ///   - [`compatible_architectures(Vec<Architecture>)`](crate::client::fluent_builders::PublishLayerVersion::compatible_architectures) / [`set_compatible_architectures(Option<Vec<Architecture>>)`](crate::client::fluent_builders::PublishLayerVersion::set_compatible_architectures): <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
                            /// - On success, responds with [`PublishLayerVersionOutput`](crate::output::PublishLayerVersionOutput) with field(s):
    ///   - [`content(Option<LayerVersionContentOutput>)`](crate::output::PublishLayerVersionOutput::content): <p>Details about the layer version.</p>
    ///   - [`layer_arn(Option<String>)`](crate::output::PublishLayerVersionOutput::layer_arn): <p>The ARN of the layer.</p>
    ///   - [`layer_version_arn(Option<String>)`](crate::output::PublishLayerVersionOutput::layer_version_arn): <p>The ARN of the layer version.</p>
    ///   - [`description(Option<String>)`](crate::output::PublishLayerVersionOutput::description): <p>The description of the version.</p>
    ///   - [`created_date(Option<String>)`](crate::output::PublishLayerVersionOutput::created_date): <p>The date that the layer version was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    ///   - [`version(i64)`](crate::output::PublishLayerVersionOutput::version): <p>The version number.</p>
    ///   - [`compatible_runtimes(Option<Vec<Runtime>>)`](crate::output::PublishLayerVersionOutput::compatible_runtimes): <p>The layer's compatible runtimes.</p>
    ///   - [`license_info(Option<String>)`](crate::output::PublishLayerVersionOutput::license_info): <p>The layer's software license.</p>
    ///   - [`compatible_architectures(Option<Vec<Architecture>>)`](crate::output::PublishLayerVersionOutput::compatible_architectures): <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
                            /// - On failure, responds with [`SdkError<PublishLayerVersionError>`](crate::error::PublishLayerVersionError)
    pub fn publish_layer_version(&self) -> crate::client::fluent_builders::PublishLayerVersion {
                                crate::client::fluent_builders::PublishLayerVersion::new(self.handle.clone())
                            }
}

