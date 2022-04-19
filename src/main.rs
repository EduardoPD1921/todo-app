mod interface;

use pancurses::*;
use interface::UI;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHTED_PAIR: i16 = 1;

fn main() {
    let window = initscr();
    let ui = UI::new(&window);
    window.keypad(true);
    noecho();

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHTED_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut todo_list: Vec<(String, bool)> = Vec::new();
    let mut curr_todo = 0;

    UI::read_saved_todo(&mut todo_list);

    loop {
        ui.show_list(&todo_list, curr_todo);
    
        let user_action = window.getch().unwrap();
    
        match user_action {
            Input::Character('w') => if curr_todo > 0 {
                curr_todo -= 1;
            },
            Input::Character('s') => {
                if todo_list.len() != 0 && curr_todo < todo_list.len() - 1 {
                    curr_todo += 1;
                }
            },
            Input::Character('\n') => {
                ui.toggle_todo(&mut todo_list, curr_todo);
            },
            Input::KeyF2 => {
                ui.delete_todo(&mut todo_list, &mut curr_todo);
            }
            Input::Character('i') => {
                ui.insert_mode(&mut todo_list);
            },
            Input::Character('e') => {
                ui.edit_mode(&mut todo_list, curr_todo);
            },
            Input::Character('q') => {
                ui.save_and_close(todo_list);
                break;
            },
            _ => {}
        }
    }

    endwin();
}
