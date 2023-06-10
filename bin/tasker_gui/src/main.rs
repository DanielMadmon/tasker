use iced::widget::{button, column, text};
use iced::{Alignment, Element, Application, Command, Theme, executor, Settings};

fn main() {
    let window_settings = iced::window::Settings{
        size: (300,300),
        position: iced::window::Position::Centered,
        min_size: Some((300,300)),
        max_size: Some((300,300)),
        visible: true,
        resizable: false,
        decorations: true,
        transparent: false,
        always_on_top: false,
        icon: None,
        platform_specific: iced::window::PlatformSpecific,
    };
    let mut custom_settings = Settings{
        id: Default::default(),
        window: window_settings,
        flags: Default::default(),
        default_font: Default::default(),
        default_text_size: Default::default(),
        text_multithreading: Default::default(),
        antialiasing: Default::default(),
        exit_on_close_request: true,
        try_opengles_first: true,
    };
    state::run(Settings::from(custom_settings));
    
}
#[derive(Debug, Clone, Copy)]
enum MainMenu{
    CurrentTasks,
    EditTasks,
    Logs,
    ServiceStatus,
    About
}
struct state{
    do_nothing:bool
}
impl Application for state {
    type Message = MainMenu;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;
    fn new(_flags: ()) -> (state,Command<Self::Message>) {
        (Self { do_nothing: true },Command::none())
    }

    fn title(&self) -> String {
        String::from("Tasker")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>{
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            button("Active Tasks")
        ]
        .padding(10)
        .align_items(Alignment::Center)
        .into()
    }
    
}



