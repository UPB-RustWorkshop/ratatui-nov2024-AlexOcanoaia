use futures::executor::block_on;
use ratatui_templates::app::{App, AppResult};
use ratatui_templates::connection::get_data;
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::{io, result};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main] 
async fn main() -> AppResult<()> {
    // Create an application.
    // let app = 

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    // TODO:  the terminal user interface
    // let mut tui =
   
    let result = get_data("Bucharest".to_string()).await.unwrap();
    println!("The temperature is: {}", result.temperature);
    println!("The time is: {}", result.time);
    println!("The humidity is: {}", result.humidity);
    println!("The description is: {}", result.description);
    println!("The wind speed is: {}", result.wind_speed);

    // let _result = get_data("Bucharest".to_string());
    // TODO: init the terminal

    // Start the main loop.
    // while app.running {
        // TODO: Render the user interface.

        // TODO: Handle events.
        
    // }

    // TODO: Reset the terminal if the app has been terminated

    Ok(())
}
