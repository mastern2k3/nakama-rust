use nakama::api::*;
use nakama::apigrpc::*;
use nakama::apigrpc_grpc::*;

use grpc::Client;
use grpc::ClientStub;

use std::sync::Arc;

#[test]
fn some_test() {
    // let conf = grpc::ClientConf::new();
    // let client = Arc::new(Client::new_plain("::1", 50051, Default::default()).unwrap());
    let client = Arc::new(Client::new_plain("127.0.0.1", 7350, Default::default()).unwrap());
    let nakama_client = NakamaClient::with_client(client);

    let mut acc = AccountDevice::new();

    acc.set_id(String::from("lol"));

    let mut auth_req = AuthenticateDeviceRequest::new();

    auth_req.set_account(acc);

    let resp = nakama_client.authenticate_device(grpc::RequestOptions::new(), auth_req);

    println!("{:?}", resp.wait());
}
