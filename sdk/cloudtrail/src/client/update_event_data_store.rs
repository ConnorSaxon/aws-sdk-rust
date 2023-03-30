// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateEventDataStore`](crate::client::fluent_builders::UpdateEventDataStore) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`event_data_store(impl Into<String>)`](crate::client::fluent_builders::UpdateEventDataStore::event_data_store) / [`set_event_data_store(Option<String>)`](crate::client::fluent_builders::UpdateEventDataStore::set_event_data_store): <p>The ARN (or the ID suffix of the ARN) of the event data store that you want to update.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateEventDataStore::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateEventDataStore::set_name): <p>The event data store name.</p>
    ///   - [`advanced_event_selectors(Vec<AdvancedEventSelector>)`](crate::client::fluent_builders::UpdateEventDataStore::advanced_event_selectors) / [`set_advanced_event_selectors(Option<Vec<AdvancedEventSelector>>)`](crate::client::fluent_builders::UpdateEventDataStore::set_advanced_event_selectors): <p>The advanced event selectors used to select events for the event data store. You can configure up to five advanced event selectors for each event data store.</p>
    ///   - [`multi_region_enabled(bool)`](crate::client::fluent_builders::UpdateEventDataStore::multi_region_enabled) / [`set_multi_region_enabled(Option<bool>)`](crate::client::fluent_builders::UpdateEventDataStore::set_multi_region_enabled): <p>Specifies whether an event data store collects events from all regions, or only from the region in which it was created.</p>
    ///   - [`organization_enabled(bool)`](crate::client::fluent_builders::UpdateEventDataStore::organization_enabled) / [`set_organization_enabled(Option<bool>)`](crate::client::fluent_builders::UpdateEventDataStore::set_organization_enabled): <p>Specifies whether an event data store collects events logged for an organization in Organizations.</p>
    ///   - [`retention_period(i32)`](crate::client::fluent_builders::UpdateEventDataStore::retention_period) / [`set_retention_period(Option<i32>)`](crate::client::fluent_builders::UpdateEventDataStore::set_retention_period): <p>The retention period, in days.</p>
    ///   - [`termination_protection_enabled(bool)`](crate::client::fluent_builders::UpdateEventDataStore::termination_protection_enabled) / [`set_termination_protection_enabled(Option<bool>)`](crate::client::fluent_builders::UpdateEventDataStore::set_termination_protection_enabled): <p>Indicates that termination protection is enabled and the event data store cannot be automatically deleted.</p>
    ///   - [`kms_key_id(impl Into<String>)`](crate::client::fluent_builders::UpdateEventDataStore::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::client::fluent_builders::UpdateEventDataStore::set_kms_key_id): <p>Specifies the KMS key ID to use to encrypt the events delivered by CloudTrail. The value can be an alias name prefixed by <code>alias/</code>, a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.</p> <important>   <p>Disabling or deleting the KMS key, or removing CloudTrail permissions on the key, prevents CloudTrail from logging events to the event data store, and prevents users from querying the data in the event data store that was encrypted with the key. After you associate an event data store with a KMS key, the KMS key cannot be removed or changed. Before you disable or delete a KMS key that you are using with an event data store, delete or back up your event data store.</p>  </important>  <p>CloudTrail also supports KMS multi-Region keys. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>Key Management Service Developer Guide</i>.</p>  <p>Examples:</p>  <ul>   <li> <p> <code>alias/MyAliasName</code> </p> </li>   <li> <p> <code>arn:aws:kms:us-east-2:123456789012:alias/MyAliasName</code> </p> </li>   <li> <p> <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>   <li> <p> <code>12345678-1234-1234-1234-123456789012</code> </p> </li>  </ul>
                            /// - On success, responds with [`UpdateEventDataStoreOutput`](crate::output::UpdateEventDataStoreOutput) with field(s):
    ///   - [`event_data_store_arn(Option<String>)`](crate::output::UpdateEventDataStoreOutput::event_data_store_arn): <p>The ARN of the event data store.</p>
    ///   - [`name(Option<String>)`](crate::output::UpdateEventDataStoreOutput::name): <p>The name of the event data store.</p>
    ///   - [`status(Option<EventDataStoreStatus>)`](crate::output::UpdateEventDataStoreOutput::status): <p>The status of an event data store. Values can be <code>ENABLED</code> and <code>PENDING_DELETION</code>.</p>
    ///   - [`advanced_event_selectors(Option<Vec<AdvancedEventSelector>>)`](crate::output::UpdateEventDataStoreOutput::advanced_event_selectors): <p>The advanced event selectors that are applied to the event data store.</p>
    ///   - [`multi_region_enabled(Option<bool>)`](crate::output::UpdateEventDataStoreOutput::multi_region_enabled): <p>Indicates whether the event data store includes events from all regions, or only from the region in which it was created.</p>
    ///   - [`organization_enabled(Option<bool>)`](crate::output::UpdateEventDataStoreOutput::organization_enabled): <p>Indicates whether an event data store is collecting logged events for an organization in Organizations.</p>
    ///   - [`retention_period(Option<i32>)`](crate::output::UpdateEventDataStoreOutput::retention_period): <p>The retention period, in days.</p>
    ///   - [`termination_protection_enabled(Option<bool>)`](crate::output::UpdateEventDataStoreOutput::termination_protection_enabled): <p>Indicates whether termination protection is enabled for the event data store.</p>
    ///   - [`created_timestamp(Option<DateTime>)`](crate::output::UpdateEventDataStoreOutput::created_timestamp): <p>The timestamp that shows when an event data store was first created.</p>
    ///   - [`updated_timestamp(Option<DateTime>)`](crate::output::UpdateEventDataStoreOutput::updated_timestamp): <p>The timestamp that shows when the event data store was last updated. <code>UpdatedTimestamp</code> is always either the same or newer than the time shown in <code>CreatedTimestamp</code>.</p>
    ///   - [`kms_key_id(Option<String>)`](crate::output::UpdateEventDataStoreOutput::kms_key_id): <p>Specifies the KMS key ID that encrypts the events delivered by CloudTrail. The value is a fully specified ARN to a KMS key in the following format.</p>  <p> <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p>
                            /// - On failure, responds with [`SdkError<UpdateEventDataStoreError>`](crate::error::UpdateEventDataStoreError)
    pub fn update_event_data_store(&self) -> crate::client::fluent_builders::UpdateEventDataStore {
                                crate::client::fluent_builders::UpdateEventDataStore::new(self.handle.clone())
                            }
}

