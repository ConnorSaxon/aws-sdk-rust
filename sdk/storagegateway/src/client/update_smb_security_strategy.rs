// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateSMBSecurityStrategy`](crate::client::fluent_builders::UpdateSMBSecurityStrategy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateSMBSecurityStrategy::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::client::fluent_builders::UpdateSMBSecurityStrategy::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    ///   - [`smb_security_strategy(SmbSecurityStrategy)`](crate::client::fluent_builders::UpdateSMBSecurityStrategy::smb_security_strategy) / [`set_smb_security_strategy(Option<SmbSecurityStrategy>)`](crate::client::fluent_builders::UpdateSMBSecurityStrategy::set_smb_security_strategy): <p>Specifies the type of security strategy.</p>  <p>ClientSpecified: if you use this option, requests are established based on what is negotiated by the client. This option is recommended when you want to maximize compatibility across different clients in your environment. Supported only in S3 File Gateway.</p>  <p>MandatorySigning: if you use this option, file gateway only allows connections from SMBv2 or SMBv3 clients that have signing enabled. This option works with SMB clients on Microsoft Windows Vista, Windows Server 2008 or newer.</p>  <p>MandatoryEncryption: if you use this option, file gateway only allows connections from SMBv3 clients that have encryption enabled. This option is highly recommended for environments that handle sensitive data. This option works with SMB clients on Microsoft Windows 8, Windows Server 2012 or newer.</p>
                            /// - On success, responds with [`UpdateSmbSecurityStrategyOutput`](crate::output::UpdateSmbSecurityStrategyOutput) with field(s):
    ///   - [`gateway_arn(Option<String>)`](crate::output::UpdateSmbSecurityStrategyOutput::gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
                            /// - On failure, responds with [`SdkError<UpdateSMBSecurityStrategyError>`](crate::error::UpdateSMBSecurityStrategyError)
    pub fn update_smb_security_strategy(&self) -> crate::client::fluent_builders::UpdateSMBSecurityStrategy {
                                crate::client::fluent_builders::UpdateSMBSecurityStrategy::new(self.handle.clone())
                            }
}

