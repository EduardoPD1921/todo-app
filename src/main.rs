use pancurses::*;

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
            Input::Character('i') => {
                ui.insert_mode(&mut todo_list);
            },
            Input::Character('q') => break,
            _ => {}
        }
    }

    endwin();
}

struct UI<'a> {
    window: &'a Window
}

impl<'a> UI<'a> {
    fn new(window: &'a Window) -> Self {
        Self { window }
    }

    fn show_list(&self, todo_list: &Vec<(String, bool)>, curr_todo: usize) {
        self.window.clear();

        if todo_list.len() == 0 {
            self.window.addstr("None todos founded.");
        } else {
            for (index, item) in todo_list.iter().enumerate() {
                self.window.mvaddstr(0, 0, "Todos:");

                let index_with_space_to_title = index + 1;
                let pair_style = if index == curr_todo { HIGHLIGHTED_PAIR as u32 } else { REGULAR_PAIR as u32 };
                let formatted_todo = if item.1 == true {
                    format!("[x] {}", item.0)
                } else {
                    format!("[ ] {}", item.0)
                };

                self.window.attron(COLOR_PAIR(pair_style));
                self.window.mvaddstr(index_with_space_to_title as i32, 0, formatted_todo);
                self.window.attroff(COLOR_PAIR(pair_style));
            }
        }

        self.draw_main_menu_ui();
        self.window.mv(self.window.get_max_y() - 1, self.window.get_max_x() - 1);
    }

    fn insert_mode(&self, todo_list: &mut Vec<(String, bool)>) {
        let mut new_todo = vec![];
        self.window.clear();

        self.draw_insert_menu_ui();

        loop {
            let new_todo_ch = self.window.getch().unwrap();

            match new_todo_ch {
                Input::Character('\n') => {
                    self.window.clear();
                    let new_todo_tuple = (String::from_iter(new_todo), false);
                    todo_list.push(new_todo_tuple);

                    break;
                },
                Input::Character(c) => {
                    new_todo.push(c);
                    self.window.addch(c);
                },
                Input::KeyBackspace => {
                    new_todo.pop();
                    self.window.mv(self.window.get_cur_y(), self.window.get_cur_x() - 1);
                    self.window.delch();
                },
                Input::KeyF1 => break,
                _ => {}
            }
        }
    }

    fn toggle_todo(&self, todo_list: &mut Vec<(String, bool)>, curr_todo: usize) {
        self.window.clear();

        let selected_todo = &todo_list[curr_todo];
        let check_todo = (selected_todo.0.to_owned(), !selected_todo.1);

        todo_list[curr_todo] = check_todo;
    }

    fn draw_main_menu_ui(&self) {
        let max_terminal_y = self.window.get_max_y() - 1;

        self.window.mvaddstr(max_terminal_y, 0, "[i]: insert todo");
        self.window.mvaddstr(max_terminal_y, 20, "[q]: exit");
        self.window.mvaddstr(max_terminal_y, 33, "[w, s]: navigation");
    }

    fn draw_insert_menu_ui(&self) {
        self.window.mvaddstr(0, 0, "Digit your todo:");
        self.window.mvaddstr(self.window.get_max_y() - 1, 0, "[Enter]: insert todo");
        self.window.mvaddstr(self.window.get_max_y() - 1, 23, "[F1]: back");
        self.window.mv(1, 0);
    }
}
