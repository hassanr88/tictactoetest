/*
By: Hassan Rana
Date: 2026-10-07
Program Details: a fully functional single-player Tic-Tac-Toe game where the user competes against the computer. The program meets the following requirements:


Clearly display who wins each game.


Prevent the player from changing an already selected spot (X or O) on the board.


Implement computer logic that attempts to win or block the player from winning.


Use a single, consistent method to handle moves for all spots on the board (no separate methods per position).


Keep track of and display the running score of wins for both the player and the computer.
*/

mod ui;
mod utils;
use crate::ui::image_button::ImageButton;
use crate::ui::label::Label;
use crate::ui::messagebox::{MessageBox, MessageBoxResult};
use crate::ui::text_button::TextButton;
use crate::utils::preload_image::TextureManager;
use macroquad::prelude::*;
use miniquad::date;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_string(),
        window_width: 1050,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

fn ai_move(ai_diff: i32, vector: &mut Vec<ImageButton>, player: bool, texturem: &TextureManager) {
    rand::srand(date::now() as u64);
    if ai_diff == 0 {
        for i in 0..9 {
            let mut random_int = rand::gen_range(0, 9);
            while vector[random_int].get_filename() != "assets/white.png" {
                random_int = rand::gen_range(0, 9);
            }
            if vector[random_int].get_filename() == "assets/white.png" {
                if player == true {
                    vector[random_int].set_preload(
                        texturem.get_preload("assets/o.png").unwrap(),
                        texturem.get_preload("assets/o.png").unwrap(),
                    );
                } else {
                    vector[random_int].set_preload(
                        texturem.get_preload("assets/x.png").unwrap(),
                        texturem.get_preload("assets/x.png").unwrap(),
                    );
                }
                vector[random_int].enabled = false;
                break;
            }
        }
    } else if ai_diff == 1 {
    } else {
    }
}

fn checkwinner(btn: &Vec<ImageButton>) -> i32 {
    for i in (0..btn.len()).step_by(3) {
        if btn[i].get_filename() != "assets/white.png"
            && btn[i].get_filename() == btn[i + 1].get_filename()
            && btn[i + 1].get_filename() == btn[i + 2].get_filename()
        {
            if btn[i].get_filename() == "assets/x.png" {
                return 1;
            } else {
                return 2;
            }
        }
        for i in 0..3 {
            if btn[i].get_filename() != "assets/white.png"
                && btn[i].get_filename() == btn[i + 3].get_filename()
                && btn[i + 3].get_filename() == btn[i + 6].get_filename()
            {
                if btn[i].get_filename() == "assets/x.png" {
                    return 1;
                } else {
                    return 2;
                }
            }
        }
        if btn[4].get_filename() != "assets/white.png" {
            if btn[0].get_filename() == btn[4].get_filename() && btn[4].get_filename() == btn[8].get_filename() {
                if btn[4].get_filename() == "assets/x.png" {
                    return 1;
                } else {
                    return 2;
                }
            }

            if btn[2].get_filename() == btn[4].get_filename() && btn[4].get_filename() == btn[6].get_filename() {
                if btn[4].get_filename() == "assets/x.png" {
                    return 1;
                } else {
                    return 2;
                }
            }
        }
    }
    0
}

