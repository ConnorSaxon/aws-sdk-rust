// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetReadSet`](crate::client::fluent_builders::GetReadSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetReadSet::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetReadSet::set_id): <p>The read set's ID.</p>
    ///   - [`sequence_store_id(impl Into<String>)`](crate::client::fluent_builders::GetReadSet::sequence_store_id) / [`set_sequence_store_id(Option<String>)`](crate::client::fluent_builders::GetReadSet::set_sequence_store_id): <p>The read set's sequence store ID.</p>
    ///   - [`file(ReadSetFile)`](crate::client::fluent_builders::GetReadSet::file) / [`set_file(Option<ReadSetFile>)`](crate::client::fluent_builders::GetReadSet::set_file): <p>The file to retrieve.</p>
    ///   - [`part_number(i32)`](crate::client::fluent_builders::GetReadSet::part_number) / [`set_part_number(Option<i32>)`](crate::client::fluent_builders::GetReadSet::set_part_number): <p>The part number to retrieve.</p>
                            /// - On success, responds with [`GetReadSetOutput`](crate::output::GetReadSetOutput) with field(s):
    ///   - [`payload(ByteStream)`](crate::output::GetReadSetOutput::payload): <p>The read set file payload.</p>
                            /// - On failure, responds with [`SdkError<GetReadSetError>`](crate::error::GetReadSetError)
    pub fn get_read_set(&self) -> crate::client::fluent_builders::GetReadSet {
                                crate::client::fluent_builders::GetReadSet::new(self.handle.clone())
                            }
}

