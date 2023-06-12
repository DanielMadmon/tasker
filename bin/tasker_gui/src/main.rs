use iced::theme::Container;
use iced::widget::{button, column, text, Column, Row, row, container};
use iced::{Alignment, Element, Application, Command, Theme, executor, Settings,theme, Renderer, Padding};
use tasker_lib::taskerctl::Task;


fn main() -> iced::Result{
    let window_settings = iced::window::Settings{
        size: (400,400),
        position: iced::window::Position::Centered,
        min_size: Some((400,400)),
        max_size: Some((400,400)),
        visible: true,
        resizable: false,
        decorations: true,
        transparent: false,
        always_on_top: false,
        icon: None,
        platform_specific: iced::window::PlatformSpecific,
    };
    let custom_settings = Settings{
        id: Default::default(),
        window: window_settings,
        flags: Default::default(),
        default_font: Default::default(),
        default_text_size: 16.0,
        text_multithreading: Default::default(),
        antialiasing: false,
        exit_on_close_request: true,
        try_opengles_first: true,
    };
    State::run(Settings::from(custom_settings)) 
    
}
#[derive(Debug, Clone, Copy,PartialEq)]
enum MainMenu{
    CurrentTasks,
    ShowLogs,
    SaveChanges,
    DiscardAndExit
}
#[derive(Clone,Copy)]
struct State{
    object:MainMenu
}
impl Application for State {
    type Message = MainMenu;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;
    fn new(_flags: ()) -> (State,Command<Self::Message>) {
        (Self { object: MainMenu::CurrentTasks },Command::none())
    }

    fn title(&self) -> String {
        String::from("Tasker")
    }

    fn update(&mut self, message: Self::Message)-> Command<MainMenu> {
        match message {
            MainMenu::CurrentTasks => {
                self.object = MainMenu::CurrentTasks;
                Command::none()
            }
            MainMenu::DiscardAndExit => {
                Command::none()
            }
            MainMenu::SaveChanges=> {
                Command::none()
            }   
            MainMenu::ShowLogs=>{
                self.object = MainMenu::ShowLogs;
                Command::none()
            }
        }
    }
    /*
    TODO:center everything
         add table widget?
     */
    fn view(&self) -> Element<'_, Self::Message> {
         let btn_row:Row<MainMenu,Renderer> = row![
            button("Active Tasks")
            .style(theme::Button::Secondary)
            .on_press(self::MainMenu::CurrentTasks)
            ,
            button("show logs")
                .style(theme::Button::Secondary)
                .on_press(self::MainMenu::ShowLogs)
                ].padding(Padding::horizontal(55.into()));

        let mut sub_view :Column<MainMenu,Renderer> = 
            column![btn_row];
        match self.object{
            MainMenu::CurrentTasks => {
                sub_view.push(text("show current tasks")).into()
            }
            MainMenu::ShowLogs => {
                sub_view.push(text("show logs")).into()
            }
            MainMenu::SaveChanges => todo!(),
            MainMenu::DiscardAndExit => todo!(),
        }
        
    }
    fn theme(&self) -> Self::Theme {
        self::Theme::Dark
    }
}




