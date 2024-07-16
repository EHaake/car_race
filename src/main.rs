use rusty_engine::prelude::*;
use rand::prelude::*;

#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
}

fn main() {
    let mut game = Game::new();

    // Create player sprite
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation.x = -500.0;
    player1.layer = 10.0;
    player1.collision = true;

    // Start background music
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // Create road lines
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    // Create obstacles
    let obstacles_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
    ];

    for (i, preset) in obstacles_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    // Add health message
    let health_message = game.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    // Finish game setup
    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
    });
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Don't run any more logic if game has ended
    if game_state.lost {
        return;
    }

    // Get keyboard input
    let mut direction = 0.0;

    if engine.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]) {
        direction -= 1.0;
    }

    // Move player sprite
    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;

    // bounds checking
    if player1.translation.y < -360.0 || player1.translation.y > 360.0 {
        game_state.health_amount = 0;
    }

    // Move road lines
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;

            // once road line is off screen, move it back to the other end
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;

            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }

    // Handle Collisions
    let health_message = engine.texts.get_mut("health_message").unwrap();
    for event in engine.collision_events.drain(..) {
        // don't care if collision is between two obstacles or the end of collision event
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.4);
        }
    }

    // Detect if game is ended if health_amount is 0
    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.2);
    }
}
