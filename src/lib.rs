use pumpkin_plugin_api::{
    self as pumpkin, Server,
    command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    events::{EventHandler, EventPriority, PlayerJoinEventData, PlayerLeaveEventData},
    text::{NamedColor, TextComponent},
};

use rand::{Rng as _, rng};
use tracing::*;

struct HelloPlugin;

struct MyJoinHandler;
impl EventHandler<PlayerJoinEventData> for MyJoinHandler {
    fn handle<'a>(&'a self, server: Server, mut event: PlayerJoinEventData) -> PlayerJoinEventData {
        tracing::info!("Player joined with ID: {}", event.player.get_id());
        tracing::info!("Difficulty upon player join: {:?}", server.get_difficulty());

        event.join_message = TextComponent::text("Hello, world!");
        event
    }
}

struct MyLeaveHandler;
impl EventHandler<PlayerLeaveEventData> for MyLeaveHandler {
    fn handle<'a>(
        &'a self,
        server: Server,
        mut event: PlayerLeaveEventData,
    ) -> PlayerLeaveEventData {
        tracing::info!("Player left with ID: {}", event.player.get_id());
        tracing::info!(
            "Difficulty upon player leave: {:?}",
            server.get_difficulty()
        );

        event.leave_message = TextComponent::text("Bye, world!");
        event
    }
}

impl pumpkin::Plugin for HelloPlugin {
    fn new() -> Self {
        HelloPlugin
    }

    fn metadata(&self) -> pumpkin::PluginMetadata {
        pumpkin::PluginMetadata {
            name: "Hello Plugin".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Bjorn".into()],
            description: "A simple example plugin".into(),
        }
    }

    fn on_load(&mut self, context: pumpkin::Context) -> pumpkin::Result<()> {
        info!("Hello from the example plugin!");
        let server = context.get_server();
        let difficulty = server.get_difficulty();
        info!("Current difficulty is: {:?}", difficulty);
        context.register_event_handler(MyJoinHandler, EventPriority::Normal, true)?;
        context.register_event_handler(MyLeaveHandler, EventPriority::Normal, true)?;

        let command = Command::new(
            &["rps".to_string(), "rockpaperscissors".to_string()],
            "Play Rock Paper Scissors with the server.",
        );
        command.then(CommandNode::literal("rock").execute(RockPaperScissorsExecutor(Choice::Rock)));
        command
            .then(CommandNode::literal("paper").execute(RockPaperScissorsExecutor(Choice::Paper)));
        command.then(
            CommandNode::literal("scissors").execute(RockPaperScissorsExecutor(Choice::Scissors)),
        );

        context.register_command(command, "hello-pumpkin:command.rockpaperscisors");
        Ok(())
    }

    fn on_unload(&mut self, _context: pumpkin::Context) -> pumpkin::Result<()> {
        info!("Example plugin unloaded. Goodbye!");
        Ok(())
    }
}

pumpkin::register_plugin!(HelloPlugin);

#[derive(PartialEq, Debug, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Choice {
    pub fn beats(&self, other: &Choice) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }

        match (self, other) {
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }
}

fn get_random_choice() -> Choice {
    let choices = [Choice::Rock, Choice::Paper, Choice::Scissors];
    let index = rng().random_range(0..3);
    choices[index]
}

struct RockPaperScissorsExecutor(Choice);
impl CommandHandler for RockPaperScissorsExecutor {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player_choice = self.0;
        let computer_choice = get_random_choice();

        let you_chose = TextComponent::text("You chose: ");
        you_chose.add_child(TextComponent::text(&format!("{:?}", player_choice)));
        you_chose.color_named(NamedColor::Aqua);
        sender.send_message(you_chose);

        let i_chose = TextComponent::text("I chose: ");
        i_chose.add_child(TextComponent::text(&format!("{:?}", computer_choice)));
        i_chose.color_named(NamedColor::Gold);
        sender.send_message(i_chose);

        match player_choice.beats(&computer_choice) {
            Outcome::Win => {
                let message = TextComponent::text("You win!");
                message.color_named(NamedColor::Green);
                sender.send_message(message);
            }
            Outcome::Lose => {
                let message = TextComponent::text("You lose!");
                message.color_named(NamedColor::Red);
                sender.send_message(message);
            }
            Outcome::Draw => {
                let message = TextComponent::text("It's a tie!");
                message.color_named(NamedColor::Yellow);
                sender.send_message(message);
            }
        }

        Ok(1)
    }
}
