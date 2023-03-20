use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, fs, env};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    n,
    i
}

struct App {
    path: String,
    contents: String,
    row: i32,
    col: i32,
    mode: Mode,
}

impl Default for App {
    fn default() -> App {
        App{
            path: "nil".to_string(),
            contents: String::new(),
            row: 0,
            col: 0,
            mode: Mode::n,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
     
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();
    if args.len() == 1 {
        app.path = "nil".to_string();
    }
    else {
        app.path = format!("{}", args[1]);
    }
    if app.path != "nil".to_string(){
        app.contents = fs::read_to_string(format!("{}", app.path)).expect("could not read file");
    }
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop { 
        if let Event::Key(key) = event::read()? {
            if app.mode == Mode::n {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
                if let KeyCode::Char('w') = key.code {
                    fs::write(format!("{}", app.path), format!("{}", app.contents)).expect("failed to write to file");
                }
                if let KeyCode::Char('i') = key.code {
                    app.mode = Mode::i;
                }
            }
            else if app.mode == Mode::i {
                if let KeyCode::Char(c) = key.code {
                    let mut temp = format!("{}", app.contents);
                    temp = temp + &c.to_string();
                    app.contents = temp.to_string();
                }
                if let KeyCode::Enter = key.code {
                    let mut temp = format!("{}", app.contents);
                    temp = temp + "\n";
                    app.contents = temp.to_string();
                }
                if let KeyCode::Esc = key.code {
                    app.mode = Mode::n;
                }
            }
        } 

        terminal.draw(|f| ui(f, app))?; 
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let vert_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(10), Constraint::Min(3)].as_ref())
        .split(f.size());
    
    let contents = Paragraph::new(format!("{}", app.contents))
        .block(Block::default()
                .title(format!("{}", app.path))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
               )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(contents, vert_chunks[1]);
}
