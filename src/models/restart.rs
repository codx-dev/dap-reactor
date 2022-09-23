use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartArguments {
    Launch(LaunchArguments),
    Attach(AttachArguments),
}

impl From<RestartArguments> for Value {
    fn from(args: RestartArguments) -> Self {
        match args {
            RestartArguments::Launch(l) => l.into(),
            RestartArguments::Attach(a) => a.into(),
        }
    }
}

impl TryFrom<&Map<String, Value>> for RestartArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let launch = map.contains_key("noDebug");

        if launch {
            LaunchArguments::try_from(map).map(Self::Launch)
        } else {
            Ok(Self::Attach(AttachArguments::from(map)))
        }
    }
}
