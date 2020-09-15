use nannou::prelude::*;

const CELL_SIZE: f32 = 50.0;
const GAP_SIZE: f32 = 10.0;
const INCREMENT_EVERY: usize = 15;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Row {
    y: f32,
    x: f32,
    sample: &'static str,
    cells: [bool; 16],
}

impl Row {
    pub fn new(sample: &'static str, x: f32, y: f32) -> Self {
        Self {
            sample,
            y: y - CELL_SIZE,
            x: x + CELL_SIZE,
            cells: [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
        }
    }

    pub fn get_rects(&self) -> Vec<(Rect, bool)> {
        let mut rects = Vec::with_capacity(self.cells.len());
        for (i, cell) in self.cells.iter().enumerate() {
            rects.push((self.cell_rect(i), *cell))
        }
        rects
    }

    pub fn click(&mut self, position: Point2<f32>) {
        // This could be done without iterating. Oh well.
        for i in 0..self.cells.len() {
            if self.cell_rect(i).contains(position) {
                self.cells[i] = !self.cells[i]
            }
        }
    }

    fn cell_rect(&self, i: usize) -> Rect {
        Rect::from_w_h(CELL_SIZE, CELL_SIZE)
            .shift_y(self.y)
            .shift_x(self.x)
            .shift_x(i as f32 * (CELL_SIZE + GAP_SIZE))
    }
}

struct Model {
    step: usize,
    frame: usize,
    rows: Vec<Row>,
    audio_stream: nannou_audio::Stream<Audio>,
}

impl Model {
    fn click(&mut self, position: Point2<f32>) {
        // This could be done without iterating. Oh well.
        for row in self.rows.iter_mut() {
            row.click(position)
        }
    }

    fn trigger_step_audio(&mut self, app: &App) {
        for row in self.rows.iter() {
            if !row.cells[self.step] {
                return;
            }

            let assets = app.assets_path().expect("could not find assets directory");
            let path = assets.join(row.sample);
            let sound = audrey::open(path).expect("failed to load sound");
            self
                .audio_stream
                .send(move |audio| {
                    audio.sounds.push(sound);
                })
                .ok();
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    let audio_host = nannou_audio::Host::new();
    let sounds = vec![];
    let audio_model = Audio { sounds };
    let audio_stream = audio_host
        .new_output_stream(audio_model)
        .render(audio)
        .build()
        .unwrap();

    let window = app.window_rect();
    Model {
        step: 0,
        frame: 0,
        audio_stream,
        rows: vec![
            Row::new(
                "tr808/kick.wav",
                window.left(),
                window.top() - 0. * (CELL_SIZE + GAP_SIZE),
            ),
            Row::new(
                "tr808/snare.wav",
                window.left(),
                window.top() - 1. * (CELL_SIZE + GAP_SIZE),
            ),
            Row::new(
                "tr808/clap.wav",
                window.left(),
                window.top() - 2. * (CELL_SIZE + GAP_SIZE),
            ),
            Row::new(
                "tr808/close-hat.wav",
                window.left(),
                window.top() - 3. * (CELL_SIZE + GAP_SIZE),
            ),
            Row::new(
                "tr808/open-hat.wav",
                window.left(),
                window.top() - 4. * (CELL_SIZE + GAP_SIZE),
            ),
            Row::new(
                "tr808/cowbell.wav",
                window.left(),
                window.top() - 5. * (CELL_SIZE + GAP_SIZE),
            ),
        ],
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.frame > INCREMENT_EVERY {
        model.frame = 0;
        model.step = (model.step + 1) % 16;
        model.trigger_step_audio(app);
    } else {
        model.frame = model.frame + 1;
    }
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    model.click(app.mouse.position())
}

fn view(app: &App, model: &Model, frame: Frame) {
    let pink = rgb(255, 105, 180);
    let grey = rgb(230, 230, 230);
    let draw = app.draw();

    draw.background().color(BLACK);

    for row in model.rows.iter() {
        for (i, (rect, active)) in row.get_rects().into_iter().enumerate() {
            let color = if active {
                pink.clone()
            } else if model.step == i {
                grey.clone()
            } else if rect.contains(app.mouse.position()) {
                grey.clone()
            } else {
                WHITE
            };
            draw.rect().xy(rect.xy()).wh(rect.wh()).color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

struct Audio {
    sounds: Vec<audrey::read::BufFileReader>,
}

fn audio(audio: &mut Audio, buffer: &mut nannou_audio::Buffer) {
    let mut have_ended = vec![];
    let len_frames = buffer.len_frames();

    // Sum all of the sounds onto the buffer.
    for (i, sound) in audio.sounds.iter_mut().enumerate() {
        let mut frame_count = 0;
        let file_frames = sound.frames::<[f32; 2]>().filter_map(Result::ok);
        for (frame, file_frame) in buffer.frames_mut().zip(file_frames) {
            for (sample, file_sample) in frame.iter_mut().zip(&file_frame) {
                *sample += *file_sample;
            }
            frame_count += 1;
        }

        // If the sound yielded less samples than are in the buffer, it must have ended.
        if frame_count < len_frames {
            have_ended.push(i);
        }
    }

    // Remove all sounds that have ended.
    for i in have_ended.into_iter().rev() {
        audio.sounds.remove(i);
    }
}
