// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_MINI_KV_SERVER_GET: ::grpcio::Method<super::minikv::GetRequest, super::minikv::GetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MiniKVServer/Get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MINI_KV_SERVER_PUT: ::grpcio::Method<super::minikv::PutRequest, super::minikv::PutResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MiniKVServer/Put",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MINI_KV_SERVER_DELETE: ::grpcio::Method<super::minikv::DeleteRequest, super::minikv::DeleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MiniKVServer/Delete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MINI_KV_SERVER_SCAN: ::grpcio::Method<super::minikv::ScanRequest, super::minikv::ScanResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MiniKVServer/Scan",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct MiniKvServerClient {
    client: ::grpcio::Client,
}

impl MiniKvServerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MiniKvServerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_opt(&self, req: &super::minikv::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::minikv::GetResponse> {
        self.client.unary_call(&METHOD_MINI_KV_SERVER_GET, req, opt)
    }

    pub fn get(&self, req: &super::minikv::GetRequest) -> ::grpcio::Result<super::minikv::GetResponse> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::minikv::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::minikv::GetResponse>> {
        self.client.unary_call_async(&METHOD_MINI_KV_SERVER_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::minikv::GetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::minikv::GetResponse>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_opt(&self, req: &super::minikv::PutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::minikv::PutResponse> {
        self.client.unary_call(&METHOD_MINI_KV_SERVER_PUT, req, opt)
    }

    pub fn put(&self, req: &super::minikv::PutRequest) -> ::grpcio::Result<super::minikv::PutResponse> {
        self.put_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_async_opt(&self, req: &super::minikv::PutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::minikv::PutResponse>> {
        self.client.unary_call_async(&METHOD_MINI_KV_SERVER_PUT, req, opt)
    }

    pub fn put_async(&self, req: &super::minikv::PutRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::minikv::PutResponse>> {
        self.put_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(&self, req: &super::minikv::DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::minikv::DeleteResponse> {
        self.client.unary_call(&METHOD_MINI_KV_SERVER_DELETE, req, opt)
    }

    pub fn delete(&self, req: &super::minikv::DeleteRequest) -> ::grpcio::Result<super::minikv::DeleteResponse> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(&self, req: &super::minikv::DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::minikv::DeleteResponse>> {
        self.client.unary_call_async(&METHOD_MINI_KV_SERVER_DELETE, req, opt)
    }

    pub fn delete_async(&self, req: &super::minikv::DeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::minikv::DeleteResponse>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn scan_opt(&self, req: &super::minikv::ScanRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::minikv::ScanResponse> {
        self.client.unary_call(&METHOD_MINI_KV_SERVER_SCAN, req, opt)
    }

    pub fn scan(&self, req: &super::minikv::ScanRequest) -> ::grpcio::Result<super::minikv::ScanResponse> {
        self.scan_opt(req, ::grpcio::CallOption::default())
    }

    pub fn scan_async_opt(&self, req: &super::minikv::ScanRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::minikv::ScanResponse>> {
        self.client.unary_call_async(&METHOD_MINI_KV_SERVER_SCAN, req, opt)
    }

    pub fn scan_async(&self, req: &super::minikv::ScanRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::minikv::ScanResponse>> {
        self.scan_async_opt(req, ::grpcio::CallOption::default())
    }
    // pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
    //     self.client.spawn(f)
    // }
}

pub trait MiniKvServer {
    fn get(&mut self, ctx: ::grpcio::RpcContext, req: super::minikv::GetRequest, sink: ::grpcio::UnarySink<super::minikv::GetResponse>);
    fn put(&mut self, ctx: ::grpcio::RpcContext, req: super::minikv::PutRequest, sink: ::grpcio::UnarySink<super::minikv::PutResponse>);
    fn delete(&mut self, ctx: ::grpcio::RpcContext, req: super::minikv::DeleteRequest, sink: ::grpcio::UnarySink<super::minikv::DeleteResponse>);
    fn scan(&mut self, ctx: ::grpcio::RpcContext, req: super::minikv::ScanRequest, sink: ::grpcio::UnarySink<super::minikv::ScanResponse>);
}

pub fn create_mini_kv_server<S: MiniKvServer + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MINI_KV_SERVER_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MINI_KV_SERVER_PUT, move |ctx, req, resp| {
        instance.put(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MINI_KV_SERVER_DELETE, move |ctx, req, resp| {
        instance.delete(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_MINI_KV_SERVER_SCAN, move |ctx, req, resp| {
        instance.scan(ctx, req, resp)
    });
    builder.build()
}
