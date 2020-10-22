use druid::{
    widget::{Button, Controller, Flex, Label, TextBox},
    AppLauncher, Data, Env, Event, EventCtx, Lens, PlatformError, Selector, Widget, WidgetExt,
    WindowDesc,
};
use druid_enums::Matcher;

const LOGGED: Selector<LoggedMainState> = Selector::new("druid-enums.basic.logged");
const UNLOGGED: Selector<UnloggedMainState> = Selector::new("druid-enums.basic.unlogged");

#[derive(Clone, Data, Matcher)]
#[matcher(matcher_name = App)] // defaults to AppStateMatcher
enum AppState {
    Login(LoginState),
    Main(MainState),
}

#[derive(Clone, Data, Lens, Default)]
struct LoginState {
    user: String,
}

#[derive(Clone, Data, Matcher)]
enum MainState {
    LoggedMainState(LoggedMainState),
    UnloggedMainState(UnloggedMainState),
}

#[derive(Clone, Data, Lens)]
struct UnloggedMainState {
    count: u32,
}

#[derive(Clone, Data, Lens)]
struct LoggedMainState {
    user: String,
    count: u32,
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui).title("Druid Enums");
    let state = AppState::Login(LoginState::default());
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(state)
}

fn ui() -> impl Widget<AppState> {
    // AppState::matcher() or
    App::new()
        .login(login_ui())
        .main(main_ui())
        .controller(LoginController)
}

fn login_ui() -> impl Widget<LoginState> {
    fn login(ctx: &mut EventCtx, state: &mut LoginState, _: &Env) {
        if state.user.is_empty() {
            ctx.submit_command(UNLOGGED.with(UnloggedMainState::from(state.clone())), None)
        } else {
            ctx.submit_command(LOGGED.with(LoggedMainState::from(state.clone())), None)
        }
    }

    Flex::row()
        .with_child(TextBox::new().lens(LoginState::user))
        .with_spacer(5.0)
        .with_child(Button::new("Login").on_click(login))
        .center()
}

fn main_ui() -> impl Widget<MainState> {
    MainStateMatcher::new()
        .logged_main_state(logged_main_ui())
        .unlogged_main_state(unlogged_main_ui())
        .controller(MainController)
}

fn logged_main_ui() -> impl Widget<LoggedMainState> {
    Flex::column()
        .with_child(Label::dynamic(LoggedMainState::welcome_label))
        .with_spacer(5.0)
        .with_child(
            Button::dynamic(LoggedMainState::count_label)
                .on_click(|_, state: &mut LoggedMainState, _| state.count += 1),
        )
        .center()
}

fn unlogged_main_ui() -> impl Widget<UnloggedMainState> {
    Flex::column()
        .with_child(
            Button::dynamic(UnloggedMainState::count_label)
                .on_click(|_, state: &mut UnloggedMainState, _| state.count += 1),
        )
        .center()
}

struct LoginController;
impl Controller<AppState, App> for LoginController {
    fn event(
        &mut self,
        child: &mut App,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(LOGGED) => {
                let main_state = cmd.get_unchecked(LOGGED).clone();
                *data = AppState::Main(MainState::LoggedMainState(main_state));
            }
            Event::Command(cmd) if cmd.is(UNLOGGED) => {
                let main_state = cmd.get_unchecked(UNLOGGED).clone();
                *data = AppState::Main(MainState::UnloggedMainState(main_state));
            }
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}

struct MainController;
impl Controller<MainState, MainStateMatcher> for MainController {
    fn event(
        &mut self,
        child: &mut MainStateMatcher,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut MainState,
        env: &Env,
    ) {
        child.event(ctx, event, data, env)
    }
}

impl LoggedMainState {
    pub fn welcome_label(&self, _: &Env) -> String {
        format!("Welcome {}!", self.user)
    }

    pub fn count_label(&self, _: &Env) -> String {
        format!("clicked {} times", self.count)
    }
}

impl UnloggedMainState {
    pub fn count_label(&self, _: &Env) -> String {
        format!("clicked {} times", self.count)
    }
}

impl From<LoginState> for LoggedMainState {
    fn from(login: LoginState) -> Self {
        Self {
            user: login.user,
            count: 0,
        }
    }
}

impl From<LoginState> for UnloggedMainState {
    fn from(_login: LoginState) -> Self {
        Self { count: 0 }
    }
}
