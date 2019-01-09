use env_logger;
use futures::sync::oneshot;
use futures::Future;
use grpc_rs::lifegame::{
    Cell, CellsRequest, CellsResponse, FieldSizeRequest, FieldSizeResponse, ResetRequest,
    ResetResponse, UpdateRequest, UpdateResponse,
};
use grpc_rs::lifegame_grpc::LifeGame;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use log::{error, info};
use std::env;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{io, thread};

pub struct LifeGameService {
    width: u32,
    height: u32,
    cells: Arc<Mutex<Vec<Cell>>>,
    next_cells: Arc<Mutex<Vec<Cell>>>,
}

impl Clone for LifeGameService {
    fn clone(&self) -> LifeGameService {
        LifeGameService {
            width: self.width,
            height: self.height,
            cells: self.cells.clone(),
            next_cells: self.next_cells.clone(),
        }
    }
}

impl LifeGameService {
    pub fn new(width: u32, height: u32) -> LifeGameService {
        let mut s = LifeGameService {
            width,
            height,
            cells: Arc::new(Mutex::new(vec![])),
            next_cells: Arc::new(Mutex::new(vec![])),
        };
        s.reset();
        s
    }

    pub fn reset(&mut self) {
        let size = (self.width * self.height) as usize;
        let mut cells = self.cells.lock().unwrap();
        *cells = vec![0; size]
            .into_iter()
            .map(|_| rand::random())
            .map(|b| if b { Cell::Alive } else { Cell::Dead })
            .collect();
        let mut next_cells = self.next_cells.lock().unwrap();
        *next_cells = cells.clone();
    }

    pub fn update_cells(&mut self) {
        let mut cells = self.cells.lock().unwrap();
        let mut next_cells = self.next_cells.lock().unwrap();
        for i in 0..cells.len() {
            let x = (i % self.width as usize) as i32;
            let y = (i / self.width as usize) as i32;
            next_cells[i] = Self::get_next_cell_state(&cells, self.width, self.height, x, y);
        }
        *cells = next_cells.to_vec();

    }

    fn get_cell_state(cells: &Vec<Cell>, width: u32, height: u32, x: i32, y: i32) -> Cell {
        let mut x = x;
        let mut y = y;
        let width = width as i32;
        let height = height as i32;
        x = if x < 0 {
            x + width
        } else if width <= x {
            x % width
        } else {
            x
        };
        y = if y < 0 {
            y + height
        } else if height <= y {
            y % height
        } else {
            y
        };

        let index = width * y + x;
        cells[index as usize]
    }

    fn get_next_cell_state(cells: &Vec<Cell>, width: u32, height: u32, x: i32, y: i32) -> Cell {
        let around_cell_indices = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        let around_alive_count = around_cell_indices
            .into_iter()
            .map(|(ix, iy)| Self::get_cell_state(&cells, width, height, *ix, *iy))
            .filter(move |cell_state| *cell_state == Cell::Alive)
            .count();

        match around_alive_count {
            0..=1 => Cell::Dead,
            2 => Self::get_cell_state(&cells, width, height, x, y),
            3 => Cell::Alive,
            _ => Cell::Dead,
        }
    }
}

impl LifeGame for LifeGameService {
    fn get_field_size(
        &mut self,
        ctx: RpcContext,
        req: FieldSizeRequest,
        sink: UnarySink<FieldSizeResponse>,
    ) {
        let mut resp = FieldSizeResponse::new();
        resp.set_width(self.width);
        resp.set_height(self.height);
        let f = sink
            .success(resp)
            .map_err(move |e| error!("field size error: {:?}, {:?}", req, e));
        ctx.spawn(f)
    }

    fn get_cells(&mut self, ctx: RpcContext, req: CellsRequest, sink: UnarySink<CellsResponse>) {
        let mut resp = CellsResponse::new();
        let cells = self.cells.lock().unwrap();
        resp.set_cells(cells.clone());
        let f = sink
            .success(resp)
            .map_err(move |e| error!("cells error: {:?}, {:?}", req, e));
        ctx.spawn(f)
    }

    fn update(&mut self, ctx: RpcContext, req: UpdateRequest, sink: UnarySink<UpdateResponse>) {
        self.update_cells();

        let cells = self.cells.lock().unwrap();
        let mut resp = UpdateResponse::new();
        resp.set_cells(cells.clone());
        let f = sink
            .success(resp)
            .map_err(move |e| error!("cells error: {:?}, {:?}", req, e));
        ctx.spawn(f)
    }

    fn reset_cells(&mut self, ctx: RpcContext, req: ResetRequest, sink: UnarySink<ResetResponse>) {
        self.reset();
        let resp = ResetResponse::new();
        let f = sink
            .success(resp)
            .map_err(move |e| error!("cells error: {:?}, {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let env = Arc::new(Environment::new(1));
    let lifegame_service = LifeGameService::new(100, 100);
    let service = grpc_rs::lifegame_grpc::create_life_game(lifegame_service);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
        .build()
        .unwrap();
    server.start();

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });

    let _ = rx.wait();
    let _ = server.shutdown();
}
