// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateGameSession`](crate::client::fluent_builders::CreateGameSession) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_id(impl Into<String>)`](crate::client::fluent_builders::CreateGameSession::fleet_id) / [`set_fleet_id(Option<String>)`](crate::client::fluent_builders::CreateGameSession::set_fleet_id): <p>A unique identifier for the fleet to create a game session in. You can use either the fleet ID or ARN value. Each request must reference either a fleet ID or alias ID, but not both.</p>
    ///   - [`alias_id(impl Into<String>)`](crate::client::fluent_builders::CreateGameSession::alias_id) / [`set_alias_id(Option<String>)`](crate::client::fluent_builders::CreateGameSession::set_alias_id): <p>A unique identifier for the alias associated with the fleet to create a game session in. You can use either the alias ID or ARN value. Each request must reference either a fleet ID or alias ID, but not both.</p>
    ///   - [`maximum_player_session_count(i32)`](crate::client::fluent_builders::CreateGameSession::maximum_player_session_count) / [`set_maximum_player_session_count(Option<i32>)`](crate::client::fluent_builders::CreateGameSession::set_maximum_player_session_count): <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateGameSession::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateGameSession::set_name): <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    ///   - [`game_properties(Vec<GameProperty>)`](crate::client::fluent_builders::CreateGameSession::game_properties) / [`set_game_properties(Option<Vec<GameProperty>>)`](crate::client::fluent_builders::CreateGameSession::set_game_properties): <p>A set of custom properties for a game session, formatted as key:value pairs. These properties are passed to a game server process with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    ///   - [`creator_id(impl Into<String>)`](crate::client::fluent_builders::CreateGameSession::creator_id) / [`set_creator_id(Option<String>)`](crate::client::fluent_builders::CreateGameSession::set_creator_id): <p>A unique identifier for a player or entity creating the game session. This parameter is required when requesting a new game session on a fleet with a resource creation limit policy. This type of policy limits the number of concurrent active game sessions that one player can create within a certain time span. GameLift uses the CreatorId to evaluate the new request against the policy.</p>
    ///   - [`game_session_id(impl Into<String>)`](crate::client::fluent_builders::CreateGameSession::game_session_id) / [`set_game_session_id(Option<String>)`](crate::client::fluent_builders::CreateGameSession::set_game_session_id): <p> <i>This parameter is deprecated. Use <code>IdempotencyToken</code> instead.</i> </p>  <p>Custom string that uniquely identifies a request for a new game session. Maximum token length is 48 characters. If provided, this string is included in the new game session's ID.</p>
    ///   - [`idempotency_token(impl Into<String>)`](crate::client::fluent_builders::CreateGameSession::idempotency_token) / [`set_idempotency_token(Option<String>)`](crate::client::fluent_builders::CreateGameSession::set_idempotency_token): <p>Custom string that uniquely identifies the new game session request. This is useful for ensuring that game session requests with the same idempotency token are processed only once. Subsequent requests with the same string return the original <code>GameSession</code> object, with an updated status. Maximum token length is 48 characters. If provided, this string is included in the new game session's ID. A game session ARN has the following format: <code>arn:aws:gamelift:   <region>    ::gamesession/    <fleet id>     /     <custom id string or idempotency token></custom>    </fleet>   </region></code>. Idempotency tokens remain in use for 30 days after a game session has ended; game session objects are retained for this time period and then deleted.</p>
    ///   - [`game_session_data(impl Into<String>)`](crate::client::fluent_builders::CreateGameSession::game_session_data) / [`set_game_session_data(Option<String>)`](crate::client::fluent_builders::CreateGameSession::set_game_session_data): <p>A set of custom game session properties, formatted as a single string value. This data is passed to a game server process with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    ///   - [`location(impl Into<String>)`](crate::client::fluent_builders::CreateGameSession::location) / [`set_location(Option<String>)`](crate::client::fluent_builders::CreateGameSession::set_location): <p>A fleet's remote location to place the new game session in. If this parameter is not set, the new game session is placed in the fleet's home Region. Specify a remote location with an Amazon Web Services Region code such as <code>us-west-2</code>. </p>
                            /// - On success, responds with [`CreateGameSessionOutput`](crate::output::CreateGameSessionOutput) with field(s):
    ///   - [`game_session(Option<GameSession>)`](crate::output::CreateGameSessionOutput::game_session): <p>Object that describes the newly created game session record.</p>
                            /// - On failure, responds with [`SdkError<CreateGameSessionError>`](crate::error::CreateGameSessionError)
    pub fn create_game_session(&self) -> crate::client::fluent_builders::CreateGameSession {
                                crate::client::fluent_builders::CreateGameSession::new(self.handle.clone())
                            }
}

