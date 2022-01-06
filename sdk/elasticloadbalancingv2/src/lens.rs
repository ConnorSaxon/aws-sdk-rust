// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_listeners_output_next_marker(
    input: &crate::output::DescribeListenersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_load_balancers_output_next_marker(
    input: &crate::output::DescribeLoadBalancersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_target_groups_output_next_marker(
    input: &crate::output::DescribeTargetGroupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_listeners_output_listeners(
    input: crate::output::DescribeListenersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Listener>> {
    let input = match input.listeners {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_load_balancers_output_load_balancers(
    input: crate::output::DescribeLoadBalancersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::LoadBalancer>> {
    let input = match input.load_balancers {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_target_groups_output_target_groups(
    input: crate::output::DescribeTargetGroupsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::TargetGroup>> {
    let input = match input.target_groups {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
