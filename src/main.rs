use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use dotenv::{dotenv, var};
use std::{thread, time::Duration};

mod app_updater;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file give error at loading");

    // Enviroment variables extractor
    let app_id = var("APP_ID").expect("APP_ID not found in enviroment variables.");
    let auth_header = var("AUTH_HEADER").expect("AUTH_HEADER not found in environment variables.");
    let app_name = var("APP_NAME").expect("APP_NAME not found in environment variables.");
    let activity_state =
        var("ACTIVITY_STATE").expect("ACTIVITY_STATE not found in environment variables.");
    let activity_details =
        var("ACTIVITY_DETAILS").expect("ACTIVITY_DETAILS not found in environment variables.");
    let large_image = var("LARGE_IMAGE").expect("LARGE_IMAGE not found in environment variables.");
    let large_text = var("LARGE_TEXT").expect("LARGE_TEXT not found in environment variables.");
    let button_label =
        var("BUTTON_LABEL").expect("BUTTON_LABEL not found in environment variables.");
    let button_url = var("BUTTON_URL").expect("BUTTON_URL not found in environment variables.");

    // Update app name
    app_updater::update(&app_id, &auth_header, &app_name)
        .await
        .expect(
        "There was an error updating the client name. Try updating Authorization on the .env file.",
    );

    // Initializing the Discord IPC Client
    // The name of the Activity is the name of your application on Discord Developer
    let mut discord =
        DiscordIpcClient::new(&app_id).expect("Error while setting App ID on Discord IPC Client.");

    // Assets builder
    let assets = activity::Assets::new()
        .large_image(&large_image)
        .large_text(&large_text);

    // Connect to Discord IPC Socket
    discord
        .connect()
        .expect("There was an error while connecting to Discord socket.");

    // Set activity
    let set_activity = discord.set_activity(
        activity::Activity::new()
            .state(&activity_state)
            .details(&activity_details)
            .assets(assets)
            .buttons(vec![activity::Button::new(&button_label, &button_url)]),
    );

    // Handle any errors setting the activity
    match set_activity {
        Ok(_) => println!("Activity updated!"),
        Err(e) => eprintln!("Error updating activity: {e:?}"),
    }

    // App main loop
    loop {
        thread::sleep(Duration::from_millis(10000));
    }
}
