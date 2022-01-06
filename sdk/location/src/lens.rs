// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_get_device_position_history_output_next_token(
    input: &crate::output::GetDevicePositionHistoryOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_device_positions_output_next_token(
    input: &crate::output::ListDevicePositionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_geofence_collections_output_next_token(
    input: &crate::output::ListGeofenceCollectionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_geofences_output_next_token(
    input: &crate::output::ListGeofencesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_maps_output_next_token(
    input: &crate::output::ListMapsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_place_indexes_output_next_token(
    input: &crate::output::ListPlaceIndexesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_route_calculators_output_next_token(
    input: &crate::output::ListRouteCalculatorsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_tracker_consumers_output_next_token(
    input: &crate::output::ListTrackerConsumersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_trackers_output_next_token(
    input: &crate::output::ListTrackersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_get_device_position_history_output_device_positions(
    input: crate::output::GetDevicePositionHistoryOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DevicePosition>> {
    let input = match input.device_positions {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_device_positions_output_entries(
    input: crate::output::ListDevicePositionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ListDevicePositionsResponseEntry>> {
    let input = match input.entries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_geofence_collections_output_entries(
    input: crate::output::ListGeofenceCollectionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ListGeofenceCollectionsResponseEntry>> {
    let input = match input.entries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_geofences_output_entries(
    input: crate::output::ListGeofencesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ListGeofenceResponseEntry>> {
    let input = match input.entries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_maps_output_entries(
    input: crate::output::ListMapsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ListMapsResponseEntry>> {
    let input = match input.entries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_place_indexes_output_entries(
    input: crate::output::ListPlaceIndexesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ListPlaceIndexesResponseEntry>> {
    let input = match input.entries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_route_calculators_output_entries(
    input: crate::output::ListRouteCalculatorsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ListRouteCalculatorsResponseEntry>> {
    let input = match input.entries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_tracker_consumers_output_consumer_arns(
    input: crate::output::ListTrackerConsumersOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.consumer_arns {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_trackers_output_entries(
    input: crate::output::ListTrackersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ListTrackersResponseEntry>> {
    let input = match input.entries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
