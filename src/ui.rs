use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

const HIGHLIGHT_SYMBOL: &str = ">";

fn highlight_style(selected: bool) -> Style {
    if selected {
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::White)
    } else {
        Style::default().add_modifier(Modifier::BOLD)
    }
}

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(20),
            Constraint::Length(1),
            Constraint::Length(16),
            Constraint::Length(1),
            Constraint::Percentage(100),
        ])
        .split(frame.size());

    render_works_list(app, frame, chunks[0]);
    render_books_list(app, frame, chunks[2]);
    render_chapters_list(app, frame, chunks[4]);

    render_chapter(app, frame, chunks[6])
}

fn render_works_list<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: Rect) {
    let works = List::new(
        app.works_titles()
            .into_iter()
            .map(ListItem::new)
            .collect::<Vec<_>>(),
    )
    .highlight_style(highlight_style(app.column_selected == 0))
    .highlight_symbol(HIGHLIGHT_SYMBOL)
    .block(
        Block::default()
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP)
            .title("Work"),
    );

    frame.render_stateful_widget(works, rect, &mut app.works_state);
}

fn render_books_list<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: Rect) {
    let books = List::new(
        app.books_titles()
            .into_iter()
            .map(ListItem::new)
            .collect::<Vec<_>>(),
    )
    .highlight_style(highlight_style(app.column_selected == 1))
    .highlight_symbol(HIGHLIGHT_SYMBOL)
    .block(
        Block::default()
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP)
            .title("Book"),
    );

    frame.render_stateful_widget(books, rect, &mut app.books_state);
}

fn render_chapters_list<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: Rect) {
    let chapters = List::new(
        app.chapters_titles()
            .into_iter()
            .map(ListItem::new)
            .collect::<Vec<_>>(),
    )
    .highlight_style(highlight_style(app.column_selected == 2))
    .highlight_symbol(HIGHLIGHT_SYMBOL)
    .block(
        Block::default()
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP)
            .title("CH"),
    );

    frame.render_stateful_widget(chapters, rect, &mut app.chapters_state);
}

fn render_chapter<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: Rect) {
    let chapter_title = app.chapter_title();
    let chapter = Block::default()
        .title(chapter_title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let rect_inside_block = chapter.inner(rect);
    frame.render_widget(chapter, rect);

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .direction(Direction::Vertical)
        .split(rect_inside_block);

    render_chapter_text(app, frame, chunks[0]);
    render_footnotes(app, frame, chunks[1]);
}

fn render_chapter_text<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: Rect) {
    let text = Paragraph::new(app.chapter_text())
        .scroll((app.text_scroll, 0))
        .wrap(Wrap { trim: false });
    frame.render_widget(text, rect);
    app.text_rect = rect;
}

fn render_footnotes<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: Rect) {
    let footnotes = Paragraph::new(app.chapter_footnotes_text())
        .scroll((app.footnote_scroll, 0))
        .block(
            Block::default()
                .title("Footnotes")
                .title_alignment(Alignment::Center)
                .borders(Borders::TOP),
        );
    frame.render_widget(footnotes, rect);
    app.footnote_rect = rect;
}
