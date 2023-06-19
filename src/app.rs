use std::error;

use rusqlite::Connection;
use tui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::ListState,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub const NUM_COLUMNS: usize = 3;

#[derive(Debug, Default, Clone)]
struct Scriptures {
    works: Vec<Work>,
}

#[derive(Debug, Clone)]
struct SqliteRow {
    html_content: String,
    chapter_title: String,
    book_title: String,
}

impl Scriptures {
    fn new() -> Self {
        Self::new_failable().unwrap_or_default()
    }

    fn new_failable() -> AppResult<Self> {
        const DATABASES: &[(&str, &str)] = &[
            ("OT", "ot.sqlite"),
            ("NT", "nt.sqlite"),
            ("BoM", "bom.sqlite"),
            ("D&C", "dc.sqlite"),
            ("PoGP", "pgp.sqlite"),
        ];
        let mut works = vec![];

        for (work_title, db) in DATABASES {
            let conn = Connection::open(db)?;
            let mut stmt = conn.prepare("SELECT content_html, subitem.title, IIF(nav_collection.nav_section_id IS NULL, nav_item.title, nav_collection.title) FROM subitem_content JOIN subitem ON subitem_content.subitem_id = subitem.id JOIN nav_item ON subitem_content.subitem_id = nav_item.subitem_id JOIN nav_section ON nav_item.nav_section_id = nav_section.id JOIN nav_collection ON nav_collection.id = nav_section.nav_collection_id ORDER BY subitem.position")?;
            let rows = stmt.query_map([], |row| {
                Ok(SqliteRow {
                    html_content: row.get(0)?,
                    chapter_title: row.get(1)?,
                    book_title: row.get(2)?,
                })
            })?;

            let mut books = vec![];
            let mut book_title = "".to_string();
            let mut chapters = vec![];
            for row in rows {
                let row = &row?;

                if book_title != row.book_title {
                    if !chapters.is_empty() {
                        books.push(Book {
                            title: book_title,
                            chapters: chapters.clone(),
                        })
                    }

                    book_title = row.book_title.clone();
                    chapters.clear();
                }

                chapters.push(Chapter {
                    title: row.chapter_title.clone(),
                    html_content: row.html_content.clone(),
                });
            }

            if !chapters.is_empty() {
                books.push(Book {
                    title: book_title,
                    chapters,
                })
            }

            works.push(Work {
                title: work_title.to_string(),
                books,
            })
        }

        Ok(Scriptures { works })
    }
}

#[derive(Debug, Default, Clone)]
struct Work {
    title: String,
    books: Vec<Book>,
}

#[derive(Debug, Default, Clone)]
struct Book {
    title: String,
    chapters: Vec<Chapter>,
}

#[derive(Debug, Default, Clone)]
struct Chapter {
    title: String,
    html_content: String,
}

impl Chapter {
    fn text(&self) -> Text {
        let mut text = Text::default();

        let tree = roxmltree::Document::parse(&self.html_content).unwrap();
        if let Some(body) = tree.descendants().find(|n| n.tag_name().name() == "body") {
            let header = body.descendants().find(|n| n.tag_name().name() == "header");
            if let Some(header) = header {
                if let Some(study_summary) = header
                    .descendants()
                    .find(|n| n.attribute("class") == Some("study-summary"))
                {
                    let mut summary_text = String::new();
                    recursive_text_as_string(study_summary, &mut summary_text);
                    if !summary_text.is_empty() {
                        text.extend(Text::styled(
                            summary_text,
                            Style::default().add_modifier(Modifier::ITALIC),
                        ));
                        text.extend(Text::raw("")); // Empty line
                    }
                }
            }

            let verses = body
                .descendants()
                .filter(|n| n.attribute("class") == Some("verse"));
            for verse in verses {
                let verse_text = verse_text(verse);
                text.extend(Text {
                    lines: vec![verse_text, "".into()],
                });
            }
        }

        text
    }
}

fn recursive_text_as_string(node: roxmltree::Node, s: &mut String) {
    if node.is_text() {
        if let Some(t) = node.text() {
            s.push_str(t);
        }
    }

    for n in node.children() {
        recursive_text_as_string(n, s);
    }
}

fn verse_text(node: roxmltree::Node) -> Line<'static> {
    let mut line = Line::default();

    for child in node.children() {
        if child.attribute("class") == Some("verse-number") {
            let verse_num_text = Span::styled(
                child.text().unwrap().to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            );
            line.spans.push(verse_num_text);
        } else if child.is_text() {
            line.spans
                .push(Span::raw(child.text().unwrap().to_string()))
        } else if child.attribute("class") == Some("study-note-ref") {
            for child2 in child.children() {
                if child2.tag_name().name() == "sup" {
                    if let Some(footnote) = footnote_unicode(child2.text()) {
                        line.spans.push(Span::raw(footnote));
                    }
                } else if child2.is_text() {
                    line.spans
                        .push(Span::raw(child2.text().unwrap().to_string()))
                }
            }
        }
    }

    line
}

