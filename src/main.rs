use simple_menu::{Menu, MenuOptions, Style, Key, Event};


fn main() {
    let mut menu = Menu::new(vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
        "f".to_string(),
    ], MenuOptions {
        key: handle_key,
        ..Default::default()
    });
    println!("{}", menu.run());
}

fn handle_key(key:Key, menu: &mut Menu) -> Event {
    match key {
        Key::Char('j') => Event::Down,
        Key::Char('k') => Event::Up,
        Key::Enter => Event::Select,
        Key::Char('a') => {
            menu.options.push(Menu::prompt("add"));
            return Event::None;
        }
        Key::Char('d') => {
            menu.options.remove(menu.selected);
            return Event::None;
        }
        _ => Event::None, 
    } 
}

