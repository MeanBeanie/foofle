use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, fs, env, time, time::Instant};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
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
    row: i64,
    col: i64,
    mode: Mode,
    start_time: Instant, 
}

impl Default for App {
    fn default() -> App {
        App{
            path: "nil".to_string(),
            contents: String::new(),
            row: 0,
            col: 0,
            mode: Mode::n,
            start_time: Instant::now(),
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

fn strToArray(input: &str) -> Vec< Vec<char> > {
    let mut result = Vec::new();
    let mut temp = Vec::new();

    // iterates through the input string and adds each line to the output array
    for c in input.chars() {
        if c == '\n' {
            result.push(temp.clone());
            temp.clear();
        }
        else{
            temp.push(c);
        }
    }
    result.push(temp); // grabs end of line in case there is no \n at end

    return result;
}

fn arrayToStr(input: Vec< Vec<char> >) -> String {
    let mut result = String::new();
   
    // loops through input array and appends each char to the output string
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let mut temp = format!("{}", result);
            temp = temp + &input[i][j].to_string();
            result = temp.to_string();
        }
        let mut temp = format!("{}", result);
        temp = temp + "\n";
        result = temp.to_string();
    }

    return result;
}

fn arrayToSpans(col: i64, row: i64, input: Vec< Vec<char> >) -> Vec<Spans<'static>> {
    let mut result_real = Vec::new();
    let mut result = Vec::new();
    for i in 0..input.len(){
        for j in 0..input[i].len(){
            if col as usize == i && row as usize == j {
                result.push(Span::styled(input[i][j].to_string(), Style::default().add_modifier(Modifier::BOLD)));
            }
            else{
                result.push(Span::raw(input[i][j].to_string()));
            }
        }
        result_real.push(Spans::from(result.clone()));
        result.clear();
    }
    
    return result_real;
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
                if let KeyCode::Char('j') = key.code {
                    if app.col - 1 >= 0 {
                        app.col -= 1;
                    }
                    else{
                        app.col = 0;
                    }
                }
                if let KeyCode::Char('k') = key.code {
                    if app.col + 1 < (strToArray(&app.contents).len() as i64) {
                        app.col += 1;
                    }
                    else {
                        app.col = (strToArray(&app.contents).len() - 1) as i64;
                    }
                }
                if let KeyCode::Char('l') = key.code {
                    if app.row + 1 < (strToArray(&app.contents)[app.col as usize].len() as i64) {
                        app.row += 1;
                    }
                    else {
                        app.row = (strToArray(&app.contents)[app.col as usize].len() - 1) as i64;
                    }
                }

                if let KeyCode::Char('h') = key.code {
                    if app.row - 1 >= 0 {
                        app.row -= 1;
                    }
                    else {
                        app.row = 0;
                    }
                } 
                if let KeyCode::Char('i') = key.code {
                    app.mode = Mode::i;
                }
            }
            else if app.mode == Mode::i {
                if let KeyCode::Char(c) = key.code {
                    let mut contentsArray = strToArray(&format!("{}", app.contents));
                    for i in 0..contentsArray.len() {
                        'r: for r in 0..contentsArray[i].len() {
                            if i == (app.col as usize) {
                                if r == (app.row as usize) {
                                    contentsArray[i].insert((app.row as usize), c);
                                    break 'r;
                                } 
                            }
                        }
                    }
                    app.contents = arrayToStr(contentsArray);
                    app.row += 1;
                }
                if let KeyCode::Enter = key.code {
                    let mut temp = format!("{}", app.contents);
                    temp = temp + "\n";
                    app.contents = temp.to_string();
                }
                if let KeyCode::Backspace = key.code {
                    let mut temp = strToArray(&format!("{}", app.contents));
                    if(app.row - 1 >= 0){
                        temp[app.col as usize].remove((app.row - 1) as usize);
                        app.row -= 1;
                    }
                    app.contents = arrayToStr(temp);
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
    let mut secs = app.start_time.elapsed().as_secs();
    
    let vert_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(10), Constraint::Min(3)].as_ref())
        .split(f.size());
    
    let contents = Paragraph::new(arrayToSpans(app.col, app.row, strToArray(&format!("{}", app.contents))))
        .block(Block::default()
                .title(format!("{}-|{}:{}|-[{}]", app.path, app.col, app.row, secs))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
               )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(contents, vert_chunks[1]);
}
