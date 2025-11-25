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
        let Self {
            profile: _,
            debugging,
            source,
        } = self;

        ConfigurationBuilder {
            profile,
            debugging,
            source,
        }
    }
}

impl<Profile, Debugging, Source> ConfigurationBuilder<Profile, Debugging, Source> {
    pub fn debugging(
        self,
        debugging: crate::configuration::Debugging,
    ) -> ConfigurationBuilder<Profile, crate::configuration::Debugging, Source> {
        let Self {
            profile,
            debugging: _,
            source,
        } = self;

        ConfigurationBuilder {
            profile,
            debugging,
            source,
        }
    }
}

impl<Profile, Debugging, Source> ConfigurationBuilder<Profile, Debugging, Source> {
    pub fn source(
        self,
        source: crate::configuration::Source,
    ) -> ConfigurationBuilder<Profile, Debugging, crate::configuration::Source> {
        let Self {
            profile,
            debugging,
            source: _,
        } = self;

        ConfigurationBuilder {
            profile,
            debugging,
            source,
        }
    }
}
