use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::User;

/// ? https://discord.com/developers/docs/topics/teams#data-models-team-object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    /// a hash of the image of the team's icon
    pub icon: Option<String>,
    /// the unique id of the team
    pub id: String,
    /// the members of the team
    pub members: Vec<TeamMember>,
    /// the name of the team
    pub name: String,
    /// the user id of the current team owner
    pub owner_user_id: String,
}

/// ? https://discord.com/developers/docs/topics/teams#data-models-team-member-object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamMember {
    /// the user's membership state on the team
    pub membership_state: MembershipState,
    pub permissions: Vec<String>,
    /// the id of the parent team of which they are a member
    pub team_id: String,
    /// the avatar, discriminator, id, and username of the user
    pub user: User,
}

/// ? https://discord.com/developers/docs/topics/teams#data-models-membership-state-enum
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum MembershipState {
    INVITED = 1,
    ACCEPTED = 2,
}
