use rand::seq::SliceRandom;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

use crate::{part1, part2};

pub struct Model {
    egui: Egui,
    base_maze: part2::Maze,
    maze: part2::Maze,
    index: usize,
    width: f32,
    height: f32,
    scale: f32,
    maze_state: part2::SolveStatus,
    guard_positions: Vec<(usize, usize)>,
    simulation_speed: u32,
    indexes: Vec<usize>,
}

impl Model {
    pub fn model(app: &App) -> Self {
        let contents = std::fs::read_to_string("input/puzzle").unwrap();
        let base_maze = part2::Maze::from_str(&contents);
        let maze = base_maze.create_with_extra_wall(0).unwrap();

        let path_maze = part1::Maze::from_str(&contents);
        let path_maze = path_maze.solve();
        let mut indexes = path_maze.get_visited_index();
        indexes.retain(|&index| maze.get_guard_pos() != (index % maze.get_size().0, index / maze.get_size().0));

        let scale = 8.0;
        let (width, height) = maze.get_size();
        let window_width = width as f32 * scale;
        let window_height = height as f32 * scale;

        let window_id = app.new_window()
            .view(Model::view)
            .size(window_width as u32, window_height as u32)
            .raw_event(Model::raw_window_event)
            .build()
            .unwrap();

        let window = app.window(window_id).unwrap();

        Self { 
            egui: Egui::from_window(&window),
            base_maze,
            maze, 
            index: 500,
            width: width as f32,
            height: height as f32,
            scale,
            maze_state: part2::SolveStatus::Pending,
            guard_positions: vec![(0,0)],
            simulation_speed: 10,
            indexes: indexes,
        }
    }

    pub fn update(_app: &App, model: &mut Self, _update: Update) {
        for _ in 0..model.simulation_speed {
            match model.maze_state {
                part2::SolveStatus::Pending => {
                    model.maze_state = match model.maze.step() {
                        part2::SolveStatus::Pending => part2::SolveStatus::Pending,
                        part2::SolveStatus::LoopFound => part2::SolveStatus::LoopFound,
                        part2::SolveStatus::OutOfMaze => part2::SolveStatus::OutOfMaze,
                    };
                    model.guard_positions.push(model.maze.get_guard_pos());
                    continue;
                },
                part2::SolveStatus::LoopFound => { 
                    println!("Found loop in maze {} after {} steps", model.index, model.guard_positions.len());
                }
                part2::SolveStatus::OutOfMaze => {
                    println!("Out of maze with index {} after {} steps", model.index, model.guard_positions.len());
                }
            }   

            model.maze = loop {
                model.index = *model.indexes.choose(&mut rand::thread_rng()).unwrap();
                if let Some(maze) = model.base_maze.create_with_extra_wall(model.index) {
                    break maze;
                }
            };

            model.maze_state = part2::SolveStatus::Pending;
            model.guard_positions = vec![(0,0)];
        }

        let ctx = model.egui.begin_frame();
        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.label(format!("Current Index: {}", model.index));
            ui.label("Simulation Speed");
            ui.add(egui::Slider::new(&mut model.simulation_speed, 1..=10000).logarithmic(true));
        });    
    }

    fn view(app: &App, model: &Model, frame: Frame) {
        let draw = app.draw();
        draw.background().color(srgba(0.2, 0.01, 0.1, 1.0));

        for y in 0..model.height as usize {
            for x in 0..model.width as usize {
                match model.maze.get(x,y) {
                    Some(part2::Cell::Wall { visited: _ }) => {
                        let color = if x + y * model.width as usize == model.index {srgba(1.0, 0.0, 0.0, 1.0)} else {srgba(0.8, 0.0, 0.8, 1.0)};
                        let (x, y) = model.calc_onscreen_pos(x, y);
                        draw.rect()
                            .x(x)
                            .y(y)
                            .w(model.scale)
                            .h(model.scale)
                            .color(color);
                    }
                    Some(part2::Cell::Empty) | None => { }
                }
            }
        }

        let guard_positions_len = model.guard_positions.len();
        for (i, (x, y)) in model.guard_positions.iter().enumerate() {
            let alpha = i as f32 / guard_positions_len as f32;
            let (x, y) = model.calc_onscreen_pos(*x, *y);
            draw.rect()
                .x(x)
                .y(y)
                .w(model.scale)
                .h(model.scale)
                .color(srgba(0.0, 0.7, 0.7, alpha));
        }

        draw.to_frame(app, &frame).unwrap();
        model.egui.draw_to_frame(&frame).unwrap();
    }

    fn calc_onscreen_pos(&self, x: usize, y: usize) -> (f32, f32) {
        (
            (x as f32 - self.width / 2.0 + 0.5) * self.scale, 
            (y as f32 - self.height / 2.0 + 0.5) * self.scale
        )
    }

    fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
        model.egui.handle_raw_event(event);
    }
}



