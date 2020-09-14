use bitvec::prelude::*;
use nannou::prelude::*;

const CELL_SIZE: f32 = 50.0;
const GAP_SIZE: f32 = 10.0;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Row {
    cells: BitVec,
}

impl Row {
    pub fn new() -> Self {
        Self {
            cells: bitvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    pub fn rects(&self) -> Vec<Rect> {
        let mut rects = Vec::with_capacity(self.cells.len());
        for i in 0..self.cells.len() {
            rects.push(
                Rect::from_w_h(CELL_SIZE, CELL_SIZE).shift_x(i as f32 * (CELL_SIZE + GAP_SIZE)),
            )
        }
        rects
    }
}

struct Model {
    rows: Vec<Row>,
}

fn model(_app: &App) -> Model {
    Model {
        rows: vec![
            Row::new(),
            Row::new(),
            Row::new(),
            Row::new(),
            Row::new(),
            Row::new(),
            Row::new(),
            Row::new(),
        ],
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.ellipse().color(WHITE).xy(app.mouse.position());

    for row in &model.rows {
        for rect in row.rects() {
            draw.rect().xy(rect.xy()).wh(rect.wh()).color(WHITE);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
