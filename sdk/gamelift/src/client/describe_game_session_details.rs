// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeGameSessionDetails`](crate::client::fluent_builders::DescribeGameSessionDetails) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeGameSessionDetails::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_id(impl Into<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::fleet_id) / [`set_fleet_id(Option<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::set_fleet_id): <p>A unique identifier for the fleet to retrieve all game sessions active on the fleet. You can use either the fleet ID or ARN value.</p>
    ///   - [`game_session_id(impl Into<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::game_session_id) / [`set_game_session_id(Option<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::set_game_session_id): <p>A unique identifier for the game session to retrieve. </p>
    ///   - [`alias_id(impl Into<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::alias_id) / [`set_alias_id(Option<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::set_alias_id): <p>A unique identifier for the alias associated with the fleet to retrieve all game sessions for. You can use either the alias ID or ARN value.</p>
    ///   - [`location(impl Into<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::location) / [`set_location(Option<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::set_location): <p>A fleet location to get game session details for. You can specify a fleet's home Region or a remote location. Use the Amazon Web Services Region code format, such as <code>us-west-2</code>. </p>
    ///   - [`status_filter(impl Into<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::status_filter) / [`set_status_filter(Option<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::set_status_filter): <p>Game session status to filter results on. Possible game session statuses include <code>ACTIVE</code>, <code>TERMINATED</code>, <code>ACTIVATING</code> and <code>TERMINATING</code> (the last two are transitory). </p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::DescribeGameSessionDetails::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::DescribeGameSessionDetails::set_limit): <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeGameSessionDetails::set_next_token): <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
                            /// - On success, responds with [`DescribeGameSessionDetailsOutput`](crate::output::DescribeGameSessionDetailsOutput) with field(s):
    ///   - [`game_session_details(Option<Vec<GameSessionDetail>>)`](crate::output::DescribeGameSessionDetailsOutput::game_session_details): <p>A collection of properties for each game session that matches the request.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeGameSessionDetailsOutput::next_token): <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
                            /// - On failure, responds with [`SdkError<DescribeGameSessionDetailsError>`](crate::error::DescribeGameSessionDetailsError)
    pub fn describe_game_session_details(&self) -> crate::client::fluent_builders::DescribeGameSessionDetails {
                                crate::client::fluent_builders::DescribeGameSessionDetails::new(self.handle.clone())
                            }
}

