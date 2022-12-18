use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::color::AnsiValue;
use termion::color;
use std::io::stdin;
use uuid::Uuid;

#[derive(Debug)]
#[derive(Clone)]
struct AnimalClass {
    icon: char,
}

#[derive(Debug)]
#[derive(Clone)]
struct Animal {
    class: AnimalClass,
    health: i32,
    attack: i32,
}

#[derive(Clone)]
#[derive(Debug)]
struct Scene {
    team1: Vec<(Uuid, Animal)>,
    team2: Vec<(Uuid, Animal)>,
}

#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Side {
    Left, Right
}

#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
enum State {
    Battling,
    Finished { winner: Option<Side> }
}

enum Event {
    Damage(Uuid, i32),
    Kill(Uuid)
}

impl Scene {
    fn get_state(&self) -> State {
        match (self.team1.len() > 0, self.team2.len() > 0) {
            (true, true) => State::Battling,
            (true, false) => State::Finished { winner: Some(Side::Left) },
            (false, true) => State::Finished { winner: Some(Side::Right) },
            (false, false) => State::Finished { winner: None },
        }
    }

    fn run_step(&mut self) {
        let mut event_queue = vec![Event::Damage(self.team1[0].0, self.team2[0].1.attack), Event::Damage(self.team2[0].0, self.team1[0].1.attack)];
        while event_queue.len() > 0 {
            let events = std::mem::take(&mut event_queue);
            for event in events {
                self.run_event(event, &mut event_queue);
            }
            for (uuid, animal) in self.team1.iter().chain(self.team2.iter()) {
                if animal.health <= 0 {
                    event_queue.push(Event::Kill(*uuid))
                }
            }
        }
    }
    
    fn animal_by_id(&mut self, uuid: Uuid) -> Option<&mut Animal> {
        (self.team1.iter_mut().chain(self.team2.iter_mut())).find(|(animal_id, animal)| *animal_id == uuid).map(|(animal_id, animal)| animal)
    }

    fn run_event(&mut self, event: Event, event_queue: &mut Vec<Event>) {
        match event {
            Event::Damage(uuid, amount) => {
                if let Some(animal) = self.animal_by_id(uuid) {
                    animal.health -= amount;
                }
            }
            Event::Kill(uuid) => {
                self.team1.retain(|(animal_id, animal)| *animal_id != uuid);
                self.team2.retain(|(animal_id, animal)| *animal_id != uuid);
            }
        }
    }
}

fn beastiary() -> Vec<Animal> {
    vec![
        Animal {
            class: AnimalClass { icon: '🐟' },
            health: 2,
            attack: 2,
        },
        Animal {
            class: AnimalClass { icon: '🐜' },
            health: 1,
            attack: 2,
        },
        Animal {
            class: AnimalClass { icon: '🐖' },
            health: 1,
            attack: 4,
        },
        Animal {
            class: AnimalClass { icon: '🐎' },
            health: 1,
            attack: 2,
        },
        Animal {
            class: AnimalClass { icon: '🦆' },
            health: 3,
            attack: 2,
        },
        Animal {
            class: AnimalClass { icon: '🦦' }, // Otter
            health: 1,
            attack: 2,
        },
        Animal {
            class: AnimalClass { icon: '🦫' }, // Beaver
            health: 1,
            attack: 2,
        },
        Animal {
            class: AnimalClass { icon: '🦟' },
            health: 2,
            attack: 2,
        },
        Animal {
            class: AnimalClass { icon: '🦗' },
            health: 2,
            attack: 1,
        },
        Animal {
            class: AnimalClass { icon: '?' },
            health: 99,
            attack: 99,
        }
    ]
}

fn render_teams(scene: &Scene) {
    print!("{}{}{}{}{}", termion::cursor::Goto(29, 9), color::Bg(color::Yellow), color::Fg(color::Black), "VS", color::Fg(color::Reset));
    for (ix, (_, animal)) in scene.team2.iter().enumerate() {
        render_animal(29 + 4 + (ix as u16) * 5, 9, animal);
    }
    for (ix, (_, animal)) in scene.team1.iter().enumerate() {
        render_animal(29 - 4 - (ix as u16) * 5, 9, animal);
    }
    print!("{}", color::Bg(color::Reset));
}

fn render_animal(x: u16, y: u16, animal: &Animal) {
    print!("{}{}{}", termion::cursor::Goto(x, y), color::Bg(color::Yellow), animal.class.icon);
    print!("{}{}{}{}", termion::cursor::Goto(x - (if animal.health < 10 { 0 } else { 1 }), y+1), color::Bg(color::Green), animal.health, "❤️");
    print!("{}{}{}{}", termion::cursor::Goto(x - (if animal.attack < 10 { 0 } else { 1 }), y+2), color::Bg(color::Green), animal.attack, "⚔️");
}

fn render_background() {
    print!("{}", termion::cursor::Goto(1, 1));
    println!("{}{}", color::Bg(color::Reset), " ".repeat(60));
    println!("{}{}", color::Bg(color::Blue), " ".repeat(60));
    println!("{}{}", color::Bg(color::Blue), " ☁️  ".repeat(15));
    println!("{}{}", color::Bg(color::Blue), " ".repeat(60));
    println!("{}{}", color::Bg(color::Blue), "⛰️ ".repeat(30));
    println!("{}{}{}", color::Bg(color::Green), " ⛰️".repeat(29), "  ");
    println!("{}{}", color::Bg(color::Green), " ".repeat(60));
    println!("{}{}", color::Bg(color::Green), " ".repeat(60));
    println!("{}{}", color::Bg(color::Yellow), " ".repeat(60));
    println!("{}{}", color::Bg(color::Green), " ".repeat(60));
    println!("{}{}", color::Bg(color::Green), " ".repeat(60));
    println!("{}{}", color::Bg(color::Reset), " ".repeat(60));
}

fn render_scene(scene: &Scene) {
    println!("{}", termion::clear::All);
    render_background();
    println!("{}Silly Animals Game", termion::cursor::Goto(1, 1));
    render_teams(scene);
    println!("{}", termion::cursor::Goto(1, 15));
}

fn main() {
    println!("{}{}Silly Animals Game", termion::clear::All, termion::cursor::Goto(1, 1));
    let team1 = beastiary().into_iter().take(5).map(|animal| (Uuid::new_v4(), animal)).collect();
    let team2 = beastiary().into_iter().skip(5).take(5).map(|animal| (Uuid::new_v4(), animal)).collect();
    let mut scene = Scene { team1: team1, team2: team2 };
    let stdin = stdin();
    while scene.get_state() == State::Battling {
        render_scene(&scene);
        stdin.read_line(&mut String::new());
        scene.run_step();
    }
    render_scene(&scene);
}
