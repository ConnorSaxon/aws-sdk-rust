// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRecord`](crate::client::fluent_builders::DeleteRecord) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`feature_group_name(impl Into<String>)`](crate::client::fluent_builders::DeleteRecord::feature_group_name) / [`set_feature_group_name(Option<String>)`](crate::client::fluent_builders::DeleteRecord::set_feature_group_name): <p>The name of the feature group to delete the record from. </p>
    ///   - [`record_identifier_value_as_string(impl Into<String>)`](crate::client::fluent_builders::DeleteRecord::record_identifier_value_as_string) / [`set_record_identifier_value_as_string(Option<String>)`](crate::client::fluent_builders::DeleteRecord::set_record_identifier_value_as_string): <p>The value for the <code>RecordIdentifier</code> that uniquely identifies the record, in string format. </p>
    ///   - [`event_time(impl Into<String>)`](crate::client::fluent_builders::DeleteRecord::event_time) / [`set_event_time(Option<String>)`](crate::client::fluent_builders::DeleteRecord::set_event_time): <p>Timestamp indicating when the deletion event occurred. <code>EventTime</code> can be used to query data at a certain point in time.</p>
    ///   - [`target_stores(Vec<TargetStore>)`](crate::client::fluent_builders::DeleteRecord::target_stores) / [`set_target_stores(Option<Vec<TargetStore>>)`](crate::client::fluent_builders::DeleteRecord::set_target_stores): <p>A list of stores from which you're deleting the record. By default, Feature Store deletes the record from all of the stores that you're using for the <code>FeatureGroup</code>.</p>
                            /// - On success, responds with [`DeleteRecordOutput`](crate::output::DeleteRecordOutput)
                            /// - On failure, responds with [`SdkError<DeleteRecordError>`](crate::error::DeleteRecordError)
    pub fn delete_record(&self) -> crate::client::fluent_builders::DeleteRecord {
                                crate::client::fluent_builders::DeleteRecord::new(self.handle.clone())
                            }
}

