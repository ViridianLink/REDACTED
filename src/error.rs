use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Family(family::Error),
    GoldStar(gold_star::Error),
    Lfg(lfg::Error),
    ReactionRole(reaction_roles::Error),
    TempVoice(temp_voice::Error),
}

impl ErrorResponse for Error {
    fn to_response(&self) -> String {
        match self {
            Error::Family(e) => e.to_response(),
            Error::GoldStar(e) => e.to_response(),
            Error::Lfg(e) => e.to_response(),
            Error::ReactionRole(e) => e.to_response(),
            Error::TempVoice(e) => e.to_response(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<family::Error> for Error {
    fn from(e: family::Error) -> Self {
        Error::Family(e)
    }
}

impl From<gold_star::Error> for Error {
    fn from(e: gold_star::Error) -> Self {
        Error::GoldStar(e)
    }
}

impl From<lfg::Error> for Error {
    fn from(e: lfg::Error) -> Self {
        Error::Lfg(e)
    }
}

impl From<reaction_roles::Error> for Error {
    fn from(e: reaction_roles::Error) -> Self {
        Error::ReactionRole(e)
    }
}

impl From<temp_voice::Error> for Error {
    fn from(e: temp_voice::Error) -> Self {
        Error::TempVoice(e)
    }
}
