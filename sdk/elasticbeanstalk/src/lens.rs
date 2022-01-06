// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_environment_managed_action_history_output_next_token(
    input: &crate::output::DescribeEnvironmentManagedActionHistoryOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_events_output_next_token(
    input: &crate::output::DescribeEventsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_platform_branches_output_next_token(
    input: &crate::output::ListPlatformBranchesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_platform_versions_output_next_token(
    input: &crate::output::ListPlatformVersionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_environment_managed_action_history_output_managed_action_history_items(
    input: crate::output::DescribeEnvironmentManagedActionHistoryOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ManagedActionHistoryItem>> {
    let input = match input.managed_action_history_items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_events_output_events(
    input: crate::output::DescribeEventsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::EventDescription>> {
    let input = match input.events {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_platform_versions_output_platform_summary_list(
    input: crate::output::ListPlatformVersionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::PlatformSummary>> {
    let input = match input.platform_summary_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
