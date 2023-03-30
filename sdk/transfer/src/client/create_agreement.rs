// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAgreement`](crate::client::fluent_builders::CreateAgreement) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateAgreement::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateAgreement::set_description): <p>A name or short description to identify the agreement. </p>
    ///   - [`server_id(impl Into<String>)`](crate::client::fluent_builders::CreateAgreement::server_id) / [`set_server_id(Option<String>)`](crate::client::fluent_builders::CreateAgreement::set_server_id): <p>A system-assigned unique identifier for a server instance. This is the specific server that the agreement uses.</p>
    ///   - [`local_profile_id(impl Into<String>)`](crate::client::fluent_builders::CreateAgreement::local_profile_id) / [`set_local_profile_id(Option<String>)`](crate::client::fluent_builders::CreateAgreement::set_local_profile_id): <p>A unique identifier for the AS2 local profile.</p>
    ///   - [`partner_profile_id(impl Into<String>)`](crate::client::fluent_builders::CreateAgreement::partner_profile_id) / [`set_partner_profile_id(Option<String>)`](crate::client::fluent_builders::CreateAgreement::set_partner_profile_id): <p>A unique identifier for the partner profile used in the agreement.</p>
    ///   - [`base_directory(impl Into<String>)`](crate::client::fluent_builders::CreateAgreement::base_directory) / [`set_base_directory(Option<String>)`](crate::client::fluent_builders::CreateAgreement::set_base_directory): <p>The landing directory (folder) for files transferred by using the AS2 protocol.</p>  <p>A <code>BaseDirectory</code> example is <i>DOC-EXAMPLE-BUCKET</i>/<i>home</i>/<i>mydirectory</i>.</p>
    ///   - [`access_role(impl Into<String>)`](crate::client::fluent_builders::CreateAgreement::access_role) / [`set_access_role(Option<String>)`](crate::client::fluent_builders::CreateAgreement::set_access_role): <p>With AS2, you can send files by calling <code>StartFileTransfer</code> and specifying the file paths in the request parameter, <code>SendFilePaths</code>. We use the file’s parent directory (for example, for <code>--send-file-paths /bucket/dir/file.txt</code>, parent directory is <code>/bucket/dir/</code>) to temporarily store a processed AS2 message file, store the MDN when we receive them from the partner, and write a final JSON file containing relevant metadata of the transmission. So, the <code>AccessRole</code> needs to provide read and write access to the parent directory of the file location used in the <code>StartFileTransfer</code> request. Additionally, you need to provide read and write access to the parent directory of the files that you intend to send with <code>StartFileTransfer</code>.</p>
    ///   - [`status(AgreementStatusType)`](crate::client::fluent_builders::CreateAgreement::status) / [`set_status(Option<AgreementStatusType>)`](crate::client::fluent_builders::CreateAgreement::set_status): <p>The status of the agreement. The agreement can be either <code>ACTIVE</code> or <code>INACTIVE</code>.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateAgreement::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateAgreement::set_tags): <p>Key-value pairs that can be used to group and search for agreements.</p>
                            /// - On success, responds with [`CreateAgreementOutput`](crate::output::CreateAgreementOutput) with field(s):
    ///   - [`agreement_id(Option<String>)`](crate::output::CreateAgreementOutput::agreement_id): <p>The unique identifier for the agreement. Use this ID for deleting, or updating an agreement, as well as in any other API calls that require that you specify the agreement ID.</p>
                            /// - On failure, responds with [`SdkError<CreateAgreementError>`](crate::error::CreateAgreementError)
    pub fn create_agreement(&self) -> crate::client::fluent_builders::CreateAgreement {
                                crate::client::fluent_builders::CreateAgreement::new(self.handle.clone())
                            }
}

