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

fn place(vector: &mut Vec<ImageButton>, spot: usize, png: &str, texturem: &TextureManager) {
    vector[spot].set_preload(
        texturem.get_preload(png).unwrap(),
        texturem.get_preload(png).unwrap(),
    );
    vector[spot].enabled = false;
}


fn first_empty(vector: &Vec<ImageButton>, spots: [usize; 4]) -> usize {
    for i in 0..4 {
        if vector[spots[i]].get_filename() == "assets/white.png" {
            return spots[i];
        }
    }
    42
}

fn fork_count(vector: &Vec<ImageButton>, lines: &[[usize; 3]; 8], i: usize) -> i32 {
    let mut n = 0;
    for l in 0..8 {
        if lines[l].contains(&i) {
            let mut x = 0;
            let mut empty = 0;
            for k in 0..3 {
                if vector[lines[l][k]].get_filename() == "assets/x.png" {
                    x += 1;
                } else if vector[lines[l][k]].get_filename() == "assets/white.png" {
                    empty += 1;
                }
            }
            if x == 1 && empty == 2 {
                n += 1;
            }
        }
    }
    n
}

fn find_spot(vector: &Vec<ImageButton>, lines: &[[usize; 3]; 8], mark: &str) -> usize {
    for l in 0..8 {
        let spot = line_spot(vector, lines[l][0], lines[l][1], lines[l][2], mark);
        if spot != 42 {
            return spot;
        }
    }
    42
}

fn line_spot(vector: &Vec<ImageButton>, a: usize, b: usize, c: usize, mark: &str) -> usize {
    if vector[a].get_filename() == mark && vector[b].get_filename() == mark && vector[c].get_filename() == "assets/white.png" {
        return c;
    } else if vector[a].get_filename() == mark && vector[c].get_filename() == mark && vector[b].get_filename() == "assets/white.png" {
        return b;
    } else if vector[b].get_filename() == mark && vector[c].get_filename() == mark && vector[a].get_filename() == "assets/white.png" {
        return a;
    }
    42
}

fn randommove(vector: &mut Vec<ImageButton>, player: bool, texturem: &TextureManager) {
    rand::srand(date::now() as u64);
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
        
    }
}

fn ai_move(ai_diff: i32, vector: &mut Vec<ImageButton>, player: bool, texturem: &TextureManager) {
    let lines: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8],
        [0, 4, 8], [2, 4, 6],
    ];

    if ai_diff == 0 {
        randommove(vector, player, texturem);
        return;
    }
 
    let mut spot = find_spot(vector, &lines, "assets/o.png");
    if spot == 42 {
        spot = find_spot(vector, &lines, "assets/x.png");
    }

    if ai_diff == 1 {
        if spot == 42 {
            randommove(vector, player, texturem);
        } else {
            place(vector, spot, "assets/o.png", texturem);
        }
        return;
    }

    let mut forks = 0;
    let mut first_fork = 42;
    for i in 0..9 {
        if spot == 42 && vector[i].get_filename() == "assets/white.png" && fork_count(vector, &lines, i) >= 2 {
            forks += 1;
            if first_fork == 42 {
                first_fork = i;
            }
        }
    }
    if forks == 1 {
        spot = first_fork;
    } else if forks > 1 {
        if vector[4].get_filename() == "assets/o.png" {
            spot = first_empty(vector, [1, 3, 5, 7]);
        } else {
            spot = first_empty(vector, [0, 2, 6, 8]);
        }
    }

    if spot == 42 && vector[4].get_filename() == "assets/white.png" {
        spot = 4;
    }
    if spot == 42 {
        spot = first_empty(vector, [0, 2, 6, 8]);
    }
    if spot == 42 {
        spot = first_empty(vector, [1, 3, 5, 7]);
    }

    place(vector, spot, "assets/o.png", texturem);
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
    if btn[0].get_filename() != "assets/white.png"
        && btn[1].get_filename() != "assets/white.png"
        && btn[2].get_filename() != "assets/white.png"
        && btn[3].get_filename() != "assets/white.png"
        && btn[4].get_filename() != "assets/white.png"
        && btn[5].get_filename() != "assets/white.png"
        && btn[6].get_filename() != "assets/white.png"
        && btn[7].get_filename() != "assets/white.png"
        && btn[8].get_filename() != "assets/white.png"
    {
        return 3;
    }
    0
}