#[macroquad::main(window_conf)]
async fn main() {
    let tm = TextureManager::new();
    tm.preload_all(&["assets/o.png", "assets/x.png", "assets/white.png"]).await;
    let mut btn: Vec<ImageButton> = vec![
        ImageButton::from_preload(
            250.0,                                       // x position
            50.0,                                        // y position
            150.0,                                       // width
            150.0,                                       // height
            tm.get_preload("assets/white.png").unwrap(), // normal state image
            tm.get_preload("assets/white.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            450.0,                                       // x position
            50.0,                                        // y position
            150.0,                                       // width
            150.0,                                       // height
            tm.get_preload("assets/white.png").unwrap(), // normal state image
            tm.get_preload("assets/white.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            650.0,                                       // x position
            50.0,                                        // y position
            150.0,                                       // width
            150.0,                                       // height
            tm.get_preload("assets/white.png").unwrap(), // normal state image
            tm.get_preload("assets/white.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            250.0,                                       // x position
            250.0,                                       // y position
            150.0,                                       // width
            150.0,                                       // height
            tm.get_preload("assets/white.png").unwrap(), // normal state image
            tm.get_preload("assets/white.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            450.0,                                       // x position
            250.0,                                       // y position
            150.0,                                       // width
            150.0,                                       // height
            tm.get_preload("assets/white.png").unwrap(), // normal state image
            tm.get_preload("assets/white.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            650.0,                                       // x position
            250.0,                                       // y position
            150.0,                                       // width
            150.0,                                       // height
            tm.get_preload("assets/white.png").unwrap(), // normal state image
            tm.get_preload("assets/white.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            250.0,                                       // x position
            450.0,                                       // y position
            150.0,                                       // width
            150.0,                                       // height
            tm.get_preload("assets/white.png").unwrap(), // normal state image
            tm.get_preload("assets/white.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            450.0,                                       // x position
            450.0,                                       // y position
            150.0,                                       // width
            150.0,                                       // height
            tm.get_preload("assets/white.png").unwrap(), // normal state image
            tm.get_preload("assets/white.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            650.0,                                       // x position
            450.0,                                       // y position
            150.0,                                       // width
            150.0,                                       // height
            tm.get_preload("assets/white.png").unwrap(), // normal state image
            tm.get_preload("assets/white.png").unwrap(), // hover state image
        ),
    ];

    let mut human_bot = MessageBox::custom("Bot or Player 2?", "Select Human or Bot", vec!["Huaman", "Bot"], Some(0));
    let mut ai_diff = MessageBox::custom(
        "Choose Difficulty",
        "Select game difficulty:",
        vec!["Easy", "Normal", "Impossible"],
        Some(1), // Default to "Normal"
    );
    let mut btn_reset = TextButton::new(250.0, 700.0, 150.0, 50.0, "Reset", GRAY, RED, 30);
    let mut btn_exit = TextButton::new(650.0, 700.0, 150.0, 50.0, "Exit", GRAY, RED, 30);
    let mut player = true;
    let mut lbl_score = Label::new("Score: X: 0 O: 0 Tie: 0", 380.0, 650.0, 30);
    let mut lbl_player = Label::new("Player: X", 464.0, 750.0, 30);
    let mut xscore = 0;
    let mut oscore = 0;
    let mut ties = 0;
    let mut ai_diff_int = 0;
    let mut check = true;
    let mut onstart = true;
    let mut bot = false;
    let mut choosing_difficulty = false;
    btn_reset.with_hover_text_color(BLACK);
    btn_exit.with_hover_text_color(BLACK);
    loop {
        clear_background(WHITE);
        if onstart == true {
            human_bot.show();
            if let Some(result) = human_bot.draw() {
                // Only runs when a button was clicked or dialog was closed
                match result {
                    MessageBoxResult::ButtonPressed(0) => {
                        onstart = false;
                        bot = false;
                    }
                    MessageBoxResult::ButtonPressed(1) => {
                        bot = true;
                        ai_diff.show();
                        choosing_difficulty = true;
                    }
                    #[allow(unused)]
                    MessageBoxResult::ButtonPressed(_) => {
                        // IMPORTANT: This catch-all pattern is required by the Rust compiler
                        // even for simple confirm dialogs to ensure all possible values are covered
                    }
                    MessageBoxResult::Closed => {}
                }
            }
            if choosing_difficulty {
                ai_diff.show();
                if let Some(result) = ai_diff.draw() {
                    // Only runs when a button was clicked or dialog was closed
                    match result {
                        MessageBoxResult::ButtonPressed(0) => {
                            ai_diff_int = 0;
                            choosing_difficulty = false;
                            onstart = false;
                        }
                        MessageBoxResult::ButtonPressed(1) => {
                            ai_diff_int = 1;
                            choosing_difficulty = false;
                            onstart = false;
                        }
                        MessageBoxResult::ButtonPressed(2) => {
                            ai_diff_int = 2;
                            choosing_difficulty = false;
                            onstart = false;
                        }
                        #[allow(unused)]
                        MessageBoxResult::ButtonPressed(_) => {
                            // IMPORTANT: This catch-all pattern is required by the Rust compiler
                            // even for simple confirm dialogs to ensure all possible values are covered
                        }
                        MessageBoxResult::Closed => {}
                    }
                }
            }
        }

        lbl_score.set_text(&format!("Score X: {} O: {} Tie: {}", xscore, oscore, ties));

        for i in 0..btn.len() {
            if btn[i].click() {
                if bot == true {
                    if player == true {
                        btn[i].set_preload(tm.get_preload("assets/x.png").unwrap(), tm.get_preload("assets/x.png").unwrap());
                        lbl_player.set_text("Player: O");
                        btn[i].enabled = false;
                        if checkwinner(&btn) == 1 {
                            xscore += 1;
                            check = false;
                        } else if checkwinner(&btn) == 0 {
                            ai_move(ai_diff_int, &mut btn, true, &tm);

                            if checkwinner(&btn) == 2 {
                                oscore += 1;
                                check = false;
                            }
                        } else if checkwinner(&btn) == 2 {
                            oscore += 1;
                            check = false;
                        }
                        if check == false {
                            for i in 0..btn.len() {
                                btn[i].enabled = false;
                            }
                        }
                        lbl_player.set_text("Player: X");
                    } else {
                        btn[i].set_preload(tm.get_preload("assets/o.png").unwrap(), tm.get_preload("assets/o.png").unwrap());
                        ai_move(ai_diff_int, &mut btn, false, &tm);
                        lbl_player.set_text("Player: X");
                    }
                }
                if check == true
                    && btn[0].get_filename() != "assets/white.png"
                    && btn[1].get_filename() != "assets/white.png"
                    && btn[2].get_filename() != "assets/white.png"
                    && btn[3].get_filename() != "assets/white.png"
                    && btn[4].get_filename() != "assets/white.png"
                    && btn[5].get_filename() != "assets/white.png"
                    && btn[6].get_filename() != "assets/white.png"
                    && btn[7].get_filename() != "assets/white.png"
                    && btn[8].get_filename() != "assets/white.png"
                {
                    ties += 1;
                    check = false;
                }
            }
        }
        if btn_reset.click() {
            for i in 0..btn.len() {
                btn[i].set_image("assets/white.png", "assets/white.png").await;
                btn[i].enabled = true;
                lbl_player.set_text("Player: X");
                player = true;
                check = true;
            }
        }
        if btn_exit.click() {
            break;
        }
        human_bot.draw();
        ai_diff.draw();
        lbl_score.draw();
        lbl_player.draw();
        next_frame().await;
    }
}
