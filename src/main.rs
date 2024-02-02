mod game;

use std::{collections::HashSet, time::Duration};

use game::{
    context::{GameContext, PlayerDirection},
    domain::{H_SIZE, PIXEL_SIZE, W_SIZE},
    renderer::Renderer,
};
use sdl2::{event::Event, keyboard::Keycode};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-invaders", W_SIZE * PIXEL_SIZE, H_SIZE * PIXEL_SIZE)
        // .position(-1300, 280)
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;

    let mut context = GameContext::new();

    let mut renderer = Renderer::new(window)?;

    let mut frame_counter: i32 = 0;

    let mut enemy_speed: i32 = 15;
    let mut enemy_tick: i32 = 0;
    let mut key_pressed: HashSet<Keycode> = HashSet::new();

    'game_loop: loop {
        for e in events.poll_iter() {
            match e {
                Event::Quit { .. } => {
                    break 'game_loop;
                }
                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => {
                    key_pressed.insert(key);
                }
                Event::KeyUp {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => {
                    key_pressed.remove(&key);
                }
                _ => {}
            }
        }

        if key_pressed.contains(&Keycode::D) {
            context.move_player(PlayerDirection::Right)
        }

        // Verificar se a tecla "A" está pressionada
        if key_pressed.contains(&Keycode::A) {
            context.move_player(PlayerDirection::Left)
        }

        enemy_tick += 1;
        if enemy_speed < enemy_tick {
            enemy_tick = 0;
            context.move_enemies();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        frame_counter += 1;

        if frame_counter % 10 == 0 {
            context.tick();
            frame_counter = 0;
        }

        renderer.draw(&context)?;
    }

    Ok(())
}
