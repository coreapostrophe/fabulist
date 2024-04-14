use super::resource::Keyed;

#[derive(Debug, Clone)]
pub struct Name {
    pub short: String,
    pub long: Option<String>,
    pub nick: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Character {
    id: String,
    name: Name,
}

impl Character {
    pub fn new(id: impl Into<String>, short: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: Name {
                short: short.into(),
                long: None,
                nick: None,
            },
        }
    }
    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn set_long(&mut self, long: impl Into<String>) {
        self.name.long = Some(long.into());
    }
    pub fn set_nick(&mut self, nick: impl Into<String>) {
        self.name.nick = Some(nick.into())
    }
}

#[derive(Debug)]
pub struct CharacterBuilder {
    pub id: String,
    pub name: Name,
}

impl CharacterBuilder {
    pub fn new(id: impl Into<String>, short: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: Name {
                short: short.into(),
                long: None,
                nick: None,
            },
        }
    }
    pub fn set_long(mut self, long: impl Into<String>) -> Self {
        self.name.long = Some(long.into());
        self
    }
    pub fn set_nick(mut self, nick: impl Into<String>) -> Self {
        self.name.nick = Some(nick.into());
        self
    }
    pub fn build(self) -> Character {
        Character {
            id: self.id,
            name: self.name,
        }
    }
}

impl Keyed for Character {
    fn id(&self) -> &String {
        &self.id
    }
}

impl From<CharacterBuilder> for Character {
    fn from(value: CharacterBuilder) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
