use std::fmt::{self, Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Slot {
    Deck,
    Forward,
    Auxiliary,
    Bridge,
    Aft,
    Engine,
    Cook,
    Engineer,
    FirstOfficer,
    Gunner,
    Surgeon,
    #[default]
    Mascot
}

impl Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Slot::Deck => "Deck",
            Slot::Forward => "Forward",
            Slot::Auxiliary => "Auxiliary",
            Slot::Bridge => "Bridge",
            Slot::Aft => "Aft",
            Slot::Engine => "Engine",
            Slot::Cook => "Cook",
            Slot::Engineer => "Chief Engineer",
            Slot::FirstOfficer => "First Officer",
            Slot::Gunner => "Gunnery Officer",
            Slot::Surgeon => "Surgeon",
            Slot::Mascot => "Mascot"
        };

        write!(f, "{}", s)
    }
}