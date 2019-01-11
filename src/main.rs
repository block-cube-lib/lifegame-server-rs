use env_logger;
use futures::sync::oneshot;
use futures::Future;
use grpc_rs::lifegame::{
    CellsRequest, CellsResponse, FieldSizeRequest, FieldSizeResponse, ResetRequest, ResetResponse,
};
use grpc_rs::lifegame_field::LifegameField;
use grpc_rs::lifegame_grpc::Lifegame;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use log::{error, info};
use std::env;
use std::io::Read;
use std::sync::{Arc, Mutex, RwLock};
use std::{io, thread};

#[derive(Clone)]
pub struct LifegameService {
    lifegame_field: Arc<RwLock<LifegameField>>,
}

impl LifegameService {
    pub fn new(width: u32, height: u32) -> LifegameService {
        LifegameService {
            lifegame_field: Arc::new(RwLock::new(LifegameField::new(width, height))),
        }
    }
}

impl Lifegame for LifegameService {
    fn get_field_size(
        &mut self,
        ctx: RpcContext,
        req: FieldSizeRequest,
        sink: UnarySink<FieldSizeResponse>,
    ) {
        let mut resp = FieldSizeResponse::new();
        let field = self.lifegame_field.read().unwrap();
        resp.set_width(field.width());
        resp.set_height(field.height());
        let f = sink
            .success(resp)
            .map_err(move |e| error!("field size error: {:?}, {:?}", req, e));
        ctx.spawn(f)
    }

    fn get_cells(&mut self, ctx: RpcContext, req: CellsRequest, sink: UnarySink<CellsResponse>) {
        let mut resp = CellsResponse::new();
        let field = self.lifegame_field.read().unwrap();
        resp.set_cells(field.cells());
        let f = sink
            .success(resp)
            .map_err(move |e| error!("cells error: {:?}, {:?}", req, e));
        ctx.spawn(f)
    }

    fn reset(&mut self, ctx: RpcContext, req: ResetRequest, sink: UnarySink<ResetResponse>) {
        let mut field = self.lifegame_field.write().unwrap();
        field.reset();
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
    let lifegame_service = LifegameService::new(100, 100);
    let update_lifegame_service = lifegame_service.clone();
    let service = grpc_rs::lifegame_grpc::create_lifegame(lifegame_service);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
        .build()
        .unwrap();
    server.start();

    let running = Arc::new(Mutex::new(true));
    let running_setter = running.clone();

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        let mut running = running_setter.lock().unwrap();
        *running = false;
        tx.send(())
    });

    while *running.lock().unwrap() {
        let mut field = update_lifegame_service.lifegame_field.write().unwrap();
        field.update_cells();
        let sleep_time = std::time::Duration::from_millis(100);
        thread::sleep(sleep_time);
    }

    let _ = rx.wait();
    let _ = server.shutdown();
}
