// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ConnectDirectory`](crate::client::fluent_builders::ConnectDirectory) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::ConnectDirectory::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::ConnectDirectory::set_name): <p>The fully qualified name of your self-managed directory, such as <code>corp.example.com</code>.</p>
    ///   - [`short_name(impl Into<String>)`](crate::client::fluent_builders::ConnectDirectory::short_name) / [`set_short_name(Option<String>)`](crate::client::fluent_builders::ConnectDirectory::set_short_name): <p>The NetBIOS name of your self-managed directory, such as <code>CORP</code>.</p>
    ///   - [`password(impl Into<String>)`](crate::client::fluent_builders::ConnectDirectory::password) / [`set_password(Option<String>)`](crate::client::fluent_builders::ConnectDirectory::set_password): <p>The password for your self-managed user account.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::ConnectDirectory::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::ConnectDirectory::set_description): <p>A description for the directory.</p>
    ///   - [`size(DirectorySize)`](crate::client::fluent_builders::ConnectDirectory::size) / [`set_size(Option<DirectorySize>)`](crate::client::fluent_builders::ConnectDirectory::set_size): <p>The size of the directory.</p>
    ///   - [`connect_settings(DirectoryConnectSettings)`](crate::client::fluent_builders::ConnectDirectory::connect_settings) / [`set_connect_settings(Option<DirectoryConnectSettings>)`](crate::client::fluent_builders::ConnectDirectory::set_connect_settings): <p>A <code>DirectoryConnectSettings</code> object that contains additional information for the operation.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::ConnectDirectory::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::ConnectDirectory::set_tags): <p>The tags to be assigned to AD Connector.</p>
                            /// - On success, responds with [`ConnectDirectoryOutput`](crate::output::ConnectDirectoryOutput) with field(s):
    ///   - [`directory_id(Option<String>)`](crate::output::ConnectDirectoryOutput::directory_id): <p>The identifier of the new directory.</p>
                            /// - On failure, responds with [`SdkError<ConnectDirectoryError>`](crate::error::ConnectDirectoryError)
    pub fn connect_directory(&self) -> crate::client::fluent_builders::ConnectDirectory {
                                crate::client::fluent_builders::ConnectDirectory::new(self.handle.clone())
                            }
}

