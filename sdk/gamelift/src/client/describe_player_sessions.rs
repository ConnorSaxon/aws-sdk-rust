// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribePlayerSessions`](crate::client::fluent_builders::DescribePlayerSessions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribePlayerSessions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`game_session_id(impl Into<String>)`](crate::client::fluent_builders::DescribePlayerSessions::game_session_id) / [`set_game_session_id(Option<String>)`](crate::client::fluent_builders::DescribePlayerSessions::set_game_session_id): <p>A unique identifier for the game session to retrieve player sessions for.</p>
    ///   - [`player_id(impl Into<String>)`](crate::client::fluent_builders::DescribePlayerSessions::player_id) / [`set_player_id(Option<String>)`](crate::client::fluent_builders::DescribePlayerSessions::set_player_id): <p>A unique identifier for a player to retrieve player sessions for.</p>
    ///   - [`player_session_id(impl Into<String>)`](crate::client::fluent_builders::DescribePlayerSessions::player_session_id) / [`set_player_session_id(Option<String>)`](crate::client::fluent_builders::DescribePlayerSessions::set_player_session_id): <p>A unique identifier for a player session to retrieve.</p>
    ///   - [`player_session_status_filter(impl Into<String>)`](crate::client::fluent_builders::DescribePlayerSessions::player_session_status_filter) / [`set_player_session_status_filter(Option<String>)`](crate::client::fluent_builders::DescribePlayerSessions::set_player_session_status_filter): <p>Player session status to filter results on. Note that when a PlayerSessionId or PlayerId is provided in a DescribePlayerSessions request, then the PlayerSessionStatusFilter has no effect on the response.</p>  <p>Possible player session statuses include the following:</p>  <ul>   <li> <p> <b>RESERVED</b> -- The player session request has been received, but the player has not yet connected to the server process and/or been validated. </p> </li>   <li> <p> <b>ACTIVE</b> -- The player has been validated by the server process and is currently connected.</p> </li>   <li> <p> <b>COMPLETED</b> -- The player connection has been dropped.</p> </li>   <li> <p> <b>TIMEDOUT</b> -- A player session request was received, but the player did not connect and/or was not validated within the timeout limit (60 seconds).</p> </li>  </ul>
    ///   - [`limit(i32)`](crate::client::fluent_builders::DescribePlayerSessions::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::DescribePlayerSessions::set_limit): <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. If a player session ID is specified, this parameter is ignored.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribePlayerSessions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribePlayerSessions::set_next_token): <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value. If a player session ID is specified, this parameter is ignored.</p>
                            /// - On success, responds with [`DescribePlayerSessionsOutput`](crate::output::DescribePlayerSessionsOutput) with field(s):
    ///   - [`player_sessions(Option<Vec<PlayerSession>>)`](crate::output::DescribePlayerSessionsOutput::player_sessions): <p>A collection of objects containing properties for each player session that matches the request.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribePlayerSessionsOutput::next_token): <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
                            /// - On failure, responds with [`SdkError<DescribePlayerSessionsError>`](crate::error::DescribePlayerSessionsError)
    pub fn describe_player_sessions(&self) -> crate::client::fluent_builders::DescribePlayerSessions {
                                crate::client::fluent_builders::DescribePlayerSessions::new(self.handle.clone())
                            }
}

