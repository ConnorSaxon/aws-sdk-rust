// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteGeofenceCollection`](crate::client::fluent_builders::DeleteGeofenceCollection) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`collection_name(impl Into<String>)`](crate::client::fluent_builders::DeleteGeofenceCollection::collection_name) / [`set_collection_name(Option<String>)`](crate::client::fluent_builders::DeleteGeofenceCollection::set_collection_name): <p>The name of the geofence collection to be deleted.</p>
                            /// - On success, responds with [`DeleteGeofenceCollectionOutput`](crate::output::DeleteGeofenceCollectionOutput)
                            /// - On failure, responds with [`SdkError<DeleteGeofenceCollectionError>`](crate::error::DeleteGeofenceCollectionError)
    pub fn delete_geofence_collection(&self) -> crate::client::fluent_builders::DeleteGeofenceCollection {
                                crate::client::fluent_builders::DeleteGeofenceCollection::new(self.handle.clone())
                            }
}

