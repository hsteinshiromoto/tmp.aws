use crate::app::{App, View};
use crate::models::Day;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table, Tabs},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_tabs(f, app, chunks[0]);

    match app.view {
        View::Recipes => draw_recipes(f, app, chunks[1]),
        View::RecipeAdd => draw_recipe_add(f, app, chunks[1]),
        View::MealPlan => draw_meal_plan(f, app, chunks[1]),
        View::MealAssign => draw_meal_assign(f, app, chunks[1]),
        View::Grocery => draw_grocery(f, app, chunks[1]),
        View::Settings => draw_settings(f, app, chunks[1]),
        View::ConfirmDelete => draw_confirm_delete(f, app, chunks[1]),
    }

    draw_status(f, app, chunks[2]);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["1:Recipes", "2:Plan", "3:Grocery", "4:Settings"];
    let selected = match app.view {
        View::Recipes | View::RecipeAdd | View::ConfirmDelete => 0,
        View::MealPlan | View::MealAssign => 1,
        View::Grocery => 2,
        View::Settings => 3,
    };
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" MealPrep "))
        .select(selected)
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, area);
}

fn draw_recipes(f: &mut Frame, app: &App, area: Rect) {
    let recipes = app.recipes.list();
    let items: Vec<ListItem> = recipes
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let style = if i == app.selected_index % recipes.len().max(1) {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(format!(
                "{} ({} servings, {} ingredients)",
                r.name,
                r.default_servings,
                r.ingredients.len()
            ))
            .style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Recipes [a]dd [d]elete "));
    f.render_widget(list, area);
}

fn draw_recipe_add(f: &mut Frame, app: &App, area: Rect) {
    let labels = ["Name", "Servings", "Ingredients (qty unit name; ...)", "Instructions"];
    let mut lines: Vec<Line> = Vec::new();
    for (i, label) in labels.iter().enumerate() {
        let marker = if i == app.input_field_index { "> " } else { "  " };
        let value = app.input_fields.get(i).map(|s| s.as_str()).unwrap_or("");
        lines.push(Line::from(vec![
            Span::styled(format!("{marker}{label}: "), Style::default().fg(Color::Cyan)),
            Span::raw(value),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Tab: next field | Enter: submit (on last field) | Esc: cancel",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Add Recipe "));
    f.render_widget(paragraph, area);
}

fn draw_meal_plan(f: &mut Frame, app: &App, area: Rect) {
    let slots = app.planner.slots();
    let mut header_cells = vec![""];
    let slot_strs: Vec<&str> = slots.iter().map(|s| s.as_str()).collect();
    header_cells.extend(slot_strs.iter());
    let header = Row::new(header_cells).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = Day::ALL
        .iter()
        .map(|day| {
            let mut cells = vec![format!("{day}")];
            for slot in slots {
                let meal_name = app
                    .planner
                    .get_meal(*day, slot)
                    .and_then(|id| app.recipes.get(id))
                    .map(|r| r.name.clone())
                    .unwrap_or_else(|| "—".to_string());
                cells.push(meal_name);
            }
            Row::new(cells)
        })
        .collect();

    let widths: Vec<Constraint> = std::iter::once(Constraint::Length(5))
        .chain(slots.iter().map(|_| Constraint::Min(15)))
        .collect();

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Meal Plan [a]ssign "));
    f.render_widget(table, area);
}

fn draw_meal_assign(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left: day/slot selector
    let slots = app.planner.slots();
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(Span::styled(
        format!("Day: {} (←/→)", Day::ALL[app.selected_day]),
        Style::default().fg(Color::Yellow),
    )));
    lines.push(Line::from(""));
    for (i, slot) in slots.iter().enumerate() {
        let marker = if i == app.selected_slot { "▶ " } else { "  " };
        let style = if i == app.selected_slot {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        lines.push(Line::from(Span::styled(format!("{marker}{slot}"), style)));
    }
    let left = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Select Slot (↑/↓) "));
    f.render_widget(left, chunks[0]);

    // Right: recipe selector
    let recipes = app.recipes.list();
    let items: Vec<ListItem> = recipes
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let style = if i == app.selected_index % recipes.len().max(1) {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(r.name.clone()).style(style)
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Select Recipe (j/k) Enter "));
    f.render_widget(list, chunks[1]);
}

fn draw_grocery(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .grocery_list
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let check = if item.checked { "✓" } else { " " };
            let style = if i == app.selected_index % app.grocery_list.items.len().max(1) {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if item.checked {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
            };
            ListItem::new(format!(
                "[{check}] {:.1} {} {}",
                item.quantity, item.unit, item.name
            ))
            .style(style)
        })
        .collect();

    let remaining = app.grocery_list.items.iter().filter(|i| !i.checked).count();
    let title = format!(
        " Grocery List ({}/{}) [g]enerate [space]toggle ",
        remaining,
        app.grocery_list.items.len()
    );
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(list, area);
}

fn draw_settings(f: &mut Frame, app: &App, area: Rect) {
    let lines = vec![
        Line::from(vec![
            Span::styled("Household size: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{}", app.config.household_size),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  (+/-)", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Meal slots: ", Style::default().fg(Color::Cyan)),
            Span::raw(app.config.slots.join(", ")),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Data directory: ", Style::default().fg(Color::Cyan)),
            Span::raw(app.config.data_dir.display().to_string()),
        ]),
    ];
    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Settings "));
    f.render_widget(paragraph, area);
}

fn draw_confirm_delete(f: &mut Frame, app: &App, area: Rect) {
    let paragraph = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            &app.status_msg,
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
    ])
    .block(Block::default().borders(Borders::ALL).title(" Confirm Delete "));
    f.render_widget(paragraph, area);
}

fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let help = match app.view {
        View::Recipes => "q:quit | 1-4:navigate | j/k:select | a:add | d:delete",
        View::RecipeAdd => "Tab:next field | Enter:submit | Esc:cancel",
        View::MealPlan => "q:quit | 1-4:navigate | a:assign meal",
        View::MealAssign => "←/→:day | ↑/↓:slot | j/k:recipe | Enter:assign | x:clear | Esc:back",
        View::Grocery => "q:quit | 1-4:navigate | g:generate | space:toggle | j/k:select",
        View::Settings => "q:quit | 1-4:navigate | +/-:household size",
        View::ConfirmDelete => "y:confirm | any:cancel",
    };
    let status = Paragraph::new(vec![
        Line::from(Span::styled(&app.status_msg, Style::default().fg(Color::Green))),
        Line::from(Span::styled(help, Style::default().fg(Color::DarkGray))),
    ])
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, area);
}
