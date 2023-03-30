// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ExportLens`](crate::client::fluent_builders::ExportLens) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`lens_alias(impl Into<String>)`](crate::client::fluent_builders::ExportLens::lens_alias) / [`set_lens_alias(Option<String>)`](crate::client::fluent_builders::ExportLens::set_lens_alias): <p>The alias of the lens.</p>  <p>For Amazon Web Services official lenses, this is either the lens alias, such as <code>serverless</code>, or the lens ARN, such as <code>arn:aws:wellarchitected:us-west-2::lens/serverless</code>.</p>  <p>For custom lenses, this is the lens ARN, such as <code>arn:aws:wellarchitected:us-east-1:123456789012:lens/my-lens</code>. </p>  <p>Each lens is identified by its <code>LensSummary$LensAlias</code>.</p>
    ///   - [`lens_version(impl Into<String>)`](crate::client::fluent_builders::ExportLens::lens_version) / [`set_lens_version(Option<String>)`](crate::client::fluent_builders::ExportLens::set_lens_version): <p>The lens version to be exported.</p>
                            /// - On success, responds with [`ExportLensOutput`](crate::output::ExportLensOutput) with field(s):
    ///   - [`lens_json(Option<String>)`](crate::output::ExportLensOutput::lens_json): <p>The JSON for the lens.</p>
                            /// - On failure, responds with [`SdkError<ExportLensError>`](crate::error::ExportLensError)
    pub fn export_lens(&self) -> crate::client::fluent_builders::ExportLens {
                                crate::client::fluent_builders::ExportLens::new(self.handle.clone())
                            }
}

