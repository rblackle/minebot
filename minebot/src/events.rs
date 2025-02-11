use crate::gamestate::GameState;
use json::JsonValue;
use packets::ServerPacket;

#[derive(Clone)]
pub enum EventMatcher {
    ChatMessage,
    HealthChanged
}

impl EventMatcher {
    pub fn match_packet(&self, packet: &ServerPacket, gamestate: &GameState) -> Option<Event> {
        match (self, packet) {
            (EventMatcher::ChatMessage, ServerPacket::ChatMessage { json, .. }) => {
                if let Some((player, message)) = parse_chat(json) {
                    if player != gamestate.username {
                        Some(Event::ChatMessage {
                            player,
                            message
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            },

            (EventMatcher::HealthChanged, ServerPacket::UpdateHealth { health, ..}) => {
                let health = health / 2.0;
                if health != gamestate.health {
                    Some(Event::HealthChanged{
                        new: health,
                        old: gamestate.health
                    })
                } else {
                    None
                }
            }
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum Event {
    ChatMessage {
        player: String,
        message: String
    },
    HealthChanged {
        new: f32,
        old: f32
    }
}

#[derive(Default)]
pub struct EventMatchers {
    matchers: Vec<EventMatcher>
}

impl EventMatchers {
    pub fn listen(&mut self, matcher: EventMatcher) {
        self.matchers.push(matcher);
    }

    pub fn match_packet(&self, packet: &ServerPacket, gamestate: &GameState) -> Option<Event> {
        self.matchers.iter()
            .filter_map(|m| m.match_packet(packet, gamestate))
            .next()
    }
}

fn parse_chat(chat: &JsonValue) -> Option<(String, String)> {
    let player: String = chat["with"][0]["text"].as_str()?.to_owned();
    let message: String = chat["with"][1]["extra"][0]["text"].as_str()?.to_owned();
    Some((player, message))
}