fn reset(btn: &mut Vec<ImageButton>, player: &mut bool, check: &mut bool, lbl_player: &mut Label, tm: &TextureManager, m_count: &mut i32) {
    for i in 0..btn.len() {
        btn[i].set_preload(tm.get_preload("assets/white.png").unwrap(), tm.get_preload("assets/white.png").unwrap());
        btn[i].enabled = true;
        lbl_player.set_text("Player: X");
        *player = true;
        *check = true;
        *m_count = 0;
    }
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

    let mut human_bot = MessageBox::custom("Bot or Player ?", "Select Human or Bot", vec!["Huaman", "Bot"], Some(0));
    let mut ai_diff = MessageBox::custom(
        "Choose Difficulty",
        "Select game difficulty:",
        vec!["Easy", "Normal", "Impossible"],
        Some(1), // Default to "Normal"
    );
    let mut btn_reset = TextButton::new(250.0, 700.0, 150.0, 50.0, "Reset", GRAY, RED, 30);
    let mut btn_exit = TextButton::new(650.0, 700.0, 150.0, 50.0, "Exit", GRAY, RED, 30);
    let btn_modeswitch = TextButton::new(50.0, 600.0, 150.0, 50.0, "Change Mode", GRAY, PURPLE, 30);
    let mut btn_settings = TextButton::new(50.0, 700.0, 150.0, 50.0, "Settings", GRAY, PURPLE, 30);
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
    let mut show_dialog_settings = false;
    let mut show_dialog_humanbot = false;
    let mut m_count = 0;
    btn_reset.with_hover_text_color(BLACK);
    btn_exit.with_hover_text_color(BLACK);
    loop {
        clear_background(WHITE);
        if onstart == true {
            for i in 0..9 {
                btn[i].enabled = false;
            }
            human_bot.show();
            if let Some(result) = human_bot.draw() {
                // Only runs when a button was clicked or dialog was closed
                match result {
                    MessageBoxResult::ButtonPressed(0) => {
                        for i in 0..9 {
                            btn[i].enabled = true;
                        }
                        onstart = false;
                        bot = false;
                    }
                    MessageBoxResult::ButtonPressed(1) => {
                        bot = true;
                        ai_diff_int = 0;
                        for i in 0..9 {
                            btn[i].enabled = true;
                        }
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
        if bot == false {
            btn_settings.enabled = false;
        } else {
            btn_settings.enabled = true;
        }
        lbl_score.set_text(&format!("Score X: {} O: {} Tie: {}", xscore, oscore, ties));

        for i in 0..btn.len() {
            if btn[i].click() {
                if bot == true {
                    if player == true {
                        m_count += 1;
                        btn[i].set_preload(tm.get_preload("assets/x.png").unwrap(), tm.get_preload("assets/x.png").unwrap());
                        lbl_player.set_text("Player: O");
                        btn[i].enabled = false;
                        if checkwinner(&btn) == 3 {
                            ties += 1;
                            check = false;
                        } else if checkwinner(&btn) == 1 {
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
                } else {
                    if player == true {
                        btn[i].set_preload(tm.get_preload("assets/x.png").unwrap(), tm.get_preload("assets/x.png").unwrap());
                        lbl_player.set_text("Player: O");
                        btn[i].enabled = false;
                        player = false;
                    } else {
                        btn[i].set_preload(tm.get_preload("assets/o.png").unwrap(), tm.get_preload("assets/o.png").unwrap());
                        lbl_player.set_text("Player: X");
                        btn[i].enabled = false;
                        player = true;
                    }
                    if checkwinner(&btn) == 1 {
                        xscore += 1;
                        check = false;
                    } else if checkwinner(&btn) == 2 {
                        oscore += 1;
                        check = false;
                    }
                    if check == false {
                        for i in 0..btn.len() {
                            btn[i].enabled = false;
                        }
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
            reset(&mut btn, &mut player, &mut check, &mut lbl_player, &tm, &mut m_count);
        }
        if btn_modeswitch.click() {
            show_dialog_humanbot = true;
            human_bot.show();
        }
        if show_dialog_humanbot == true {
            if let Some(result) = human_bot.draw() {
                // Only runs when a button was clicked or dialog was closed
                match result {
                    MessageBoxResult::ButtonPressed(0) => {
                        reset(&mut btn, &mut player, &mut check, &mut lbl_player, &tm, &mut m_count);
                        bot = false;
                    }
                    MessageBoxResult::ButtonPressed(1) => {
                        reset(&mut btn, &mut player, &mut check, &mut lbl_player, &tm, &mut m_count);
                        bot = true;
                        ai_diff_int = 0;
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
        if btn_settings.click() {
            show_dialog_settings = true;
            ai_diff.show();
        }
        if show_dialog_settings == true {
            if let Some(result) = ai_diff.draw() {
                // Only runs when a button was clicked or dialog was closed
                match result {
                    MessageBoxResult::ButtonPressed(0) => {
                        ai_diff_int = 0;
                        bot = true;
                        reset(&mut btn, &mut player, &mut check, &mut lbl_player, &tm, &mut m_count);
                    }
                    MessageBoxResult::ButtonPressed(1) => {
                        bot = true;
                        ai_diff_int = 1;
                        reset(&mut btn, &mut player, &mut check, &mut lbl_player, &tm, &mut m_count);
                        m_count = 0;
                    }
                    MessageBoxResult::ButtonPressed(2) => {
                        bot = true;
                        ai_diff_int = 2;
                        reset(&mut btn, &mut player, &mut check, &mut lbl_player, &tm, &mut m_count);
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
        if btn_exit.click() {
            break;
        }
        //draw_grid(50.0, BROWN);
        human_bot.draw();
        ai_diff.draw();
        lbl_score.draw();
        lbl_player.draw();
        next_frame().await;
    }
}
