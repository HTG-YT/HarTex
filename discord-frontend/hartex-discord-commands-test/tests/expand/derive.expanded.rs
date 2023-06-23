use hartex_discord_commands_core::CommandMetadata;
#[metadata(command_type = 1)]
#[metadata(interaction_only = true)]
#[metadata(name = "derive")]
pub struct Derive;
extern crate hartex_discord_commands_core as _commands_core;
#[automatically_derived]
impl _commands_core::traits::CommandMetadata for Derive {
    fn command_type(&self) -> u8 {
        1
    }
    fn interaction_only(&self) -> bool {
        true
    }
    fn name(&self) -> String {
        String::from("derive")
    }
    fn minimum_level(&self) -> u16 {
        0
    }
}
