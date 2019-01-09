// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_LIFE_GAME_GET_FIELD_SIZE: ::grpcio::Method<super::lifegame::FieldSizeRequest, super::lifegame::FieldSizeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lifegame.LifeGame/GetFieldSize",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIFE_GAME_GET_CELLS: ::grpcio::Method<super::lifegame::CellsRequest, super::lifegame::CellsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lifegame.LifeGame/GetCells",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIFE_GAME_UPDATE: ::grpcio::Method<super::lifegame::UpdateRequest, super::lifegame::UpdateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lifegame.LifeGame/Update",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIFE_GAME_RESET_CELLS: ::grpcio::Method<super::lifegame::ResetRequest, super::lifegame::ResetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lifegame.LifeGame/ResetCells",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct LifeGameClient {
    client: ::grpcio::Client,
}

impl LifeGameClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        LifeGameClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_field_size_opt(&self, req: &super::lifegame::FieldSizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lifegame::FieldSizeResponse> {
        self.client.unary_call(&METHOD_LIFE_GAME_GET_FIELD_SIZE, req, opt)
    }

    pub fn get_field_size(&self, req: &super::lifegame::FieldSizeRequest) -> ::grpcio::Result<super::lifegame::FieldSizeResponse> {
        self.get_field_size_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_field_size_async_opt(&self, req: &super::lifegame::FieldSizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lifegame::FieldSizeResponse>> {
        self.client.unary_call_async(&METHOD_LIFE_GAME_GET_FIELD_SIZE, req, opt)
    }

    pub fn get_field_size_async(&self, req: &super::lifegame::FieldSizeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lifegame::FieldSizeResponse>> {
        self.get_field_size_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_cells_opt(&self, req: &super::lifegame::CellsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lifegame::CellsResponse> {
        self.client.unary_call(&METHOD_LIFE_GAME_GET_CELLS, req, opt)
    }

    pub fn get_cells(&self, req: &super::lifegame::CellsRequest) -> ::grpcio::Result<super::lifegame::CellsResponse> {
        self.get_cells_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_cells_async_opt(&self, req: &super::lifegame::CellsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lifegame::CellsResponse>> {
        self.client.unary_call_async(&METHOD_LIFE_GAME_GET_CELLS, req, opt)
    }

    pub fn get_cells_async(&self, req: &super::lifegame::CellsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lifegame::CellsResponse>> {
        self.get_cells_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_opt(&self, req: &super::lifegame::UpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lifegame::UpdateResponse> {
        self.client.unary_call(&METHOD_LIFE_GAME_UPDATE, req, opt)
    }

    pub fn update(&self, req: &super::lifegame::UpdateRequest) -> ::grpcio::Result<super::lifegame::UpdateResponse> {
        self.update_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_async_opt(&self, req: &super::lifegame::UpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lifegame::UpdateResponse>> {
        self.client.unary_call_async(&METHOD_LIFE_GAME_UPDATE, req, opt)
    }

    pub fn update_async(&self, req: &super::lifegame::UpdateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lifegame::UpdateResponse>> {
        self.update_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reset_cells_opt(&self, req: &super::lifegame::ResetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lifegame::ResetResponse> {
        self.client.unary_call(&METHOD_LIFE_GAME_RESET_CELLS, req, opt)
    }

    pub fn reset_cells(&self, req: &super::lifegame::ResetRequest) -> ::grpcio::Result<super::lifegame::ResetResponse> {
        self.reset_cells_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reset_cells_async_opt(&self, req: &super::lifegame::ResetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lifegame::ResetResponse>> {
        self.client.unary_call_async(&METHOD_LIFE_GAME_RESET_CELLS, req, opt)
    }

    pub fn reset_cells_async(&self, req: &super::lifegame::ResetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lifegame::ResetResponse>> {
        self.reset_cells_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait LifeGame {
    fn get_field_size(&mut self, ctx: ::grpcio::RpcContext, req: super::lifegame::FieldSizeRequest, sink: ::grpcio::UnarySink<super::lifegame::FieldSizeResponse>);
    fn get_cells(&mut self, ctx: ::grpcio::RpcContext, req: super::lifegame::CellsRequest, sink: ::grpcio::UnarySink<super::lifegame::CellsResponse>);
    fn update(&mut self, ctx: ::grpcio::RpcContext, req: super::lifegame::UpdateRequest, sink: ::grpcio::UnarySink<super::lifegame::UpdateResponse>);
    fn reset_cells(&mut self, ctx: ::grpcio::RpcContext, req: super::lifegame::ResetRequest, sink: ::grpcio::UnarySink<super::lifegame::ResetResponse>);
}

pub fn create_life_game<S: LifeGame + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIFE_GAME_GET_FIELD_SIZE, move |ctx, req, resp| {
        instance.get_field_size(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIFE_GAME_GET_CELLS, move |ctx, req, resp| {
        instance.get_cells(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIFE_GAME_UPDATE, move |ctx, req, resp| {
        instance.update(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIFE_GAME_RESET_CELLS, move |ctx, req, resp| {
        instance.reset_cells(ctx, req, resp)
    });
    builder.build()
}
