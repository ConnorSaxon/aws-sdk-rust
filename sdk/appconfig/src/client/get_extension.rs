// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetExtension`](crate::client::fluent_builders::GetExtension) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`extension_identifier(impl Into<String>)`](crate::client::fluent_builders::GetExtension::extension_identifier) / [`set_extension_identifier(Option<String>)`](crate::client::fluent_builders::GetExtension::set_extension_identifier): <p>The name, the ID, or the Amazon Resource Name (ARN) of the extension.</p>
    ///   - [`version_number(i32)`](crate::client::fluent_builders::GetExtension::version_number) / [`set_version_number(Option<i32>)`](crate::client::fluent_builders::GetExtension::set_version_number): <p>The extension version number. If no version number was defined, AppConfig uses the highest version.</p>
                            /// - On success, responds with [`GetExtensionOutput`](crate::output::GetExtensionOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::GetExtensionOutput::id): <p>The system-generated ID of the extension.</p>
    ///   - [`name(Option<String>)`](crate::output::GetExtensionOutput::name): <p>The extension name.</p>
    ///   - [`version_number(i32)`](crate::output::GetExtensionOutput::version_number): <p>The extension version number.</p>
    ///   - [`arn(Option<String>)`](crate::output::GetExtensionOutput::arn): <p>The system-generated Amazon Resource Name (ARN) for the extension.</p>
    ///   - [`description(Option<String>)`](crate::output::GetExtensionOutput::description): <p>Information about the extension.</p>
    ///   - [`actions(Option<HashMap<ActionPoint, Vec<Action>>>)`](crate::output::GetExtensionOutput::actions): <p>The actions defined in the extension.</p>
    ///   - [`parameters(Option<HashMap<String, Parameter>>)`](crate::output::GetExtensionOutput::parameters): <p>The parameters accepted by the extension. You specify parameter values when you associate the extension to an AppConfig resource by using the <code>CreateExtensionAssociation</code> API action. For Lambda extension actions, these parameters are included in the Lambda request object.</p>
                            /// - On failure, responds with [`SdkError<GetExtensionError>`](crate::error::GetExtensionError)
    pub fn get_extension(&self) -> crate::client::fluent_builders::GetExtension {
                                crate::client::fluent_builders::GetExtension::new(self.handle.clone())
                            }
}

