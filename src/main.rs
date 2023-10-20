/*Name: Garrett Thompson
*Date 10/20/23
*Purpose: Program that takes data from weather api and displays it via TUI
*/

//MOVING CRATES

use openweathermap::{init, update}; //for weather api data

use crossterm::{ //for key press recognition and terminal functionality
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{ //for personalizing the TUI
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

use std::cell::RefCell; //used for creating mutable refrences of immutable objects

use std::io::{stderr, Result}; //used for displaying terminal as a result of the main function

fn main() -> Result<()> {
    
    stderr().execute(EnterAlternateScreen)?; //moving user to alternate screen
    enable_raw_mode(); //raw mode turns off inputs and output processing by the terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?; //Creation of backend terminal
    terminal.clear()?; //clears screen

    let weather_stream = &init("Berlin, DE", "imperial", "en", "f1e875bd567884ff618ff3c7bb8d6e19", 10); //storing weather data into a data reciever object
    //Parameters include location string, units string, language string, api_key, and poll minutes
    // documentation-> https://docs.rs/openweathermap/latest/openweathermap/fn.init.html

    let weather_string = RefCell::new(String::new()); //creation of empty string using refcell to reference it later

    // begin the weather and terminal observation loop
loop {

    let mut weather_string_temp = weather_string.borrow_mut(); //creating a mutable reference to the weather string

    //match case for update function 
    //this function accepts only a receiver object from the init() function
    //this function has different outputs depending on the status of the data retrevial process, this is why a match case is used
    // if the update is sucessful, the function changes the content of the mutable string with random weather data
    match update(weather_stream){
        Some(response) => match response {
            Ok(current) => *weather_string_temp = format!("Today's weather in {} is {} and clouds are at {} percent",
            current.name.as_str(),
            current.weather[0].main.as_str(), //addition of random data documentation of the properties this object has-> https://docs.rs/openweathermap/latest/openweathermap/struct.CurrentWeather.html
            current.clouds.all),
            Err(e) => *weather_string_temp = format!("Could not fetch weather because: {}", e), //if the update function fails it writes an error instead
        },
        None => (),
    }

    //Here is where ratatui is utilized
    //The terminal.draw function is calle and an area is set
    //a paragraph object is written to the screen, it is loaded with the weather string information
    //the string must be cloned because of rust's ownership property
    terminal.draw(|frame| {

        let area = frame.size();
        frame.render_widget(
            Paragraph::new(weather_string_temp.clone())
            .white()
            .on_blue(),
            area,
        );
    })?;

    //after the screen is rendered the program checks for any events that have occured
    //if the event is of type key then read it, then if the key is of KeyCode::Esc break the loop and stop drawing the terminal
    if event::poll(std::time::Duration::from_millis(100))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc{
                break;
            }
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(100))
}

    //once loop is broken the alternate screen is closed and raw mode is diabled to allow terminal access again
    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}