use std::{
    io::{stdout, Write},
    mem::take,
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy_ecs::prelude::*;
use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    style::Print,
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use parking_lot::RwLock;
use stanza::{
    renderer::{
        console::{Console, Decor},
        Renderer,
    },
    style::{HAlign, MinWidth, Styles as StanzaStyles},
    table::{Cell, Col, Content as StanzaContent, Row, Table},
};

use crate::style::Styles;

const LINE_SPACE: &str = "   ";
const LINE_VERTICAL: &str = "│";
const LINE_CORNER: &str = "╰─ ";
const LINE_JUNCTION: &str = "├─ ";

#[derive(Debug, Default)]
pub struct Nesti {
    world: RwLock<World>,
    last_line_count: RwLock<usize>,
    insertion_counter: AtomicUsize,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Content(pub String);

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct Style(pub Styles);

pub trait Element {
    fn spawn(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>);

    fn tick(&self, entity: &mut EntityWorldMut, style_override: Option<Styles>) {
        self.spawn(entity, style_override);
    }
}

impl Nesti {
    pub fn put<P, E>(&self, path: P, element: E)
    where
        P: Into<String>,
        E: Element,
    {
        let path = path.into();
        let mut world = self.world.write();

        let entity = {
            let mut query = world.query::<(Entity, &Path)>();
            query
                .iter(&world)
                .find(|(_, p)| &p.0 == &path)
                .map(|(e, _)| e)
        };

        if let Some(entity) = entity {
            // Entity already exists at path
            let mut ent = world.entity_mut(entity);
            element.tick(&mut ent, None);
        } else {
            let insertion_order = self.insertion_counter.fetch_add(1, Ordering::SeqCst);
            let mut ent = world.spawn((Path(path), InsertionOrder(insertion_order)));
            element.spawn(&mut ent, None);
        }
    }

    pub fn pop(&self, path: &str) {
        let mut world = self.world.write();
        let mut cmds = world.commands();
        let path = path.to_owned();

        cmds.queue({
            let path = path.to_owned();
            move |world: &mut World| {
                let entities: Vec<Entity> = {
                    let mut q = world.query::<(Entity, &Path)>();
                    q.iter(world)
                        .filter(|(_, p)| p.starts_with(&path))
                        .map(|(e, _)| e)
                        .collect()
                };

                for entity in entities {
                    world.despawn(entity);
                }
            }
        });
    }

    pub fn flush(&self) -> Result<(), std::io::Error> {
        let mut world = self.world.write();
        world.flush();

        let content = self.render(&mut world);

        let new_line_count = content.lines().count();
        let mut guard = self.last_line_count.write();
        let last_line_count = *guard;

        let mut stdout = stdout();

        if last_line_count > 0 {
            stdout.queue(MoveToColumn(0))?;
            stdout.queue(MoveUp(last_line_count as u16))?;
        }

        stdout.queue(Clear(ClearType::FromCursorDown))?;
        stdout.queue(Print(content))?;
        stdout.flush()?;

        *guard = new_line_count;

        Ok(())
    }

    fn render(&self, world: &mut World) -> String {
        let mut q = world.query::<(&Path, &Content, Option<&Style>, &InsertionOrder)>();

        let mut rows: Vec<(Vec<&str>, &Content, Option<&Style>, usize)> = q
            .iter(&world)
            .map(|(path, c, s, order)| {
                (
                    path.split('/')
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>(),
                    c,
                    s,
                    order.0,
                )
            })
            .collect();

        rows.sort_unstable_by(|a, b| a.3.cmp(&b.3));

        let mut prefix = String::new();
        let mut buffer = String::new();
        let mut table_rows = Vec::with_capacity(rows.len());

        render_range(
            &rows,
            0,
            rows.len(),
            0,
            &mut prefix,
            &mut table_rows,
            &mut buffer,
        );

        if table_rows.is_empty() {
            return String::new();
        }

        let table = Table::default()
            .with_cols(vec![
                Col::new(
                    StanzaStyles::default()
                        .with(HAlign::Left)
                        .with(MinWidth(30)),
                ),
                Col::new(StanzaStyles::default().with(HAlign::Right)),
            ])
            .with_rows(table_rows);

        let renderer = Console(Decor {
            up_thin_down_thin: ' ',
            draw_outer_border: false,
            draw_inner_horizontal_border: false,
            ..Decor::default()
        });

        format!("\n{}\n", renderer.render(&table))
    }
}

#[derive(Component, PartialEq, Eq)]
#[component(storage = "SparseSet")]
struct Path(pub(crate) String);

#[derive(Component)]
#[component(storage = "SparseSet")]
struct InsertionOrder(pub usize);

impl Deref for Path {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn render_range(
    rows: &[(Vec<&str>, &Content, Option<&Style>, usize)],
    start: usize,
    end: usize,
    depth: usize,
    prefix: &mut String,
    out: &mut Vec<Row>,
    buffer: &mut String,
) {
    let mut i = start;
    while i < end {
        if rows[i].0.len() <= depth {
            i += 1;
            continue;
        }

        let seg = rows[i].0[depth];

        let mut j = i + 1;
        while j < end
            && rows[j].0.len() > depth
            && rows[j].0[..depth] == rows[i].0[..depth]
            && rows[j].0[depth] == seg
        {
            j += 1;
        }

        let mut k = j;
        while k < end && rows[k].0.len() <= depth {
            k += 1;
        }
        let is_last = k == end || rows[k].0[..depth] != rows[i].0[..depth];

        let conn = if depth == 0 {
            ""
        } else if is_last {
            LINE_CORNER
        } else {
            LINE_JUNCTION
        };

        buffer.clear();
        buffer.push_str("\x1b[90m");
        buffer.push_str("   ");
        buffer.push_str(prefix);
        buffer.push_str(conn);
        buffer.push_str("\x1b[0m");
        buffer.push_str(seg);

        let mut cells = Vec::with_capacity(2);

        cells.push(Cell::new(
            StanzaStyles::default(),
            StanzaContent::Label(take(buffer)),
        ));

        if let Some(idx) = (i..j).find(|&idx| rows[idx].0.len() == depth + 1) {
            let (_, content, style, _) = &rows[idx];
            cells.push(Cell::new(
                match style {
                    Some(s) => s.0 .0.clone(),
                    None => StanzaStyles::default(),
                },
                StanzaContent::Label(content.0.clone()),
            ));
        }

        out.push(Row::new(StanzaStyles::default(), cells));

        let saved = prefix.len();
        if depth != 0 {
            if is_last {
                prefix.push_str(LINE_SPACE);
            } else {
                prefix.push_str(LINE_VERTICAL);
                prefix.push_str(&LINE_SPACE[..2]);
            }
        }

        if rows[i..j].iter().any(|r| r.0.len() > depth + 1) {
            render_range(rows, i, j, depth + 1, prefix, out, buffer);
        }

        prefix.truncate(saved);
        i = j;
    }
}
