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

    let mut acc = AccountDevice::new();

    acc.set_id(Uuid::new_v4().to_string());

    let mut auth_req = AuthenticateDeviceRequest::new();

    let mut create = BoolValue::new();

    create.set_value(true);

    auth_req.set_account(acc);
    auth_req.set_create(create);

    let mut req_opt = grpc::RequestOptions::new();

    let mut auth_token = bytes::BytesMut::new();

    auth_token.put("Basic ");
    auth_token.put(base64::encode("defaultkey:"));

    req_opt.metadata.add(
        grpc::MetadataKey::from("authorization"),
        auth_token.freeze(),
    );

    let resp = nakama_client.authenticate_device(req_opt, auth_req);

    let dat = resp.wait();

    println!("{:?}", dat);

    assert!(dat.is_ok());
}
