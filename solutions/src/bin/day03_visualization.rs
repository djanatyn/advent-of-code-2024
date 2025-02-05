use nannou::{color::rgb_u32, prelude::*};

mod day03;

use day03::{Instructions, ParserConfig, Trace};

const PADDING: f32 = 12.0;
const BLOCK_SIZE: f32 = 64.0;
const BLOCKS_PER_ROW: i32 = 8;
const FONT_SIZE_MIN: u32 = 24;
const FONT_SIZE_MAX: u32 = 32;

const COLOR_BACKGROUND_DIM: u32 = 0x21282c;
const COLOR_BACKGROUND_0: u32 = 0x273136;
const COLOR_BACKGROUND_1: u32 = 0x313b42;
const COLOR_BACKGROUND_2: u32 = 0x313b42;
const COLOR_BLACK: u32 = 0x1c1e1f;
const COLOR_FOREGROUND: u32 = 0xe1e2e3;

struct Model {
    input: Vec<char>,
    trace: Trace,
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}
fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn model(app: &App) -> Model {
    app.new_window()
        .size_pixels(720, 1280)
        // .fullscreen()
        .build()
        .unwrap();
    let (_, trace) = Instructions::parse(day03::PART1_EXAMPLE, ParserConfig::Part1);
    Model {
        input: day03::PART1_EXAMPLE.chars().collect(),
        trace,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(rgb_u32(COLOR_BACKGROUND_0));

    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    let win = app.window_rect();

    let winp = win.pad(PADDING);

    let font_sine = (t * 2.0).sin();
    let pos_sine = (t / 4.0).sin();

    let font_size = map_range(font_sine, -1.0, 1.0, FONT_SIZE_MIN, FONT_SIZE_MAX);
    let active_pos = map_range(pos_sine, -1.0, 1.0, 0, model.input.len());

    let mut next_block = Rect::from_w_h(BLOCK_SIZE, BLOCK_SIZE).top_left_of(winp);
    let mut row_start: Rect = next_block;
    let mut col: i32 = 1;

    for (pos, character) in model.input.iter().enumerate() {
        let bg = if active_pos == pos {
            COLOR_BLACK
        } else {
            COLOR_BACKGROUND_DIM
        };
        // draw block
        draw.rect()
            .xy(next_block.xy())
            .wh(next_block.wh())
            .color(rgb_u32(bg));
        draw.text(character.to_string().as_str())
            .font_size(font_size)
            .font(text::font::from_file("./fonts/scientifica.ttf").unwrap())
            .xy(next_block.xy())
            .color(rgb_u32(COLOR_FOREGROUND));

        // new row?
        if col >= BLOCKS_PER_ROW {
            next_block = row_start.below(row_start).shift_y(-PADDING);
            col = 1;
            row_start = next_block.clone();
        } else {
            next_block = next_block.right_of(next_block).shift_x(PADDING);
            col += 1;
        }
    }

    let status = row_start
        .below(row_start)
        .shift_y(-PADDING)
        .shift_x((BLOCK_SIZE + PADDING) * (BLOCKS_PER_ROW as f32 / 2.0) - (BLOCK_SIZE / 2.0));
    draw.rect()
        .xy(status.xy())
        .w_h((BLOCK_SIZE) * BLOCKS_PER_ROW as f32, BLOCK_SIZE)
        .color(rgb_u32(COLOR_BLACK));
    draw.text("advent of code day 3")
        .font_size(font_size)
        .font(text::font::from_file("./fonts/scientifica.ttf").unwrap())
        .no_line_wrap()
        .xy(status.xy())
        .color(rgb_u32(COLOR_FOREGROUND));

    app.main_window()
        .capture_frame(captured_frame_path(app, &frame));
    draw.to_frame(app, &frame).unwrap();
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}
