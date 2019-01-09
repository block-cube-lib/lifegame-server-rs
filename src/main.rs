use env_logger;
use futures::sync::oneshot;
use futures::Future;
use grpc_rs::lifegame::{
    Cell, CellsRequest, CellsResponse, FieldSizeRequest, FieldSizeResponse, ResetRequest,
    ResetResponse, UpdateRequest, UpdateResponse,
};
use grpc_rs::lifegame_grpc::LifeGame;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use log::{debug, error, info};
use std::env;
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

//use grpc_rs::helloworld::{HelloReply, HelloRequest};
//use grpc_rs::helloworld_grpc::Greeter;

//#[derive(Clone)]
//struct GreeterService;

//impl Greeter for GreeterService {
//    fn say_hello(&mut self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {
//        debug!("request: name = {}", req.get_name());
//        let message = format!("Hello {}", req.get_name());
//        let mut resp = HelloReply::new();
//        resp.set_message(message);
//        let f = sink
//            .success(resp)
//            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
//        ctx.spawn(f)
//    }
//}

pub struct LifeGameService {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    next_cells: Vec<Cell>,
    id: u32,
}

impl Clone for LifeGameService {
    fn clone(&self) -> LifeGameService {
        LifeGameService {
            width: self.width,
            height: self.height,
            cells: self.cells.clone(),
            next_cells: self.next_cells.clone(),
            id: self.id + 1
        }
    }
}

impl LifeGameService {
    pub fn new(width: u32, height: u32) -> LifeGameService {
        let mut s = LifeGameService {
            width,
            height,
            cells: vec![],
            next_cells: vec![],
            id: 0,
        };
        s.reset();
        s
    }

    pub fn reset(&mut self) {
        let size = (self.width * self.height) as usize;
        self.cells = vec![0; size]
            .into_iter()
            .map(|_| rand::random())
            .map(|b| if b { Cell::Alive } else { Cell::Dead })
            .collect();
        self.next_cells = self.cells.clone();
    }

    fn get_cell_state(&self, x: i32, y: i32) -> Cell {
        let mut x = x;
        let mut y = y;
        let width = self.width as i32;
        let height = self.height as i32;
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
        self.cells[index as usize]
    }

    fn get_next_cell(&self, x: i32, y: i32) -> Cell {
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
            .map(move |(ix, iy)| self.get_cell_state(*ix, *iy))
            .filter(move |cell_state| *cell_state == Cell::Alive)
            .count();

        match around_alive_count {
            0..=1 => Cell::Dead,
            2 => self.get_cell_state(x, y),
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
        debug!("get_field_size: id = {}", self.id);
        let mut resp = FieldSizeResponse::new();
        resp.set_width(self.width);
        resp.set_height(self.height);
        let f = sink
            .success(resp)
            .map_err(move |e| error!("field size error: {:?}, {:?}", req, e));
        ctx.spawn(f)
    }

    fn get_cells(&mut self, ctx: RpcContext, req: CellsRequest, sink: UnarySink<CellsResponse>) {
        debug!("get_cells: id = {}", self.id);
        let mut resp = CellsResponse::new();
        let cells = self.cells.clone();
        resp.set_cells(cells);
        let f = sink
            .success(resp)
            .map_err(move |e| error!("cells error: {:?}, {:?}", req, e));
        ctx.spawn(f)
    }

    fn update(&mut self, ctx: RpcContext, req: UpdateRequest, sink: UnarySink<UpdateResponse>) {
        debug!("update: id = {}", self.id);
        for i in 0..self.cells.len() {
            let x = (i % self.width as usize) as i32;
            let y = (i / self.width as usize) as i32;
            self.next_cells[i] = self.get_next_cell(x, y);
        }
        self.cells = self.next_cells.to_vec();

        let mut resp = UpdateResponse::new();
        resp.set_cells(self.cells.clone());
        let f = sink
            .success(resp)
            .map_err(move |e| error!("cells error: {:?}, {:?}", req, e));
        ctx.spawn(f)
    }

    fn reset_cells(&mut self, ctx: RpcContext, req: ResetRequest, sink: UnarySink<ResetResponse>) {
        debug!("reset_cells: id = {}", self.id);
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
    let mut lifegame_service = LifeGameService::new(100, 100);
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

//fn _helloworld() {
//    let env = Arc::new(Environment::new(1));
//    let service = grpc_rs::helloworld_grpc::create_greeter(GreeterService);
//    let mut server = ServerBuilder::new(env)
//        .register_service(service)
//        .bind("127.0.0.1", 50_051)
//        .build()
//        .unwrap();
//    server.start();
//
//    for &(ref host, port) in server.bind_addrs() {
//        info!("listening on {}:{}", host, port);
//    }
//
//    let (tx, rx) = oneshot::channel();
//    thread::spawn(move || {
//        info!("Press ENTER to exit...");
//        let _ = io::stdin().read(&mut [0]).unwrap();
//        tx.send(())
//    });
//
//    let _ = rx.wait();
//    let _ = server.shutdown();
//}
