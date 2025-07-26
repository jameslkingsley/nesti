use std::{
    io::{stdout, Write},
    sync::LazyLock,
    thread::sleep,
    time::Duration,
};

use parking_lot::RwLock;

use crossterm::{
    cursor::MoveUp,
    style::Print,
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use stanza::{
    renderer::{
        console::{Console, Decor},
        Renderer as ConsoleRenderer,
    },
    style::{HAlign, Palette16, Styles, TextFg},
    table::{Cell, Col, Content as TableContent, Row, Table},
};

mod color;
mod content;

pub use crate::color::*;
pub use crate::content::*;

static GLOBAL_NESTI: LazyLock<Nesti> = LazyLock::new(Nesti::default);

#[derive(Debug, Clone)]
pub struct OutputOptions {
    pub refresh_rate: u16,
    pub margin: Margin,
}

#[derive(Debug, Clone)]
pub struct Margin {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
}

impl Default for OutputOptions {
    fn default() -> Self {
        OutputOptions {
            refresh_rate: 128,
            margin: Margin {
                top: 1,
                bottom: 1,
                left: 3,
            },
        }
    }
}

pub fn nesti<T>(path: &str, content: T)
where
    T: Into<StyledContent>,
{
    GLOBAL_NESTI.put(path, content);
}

pub fn output(opt: OutputOptions) -> impl FnOnce() {
    move || loop {
        GLOBAL_NESTI.write_stdout(&opt.margin).unwrap();
        sleep(Duration::from_millis(opt.refresh_rate as u64));
    }
}

const LINE_VERTICAL: &str = "│";
const LINE_CORNER: &str = "└─ ";
const LINE_JUNCTION: &str = "├─ ";

#[derive(Debug)]
struct Node {
    segment: Vec<u8>,
    content: Option<StyledContent>,
    children: Vec<usize>,
}

impl Node {
    fn new(segment: Vec<u8>) -> Self {
        Self {
            segment,
            content: None,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Nesti {
    arena: RwLock<Vec<Node>>,
    roots: RwLock<Vec<usize>>,
    last_line_count: RwLock<usize>,
}

impl Nesti {
    pub fn put<T: Into<StyledContent>>(&self, path: &str, content: T) {
        let path = path.as_bytes();
        let segments = self.split_path(path);

        if segments.is_empty() {
            return;
        }

        let mut arena = self.arena.write();
        let mut roots = self.roots.write();

        // Find or create root node
        let root_idx = roots
            .iter()
            .find(|&&idx| arena[idx].segment == segments[0])
            .copied()
            .unwrap_or_else(|| {
                let root_node = Node::new(segments[0].clone());
                arena.push(root_node);
                let idx = arena.len() - 1;
                roots.push(idx);
                idx
            });

        let mut current_idx = root_idx;

        // Navigate through the rest of the segments
        for segment in segments.iter().skip(1) {
            let child_idx = arena[current_idx]
                .children
                .iter()
                .find(|&&child_idx| arena[child_idx].segment == *segment)
                .copied();

            match child_idx {
                Some(idx) => {
                    current_idx = idx;
                }
                None => {
                    let new_node = Node::new(segment.clone());
                    arena.push(new_node);
                    let new_idx = arena.len() - 1;
                    arena[current_idx].children.push(new_idx);
                    current_idx = new_idx;
                }
            }
        }

        arena[current_idx].content = Some(content.into());
    }

    pub fn pop(&self, path: &str) -> Option<StyledContent> {
        let path = path.as_bytes();
        let segments = self.split_path(path);

        if segments.is_empty() {
            return None;
        }

        let mut arena = self.arena.write();
        let mut roots = self.roots.write();

        // Find the root node
        let root_idx = roots
            .iter()
            .position(|&idx| arena[idx].segment == segments[0])?;
        let root_node_idx = roots[root_idx];

        if segments.len() == 1 {
            // Removing a root node
            let content = arena[root_node_idx].content.take();

            // If the root has no content and no children, remove it entirely
            if content.is_some() && arena[root_node_idx].children.is_empty() {
                roots.remove(root_idx);
            }

            return content;
        }

        // Navigate to the target node and track the path
        let mut path_indices = vec![root_node_idx];
        let mut current_idx = root_node_idx;

        for segment in segments.iter().skip(1) {
            let child_idx = arena[current_idx]
                .children
                .iter()
                .find(|&&child_idx| arena[child_idx].segment == *segment)
                .copied();

            match child_idx {
                Some(idx) => {
                    path_indices.push(idx);
                    current_idx = idx;
                }
                None => return None,
            }
        }

        // Remove content from the target node
        let content = arena[current_idx].content.take();

        // Clean up empty nodes from leaf to root
        for i in (1..path_indices.len()).rev() {
            let node_idx = path_indices[i];
            let parent_idx = path_indices[i - 1];

            // If node has no content and no children, remove it
            if arena[node_idx].content.is_none() && arena[node_idx].children.is_empty() {
                // Remove this node from its parent's children
                arena[parent_idx]
                    .children
                    .retain(|&child| child != node_idx);
            } else {
                // Stop cleanup if we encounter a node that should remain
                break;
            }
        }

        // Check if root node should be removed
        let root_idx_in_arena = path_indices[0];
        if arena[root_idx_in_arena].content.is_none()
            && arena[root_idx_in_arena].children.is_empty()
        {
            roots.remove(root_idx);
        }

        content
    }
    pub fn render(&self, margin: &Margin) -> String {
        let arena = self.arena.read();
        let roots = self.roots.read();

        if roots.is_empty() {
            return String::new();
        }

        let mut main_table = Table::default().with_cols(vec![
            Col::new(Styles::default().with(HAlign::Left)),
            Col::new(Styles::default().with(HAlign::Right)),
        ]);

        for &root_idx in roots.iter() {
            self.add_node_to_table(&arena, root_idx, 0, true, "", margin, &mut main_table);
        }

        let renderer = Console(Decor {
            up_thin_down_thin: ' ',
            draw_outer_border: false,
            draw_inner_horizontal_border: false,
            ..Decor::default()
        });

        format!(
            "{}{}{}",
            "\n".repeat(margin.top),
            renderer.render(&main_table),
            "\n".repeat(margin.bottom),
        )
    }

    pub fn write_stdout(&self, margin: &Margin) -> std::io::Result<()> {
        let output = self.render(margin);
        let new_line_count = output.lines().count();

        let mut guard = self.last_line_count.write();
        let last_line_count = *guard;

        let mut stdout = stdout();

        // Move cursor up to start of our previous output
        if last_line_count > 0 {
            stdout.queue(MoveUp(last_line_count as u16))?;
        }

        // Clear from cursor down to remove old content
        stdout.queue(Clear(ClearType::FromCursorDown))?;

        stdout.queue(Print(output))?;
        stdout.flush()?;

        *guard = new_line_count;

        Ok(())
    }

    fn add_node_to_table(
        &self,
        arena: &[Node],
        node_idx: usize,
        depth: usize,
        is_last: bool,
        prefix: &str,
        margin: &Margin,
        table: &mut Table,
    ) {
        let node = &arena[node_idx];
        let segment_str = String::from_utf8_lossy(&node.segment);

        let tree_char = if depth == 0 {
            ""
        } else if is_last {
            LINE_CORNER
        } else {
            LINE_JUNCTION
        };

        let tree_prefix = format!(
            "{}{}{}{}",
            " ".repeat(margin.left),
            prefix,
            tree_char,
            segment_str
        );

        let row = Row::new(
            Styles::default(),
            vec![
                Cell::new(Styles::default(), TableContent::Label(tree_prefix)),
                match &node.content {
                    Some(content) => content.to_cell(),
                    None => Cell::new(
                        Styles::default().with(TextFg(Palette16::Black)),
                        TableContent::Label(String::new()),
                    ),
                },
            ],
        );

        table.push_row(row);

        let child_prefix = if depth == 0 {
            String::new()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}{}   ", prefix, LINE_VERTICAL)
        };

        for (i, &child_idx) in node.children.iter().enumerate() {
            let is_last_child = i == node.children.len() - 1;

            self.add_node_to_table(
                arena,
                child_idx,
                depth + 1,
                is_last_child,
                &child_prefix,
                margin,
                table,
            );
        }
    }

    fn split_path(&self, path: &[u8]) -> Vec<Vec<u8>> {
        let end = path.iter().position(|&b| b == 0).unwrap_or(path.len());
        let path = &path[..end];

        path.split(|&b| b == b'/')
            .filter(|segment| !segment.is_empty())
            .map(|segment| segment.to_vec())
            .collect()
    }
}
