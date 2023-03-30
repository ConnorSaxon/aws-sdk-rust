// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetGroupsForCapacityReservation`](crate::client::fluent_builders::GetGroupsForCapacityReservation) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetGroupsForCapacityReservation::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`capacity_reservation_id(impl Into<String>)`](crate::client::fluent_builders::GetGroupsForCapacityReservation::capacity_reservation_id) / [`set_capacity_reservation_id(Option<String>)`](crate::client::fluent_builders::GetGroupsForCapacityReservation::set_capacity_reservation_id): <p>The ID of the Capacity Reservation.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetGroupsForCapacityReservation::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetGroupsForCapacityReservation::set_next_token): <p>The token to use to retrieve the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetGroupsForCapacityReservation::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetGroupsForCapacityReservation::set_max_results): <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::GetGroupsForCapacityReservation::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::GetGroupsForCapacityReservation::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`GetGroupsForCapacityReservationOutput`](crate::output::GetGroupsForCapacityReservationOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::GetGroupsForCapacityReservationOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    ///   - [`capacity_reservation_groups(Option<Vec<CapacityReservationGroup>>)`](crate::output::GetGroupsForCapacityReservationOutput::capacity_reservation_groups): <p>Information about the resource groups to which the Capacity Reservation has been added.</p>
                            /// - On failure, responds with [`SdkError<GetGroupsForCapacityReservationError>`](crate::error::GetGroupsForCapacityReservationError)
    pub fn get_groups_for_capacity_reservation(&self) -> crate::client::fluent_builders::GetGroupsForCapacityReservation {
                                crate::client::fluent_builders::GetGroupsForCapacityReservation::new(self.handle.clone())
                            }
}

