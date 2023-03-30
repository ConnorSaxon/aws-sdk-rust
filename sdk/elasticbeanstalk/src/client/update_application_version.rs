// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateApplicationVersion`](crate::client::fluent_builders::UpdateApplicationVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::client::fluent_builders::UpdateApplicationVersion::application_name) / [`set_application_name(Option<String>)`](crate::client::fluent_builders::UpdateApplicationVersion::set_application_name): <p>The name of the application associated with this version.</p>  <p> If no application is found with this name, <code>UpdateApplication</code> returns an <code>InvalidParameterValue</code> error.</p>
    ///   - [`version_label(impl Into<String>)`](crate::client::fluent_builders::UpdateApplicationVersion::version_label) / [`set_version_label(Option<String>)`](crate::client::fluent_builders::UpdateApplicationVersion::set_version_label): <p>The name of the version to update.</p>  <p>If no application version is found with this label, <code>UpdateApplication</code> returns an <code>InvalidParameterValue</code> error. </p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateApplicationVersion::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateApplicationVersion::set_description): <p>A new description for this version.</p>
                            /// - On success, responds with [`UpdateApplicationVersionOutput`](crate::output::UpdateApplicationVersionOutput) with field(s):
    ///   - [`application_version(Option<ApplicationVersionDescription>)`](crate::output::UpdateApplicationVersionOutput::application_version): <p> The <code>ApplicationVersionDescription</code> of the application version. </p>
                            /// - On failure, responds with [`SdkError<UpdateApplicationVersionError>`](crate::error::UpdateApplicationVersionError)
    pub fn update_application_version(&self) -> crate::client::fluent_builders::UpdateApplicationVersion {
                                crate::client::fluent_builders::UpdateApplicationVersion::new(self.handle.clone())
                            }
}

