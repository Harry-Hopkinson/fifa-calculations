use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Player {
    pub Acceleration: String,
    pub Marking: String,
    pub Strength: String,
    pub Dribbling: String,
    pub Name: String,
    pub Stamina: String,
    pub LongPassing: String,
    pub StandingTackle: String,
    pub SlidingTackle: String,
    pub BallControl: String,
    pub Passing: String,
    pub Interceptions: String,
    pub Agility: String,
    pub LongShots: String,
    pub ShotPower: String,
    pub POS: String,
    pub Jumping: String,
    pub Positioning: String,
    pub Curve: String,
    pub SprintSpeed: String,
    pub Crossing: String,
    pub Finishing: String,
    pub Balance: String,
    pub Heading: String,
    pub Vision: String,
    pub OVR: String,
    pub Reactions: String,
    pub FreeKick: String,
    pub ShortPassing: String,
    pub Pace: String,
    pub Penalties: String,
    pub Defending: String,
    pub Volleys: String,
    pub Shooting: String,
    pub Aggression: String,
    pub Physical: String,
}
