#[derive(Debug, Clone)]
pub enum Player {
    Player1,
    Player2,
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        match self {
            Self::Player1 => match other {
                Self::Player1 => true,
                Self::Player2 => false,
            },
            Self::Player2 => match other {
                Self::Player1 => false,
                Self::Player2 => true,
            },
        }
    }
}
