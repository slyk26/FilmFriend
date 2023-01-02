use mongodb::bson::{DateTime};
use mongodb::Database;
use crate::commands::slash_command::{SlashCommand};
use serenity::builder::{CreateApplicationCommand};
use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::SerenityError;
use serenity::async_trait;
use serenity::model::application::component::{ActionRow, ActionRowComponent, InputTextStyle};
use serenity::model::prelude::ChannelId;
use serenity::model::user::User;
use crate::backend::database;
use crate::backend::models::movie::Movie;
use crate::backend::utils::is_valid_imdb_link;

pub struct Submit;

#[async_trait]
impl SlashCommand for Submit {
    fn name(&self) -> String {
        "submit".to_string()
    }

    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        command.name(self.name()).description("submit a movie for Movie Nights")
    }

    // the modal popup
    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::Modal)
                    .interaction_response_data(|body| {
                        body.custom_id(self.name())
                            .title("Submit a Movie")
                            .components(|cc| {
                                cc.create_action_row(|ar| {
                                    ar.create_input_text(|it| {
                                        it.custom_id("imdb-link")
                                            .label("imdb-link")
                                            .style(InputTextStyle::Short)
                                            .placeholder("e.g. https://www.imdb.com/title/tt0264263/")
                                    })
                                })
                                    .create_action_row(|ar| {
                                        ar.create_input_text(|it| {
                                            it.custom_id("info").label("info").style(InputTextStyle::Paragraph).placeholder("Additional Information\n(Movie Name, why you suggest it, etc...)")
                                        })
                                    })
                            })
                    })
            }).await
    }
}

impl Submit {
    /// iterates over each [ActionRow] to process each input field of a [InteractionResponseType::Modal]
    ///
    /// returns a response message (text)
    pub async fn handle_modal(components: &Vec<ActionRow>, submitter: &User, server: &ChannelId, db: &Database) -> String {
        let mut link: String = String::new();
        let mut info: String = String::new();

        // get data for movie
        for row in components {
            for c in &row.components {
                match c {
                    ActionRowComponent::InputText(c) => {
                        match c.custom_id.as_str() {
                            "imdb-link" => {
                                link = c.value.clone();
                            }

                            "info" => {
                                info = c.value.clone();
                            }
                            _ => {}
                        };
                    }

                    _ => {}
                }
            }
        }

        if is_valid_imdb_link(&link) {
            // extract id
            let idx = link.find("/tt").unwrap();
            let id = &link[idx + 1..idx + 10];

            // create movie
            let movie = Movie {
                id: id.to_string(),
                link,
                submitted_by: submitter.id.0,
                info,
                added: DateTime::now(),
                watched: false,
            };

            // add to database
            database::insert(server.to_string(), &movie, &db).await
        } else {
            "Check your IMDb Url again, it was not valid \nHint: Try removing the refferal part at the end (?ref_= ...).".to_string()
        }
    }
}