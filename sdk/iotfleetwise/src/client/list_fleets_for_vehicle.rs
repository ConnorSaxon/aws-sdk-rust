// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListFleetsForVehicle`](crate::client::fluent_builders::ListFleetsForVehicle) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListFleetsForVehicle::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`vehicle_name(impl Into<String>)`](crate::client::fluent_builders::ListFleetsForVehicle::vehicle_name) / [`set_vehicle_name(Option<String>)`](crate::client::fluent_builders::ListFleetsForVehicle::set_vehicle_name): <p> The ID of the vehicle to retrieve information about. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListFleetsForVehicle::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListFleetsForVehicle::set_next_token): <p>A pagination token for the next set of results.</p>  <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next set of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListFleetsForVehicle::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListFleetsForVehicle::set_max_results): <p> The maximum number of items to return, between 1 and 100, inclusive. </p>
                            /// - On success, responds with [`ListFleetsForVehicleOutput`](crate::output::ListFleetsForVehicleOutput) with field(s):
    ///   - [`fleets(Option<Vec<String>>)`](crate::output::ListFleetsForVehicleOutput::fleets): <p> A list of fleet IDs that the vehicle is associated with. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListFleetsForVehicleOutput::next_token): <p> The token to retrieve the next set of results, or <code>null</code> if there are no more results. </p>
                            /// - On failure, responds with [`SdkError<ListFleetsForVehicleError>`](crate::error::ListFleetsForVehicleError)
    pub fn list_fleets_for_vehicle(&self) -> crate::client::fluent_builders::ListFleetsForVehicle {
                                crate::client::fluent_builders::ListFleetsForVehicle::new(self.handle.clone())
                            }
}

