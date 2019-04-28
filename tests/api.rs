use nakama::api::*;
use nakama::apigrpc::*;
use nakama::apigrpc_grpc::*;

use protobuf::well_known_types::BoolValue;

use grpc::Client;
use grpc::ClientStub;

use bytes::BufMut;

use uuid::Uuid;

use std::sync::Arc;

#[test]
fn authenticate_device_test() {
    let client = Arc::new(Client::new_plain("::1", 7349, Default::default()).unwrap());
    let nakama_client = NakamaClient::with_client(client);

    let mut req_opt = grpc::RequestOptions::new();

    let mut auth_token = bytes::BytesMut::new();

    auth_token.put("Basic ");
    auth_token.put(base64::encode("defaultkey:"));

    req_opt.metadata.add(
        grpc::MetadataKey::from("authorization"),
        auth_token.freeze(),
    );

    let mut acc = AccountDevice::new();

    acc.set_id(Uuid::new_v4().to_string());

    let mut auth_req = AuthenticateDeviceRequest::new();

    let mut create = BoolValue::new();

    create.set_value(true);

    auth_req.set_account(acc);
    auth_req.set_create(create);

    let resp = nakama_client.authenticate_device(req_opt, auth_req);

    let dat = resp.wait();

    println!("{:?}", dat);

    assert!(dat.is_ok());
}

#[test]
fn rpc_func_test() {
    let client = Arc::new(Client::new_plain("::1", 7349, Default::default()).unwrap());
    let nakama_client = NakamaClient::with_client(client);

    let mut auth_req_opt = grpc::RequestOptions::new();

    let mut auth_token = bytes::BytesMut::new();

    auth_token.put("Basic ");
    auth_token.put(base64::encode("defaultkey:"));

    auth_req_opt.metadata.add(
        grpc::MetadataKey::from("authorization"),
        auth_token.freeze(),
    );

    let mut acc = AccountDevice::new();

    acc.set_id(Uuid::new_v4().to_string());

    let mut auth_req = AuthenticateDeviceRequest::new();

    let mut create = BoolValue::new();

    create.set_value(true);

    auth_req.set_account(acc);
    auth_req.set_create(create);

    let resp = nakama_client.authenticate_device(auth_req_opt, auth_req);

    let dat = resp.wait();

    println!("{:?}", dat);

    assert!(dat.is_ok());

    let auth_data = dat.ok().expect("missing authentication data");

    // Rpc request

    let mut rpc_req_opt = grpc::RequestOptions::new();

    let mut bearer_token = bytes::BytesMut::new();

    bearer_token.extend(b"Bearer ");
    bearer_token.extend(auth_data.1.token.bytes());

    rpc_req_opt.metadata.add(
        grpc::MetadataKey::from("authorization"),
        bearer_token.freeze(),
    );

    let mut rpc_req = Rpc::new();

    rpc_req.set_id(String::from("version"));

    let resp = nakama_client.rpc_func(rpc_req_opt, rpc_req);
    let res = resp.wait();

    println!("{:?}", res);

    assert!(res.is_ok());
}
