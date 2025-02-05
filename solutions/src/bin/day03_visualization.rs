use nannou::{color::rgb_u32, prelude::*};

mod day03;

use day03::{Instructions, ParserConfig, Trace, TraceEvent};

const PADDING: f32 = 12.0;
const BLOCK_SIZE: f32 = 64.0;
const BLOCKS_PER_ROW: i32 = 8;
const FONT_SIZE_MIN: u32 = 24;
const FONT_SIZE_MAX: u32 = 32;

const COLOR_BACKGROUND: u32 = 0x273136;
const COLOR_BACKGROUND_DIM: u32 = 0x21282c;
const COLOR_BACKGROUND_BLUE: u32 = 0x354157;
const COLOR_BLACK: u32 = 0x1c1e1f;
const COLOR_GREEN: u32 = 0xa2e57b;
const COLOR_FOREGROUND: u32 = 0xe1e2e3;

const FRAMES_PER_STEP: usize = 14;

const FONT_PATH: &str = "./fonts/scientifica.ttf";

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
    frame.clear(rgb_u32(COLOR_BACKGROUND));

    let t: f32 = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    let win = app.window_rect();

    let winp = win.pad(PADDING);

    let font_sine = (t * 2.0).sin();
    let pos_sine = (t / 4.0).sin();

    let font_size = map_range(font_sine, -1.0, 1.0, FONT_SIZE_MIN, FONT_SIZE_MAX);

    // determine step
    let step = (frame.nth() as usize) % (model.trace.0.len() * FRAMES_PER_STEP);
    if (step / FRAMES_PER_STEP) == 0 && frame.nth() > 60 {
        std::process::exit(0);
    }

    // determine active block
    let mut msg: Option<String> = None;
    let mut active_pos: usize = 0;
    match model.trace.0.get(step / FRAMES_PER_STEP) {
        Some(TraceEvent::TokenizerEvent {
            pos, evaluation, ..
        }) => {
            active_pos = *pos;
            let msg = match msg {
                Some(eval) => msg = Some(eval.clone()),
                None => {
                    // search for previous message
                    msg = model
                        .trace
                        .0
                        .iter()
                        .take(step / FRAMES_PER_STEP)
                        .rev()
                        .find_map(|trace| trace.evaluation())
                }
            };
        }
        _ => todo!(),
    }

    // draw status pane
    let status = Rect::from_w_h((BLOCK_SIZE + PADDING) * BLOCKS_PER_ROW as f32, BLOCK_SIZE)
        .top_left_of(winp);
    // .shift_x((BLOCK_SIZE + PADDING) * (BLOCKS_PER_ROW as f32 / 2.0));
    draw.rect()
        .xy(status.xy())
        .w_h((BLOCK_SIZE) * BLOCKS_PER_ROW as f32, BLOCK_SIZE)
        .color(rgb_u32(COLOR_BLACK));
    if let Some(text) = msg {
        draw.text(&text)
            .font_size(40)
            .font(text::font::from_file(FONT_PATH).unwrap())
            .no_line_wrap()
            .xy(status.xy())
            .color(rgb_u32(COLOR_FOREGROUND));
    }

    // draw input blocks
    let mut next_block = Rect::from_w_h(BLOCK_SIZE, BLOCK_SIZE)
        .top_left_of(winp)
        .shift_y(-PADDING - BLOCK_SIZE);
    let mut row_start: Rect = next_block;
    let mut col: i32 = 1;

    for (pos, character) in model.input.iter().enumerate() {
        let (fg, bg) = if active_pos == pos {
            (COLOR_BLACK, COLOR_GREEN)
        } else {
            (COLOR_FOREGROUND, COLOR_BACKGROUND_DIM)
        };
        // draw block
        draw.rect()
            .xy(next_block.xy())
            .wh(next_block.wh())
            .color(rgb_u32(bg));
        draw.text(character.to_string().as_str())
            .font_size(font_size)
            .font(text::font::from_file(FONT_PATH).unwrap())
            .xy(next_block.xy())
            .color(rgb_u32(fg));

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
