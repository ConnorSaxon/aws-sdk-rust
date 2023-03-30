// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendSSHPublicKey`](crate::client::fluent_builders::SendSSHPublicKey) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::SendSSHPublicKey::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::SendSSHPublicKey::set_instance_id): <p>The ID of the EC2 instance.</p>
    ///   - [`instance_os_user(impl Into<String>)`](crate::client::fluent_builders::SendSSHPublicKey::instance_os_user) / [`set_instance_os_user(Option<String>)`](crate::client::fluent_builders::SendSSHPublicKey::set_instance_os_user): <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
    ///   - [`ssh_public_key(impl Into<String>)`](crate::client::fluent_builders::SendSSHPublicKey::ssh_public_key) / [`set_ssh_public_key(Option<String>)`](crate::client::fluent_builders::SendSSHPublicKey::set_ssh_public_key): <p>The public key material. To use the public key, you must have the matching private key.</p>
    ///   - [`availability_zone(impl Into<String>)`](crate::client::fluent_builders::SendSSHPublicKey::availability_zone) / [`set_availability_zone(Option<String>)`](crate::client::fluent_builders::SendSSHPublicKey::set_availability_zone): <p>The Availability Zone in which the EC2 instance was launched.</p>
                            /// - On success, responds with [`SendSshPublicKeyOutput`](crate::output::SendSshPublicKeyOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::output::SendSshPublicKeyOutput::request_id): <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
    ///   - [`success(bool)`](crate::output::SendSshPublicKeyOutput::success): <p>Is true if the request succeeds and an error otherwise.</p>
                            /// - On failure, responds with [`SdkError<SendSSHPublicKeyError>`](crate::error::SendSSHPublicKeyError)
    pub fn send_ssh_public_key(&self) -> crate::client::fluent_builders::SendSSHPublicKey {
                                crate::client::fluent_builders::SendSSHPublicKey::new(self.handle.clone())
                            }
}

