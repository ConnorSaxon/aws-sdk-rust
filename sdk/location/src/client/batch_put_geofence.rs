// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchPutGeofence`](crate::client::fluent_builders::BatchPutGeofence) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`collection_name(impl Into<String>)`](crate::client::fluent_builders::BatchPutGeofence::collection_name) / [`set_collection_name(Option<String>)`](crate::client::fluent_builders::BatchPutGeofence::set_collection_name): <p>The geofence collection storing the geofences.</p>
    ///   - [`entries(Vec<BatchPutGeofenceRequestEntry>)`](crate::client::fluent_builders::BatchPutGeofence::entries) / [`set_entries(Option<Vec<BatchPutGeofenceRequestEntry>>)`](crate::client::fluent_builders::BatchPutGeofence::set_entries): <p>The batch of geofences to be stored in a geofence collection.</p>
                            /// - On success, responds with [`BatchPutGeofenceOutput`](crate::output::BatchPutGeofenceOutput) with field(s):
    ///   - [`successes(Option<Vec<BatchPutGeofenceSuccess>>)`](crate::output::BatchPutGeofenceOutput::successes): <p>Contains each geofence that was successfully stored in a geofence collection.</p>
    ///   - [`errors(Option<Vec<BatchPutGeofenceError>>)`](crate::output::BatchPutGeofenceOutput::errors): <p>Contains additional error details for each geofence that failed to be stored in a geofence collection.</p>
                            /// - On failure, responds with [`SdkError<BatchPutGeofenceError>`](crate::error::BatchPutGeofenceError)
    pub fn batch_put_geofence(&self) -> crate::client::fluent_builders::BatchPutGeofence {
                                crate::client::fluent_builders::BatchPutGeofence::new(self.handle.clone())
                            }
}

