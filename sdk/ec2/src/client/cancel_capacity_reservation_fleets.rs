// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelCapacityReservationFleets`](crate::client::fluent_builders::CancelCapacityReservationFleets) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CancelCapacityReservationFleets::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CancelCapacityReservationFleets::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`capacity_reservation_fleet_ids(Vec<String>)`](crate::client::fluent_builders::CancelCapacityReservationFleets::capacity_reservation_fleet_ids) / [`set_capacity_reservation_fleet_ids(Option<Vec<String>>)`](crate::client::fluent_builders::CancelCapacityReservationFleets::set_capacity_reservation_fleet_ids): <p>The IDs of the Capacity Reservation Fleets to cancel.</p>
                            /// - On success, responds with [`CancelCapacityReservationFleetsOutput`](crate::output::CancelCapacityReservationFleetsOutput) with field(s):
    ///   - [`successful_fleet_cancellations(Option<Vec<CapacityReservationFleetCancellationState>>)`](crate::output::CancelCapacityReservationFleetsOutput::successful_fleet_cancellations): <p>Information about the Capacity Reservation Fleets that were successfully cancelled.</p>
    ///   - [`failed_fleet_cancellations(Option<Vec<FailedCapacityReservationFleetCancellationResult>>)`](crate::output::CancelCapacityReservationFleetsOutput::failed_fleet_cancellations): <p>Information about the Capacity Reservation Fleets that could not be cancelled.</p>
                            /// - On failure, responds with [`SdkError<CancelCapacityReservationFleetsError>`](crate::error::CancelCapacityReservationFleetsError)
    pub fn cancel_capacity_reservation_fleets(&self) -> crate::client::fluent_builders::CancelCapacityReservationFleets {
                                crate::client::fluent_builders::CancelCapacityReservationFleets::new(self.handle.clone())
                            }
}

