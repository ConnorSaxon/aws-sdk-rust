// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_eula_acceptances_output_next_token(
    input: &crate::output::ListEulaAcceptancesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_eulas_output_next_token(
    input: &crate::output::ListEulasOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_launch_profile_members_output_next_token(
    input: &crate::output::ListLaunchProfileMembersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_launch_profiles_output_next_token(
    input: &crate::output::ListLaunchProfilesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_streaming_images_output_next_token(
    input: &crate::output::ListStreamingImagesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_streaming_sessions_output_next_token(
    input: &crate::output::ListStreamingSessionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_studio_components_output_next_token(
    input: &crate::output::ListStudioComponentsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_studio_members_output_next_token(
    input: &crate::output::ListStudioMembersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_studios_output_next_token(
    input: &crate::output::ListStudiosOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_eula_acceptances_output_eula_acceptances(
    input: crate::output::ListEulaAcceptancesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::EulaAcceptance>> {
    let input = match input.eula_acceptances {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_eulas_output_eulas(
    input: crate::output::ListEulasOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Eula>> {
    let input = match input.eulas {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_launch_profile_members_output_members(
    input: crate::output::ListLaunchProfileMembersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::LaunchProfileMembership>> {
    let input = match input.members {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_launch_profiles_output_launch_profiles(
    input: crate::output::ListLaunchProfilesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::LaunchProfile>> {
    let input = match input.launch_profiles {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_streaming_images_output_streaming_images(
    input: crate::output::ListStreamingImagesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::StreamingImage>> {
    let input = match input.streaming_images {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_streaming_sessions_output_sessions(
    input: crate::output::ListStreamingSessionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::StreamingSession>> {
    let input = match input.sessions {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_studio_components_output_studio_components(
    input: crate::output::ListStudioComponentsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::StudioComponent>> {
    let input = match input.studio_components {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_studio_members_output_members(
    input: crate::output::ListStudioMembersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::StudioMembership>> {
    let input = match input.members {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_studios_output_studios(
    input: crate::output::ListStudiosOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Studio>> {
    let input = match input.studios {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
