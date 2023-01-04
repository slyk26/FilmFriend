use mongodb::bson::{DateTime};
use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::SerenityError;
use serenity::async_trait;
use serenity::model::application::component::{ActionRow, ActionRowComponent, InputTextStyle};
use serenity::model::id::GuildId;
use serenity::model::user::User;
use crate::backend::database;
use crate::backend::models::movie::Movie;
use crate::backend::utils::is_valid_imdb_link;
use crate::commands::slash_command::{SlashCommand};

pub struct Submit;

#[async_trait]
impl SlashCommand for Submit {
    fn name(&self) -> String {
        "submit".to_string()
    }

    fn description(&self) -> String {
        "Submit a movie for Movie Nights".to_string()
    }

    // the modal popup
    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::Modal)
                    .interaction_response_data(|body| {
                        body.custom_id(format!("{}-{}", "submit", command.user.id.0))
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
                                            it.custom_id("comment").label("comment").style(InputTextStyle::Paragraph).placeholder("comment your submission (optional)").required(false)
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
    /// returns a response message (text) and bool if response should be ephermal
    pub async fn handle_modal(components: &Vec<ActionRow>, submitter: &User, server: &GuildId) ->  (bool, String) {
        let mut link: String = String::new();
        let mut comment: String = String::new();

        // get data for movie
        for row in components {
            for c in &row.components {
                match c {
                    ActionRowComponent::InputText(c) => {
                        match c.custom_id.as_str() {
                            "imdb-link" => {
                                link = c.value.clone();
                            }

                            "comment" => {
                                comment = c.value.clone();
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
                comment,
                added: DateTime::now(),
                watched: false,
            };

            // add to database
            database::insert(server.to_string(), &movie).await
        } else {
            (true, "Check your IMDb Url again, it was not valid.\
            \n* Link is case sensitive\
            \n* Try removing the referral part at the end (?ref_= ...)\
            \n* dont forget to keep the forward slash (/) at the end".to_string())
        }
    }
}