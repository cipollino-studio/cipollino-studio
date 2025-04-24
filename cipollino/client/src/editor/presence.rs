
use pierro::ColorSpace;
use project::{Clip, Message, PresenceData, Ptr};

use super::Socket;

pub struct Presence {
    data: PresenceData,
    modified: bool
}

impl Presence {

    pub fn new() -> Self {
        Self {
            data: PresenceData::default(),
            modified: false,
        }
    }

    pub fn set_open_clip(&mut self, clip: Ptr<Clip>) {
        self.data.open_clip = clip;
        self.modified = true;
    }

    pub fn set_mouse_pos(&mut self, mouse_pos: Option<elic::Vec2>) {
        if mouse_pos.is_none() && self.data.mouse_pos.is_none() {
            return;
        }
        if let Some(mouse_pos) = mouse_pos {
            if let Some(prev_mouse_pos) = self.data.mouse_pos {
                if mouse_pos.distance(elic::vec2(prev_mouse_pos[0], prev_mouse_pos[1])) < 1.0 {
                    return;
                }
            }
        }

        self.data.mouse_pos = mouse_pos.map(|pos| pos.into());
        self.modified = true;
    }

    pub fn update(&mut self, socket: &mut Socket) {
        if !self.modified {
            return;
        }

        socket.send(Message::Presence(self.data.clone()));

        self.modified = false;
    }

}

const PRESENCE_ICONS: &[&'static str] = &[
    pierro::icons::RABBIT,
    pierro::icons::BIRD,
    pierro::icons::BUG,
    pierro::icons::BUTTERFLY,
    pierro::icons::CAT,
    pierro::icons::COW,
    pierro::icons::DOG,
    pierro::icons::FISH_SIMPLE,
    pierro::icons::HORSE,
    pierro::icons::LINUX_LOGO,
    pierro::icons::FLYING_SAUCER,
    pierro::icons::FLOWER_TULIP,
    pierro::icons::GHOST,
    pierro::icons::ALIEN,
];

pub fn presence_icon(client_id: u64) -> &'static str {
    let idx = (pierro::hash(&client_id) as usize) % PRESENCE_ICONS.len();
    PRESENCE_ICONS[idx]
}

pub fn presence_color(client_id: u64) -> pierro::Color {
    let t = (pierro::hash(&client_id) & 0xFF) as f32 / 255.0;
    let hue = 1.0 / 3.0 + 2.0 / 3.0 * t;
    let [r, g, b] = pierro::HSVColorSpace::to_rgb([hue, 0.9, 0.6]);
    pierro::Color::rgb(r, g, b)
}