fn footnote_unicode(string: Option<&str>) -> Option<&'static str> {
    let input = string?;
    match input {
        "a" => Some("ᵃ"),
        "b" => Some("ᵇ"),
        "c" => Some("ᶜ"),
        "d" => Some("ᵈ"),
        "e" => Some("ᵉ"),
        "f" => Some("ᶠ"),
        "g" => Some("ᵍ"),
        "h" => Some("ʰ"),
        "i" => Some("ⁱ"),
        "j" => Some("ʲ"),
        "k" => Some("ᵏ"),
        "l" => Some("ˡ"),
        "m" => Some("ᵐ"),
        "n" => Some("ⁿ"),
        "o" => Some("ᵒ"),
        "p" => Some("ᵖ"),
        "q" => Some("q"),
        "r" => Some("ʳ"),
        "s" => Some("ˢ"),
        "t" => Some("ᵗ"),
        "u" => Some("ᵘ"),
        "v" => Some("ᵛ"),
        "w" => Some("ʷ"),
        "x" => Some("ˣ"),
        "y" => Some("ʸ"),
        "z" => Some("ᶻ"),
        _ => None,
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    data: Scriptures,
    pub column_selected: usize,
    pub works_state: ListState,
    pub books_state: ListState,
    pub chapters_state: ListState,
    pub text_scroll: u16,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            data: Scriptures::new(),
            column_selected: 0,
            works_state: ListState::default().with_selected(Some(0)),
            books_state: ListState::default().with_selected(Some(0)),
            chapters_state: ListState::default().with_selected(Some(0)),
            text_scroll: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn verse_title(&self) -> String {
        let chapter = &self.data.works[self.works_state.selected().unwrap_or_default()].books
            [self.books_state.selected().unwrap_or_default()]
        .chapters[self.chapters_state.selected().unwrap_or_default()];
        chapter.title.clone()
    }

    pub fn verse_text(&self) -> Text {
        let chapter = &self.data.works[self.works_state.selected().unwrap_or_default()].books
            [self.books_state.selected().unwrap_or_default()]
        .chapters[self.chapters_state.selected().unwrap_or_default()];
        chapter.text()
    }

    pub fn works_titles(&self) -> Vec<String> {
        self.data
            .works
            .iter()
            .map(|w| w.title.clone())
            .collect::<Vec<_>>()
    }

    pub fn books_titles(&self) -> Vec<String> {
        self.data.works[self.works_state.selected().unwrap_or_default()]
            .books
            .iter()
            .map(|b| b.title.clone())
            .collect::<Vec<_>>()
    }

    pub fn chapters_titles(&self) -> Vec<String> {
        self.data.works[self.works_state.selected().unwrap_or_default()].books
            [self.books_state.selected().unwrap_or_default()]
        .chapters
        .iter()
        .map(|c| c.title.clone())
        .collect::<Vec<_>>()
    }

    pub fn arrow_down(&mut self) {
        match self.column_selected {
            0 => self.update_works(true),
            1 => self.update_books(true),
            2 => self.update_chapters(true),
            _ => unreachable!(),
        }
    }

    pub fn arrow_up(&mut self) {
        match self.column_selected {
            0 => self.update_works(false),
            1 => self.update_books(false),
            2 => self.update_chapters(false),
            _ => unreachable!(),
        }
    }

    pub fn arrow_left(&mut self) {
        if self.column_selected == 0 {
            self.column_selected = NUM_COLUMNS - 1;
        } else {
            self.column_selected -= 1;
        }
    }

    pub fn arrow_right(&mut self) {
        if self.column_selected == NUM_COLUMNS - 1 {
            self.column_selected = 0;
        } else {
            self.column_selected += 1;
        }
    }

    fn update_works(&mut self, down: bool) {
        let i = if down {
            match self.works_state.selected() {
                Some(i) => {
                    if i == self.data.works.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            }
        } else {
            match self.works_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.data.works.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            }
        };

        self.works_state.select(Some(i));
        self.books_state = ListState::default().with_selected(Some(0));
        self.chapters_state = ListState::default().with_selected(Some(0));
        self.text_scroll = 0;
    }

    fn update_books(&mut self, down: bool) {
        let i = if down {
            match self.books_state.selected() {
                Some(i) => {
                    if i == self.data.works[self.works_state.selected().unwrap_or_default()]
                        .books
                        .len()
                        - 1
                    {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            }
        } else {
            match self.books_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.data.works[self.works_state.selected().unwrap_or_default()]
                            .books
                            .len()
                            - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            }
        };

        self.books_state.select(Some(i));
        self.chapters_state = ListState::default().with_selected(Some(0));
        self.text_scroll = 0;
    }

    fn update_chapters(&mut self, down: bool) {
        let i = if down {
            match self.chapters_state.selected() {
                Some(i) => {
                    if i == self.data.works[self.works_state.selected().unwrap_or_default()].books
                        [self.books_state.selected().unwrap_or_default()]
                    .chapters
                    .len()
                        - 1
                    {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            }
        } else {
            match self.chapters_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.data.works[self.works_state.selected().unwrap_or_default()].books
                            [self.books_state.selected().unwrap_or_default()]
                        .chapters
                        .len()
                            - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            }
        };

        self.chapters_state.select(Some(i));
        self.text_scroll = 0;
    }
}
