use xplm::plugin::{Plugin, PluginInfo};
use xplm::{debug, xplane_plugin};


struct MinimalPlugin;

impl Plugin for MinimalPlugin {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        // The following message should be visible in the developer console and the Log.txt file
        debug("Hello, World! From the Minimal Rust Plugin\n");
        Ok(MinimalPlugin)
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from("Minimal Rust Plugin"),
            signature: String::from("org.samcrow.xplm.examples.minimal"),
            description: String::from("A plugin written in Rust"),
        }
    }
}

xplane_plugin!(MinimalPlugin);
