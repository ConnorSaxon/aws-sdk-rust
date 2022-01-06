// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_organization_resource_collection_health_output_next_token(
    input: &crate::output::DescribeOrganizationResourceCollectionHealthOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_resource_collection_health_output_next_token(
    input: &crate::output::DescribeResourceCollectionHealthOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_get_cost_estimation_output_next_token(
    input: &crate::output::GetCostEstimationOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_get_resource_collection_output_next_token(
    input: &crate::output::GetResourceCollectionOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_anomalies_for_insight_output_next_token(
    input: &crate::output::ListAnomaliesForInsightOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_events_output_next_token(
    input: &crate::output::ListEventsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_insights_output_next_token(
    input: &crate::output::ListInsightsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_notification_channels_output_next_token(
    input: &crate::output::ListNotificationChannelsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_organization_insights_output_next_token(
    input: &crate::output::ListOrganizationInsightsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_recommendations_output_next_token(
    input: &crate::output::ListRecommendationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_search_insights_output_next_token(
    input: &crate::output::SearchInsightsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_search_organization_insights_output_next_token(
    input: &crate::output::SearchOrganizationInsightsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_events_output_events(
    input: crate::output::ListEventsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Event>> {
    let input = match input.events {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_notification_channels_output_channels(
    input: crate::output::ListNotificationChannelsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::NotificationChannel>> {
    let input = match input.channels {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_recommendations_output_recommendations(
    input: crate::output::ListRecommendationsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Recommendation>> {
    let input = match input.recommendations {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
