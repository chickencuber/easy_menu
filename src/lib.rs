use console::Term;
pub use console::{Key, Style, Color};
use std::cmp::min;
use std::thread;


pub struct Menu {
    pub options: Vec<String>,      
    settings: MenuOptions,     
    pub selected: usize,           
    scroll: usize,             
}

impl Menu {
    pub fn new(options: Vec<String>, settings: MenuOptions) -> Self {
        Self {
            options,
            settings,
            selected: 0,    
            scroll: 0,      
        }
    }

    
    pub fn prompt(prompt: &str) -> String {
        let term = Term::stdout();
        let y = term.size().0 as usize;
        term.move_cursor_to(0, y).unwrap();
        print!("{}: ", prompt);
        term.show_cursor().unwrap();
        let user_input = term.read_line().unwrap(); 
        return user_input; 
    }

    fn get_event(&mut self, term: &Term) -> Event {
        let t = thread::spawn(|| {
            return Term::stdout().read_key().unwrap();
        });
        let e: Event;
        let mut tick: u32 = 0;
        loop {
            if t.is_finished() {
                e = (self.settings.key)(t.join().unwrap(), self);
                term.clear_screen().unwrap();
                break;
            }
            if tick % 1000000 == 0 {
                term.clear_screen().unwrap();
                let (term_height, term_width) = term.size();


                self.render_options(term_width as usize, term_height as usize - 1);
            }
            tick = tick.wrapping_add(1);
        }
        return e;
    }

    pub fn run(&mut self) -> String {
        let term = Term::stdout();  

        term.clear_screen().unwrap();
        let (term_height, term_width) = term.size();


        self.render_options(term_width as usize, term_height as usize - 1);
        (self.settings.start)();
        term.hide_cursor().unwrap();
        let mut canceled = false;
        let mut value = "".to_string();
        loop {
            if self.selected >= self.options.len() && self.selected != 0 {
                self.selected-=1;
                continue;
            }
            if self.options.len() == 0 {
                term.show_cursor().unwrap();   
                term.clear_screen().unwrap();
                return String::from("");
            }
            term.hide_cursor().unwrap();   
            term.clear_screen().unwrap();
            let (term_height, term_width) = term.size();


            self.render_options(term_width as usize, term_height as usize - 1);
            
            let event = self.get_event(&term);
            
            match event {
                Event::Up => self.scroll_up(),
                Event::Down => self.scroll_down(term_height as usize - 1),
                Event::Select => break,  
                Event::Return(v) => {
                    value = v;
                    canceled = true;
                    break;
                }
                Event::None => (),
            }
        }
        term.clear_screen().unwrap();

        term.show_cursor().unwrap();

        if canceled {
            return value;
        }
        
        self.options[self.selected].clone()
    }

    
    fn render_options(&self, term_width: usize, term_height: usize) {
        let visible_options = min(term_height, self.options.len());

        for (i, o) in self.options.iter().enumerate().skip(self.scroll).take(visible_options) {
            let mut option = o.clone();
            option.push_str(" ".repeat(term_width - o.len()).as_str());
            if i == self.selected {
                println!("{}", self.settings.style_selected.apply_to(option));
            } else {
                
                println!("{}", self.settings.style_normal.apply_to(option));
            }
        }
    }

    
    fn scroll_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            if self.selected < self.scroll {
                self.scroll -= 1;
            }
        }
    }

    
    fn scroll_down(&mut self, term_height: usize) {
        if self.selected < self.options.len() - 1 {
            self.selected += 1;
            if self.selected >= self.scroll + term_height {
                self.scroll += 1;
            }
        }
    }
}

pub enum Event {
    Return(String),
    Up,
    Down,
    Select,
    None,
}

pub struct MenuOptions {
    pub style_selected: Style,        
    pub style_normal: Style,          
    pub key: fn(Key, &mut Menu) -> Event,
    pub start: fn(),
}

impl Default for MenuOptions {
    fn default() -> Self {
        Self {
            style_selected: Style::new().bg(Color::Color256(241)),  
            style_normal: Style::new(),
            key: |key, _| -> Event {
               match key {
                   Key::ArrowDown => Event::Down,
                   Key::ArrowUp => Event::Up,
                   Key::Enter => Event::Select,
                   _ => Event::None,
               } 
            },
            start: || {},
        }
    }
}

