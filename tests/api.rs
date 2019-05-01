use nakama::api::*;
use nakama::apigrpc::*;
use nakama::apigrpc_grpc::*;
use nakama::realtime::*;

use protobuf::well_known_types::BoolValue;
use protobuf::Message;

use grpc::Client;
use grpc::ClientStub;

use bytes::Bytes;

use uuid::Uuid;

use std::sync::Arc;

#[test]
fn authenticate_device_test() {
    let client = Arc::new(Client::new_plain("::1", 7349, Default::default()).unwrap());
    let nakama_client = NakamaClient::with_client(client);

    let mut req_opt = grpc::RequestOptions::new();

    req_opt.metadata.add(
        grpc::MetadataKey::from("authorization"),
        Bytes::from(format!("Basic {}", base64::encode("defaultkey:"))),
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

    auth_req_opt.metadata.add(
        grpc::MetadataKey::from("authorization"),
        Bytes::from(format!("Basic {}", base64::encode("defaultkey:"))),
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

    rpc_req_opt.metadata.add(
        grpc::MetadataKey::from("authorization"),
        Bytes::from(format!("Bearer {}", auth_data.1.token)),
    );

    let mut rpc_req = Rpc::new();

    rpc_req.set_id(String::from("version"));

    let resp = nakama_client.rpc_func(rpc_req_opt, rpc_req);
    let res = resp.wait();

    println!("{:?}", res);

    assert!(res.is_ok());
}

#[test]
fn match_data_test() {
    let client = Arc::new(Client::new_plain("::1", 7349, Default::default()).unwrap());
    let nakama_client = NakamaClient::with_client(client);

    let mut auth_req_opt = grpc::RequestOptions::new();

    auth_req_opt.metadata.add(
        grpc::MetadataKey::from("authorization"),
        Bytes::from(format!("Basic {}", base64::encode("defaultkey:"))),
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

    let auth_data = dat.unwrap();

    // Rpc request
    let mut rpc_req_opt = grpc::RequestOptions::new();

    rpc_req_opt.metadata.add(
        grpc::MetadataKey::from("authorization"),
        Bytes::from(format!("Bearer {}", auth_data.1.token)),
    );

    let mut rpc_req = Rpc::new();

    rpc_req.set_id(String::from("debug_match"));

    let resp = nakama_client.rpc_func(rpc_req_opt, rpc_req);
    let res = resp.wait();

    println!("{:?}", res);

    assert!(res.is_ok());

    let new_match_res = res.unwrap();

    let new_match_id = new_match_res.1.payload;

    let mut client_builder = websocket::ClientBuilder::new(&format!(
        "ws://[::1]:7350/ws?format=protobuf&lang=en&status=true&token={}",
        auth_data.1.token
    ))
    .unwrap();

    let mut client = client_builder.connect_insecure().unwrap();

    let mut join_match = MatchJoin::new();

    join_match.set_match_id(new_match_id);

    let mut envelope = Envelope::new();
    envelope.set_match_join(join_match);

    let bytes = envelope.write_to_bytes().unwrap();

    let message = websocket::Message::binary(bytes);

    let res = client.send_message(&message).unwrap();

    println!("{:?}", res);

    let (tx, rx) = std::sync::mpsc::channel::<Envelope>();

    let receive_loop = std::thread::spawn(move || loop {
        let ret_message = client.recv_message();

        let msg = ret_message.unwrap();

        println!("{:?}", msg);

        match msg {
            websocket::OwnedMessage::Binary(data) => {
                let envelope = protobuf::parse_from_bytes::<Envelope>(&data).unwrap();
                println!("envelope: {:?}", envelope);
                tx.send(envelope).unwrap();
            }
            _ => (),
        }
    });

    receive_loop.join().unwrap();
}
