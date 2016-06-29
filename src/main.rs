extern crate ncurses;
extern crate libc;

use ncurses::*;
use libc::{ SIGINT };

#[test]
fn it_works(){
    assert!(false);
}

struct GameState{
    x: i32,
    y: i32,
    score: i32,
    screen_size: ScreenSize
}

struct ScreenSize{
    width: i32,
    height: i32
}

struct TitlePosition{
    x: i32,
    y: i32
}

impl ScreenSize{
    fn get_title_position(&self, title:&Vec<&'static str>) -> TitlePosition{
        let len = title.len() as i32;
        let str_len = title[0].len() as i32;

        let mid_y = (self.height / 2) - (len / 2);
        let mid_x = (self.width / 2) - (str_len / 2);
        TitlePosition { x: mid_x, y: mid_y}
    }
}

fn print_title(screen:&ScreenSize){
    let title_string:Vec<&'static str> = vec![
        "               Welcome to              ",
        " ",
        ".d8888. d8b   db  .d8b.  db   dD d88888b",
        "88'  YP 888o  88 d8' `8b 88 ,8P' 88'",
        "`8bo.   88V8o 88 88ooo88 88,8P   88ooooo",
        "  `Y8b. 88 V8o88 88~~~88 88`8b   88~~~~~",
        "db   8D 88  V888 88   88 88 `88. 88.",
        "`8888Y' VP   V8P YP   YP YP   YD Y88888P",
        " ",
        "      Press the any key to continue"                                       
    ];

    let pos = screen.get_title_position(&title_string);
    
    for (i, line) in title_string.iter().enumerate() {
        let curr = i as i32;
        let line_num = pos.y + curr;
        mvprintw(line_num, pos.x, line);
    }
}

fn get_screen_size() -> ScreenSize {
    let mut max_x:i32 = 0;
    let mut max_y:i32 = 0;
    getmaxyx(stdscr, &mut max_y, &mut max_x);
    ScreenSize { width: max_x, height: max_y}
}

fn do_title_screen(screen_size:&ScreenSize){
    start_color();
    init_pair(1,COLOR_BLUE,COLOR_BLACK);
    keypad(stdscr,true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    print_title(screen_size);
}

fn init_game_state(size:ScreenSize) -> GameState{
    GameState { x: 0, y: 0, score: 0, screen_size: size}
}

fn render(state:GameState) {
}

fn main() {
    
    /* Start ncurses. */
    initscr();
    let screen_size = get_screen_size();

    do_title_screen(&screen_size);

    /* Update the screen. */
    refresh();

    /* Wait for a key press. */
    getch();

    let state = init_game_state(screen_size);

    render(state);

    /* Terminate ncurses. */
    endwin();
}
