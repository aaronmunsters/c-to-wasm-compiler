use crate::configuration::Configuration;

#[derive(Clone, Debug)]
pub struct ConfigurationBuilder<Profile, Debugging, Source, Filename> {
    profile: Profile,
    debugging: Debugging,
    source: Source,
    filename: Filename,
}

impl ConfigurationBuilder<(), (), (), ()> {
    #[must_use]
    pub fn init() -> Self {
        Self {
            profile: (),
            debugging: (),
            source: (),
            filename: (),
        }
    }
}

impl
    ConfigurationBuilder<
        crate::configuration::Profile,
        crate::configuration::Debugging,
        crate::configuration::Source,
        crate::configuration::Filename,
    >
{
    #[must_use]
    pub fn build(self) -> Configuration {
        let Self {
            profile,
            debugging,
            source,
            filename,
        } = self;

        Configuration {
            profile,
            debugging,
            source,
            filename,
        }
    }
}

impl<Profile, Debugging, Source, Filename>
    ConfigurationBuilder<Profile, Debugging, Source, Filename>
{
    pub fn profile(
        self,
        profile: crate::configuration::Profile,
    ) -> ConfigurationBuilder<crate::configuration::Profile, Debugging, Source, Filename> {
        ConfigurationBuilder {
            profile,
            debugging: self.debugging,
            source: self.source,
            filename: self.filename,
        }
    }
}

impl<Profile, Debugging, Source, Filename>
    ConfigurationBuilder<Profile, Debugging, Source, Filename>
{
    pub fn debugging(
        self,
        debugging: crate::configuration::Debugging,
    ) -> ConfigurationBuilder<Profile, crate::configuration::Debugging, Source, Filename> {
        ConfigurationBuilder {
            profile: self.profile,
            debugging,
            source: self.source,
            filename: self.filename,
        }
    }
}

impl<Profile, Debugging, Source, Filename>
    ConfigurationBuilder<Profile, Debugging, Source, Filename>
{
    pub fn source(
        self,
        source: crate::configuration::Source,
    ) -> ConfigurationBuilder<Profile, Debugging, crate::configuration::Source, Filename> {
        ConfigurationBuilder {
            profile: self.profile,
            debugging: self.debugging,
            source,
            filename: self.filename,
        }
    }
}

impl<Profile, Debugging, Source, Filename>
    ConfigurationBuilder<Profile, Debugging, Source, Filename>
{
    pub fn filename(
        self,
        filename: crate::configuration::Filename,
    ) -> ConfigurationBuilder<Profile, Debugging, Source, crate::configuration::Filename> {
        ConfigurationBuilder {
            profile: self.profile,
            debugging: self.debugging,
            source: self.source,
            filename,
        }
    }
}
