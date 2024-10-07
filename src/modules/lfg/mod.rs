use async_trait::async_trait;
use lazy_static::lazy_static;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateActionRow, CreateCommand,
    CreateCommandOption, CreateInputText, CreateInteractionResponse, CreateModal, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption, EditInteractionResponse, InputTextStyle,
    ResolvedValue,
};
use std::collections::HashMap;
use zayden_core::{parse_options, SlashCommand};

use crate::{Error, Result};

lazy_static! {
    static ref ACTIVITY_MAP: HashMap<&'static str, Vec<(&'static str, &'static str)>> = {
        let mut m = HashMap::new();
        m.insert(
            "raid",
            vec![
                ("Salvation's Edge", "se"),
                ("Crota's End", "crota"),
                ("Root of Nightmares", "ron"),
                ("King's Fall", "kf"),
                ("Vow of the Disciple", "VOTD"),
                ("Vault of Glass", "vog"),
                ("Deep Stone Crypt", "dsc"),
                ("Garden of Salvation", "gos"),
                ("Last Wish", "lw"),
            ],
        );
        m.insert(
            "dungeon",
            vec![
                ("Warlord's Ruin", "warlords ruin"),
                ("Ghosts of the Deep", "gotd"),
                ("Spire of the Watcher", "sotw"),
                ("Duality", "duality"),
                ("Grasp of Avarice", "goa"),
                ("Prophecy", "prophecy"),
                ("Pit of Heresy", "poh"),
                ("Shattered Throne", "st"),
            ],
        );
        m.insert(
            "exotic mission",
            vec![
                ("The Whisper", "whisper"),
                ("Zero Hour", "zero hour"),
                ("Harbinger", "harbinger"),
                ("Presage", "presage"),
                ("Vox Obscura", "vox"),
                ("Operation: Seraph's Shield", "seraphs sheild"),
                ("Node.Ovrd.Avalon", "avalon"),
                ("Starcrossed", "starcrossed"),
            ],
        );
        m.insert(
            "vanguard",
            vec![
                ("Strike", "strike"),
                ("Nightfall", "nightfall"),
                ("Grandmaster", "gm"),
            ],
        );
        m.insert(
            "crucible",
            vec![
                ("Crucible", "crucible"),
                ("Iron Banner", "ib"),
                ("Trials of Osiris", "trials"),
            ],
        );
        // m.insert("seasonal", vec![("")]);
        m
    };
}

pub fn register() -> Vec<CreateCommand> {
    vec![LfgCommand::register()]
}

pub struct LfgCommand;

#[async_trait]
impl SlashCommand<Error> for LfgCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer_ephemeral(ctx).await?;

        let options = interaction.data.options();
        let options = parse_options(&options);
        let activity = match options.get("activity") {
            Some(ResolvedValue::String(activity)) => *activity,
            _ => unreachable!("Activity is required"),
        };

        if let Some(sub_activity) = ACTIVITY_MAP.get(activity) {
            let menu = CreateSelectMenu::new(
                "lfg_activity",
                CreateSelectMenuKind::String {
                    options: sub_activity
                        .into_iter()
                        .map(|a| CreateSelectMenuOption::new(a.0, a.1))
                        .collect(),
                },
            );

            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new()
                        .select_menu(menu)
                        .content("Select the activity you are looking to do"),
                )
                .await?;
        }

        let modal = create_modal(activity);

        interaction.create_response(ctx, CreateInteractionResponse::Modal(modal));

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("lfg")
            .description("Create a looking for group post")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "create",
                    "Create a new looking for group post",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "activity",
                        "The activity you are looking to do",
                    )
                    .required(true)
                    .add_string_choice("Raid", "raid")
                    .add_string_choice("Dungeon", "dungeon")
                    .add_string_choice("Exotic Mission", "exotic mission")
                    .add_string_choice("Vangard", "vanguard")
                    .add_string_choice("Gambit", "gambit")
                    .add_string_choice("Crucible", "crucible")
                    .add_string_choice("Seasonal", "seasonal")
                    .add_string_choice("Other", "other"),
                ),
            )
    }
}

fn create_modal(activity: &str) -> CreateModal {
    let row = vec![
        CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, "Activity", "activity").value(activity),
        ),
        CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, "Start Time", "start time")
                .placeholder("YYYY-MM-DD HH:MM"),
        ),
        CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Paragraph, "Description", "description")
                .placeholder(activity),
        ),
    ];

    CreateModal::new("lfg_create", "Create Event").components(row)
}
