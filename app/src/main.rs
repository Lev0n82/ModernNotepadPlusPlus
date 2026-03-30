use iced::{
    Application, Settings, Theme, executor, Command, Element, Length, Color,
    widget::{column, row, text, button, text_input, container, Space, Rule},
    alignment, Background, Font,
};
use rfd::FileDialog;

// Core modules
use app::core::buffer::TextBuffer;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    // Modern apps default to elegant sizes
    settings.window.size = (1100, 800);
    ModernNotepad::run(settings)
}

struct ModernNotepad {
    open_files: Vec<String>,
    buffer: TextBuffer,
    status: String,
    selected_tab: usize,
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(usize),
    ContentChanged(String),
    Undo,
    Redo,
    Open,
    Save,
}

impl Application for ModernNotepad {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            ModernNotepad {
                open_files: vec!["Untitled.rs".to_string()],
                buffer: TextBuffer::new(),
                status: "Idle".to_string(),
                selected_tab: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Modern Notepad++ – {}", self.open_files.get(self.selected_tab).unwrap_or(&"Untitled".to_string()))
    }

    // Force Dark Mode to achieve a sleek, premium development vibe
    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TabSelected(idx) => {
                self.selected_tab = idx;
            }
            Message::ContentChanged(new) => {
                self.buffer.delete_range(0, self.buffer.content.len());
                self.buffer.insert(0, &new);
                self.status = "Editing...".to_string();
            }
            Message::Undo => {
                self.buffer.undo();
                self.status = "Restored previous state".to_string();
            }
            Message::Redo => {
                self.buffer.redo();
                self.status = "Redo applied".to_string();
            }
            Message::Open => {
                if let Some(path) = FileDialog::new().pick_file() {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        self.buffer = TextBuffer::new();
                        self.buffer.insert(0, &content);
                        self.open_files[0] = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                        self.status = format!("Loaded ¬ {:?}", path);
                    }
                }
            }
            Message::Save => {
                if let Some(path) = FileDialog::new().save_file() {
                    let _ = std::fs::write(&path, &self.buffer.content);
                    self.open_files[0] = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    self.status = format!("Saved ¬ {:?}", path);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // --- 1. SLEEK HEADER & TOOLBAR ---
        let title_text = text("NOTEPAD++ ")
            .size(24)
            // Accent brand color
            .style(Color::from_rgb(0.9, 0.4, 0.2));

        let ai_status = text("Null Claw: Active")
            .size(14)
            .style(Color::from_rgb(0.2, 0.8, 0.4));

        let toolbar_controls = row![
            button(text("Open").size(15)).on_press(Message::Open).padding([6, 12]),
            button(text("Save").size(15)).on_press(Message::Save).padding([6, 12]),
            Space::with_width(Length::Fixed(15.0)),
            button(text("Undo").size(15)).on_press(Message::Undo).padding([6, 12]),
            button(text("Redo").size(15)).on_press(Message::Redo).padding([6, 12])
        ].spacing(8);

        let header = container(
            row![title_text, ai_status, Space::with_width(Length::Fill), toolbar_controls]
                .align_items(alignment::Alignment::Center)
        )
        .width(Length::Fill)
        .padding([15, 25]);

        // --- 2. PREMIUM TAB BAR ---
        let mut tabs_row = row![].spacing(8);
        for (i, name) in self.open_files.iter().enumerate() {
            let is_active = i == self.selected_tab;
            let display_name = if is_active { format!("✦ {}", name) } else { name.clone() };

            tabs_row = tabs_row.push(
                button(text(display_name).size(14))
                    .on_press(Message::TabSelected(i))
                    .padding([8, 16])
            );
        }

        let tabs_container = container(tabs_row)
            .width(Length::Fill)
            .padding([0, 25, 10, 25]);

        // --- 3. THE TEXT EDITOR CANVAS ---
        // Mimicking a beautiful code window (borderless inside container, heavily padded)
        let editor = text_input("Write code, manifest the future...", &self.buffer.content)
            .on_input(Message::ContentChanged)
            .size(16)
            .padding(20);

        let editor_container = container(editor)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([5, 20]);

        // --- 4. DISCREET STATUS BAR ---
        let status_bar = container(
            row![
                text(&self.status).size(13).style(Color::from_rgb(0.6, 0.6, 0.6)),
                Space::with_width(Length::Fill),
                text("UTF-8").size(13).style(Color::from_rgb(0.5, 0.5, 0.5)),
                Space::with_width(Length::Fixed(15.0)),
                text("Rust").size(13).style(Color::from_rgb(0.9, 0.6, 0.2))
            ]
        )
        .width(Length::Fill)
        .padding([10, 25]);

        // Stack the full composition flawlessly with separators
        column![
            header,
            tabs_container,
            Rule::horizontal(1),
            editor_container,
            Rule::horizontal(1),
            status_bar
        ]
        .into()
    }
}
