use pancurses::{Input, COLOR_PAIR};

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;
use std::path::Path;
use pancurses::Window;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHTED_PAIR: i16 = 1;

pub struct UI<'a> {
    window: &'a Window
}

impl<'a> UI<'a> {
    pub fn new(window: &'a Window) -> Self {
        Self { window }
    }

    pub fn show_list(&self, todo_list: &Vec<(String, bool)>, curr_todo: usize) {
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

    pub fn insert_mode(&self, todo_list: &mut Vec<(String, bool)>) {
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

    pub fn edit_mode(&self, todo_list: &mut Vec<(String, bool)>, curr_todo: usize) {
        if todo_list.len() > 0 {
            self.window.clear();
            self.draw_edit_menu_ui();
    
            let selected_todo = &todo_list[curr_todo];
            let mut sel_todo_as_vec: Vec<char> = selected_todo.0.chars().collect();
            let is_checked = selected_todo.1;
    
            self.window.mvaddstr(1, 0, &selected_todo.0);
    
            loop {
                let user_action = self.window.getch().unwrap();
    
                match user_action {
                    Input::Character('\n') => {
                        let edited_todo_str = String::from_iter(&sel_todo_as_vec);
                        let edited_todo_tp = (edited_todo_str, is_checked);
    
                        todo_list.remove(curr_todo);
                        todo_list.insert(curr_todo, edited_todo_tp);
                        break;
                    },
                    Input::Character(c) => {
                        sel_todo_as_vec.push(c);
                        self.window.addch(c);
                    },
                    Input::KeyBackspace => {
                        sel_todo_as_vec.pop();
                        self.window.mv(self.window.get_cur_y(), self.window.get_cur_x() - 1);
                        self.window.delch();
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn toggle_todo(&self, todo_list: &mut Vec<(String, bool)>, curr_todo: usize) {
        self.window.clear();

        let selected_todo = &todo_list[curr_todo];
        let check_todo = (selected_todo.0.to_owned(), !selected_todo.1);

        todo_list[curr_todo] = check_todo;
    }

    pub fn delete_todo(&self, todo_list: &mut Vec<(String, bool)>, curr_todo: &mut usize) {
        if todo_list.len() > 0 {
            todo_list.remove(*curr_todo);
    
            if *curr_todo != 0 {
                *curr_todo -= 1;
            }
        }
    }

    pub fn save_and_close(self, todo_list: Vec<(String, bool)>) {
        let file = File::create("todo.txt").unwrap();
        let mut file = LineWriter::new(file);

        for (mut todo, is_checked) in todo_list {
            if is_checked {
                todo.push_str(";true\n");
            } else {
                todo.push_str(";false\n");
            }

            file.write_all(todo.as_bytes()).unwrap();
        }

        file.flush().unwrap();
    }

    pub fn read_saved_todo(todo_list: &mut Vec<(String, bool)>) {
        let is_todo_file_created = Path::new("todo.txt").exists();

        if is_todo_file_created {
            let file_buf = fs::read_to_string("todo.txt").unwrap();

            for line in file_buf.lines() {
                let mut chunk = line.split(';');

                let todo = chunk.next().unwrap();
                let is_checked = chunk.next().unwrap();

                if is_checked == "true" {
                    todo_list.push((todo.to_owned(), true));
                } else if is_checked == "false" {
                    todo_list.push((todo.to_owned(), false));
                }
            }
        }
    }

    fn draw_main_menu_ui(&self) {
        let max_terminal_y = self.window.get_max_y() - 1;

        self.window.mvaddstr(max_terminal_y, 0, "[i]: insert todo");
        self.window.mvaddstr(max_terminal_y, 19, "[F2]: delete todo");
        self.window.mvaddstr(max_terminal_y, 39, "[e]: edit todo");
        self.window.mvaddstr(max_terminal_y, 56, "[w s]: navigation");
        self.window.mvaddstr(max_terminal_y, 76, "[q]: Save and exit");
    }

    fn draw_insert_menu_ui(&self) {
        self.window.mvaddstr(0, 0, "Digit your todo:");
        self.window.mvaddstr(self.window.get_max_y() - 1, 0, "[Enter]: insert todo");
        self.window.mvaddstr(self.window.get_max_y() - 1, 23, "[F1]: back");
        self.window.mv(1, 0);
    }

    fn draw_edit_menu_ui(&self) {
        self.window.mvaddstr(0, 0, "Edit your todo:");
        self.window.mvaddstr(self.window.get_max_y() - 1, 0, "[Enter]: save edit");
        self.window.mvaddstr(self.window.get_max_y() - 1, 23, "[F1]: back");
    }
}