use std::fmt::{Display, Formatter};

use inquire::ui::{Color, RenderConfig};
use inquire::validator::{ErrorMessage, Validation};
use inquire::{Select, Text};

use cargo_kit::Profile;

use crate::cli::CliConfig;
use crate::dialog::utils::{colorize_render_config, create_render_config};
use crate::dialog::PromptResult;

pub fn prompt_select_profile(
    cli_config: &CliConfig,
    existing_profiles: Vec<Profile>,
) -> PromptResult<Profile> {
    enum ProfileRow {
        Dev,
        Release,
        Custom(String),
        CreateNew,
    }
    impl Display for ProfileRow {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                ProfileRow::Dev => f.write_str("dev (builtin)"),
                ProfileRow::Release => f.write_str("release (builtin)"),
                ProfileRow::Custom(name) => f.write_str(name),
                ProfileRow::CreateNew => f.write_str("<Create a new profile>"),
            }
        }
    }

    let mut profiles = vec![ProfileRow::Dev, ProfileRow::Release];
    let mut original_profiles: Vec<_> = existing_profiles
        .into_iter()
        .filter_map(|p| match p {
            Profile::Builtin(_) => None,
            Profile::Custom(custom) => Some(custom.clone()),
        })
        .collect();
    original_profiles.sort();
    profiles.extend(original_profiles.into_iter().map(ProfileRow::Custom));
    profiles.push(ProfileRow::CreateNew);

    let selected = Select::new(
        "Select the profile that you want to update/create:",
        profiles,
    )
    .with_render_config(profile_render_config(cli_config))
    .prompt()?;

    let profile = match selected {
        ProfileRow::Dev => Profile::dev(),
        ProfileRow::Release => Profile::release(),
        ProfileRow::Custom(name) => Profile::Custom(name),
        ProfileRow::CreateNew => prompt_enter_profile_name(cli_config)?,
    };

    Ok(profile)
}

fn prompt_enter_profile_name(cli_config: &CliConfig) -> PromptResult<Profile> {
    let profile = Text::new("Select profile name:")
        .with_validator(|input: &str| {
            if input.is_empty() {
                return Ok(Validation::Invalid(ErrorMessage::Custom(
                    "Profile name must not be empty".to_string(),
                )));
            } else if !input
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
            {
                // Yes, Cargo does allow any unicode alphabet char or digit :)
                return Ok(Validation::Invalid(ErrorMessage::Custom(
                    "Profile name may contain only letters, numbers, underscore and hyphen"
                        .to_string(),
                )));
            }
            Ok(Validation::Valid)
        })
        .with_render_config(profile_render_config(cli_config))
        .prompt()?;
    let profile = match profile.as_str() {
        "dev" => Profile::dev(),
        "release" => Profile::release(),
        _ => Profile::Custom(profile),
    };
    Ok(profile)
}

fn profile_render_config(cli_config: &CliConfig) -> RenderConfig<'static> {
    let render_config = create_render_config(cli_config);
    colorize_render_config(cli_config, render_config, Color::DarkGreen)
}
