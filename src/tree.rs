use std::{
    io::{stderr, stdout, Result as IoResult, Write},
    time::{Duration, Instant},
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
    style::{HAlign, MinWidth, Palette16, Styles as StanzaStyles, TextFg},
    table::{Cell, Col, Content as TableContent, Row, Table},
};

use crate::{elements::Element, style::Styles};

const LINE_VERTICAL: &str = "│";
const LINE_CORNER: &str = "└─ ";
const LINE_JUNCTION: &str = "├─ ";

#[derive(Debug, Clone, Copy)]
pub enum Buffer {
    Stdout,
    Stderr,
}

#[derive(Debug)]
pub struct Nesti {
    arena: RwLock<Vec<Node>>,
    roots: RwLock<Vec<usize>>,
    last_line_count: RwLock<usize>,
    context: GlobalContext,
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalContext {
    pub start: Instant,
    pub delta: Duration,
}

#[derive(Debug, Clone)]
pub struct OutputOptions {
    pub refresh_rate: u16,
    pub margin: Margin,
    pub buffer: Buffer,
}

#[derive(Debug, Clone)]
pub struct Margin {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
}

#[derive(Debug)]
struct Node {
    segment: Vec<u8>,
    children: Vec<usize>,
    element: Option<Box<dyn DynElement>>,
    context: Option<Box<dyn std::any::Any + Send + Sync>>,
}

impl Nesti {
    pub fn put<T: Element>(&self, path: &str, element: T)
    where
        T: 'static + Send + Sync + std::fmt::Debug,
        T::Context: Default + Send + Sync + 'static,
    {
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

        // Create context for this element
        let context = Box::new(T::Context::default()) as Box<dyn std::any::Any + Send + Sync>;

        // Store the element and its context
        arena[current_idx].element = Some(Box::new(ElementWrapper(element)));
        arena[current_idx].context = Some(context);
    }

    pub fn pop(&self, path: &str) {
        let path = path.as_bytes();
        let segments = self.split_path(path);

        if segments.is_empty() {
            return;
        }

        let mut arena = self.arena.write();
        let mut roots = self.roots.write();

        // Find the root node
        match roots
            .iter()
            .position(|&idx| arena[idx].segment == segments[0])
        {
            Some(root_idx) => {
                let root_node_idx = roots[root_idx];

                if segments.len() == 1 {
                    roots.remove(root_idx);
                    return;
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
                        None => return,
                    }
                }

                // Clean up empty nodes from leaf to root
                for i in (1..path_indices.len()).rev() {
                    let node_idx = path_indices[i];
                    let parent_idx = path_indices[i - 1];

                    // If node has no content and no children, remove it
                    if arena[node_idx].element.is_none() && arena[node_idx].children.is_empty() {
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
                if arena[root_idx_in_arena].element.is_none()
                    && arena[root_idx_in_arena].children.is_empty()
                {
                    roots.remove(root_idx);
                }
            }
            None => {}
        }
    }

    pub fn render(&self, margin: &Margin) -> String {
        let global_ctx = &self.context;

        let arena = self.arena.read();
        let roots = self.roots.read();

        if roots.is_empty() {
            return String::new();
        }

        let mut main_table = Table::default().with_cols(vec![
            Col::new(
                StanzaStyles::default()
                    .with(HAlign::Left)
                    .with(MinWidth(20)),
            ),
            Col::new(StanzaStyles::default().with(HAlign::Right)),
        ]);

        for &root_idx in roots.iter() {
            self.add_node_to_table(
                &arena,
                root_idx,
                0,
                true,
                "",
                &global_ctx,
                margin,
                &mut main_table,
            );
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

    pub fn write_to_buffer(&self, opt: &OutputOptions) -> IoResult<()> {
        let output = self.render(&opt.margin);
        let new_line_count = output.lines().count();

        let mut guard = self.last_line_count.write();
        let last_line_count = *guard;

        let mut move_cmd = None;
        let clear_cmd = Clear(ClearType::FromCursorDown);
        let print_cmd = Print(output);

        if last_line_count > 0 {
            move_cmd = Some(MoveUp(last_line_count as u16));
        }

        match opt.buffer {
            Buffer::Stdout => self.write_to_stdout(move_cmd, clear_cmd, print_cmd)?,
            Buffer::Stderr => self.write_to_stderr(move_cmd, clear_cmd, print_cmd)?,
        };

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
        global_ctx: &GlobalContext,
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
            StanzaStyles::default(),
            vec![
                Cell::new(StanzaStyles::default(), TableContent::Label(tree_prefix)),
                match (&node.element, &node.context) {
                    (Some(element), Some(context)) => {
                        let content = element.content(context.as_ref(), &global_ctx);
                        let styles = element.styles();
                        Cell::new(styles.0.clone(), TableContent::Label(content))
                    }
                    _ => Cell::new(
                        StanzaStyles::default().with(TextFg(Palette16::Black)),
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
                global_ctx,
                margin,
                table,
            );
        }
    }

    fn write_to_stdout(
        &self,
        move_cmd: Option<MoveUp>,
        clear_cmd: Clear,
        print_cmd: Print<String>,
    ) -> IoResult<()> {
        let mut stdout = stdout();

        if let Some(move_cmd) = move_cmd {
            stdout.queue(move_cmd)?;
        }

        stdout.queue(clear_cmd)?;
        stdout.queue(print_cmd)?;
        stdout.flush()?;

        Ok(())
    }

    fn write_to_stderr(
        &self,
        move_cmd: Option<MoveUp>,
        clear_cmd: Clear,
        print_cmd: Print<String>,
    ) -> IoResult<()> {
        let mut stderr = stderr();

        if let Some(move_cmd) = move_cmd {
            stderr.queue(move_cmd)?;
        }

        stderr.queue(clear_cmd)?;
        stderr.queue(print_cmd)?;
        stderr.flush()?;

        Ok(())
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

impl Node {
    fn new(segment: Vec<u8>) -> Self {
        Self {
            segment,
            element: None,
            children: Vec::new(),
            context: None,
        }
    }
}

// Type-erased element trait
trait DynElement: Send + Sync + std::fmt::Debug {
    fn content(&self, context: &dyn std::any::Any, global: &GlobalContext) -> String;
    fn styles(&self) -> Styles;
}

// Wrapper to implement DynElement for any Element
struct ElementWrapper<E: Element>(E);

impl<E: Element + Send + Sync> DynElement for ElementWrapper<E>
where
    E::Context: 'static,
{
    fn content(&self, context: &dyn std::any::Any, global: &GlobalContext) -> String {
        let ctx = context
            .downcast_ref::<E::Context>()
            .expect("Context type mismatch");
        self.0.content(ctx, global)
    }

    fn styles(&self) -> Styles {
        self.0.styles()
    }
}

impl<E: Element + Send + Sync> std::fmt::Debug for ElementWrapper<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElementWrapper").finish()
    }
}

impl Default for Nesti {
    fn default() -> Self {
        Self {
            arena: RwLock::default(),
            roots: RwLock::default(),
            last_line_count: RwLock::default(),
            context: GlobalContext {
                start: Instant::now(),
                delta: Duration::ZERO,
            },
        }
    }
}

impl Default for OutputOptions {
    fn default() -> Self {
        OutputOptions {
            refresh_rate: 32,
            buffer: Buffer::Stdout,
            margin: Margin {
                top: 1,
                bottom: 1,
                left: 3,
            },
        }
    }
}
