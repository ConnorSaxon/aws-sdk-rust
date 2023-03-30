// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartFileTransfer`](crate::client::fluent_builders::StartFileTransfer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`connector_id(impl Into<String>)`](crate::client::fluent_builders::StartFileTransfer::connector_id) / [`set_connector_id(Option<String>)`](crate::client::fluent_builders::StartFileTransfer::set_connector_id): <p>The unique identifier for the connector. </p>
    ///   - [`send_file_paths(Vec<String>)`](crate::client::fluent_builders::StartFileTransfer::send_file_paths) / [`set_send_file_paths(Option<Vec<String>>)`](crate::client::fluent_builders::StartFileTransfer::set_send_file_paths): <p>An array of strings. Each string represents the absolute path for one outbound file transfer. For example, <code> <i>DOC-EXAMPLE-BUCKET</i>/<i>myfile.txt</i> </code>. </p>
                            /// - On success, responds with [`StartFileTransferOutput`](crate::output::StartFileTransferOutput) with field(s):
    ///   - [`transfer_id(Option<String>)`](crate::output::StartFileTransferOutput::transfer_id): <p>Returns the unique identifier for this file transfer. </p>
                            /// - On failure, responds with [`SdkError<StartFileTransferError>`](crate::error::StartFileTransferError)
    pub fn start_file_transfer(&self) -> crate::client::fluent_builders::StartFileTransfer {
                                crate::client::fluent_builders::StartFileTransfer::new(self.handle.clone())
                            }
}

