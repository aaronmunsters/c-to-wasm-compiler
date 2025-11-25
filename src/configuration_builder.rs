use crate::configuration::Configuration;

#[derive(Clone, Debug)]
pub struct ConfigurationBuilder<Profile, Debugging, Source> {
    profile: Profile,
    debugging: Debugging,
    source: Source,
}

impl ConfigurationBuilder<(), (), ()> {
    #[must_use]
    pub fn init() -> Self {
        Self {
            profile: (),
            debugging: (),
            source: (),
        }
    }
}

impl
    ConfigurationBuilder<
        crate::configuration::Profile,
        crate::configuration::Debugging,
        crate::configuration::Source,
    >
{
    #[must_use]
    pub fn build(self) -> Configuration {
        let Self {
            profile,
            debugging,
            source,
        } = self;

        Configuration {
            profile,
            debugging,
            source,
        }
    }
}

impl<Profile, Debugging, Source> ConfigurationBuilder<Profile, Debugging, Source> {
    pub fn profile(
        self,
        profile: crate::configuration::Profile,
    ) -> ConfigurationBuilder<crate::configuration::Profile, Debugging, Source> {
        ConfigurationBuilder {
            profile,
            debugging: self.debugging,
            source: self.source,
        }
    }
}

impl<Profile, Debugging, Source> ConfigurationBuilder<Profile, Debugging, Source> {
    pub fn debugging(
        self,
        debugging: crate::configuration::Debugging,
    ) -> ConfigurationBuilder<Profile, crate::configuration::Debugging, Source> {
        ConfigurationBuilder {
            profile: self.profile,
            debugging,
            source: self.source,
        }
    }
}

impl<Profile, Debugging, Source> ConfigurationBuilder<Profile, Debugging, Source> {
    pub fn source(
        self,
        source: crate::configuration::Source,
    ) -> ConfigurationBuilder<Profile, Debugging, crate::configuration::Source> {
        ConfigurationBuilder {
            profile: self.profile,
            debugging: self.debugging,
            source,
        }
    }
}
