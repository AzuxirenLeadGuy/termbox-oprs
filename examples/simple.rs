use termbox_oprs::KeyType;

struct HelloWorldApp {
    width: i32,
    height: i32,
}

impl HelloWorldApp {
    fn draw(&self, runtime: &termbox_oprs::Runtime) {
        runtime.clear();
        let string_val = "Hello from Termbox Rust";
        if (self.width as usize) < string_val.len() || self.height < 1 {
            return;
        }
        runtime.write_cells(0, 0, string_val, 7, 2);
        runtime.present();
    }
    fn new() -> Self {
        HelloWorldApp {
            width: 0,
            height: 0,
        }
    }
}

impl termbox_oprs::Application for HelloWorldApp {
    fn run(&mut self, runtime: &termbox_oprs::Runtime) -> Result<(), i32> {
        let size = runtime.size();
        match size {
            Some(value) => {
                self.width = value.0;
                self.height = value.1;
            }
            None => return Err(1),
        }
        self.draw(runtime);
        loop {
            match runtime.poll_event() {
                termbox_oprs::Event::Key {
                    modifier: _,
                    keycode,
                    character: _,
                } => {
                    if keycode == KeyType::Escape as u16 {
                        break;
                    }
                }
                termbox_oprs::Event::Resize {
                    new_width,
                    new_height,
                } => {
                    self.width = new_width;
                    self.height = new_height;
                    self.draw(runtime);
                }
                _ => {}
            }
        }
        return Ok(());
    }
}

fn main() -> Result<(), i32> {
    termbox_oprs::Runtime::launch(HelloWorldApp::new())
}
