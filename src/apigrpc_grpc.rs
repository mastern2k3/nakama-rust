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


// interface

pub trait Nakama {
    fn add_friends(&self, o: ::grpc::RequestOptions, p: super::api::AddFriendsRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn add_group_users(&self, o: ::grpc::RequestOptions, p: super::api::AddGroupUsersRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn authenticate_custom(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateCustomRequest) -> ::grpc::SingleResponse<super::api::Session>;

    fn authenticate_device(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateDeviceRequest) -> ::grpc::SingleResponse<super::api::Session>;

    fn authenticate_email(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateEmailRequest) -> ::grpc::SingleResponse<super::api::Session>;

    fn authenticate_facebook(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateFacebookRequest) -> ::grpc::SingleResponse<super::api::Session>;

    fn authenticate_game_center(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateGameCenterRequest) -> ::grpc::SingleResponse<super::api::Session>;

    fn authenticate_google(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateGoogleRequest) -> ::grpc::SingleResponse<super::api::Session>;

    fn authenticate_steam(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateSteamRequest) -> ::grpc::SingleResponse<super::api::Session>;

    fn block_friends(&self, o: ::grpc::RequestOptions, p: super::api::BlockFriendsRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn create_group(&self, o: ::grpc::RequestOptions, p: super::api::CreateGroupRequest) -> ::grpc::SingleResponse<super::api::Group>;

    fn delete_friends(&self, o: ::grpc::RequestOptions, p: super::api::DeleteFriendsRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn delete_group(&self, o: ::grpc::RequestOptions, p: super::api::DeleteGroupRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn delete_leaderboard_record(&self, o: ::grpc::RequestOptions, p: super::api::DeleteLeaderboardRecordRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn delete_notifications(&self, o: ::grpc::RequestOptions, p: super::api::DeleteNotificationsRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn delete_storage_objects(&self, o: ::grpc::RequestOptions, p: super::api::DeleteStorageObjectsRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn get_account(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::api::Account>;

    fn get_users(&self, o: ::grpc::RequestOptions, p: super::api::GetUsersRequest) -> ::grpc::SingleResponse<super::api::Users>;

    fn healthcheck(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn import_facebook_friends(&self, o: ::grpc::RequestOptions, p: super::api::ImportFacebookFriendsRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn join_group(&self, o: ::grpc::RequestOptions, p: super::api::JoinGroupRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn join_tournament(&self, o: ::grpc::RequestOptions, p: super::api::JoinTournamentRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn kick_group_users(&self, o: ::grpc::RequestOptions, p: super::api::KickGroupUsersRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn leave_group(&self, o: ::grpc::RequestOptions, p: super::api::LeaveGroupRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn link_custom(&self, o: ::grpc::RequestOptions, p: super::api::AccountCustom) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn link_device(&self, o: ::grpc::RequestOptions, p: super::api::AccountDevice) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn link_email(&self, o: ::grpc::RequestOptions, p: super::api::AccountEmail) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn link_facebook(&self, o: ::grpc::RequestOptions, p: super::api::LinkFacebookRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn link_game_center(&self, o: ::grpc::RequestOptions, p: super::api::AccountGameCenter) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn link_google(&self, o: ::grpc::RequestOptions, p: super::api::AccountGoogle) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn link_steam(&self, o: ::grpc::RequestOptions, p: super::api::AccountSteam) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn list_channel_messages(&self, o: ::grpc::RequestOptions, p: super::api::ListChannelMessagesRequest) -> ::grpc::SingleResponse<super::api::ChannelMessageList>;

    fn list_friends(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::api::Friends>;

    fn list_groups(&self, o: ::grpc::RequestOptions, p: super::api::ListGroupsRequest) -> ::grpc::SingleResponse<super::api::GroupList>;

    fn list_group_users(&self, o: ::grpc::RequestOptions, p: super::api::ListGroupUsersRequest) -> ::grpc::SingleResponse<super::api::GroupUserList>;

    fn list_leaderboard_records(&self, o: ::grpc::RequestOptions, p: super::api::ListLeaderboardRecordsRequest) -> ::grpc::SingleResponse<super::api::LeaderboardRecordList>;

    fn list_leaderboard_records_around_owner(&self, o: ::grpc::RequestOptions, p: super::api::ListLeaderboardRecordsAroundOwnerRequest) -> ::grpc::SingleResponse<super::api::LeaderboardRecordList>;

    fn list_matches(&self, o: ::grpc::RequestOptions, p: super::api::ListMatchesRequest) -> ::grpc::SingleResponse<super::api::MatchList>;

    fn list_notifications(&self, o: ::grpc::RequestOptions, p: super::api::ListNotificationsRequest) -> ::grpc::SingleResponse<super::api::NotificationList>;

    fn list_storage_objects(&self, o: ::grpc::RequestOptions, p: super::api::ListStorageObjectsRequest) -> ::grpc::SingleResponse<super::api::StorageObjectList>;

    fn list_tournaments(&self, o: ::grpc::RequestOptions, p: super::api::ListTournamentsRequest) -> ::grpc::SingleResponse<super::api::TournamentList>;

    fn list_tournament_records(&self, o: ::grpc::RequestOptions, p: super::api::ListTournamentRecordsRequest) -> ::grpc::SingleResponse<super::api::TournamentRecordList>;

    fn list_tournament_records_around_owner(&self, o: ::grpc::RequestOptions, p: super::api::ListTournamentRecordsAroundOwnerRequest) -> ::grpc::SingleResponse<super::api::TournamentRecordList>;

    fn list_user_groups(&self, o: ::grpc::RequestOptions, p: super::api::ListUserGroupsRequest) -> ::grpc::SingleResponse<super::api::UserGroupList>;

    fn promote_group_users(&self, o: ::grpc::RequestOptions, p: super::api::PromoteGroupUsersRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn read_storage_objects(&self, o: ::grpc::RequestOptions, p: super::api::ReadStorageObjectsRequest) -> ::grpc::SingleResponse<super::api::StorageObjects>;

    fn rpc_func(&self, o: ::grpc::RequestOptions, p: super::api::Rpc) -> ::grpc::SingleResponse<super::api::Rpc>;

    fn unlink_custom(&self, o: ::grpc::RequestOptions, p: super::api::AccountCustom) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn unlink_device(&self, o: ::grpc::RequestOptions, p: super::api::AccountDevice) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn unlink_email(&self, o: ::grpc::RequestOptions, p: super::api::AccountEmail) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn unlink_facebook(&self, o: ::grpc::RequestOptions, p: super::api::AccountFacebook) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn unlink_game_center(&self, o: ::grpc::RequestOptions, p: super::api::AccountGameCenter) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn unlink_google(&self, o: ::grpc::RequestOptions, p: super::api::AccountGoogle) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn unlink_steam(&self, o: ::grpc::RequestOptions, p: super::api::AccountSteam) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn update_account(&self, o: ::grpc::RequestOptions, p: super::api::UpdateAccountRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn update_group(&self, o: ::grpc::RequestOptions, p: super::api::UpdateGroupRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn write_leaderboard_record(&self, o: ::grpc::RequestOptions, p: super::api::WriteLeaderboardRecordRequest) -> ::grpc::SingleResponse<super::api::LeaderboardRecord>;

    fn write_storage_objects(&self, o: ::grpc::RequestOptions, p: super::api::WriteStorageObjectsRequest) -> ::grpc::SingleResponse<super::api::StorageObjectAcks>;

    fn write_tournament_record(&self, o: ::grpc::RequestOptions, p: super::api::WriteTournamentRecordRequest) -> ::grpc::SingleResponse<super::api::LeaderboardRecord>;
}

// client

pub struct NakamaClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_AddFriends: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AddFriendsRequest, super::empty::Empty>>,
    method_AddGroupUsers: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AddGroupUsersRequest, super::empty::Empty>>,
    method_AuthenticateCustom: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AuthenticateCustomRequest, super::api::Session>>,
    method_AuthenticateDevice: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AuthenticateDeviceRequest, super::api::Session>>,
    method_AuthenticateEmail: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AuthenticateEmailRequest, super::api::Session>>,
    method_AuthenticateFacebook: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AuthenticateFacebookRequest, super::api::Session>>,
    method_AuthenticateGameCenter: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AuthenticateGameCenterRequest, super::api::Session>>,
    method_AuthenticateGoogle: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AuthenticateGoogleRequest, super::api::Session>>,
    method_AuthenticateSteam: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AuthenticateSteamRequest, super::api::Session>>,
    method_BlockFriends: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::BlockFriendsRequest, super::empty::Empty>>,
    method_CreateGroup: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::CreateGroupRequest, super::api::Group>>,
    method_DeleteFriends: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::DeleteFriendsRequest, super::empty::Empty>>,
    method_DeleteGroup: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::DeleteGroupRequest, super::empty::Empty>>,
    method_DeleteLeaderboardRecord: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::DeleteLeaderboardRecordRequest, super::empty::Empty>>,
    method_DeleteNotifications: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::DeleteNotificationsRequest, super::empty::Empty>>,
    method_DeleteStorageObjects: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::DeleteStorageObjectsRequest, super::empty::Empty>>,
    method_GetAccount: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::empty::Empty, super::api::Account>>,
    method_GetUsers: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::GetUsersRequest, super::api::Users>>,
    method_Healthcheck: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::empty::Empty, super::empty::Empty>>,
    method_ImportFacebookFriends: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ImportFacebookFriendsRequest, super::empty::Empty>>,
    method_JoinGroup: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::JoinGroupRequest, super::empty::Empty>>,
    method_JoinTournament: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::JoinTournamentRequest, super::empty::Empty>>,
    method_KickGroupUsers: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::KickGroupUsersRequest, super::empty::Empty>>,
    method_LeaveGroup: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::LeaveGroupRequest, super::empty::Empty>>,
    method_LinkCustom: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountCustom, super::empty::Empty>>,
    method_LinkDevice: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountDevice, super::empty::Empty>>,
    method_LinkEmail: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountEmail, super::empty::Empty>>,
    method_LinkFacebook: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::LinkFacebookRequest, super::empty::Empty>>,
    method_LinkGameCenter: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountGameCenter, super::empty::Empty>>,
    method_LinkGoogle: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountGoogle, super::empty::Empty>>,
    method_LinkSteam: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountSteam, super::empty::Empty>>,
    method_ListChannelMessages: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListChannelMessagesRequest, super::api::ChannelMessageList>>,
    method_ListFriends: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::empty::Empty, super::api::Friends>>,
    method_ListGroups: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListGroupsRequest, super::api::GroupList>>,
    method_ListGroupUsers: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListGroupUsersRequest, super::api::GroupUserList>>,
    method_ListLeaderboardRecords: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListLeaderboardRecordsRequest, super::api::LeaderboardRecordList>>,
    method_ListLeaderboardRecordsAroundOwner: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListLeaderboardRecordsAroundOwnerRequest, super::api::LeaderboardRecordList>>,
    method_ListMatches: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListMatchesRequest, super::api::MatchList>>,
    method_ListNotifications: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListNotificationsRequest, super::api::NotificationList>>,
    method_ListStorageObjects: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListStorageObjectsRequest, super::api::StorageObjectList>>,
    method_ListTournaments: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListTournamentsRequest, super::api::TournamentList>>,
    method_ListTournamentRecords: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListTournamentRecordsRequest, super::api::TournamentRecordList>>,
    method_ListTournamentRecordsAroundOwner: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListTournamentRecordsAroundOwnerRequest, super::api::TournamentRecordList>>,
    method_ListUserGroups: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ListUserGroupsRequest, super::api::UserGroupList>>,
    method_PromoteGroupUsers: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::PromoteGroupUsersRequest, super::empty::Empty>>,
    method_ReadStorageObjects: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::ReadStorageObjectsRequest, super::api::StorageObjects>>,
    method_RpcFunc: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::Rpc, super::api::Rpc>>,
    method_UnlinkCustom: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountCustom, super::empty::Empty>>,
    method_UnlinkDevice: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountDevice, super::empty::Empty>>,
    method_UnlinkEmail: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountEmail, super::empty::Empty>>,
    method_UnlinkFacebook: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountFacebook, super::empty::Empty>>,
    method_UnlinkGameCenter: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountGameCenter, super::empty::Empty>>,
    method_UnlinkGoogle: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountGoogle, super::empty::Empty>>,
    method_UnlinkSteam: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::AccountSteam, super::empty::Empty>>,
    method_UpdateAccount: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::UpdateAccountRequest, super::empty::Empty>>,
    method_UpdateGroup: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::UpdateGroupRequest, super::empty::Empty>>,
    method_WriteLeaderboardRecord: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::WriteLeaderboardRecordRequest, super::api::LeaderboardRecord>>,
    method_WriteStorageObjects: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::WriteStorageObjectsRequest, super::api::StorageObjectAcks>>,
    method_WriteTournamentRecord: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::api::WriteTournamentRecordRequest, super::api::LeaderboardRecord>>,
}

impl ::grpc::ClientStub for NakamaClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        NakamaClient {
            grpc_client: grpc_client,
            method_AddFriends: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/AddFriends".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AddGroupUsers: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/AddGroupUsers".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AuthenticateCustom: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/AuthenticateCustom".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AuthenticateDevice: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/AuthenticateDevice".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AuthenticateEmail: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/AuthenticateEmail".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AuthenticateFacebook: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/AuthenticateFacebook".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AuthenticateGameCenter: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/AuthenticateGameCenter".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AuthenticateGoogle: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/AuthenticateGoogle".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AuthenticateSteam: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/AuthenticateSteam".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BlockFriends: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/BlockFriends".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CreateGroup: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/CreateGroup".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteFriends: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/DeleteFriends".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteGroup: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/DeleteGroup".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteLeaderboardRecord: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/DeleteLeaderboardRecord".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteNotifications: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/DeleteNotifications".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteStorageObjects: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/DeleteStorageObjects".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetAccount: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/GetAccount".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetUsers: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/GetUsers".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Healthcheck: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/Healthcheck".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ImportFacebookFriends: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ImportFacebookFriends".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_JoinGroup: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/JoinGroup".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_JoinTournament: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/JoinTournament".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_KickGroupUsers: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/KickGroupUsers".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LeaveGroup: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/LeaveGroup".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LinkCustom: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/LinkCustom".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LinkDevice: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/LinkDevice".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LinkEmail: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/LinkEmail".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LinkFacebook: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/LinkFacebook".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LinkGameCenter: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/LinkGameCenter".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LinkGoogle: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/LinkGoogle".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LinkSteam: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/LinkSteam".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListChannelMessages: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListChannelMessages".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListFriends: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListFriends".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListGroups: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListGroups".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListGroupUsers: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListGroupUsers".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListLeaderboardRecords: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListLeaderboardRecords".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListLeaderboardRecordsAroundOwner: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListLeaderboardRecordsAroundOwner".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListMatches: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListMatches".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListNotifications: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListNotifications".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListStorageObjects: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListStorageObjects".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListTournaments: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListTournaments".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListTournamentRecords: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListTournamentRecords".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListTournamentRecordsAroundOwner: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListTournamentRecordsAroundOwner".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListUserGroups: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ListUserGroups".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_PromoteGroupUsers: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/PromoteGroupUsers".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ReadStorageObjects: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/ReadStorageObjects".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RpcFunc: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/RpcFunc".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UnlinkCustom: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/UnlinkCustom".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UnlinkDevice: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/UnlinkDevice".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UnlinkEmail: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/UnlinkEmail".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UnlinkFacebook: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/UnlinkFacebook".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UnlinkGameCenter: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/UnlinkGameCenter".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UnlinkGoogle: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/UnlinkGoogle".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UnlinkSteam: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/UnlinkSteam".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UpdateAccount: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/UpdateAccount".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UpdateGroup: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/UpdateGroup".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_WriteLeaderboardRecord: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/WriteLeaderboardRecord".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_WriteStorageObjects: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/WriteStorageObjects".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_WriteTournamentRecord: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/nakama.api.Nakama/WriteTournamentRecord".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Nakama for NakamaClient {
    fn add_friends(&self, o: ::grpc::RequestOptions, p: super::api::AddFriendsRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_AddFriends.clone())
    }

    fn add_group_users(&self, o: ::grpc::RequestOptions, p: super::api::AddGroupUsersRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_AddGroupUsers.clone())
    }

    fn authenticate_custom(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateCustomRequest) -> ::grpc::SingleResponse<super::api::Session> {
        self.grpc_client.call_unary(o, p, self.method_AuthenticateCustom.clone())
    }

    fn authenticate_device(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateDeviceRequest) -> ::grpc::SingleResponse<super::api::Session> {
        self.grpc_client.call_unary(o, p, self.method_AuthenticateDevice.clone())
    }

    fn authenticate_email(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateEmailRequest) -> ::grpc::SingleResponse<super::api::Session> {
        self.grpc_client.call_unary(o, p, self.method_AuthenticateEmail.clone())
    }

    fn authenticate_facebook(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateFacebookRequest) -> ::grpc::SingleResponse<super::api::Session> {
        self.grpc_client.call_unary(o, p, self.method_AuthenticateFacebook.clone())
    }

    fn authenticate_game_center(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateGameCenterRequest) -> ::grpc::SingleResponse<super::api::Session> {
        self.grpc_client.call_unary(o, p, self.method_AuthenticateGameCenter.clone())
    }

    fn authenticate_google(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateGoogleRequest) -> ::grpc::SingleResponse<super::api::Session> {
        self.grpc_client.call_unary(o, p, self.method_AuthenticateGoogle.clone())
    }

    fn authenticate_steam(&self, o: ::grpc::RequestOptions, p: super::api::AuthenticateSteamRequest) -> ::grpc::SingleResponse<super::api::Session> {
        self.grpc_client.call_unary(o, p, self.method_AuthenticateSteam.clone())
    }

    fn block_friends(&self, o: ::grpc::RequestOptions, p: super::api::BlockFriendsRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_BlockFriends.clone())
    }

    fn create_group(&self, o: ::grpc::RequestOptions, p: super::api::CreateGroupRequest) -> ::grpc::SingleResponse<super::api::Group> {
        self.grpc_client.call_unary(o, p, self.method_CreateGroup.clone())
    }

    fn delete_friends(&self, o: ::grpc::RequestOptions, p: super::api::DeleteFriendsRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeleteFriends.clone())
    }

    fn delete_group(&self, o: ::grpc::RequestOptions, p: super::api::DeleteGroupRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeleteGroup.clone())
    }

    fn delete_leaderboard_record(&self, o: ::grpc::RequestOptions, p: super::api::DeleteLeaderboardRecordRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeleteLeaderboardRecord.clone())
    }

    fn delete_notifications(&self, o: ::grpc::RequestOptions, p: super::api::DeleteNotificationsRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeleteNotifications.clone())
    }

    fn delete_storage_objects(&self, o: ::grpc::RequestOptions, p: super::api::DeleteStorageObjectsRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeleteStorageObjects.clone())
    }

    fn get_account(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::api::Account> {
        self.grpc_client.call_unary(o, p, self.method_GetAccount.clone())
    }

    fn get_users(&self, o: ::grpc::RequestOptions, p: super::api::GetUsersRequest) -> ::grpc::SingleResponse<super::api::Users> {
        self.grpc_client.call_unary(o, p, self.method_GetUsers.clone())
    }

    fn healthcheck(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_Healthcheck.clone())
    }

    fn import_facebook_friends(&self, o: ::grpc::RequestOptions, p: super::api::ImportFacebookFriendsRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_ImportFacebookFriends.clone())
    }

    fn join_group(&self, o: ::grpc::RequestOptions, p: super::api::JoinGroupRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_JoinGroup.clone())
    }

    fn join_tournament(&self, o: ::grpc::RequestOptions, p: super::api::JoinTournamentRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_JoinTournament.clone())
    }

    fn kick_group_users(&self, o: ::grpc::RequestOptions, p: super::api::KickGroupUsersRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_KickGroupUsers.clone())
    }

    fn leave_group(&self, o: ::grpc::RequestOptions, p: super::api::LeaveGroupRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_LeaveGroup.clone())
    }

    fn link_custom(&self, o: ::grpc::RequestOptions, p: super::api::AccountCustom) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_LinkCustom.clone())
    }

    fn link_device(&self, o: ::grpc::RequestOptions, p: super::api::AccountDevice) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_LinkDevice.clone())
    }

    fn link_email(&self, o: ::grpc::RequestOptions, p: super::api::AccountEmail) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_LinkEmail.clone())
    }

    fn link_facebook(&self, o: ::grpc::RequestOptions, p: super::api::LinkFacebookRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_LinkFacebook.clone())
    }

    fn link_game_center(&self, o: ::grpc::RequestOptions, p: super::api::AccountGameCenter) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_LinkGameCenter.clone())
    }

    fn link_google(&self, o: ::grpc::RequestOptions, p: super::api::AccountGoogle) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_LinkGoogle.clone())
    }

    fn link_steam(&self, o: ::grpc::RequestOptions, p: super::api::AccountSteam) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_LinkSteam.clone())
    }

    fn list_channel_messages(&self, o: ::grpc::RequestOptions, p: super::api::ListChannelMessagesRequest) -> ::grpc::SingleResponse<super::api::ChannelMessageList> {
        self.grpc_client.call_unary(o, p, self.method_ListChannelMessages.clone())
    }

    fn list_friends(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::api::Friends> {
        self.grpc_client.call_unary(o, p, self.method_ListFriends.clone())
    }

    fn list_groups(&self, o: ::grpc::RequestOptions, p: super::api::ListGroupsRequest) -> ::grpc::SingleResponse<super::api::GroupList> {
        self.grpc_client.call_unary(o, p, self.method_ListGroups.clone())
    }

    fn list_group_users(&self, o: ::grpc::RequestOptions, p: super::api::ListGroupUsersRequest) -> ::grpc::SingleResponse<super::api::GroupUserList> {
        self.grpc_client.call_unary(o, p, self.method_ListGroupUsers.clone())
    }

    fn list_leaderboard_records(&self, o: ::grpc::RequestOptions, p: super::api::ListLeaderboardRecordsRequest) -> ::grpc::SingleResponse<super::api::LeaderboardRecordList> {
        self.grpc_client.call_unary(o, p, self.method_ListLeaderboardRecords.clone())
    }

    fn list_leaderboard_records_around_owner(&self, o: ::grpc::RequestOptions, p: super::api::ListLeaderboardRecordsAroundOwnerRequest) -> ::grpc::SingleResponse<super::api::LeaderboardRecordList> {
        self.grpc_client.call_unary(o, p, self.method_ListLeaderboardRecordsAroundOwner.clone())
    }

    fn list_matches(&self, o: ::grpc::RequestOptions, p: super::api::ListMatchesRequest) -> ::grpc::SingleResponse<super::api::MatchList> {
        self.grpc_client.call_unary(o, p, self.method_ListMatches.clone())
    }

    fn list_notifications(&self, o: ::grpc::RequestOptions, p: super::api::ListNotificationsRequest) -> ::grpc::SingleResponse<super::api::NotificationList> {
        self.grpc_client.call_unary(o, p, self.method_ListNotifications.clone())
    }

    fn list_storage_objects(&self, o: ::grpc::RequestOptions, p: super::api::ListStorageObjectsRequest) -> ::grpc::SingleResponse<super::api::StorageObjectList> {
        self.grpc_client.call_unary(o, p, self.method_ListStorageObjects.clone())
    }

    fn list_tournaments(&self, o: ::grpc::RequestOptions, p: super::api::ListTournamentsRequest) -> ::grpc::SingleResponse<super::api::TournamentList> {
        self.grpc_client.call_unary(o, p, self.method_ListTournaments.clone())
    }

    fn list_tournament_records(&self, o: ::grpc::RequestOptions, p: super::api::ListTournamentRecordsRequest) -> ::grpc::SingleResponse<super::api::TournamentRecordList> {
        self.grpc_client.call_unary(o, p, self.method_ListTournamentRecords.clone())
    }

    fn list_tournament_records_around_owner(&self, o: ::grpc::RequestOptions, p: super::api::ListTournamentRecordsAroundOwnerRequest) -> ::grpc::SingleResponse<super::api::TournamentRecordList> {
        self.grpc_client.call_unary(o, p, self.method_ListTournamentRecordsAroundOwner.clone())
    }

    fn list_user_groups(&self, o: ::grpc::RequestOptions, p: super::api::ListUserGroupsRequest) -> ::grpc::SingleResponse<super::api::UserGroupList> {
        self.grpc_client.call_unary(o, p, self.method_ListUserGroups.clone())
    }

    fn promote_group_users(&self, o: ::grpc::RequestOptions, p: super::api::PromoteGroupUsersRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_PromoteGroupUsers.clone())
    }

    fn read_storage_objects(&self, o: ::grpc::RequestOptions, p: super::api::ReadStorageObjectsRequest) -> ::grpc::SingleResponse<super::api::StorageObjects> {
        self.grpc_client.call_unary(o, p, self.method_ReadStorageObjects.clone())
    }

    fn rpc_func(&self, o: ::grpc::RequestOptions, p: super::api::Rpc) -> ::grpc::SingleResponse<super::api::Rpc> {
        self.grpc_client.call_unary(o, p, self.method_RpcFunc.clone())
    }

    fn unlink_custom(&self, o: ::grpc::RequestOptions, p: super::api::AccountCustom) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_UnlinkCustom.clone())
    }

    fn unlink_device(&self, o: ::grpc::RequestOptions, p: super::api::AccountDevice) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_UnlinkDevice.clone())
    }

    fn unlink_email(&self, o: ::grpc::RequestOptions, p: super::api::AccountEmail) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_UnlinkEmail.clone())
    }

    fn unlink_facebook(&self, o: ::grpc::RequestOptions, p: super::api::AccountFacebook) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_UnlinkFacebook.clone())
    }

    fn unlink_game_center(&self, o: ::grpc::RequestOptions, p: super::api::AccountGameCenter) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_UnlinkGameCenter.clone())
    }

    fn unlink_google(&self, o: ::grpc::RequestOptions, p: super::api::AccountGoogle) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_UnlinkGoogle.clone())
    }

    fn unlink_steam(&self, o: ::grpc::RequestOptions, p: super::api::AccountSteam) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_UnlinkSteam.clone())
    }

    fn update_account(&self, o: ::grpc::RequestOptions, p: super::api::UpdateAccountRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_UpdateAccount.clone())
    }

    fn update_group(&self, o: ::grpc::RequestOptions, p: super::api::UpdateGroupRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_UpdateGroup.clone())
    }

    fn write_leaderboard_record(&self, o: ::grpc::RequestOptions, p: super::api::WriteLeaderboardRecordRequest) -> ::grpc::SingleResponse<super::api::LeaderboardRecord> {
        self.grpc_client.call_unary(o, p, self.method_WriteLeaderboardRecord.clone())
    }

    fn write_storage_objects(&self, o: ::grpc::RequestOptions, p: super::api::WriteStorageObjectsRequest) -> ::grpc::SingleResponse<super::api::StorageObjectAcks> {
        self.grpc_client.call_unary(o, p, self.method_WriteStorageObjects.clone())
    }

    fn write_tournament_record(&self, o: ::grpc::RequestOptions, p: super::api::WriteTournamentRecordRequest) -> ::grpc::SingleResponse<super::api::LeaderboardRecord> {
        self.grpc_client.call_unary(o, p, self.method_WriteTournamentRecord.clone())
    }
}

// server

pub struct NakamaServer;


impl NakamaServer {
    pub fn new_service_def<H : Nakama + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/nakama.api.Nakama",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/AddFriends".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_friends(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/AddGroupUsers".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_group_users(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/AuthenticateCustom".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.authenticate_custom(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/AuthenticateDevice".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.authenticate_device(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/AuthenticateEmail".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.authenticate_email(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/AuthenticateFacebook".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.authenticate_facebook(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/AuthenticateGameCenter".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.authenticate_game_center(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/AuthenticateGoogle".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.authenticate_google(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/AuthenticateSteam".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.authenticate_steam(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/BlockFriends".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.block_friends(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/CreateGroup".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.create_group(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/DeleteFriends".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_friends(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/DeleteGroup".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_group(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/DeleteLeaderboardRecord".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_leaderboard_record(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/DeleteNotifications".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_notifications(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/DeleteStorageObjects".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_storage_objects(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/GetAccount".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_account(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/GetUsers".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_users(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/Healthcheck".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.healthcheck(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ImportFacebookFriends".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.import_facebook_friends(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/JoinGroup".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.join_group(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/JoinTournament".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.join_tournament(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/KickGroupUsers".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.kick_group_users(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/LeaveGroup".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.leave_group(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/LinkCustom".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.link_custom(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/LinkDevice".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.link_device(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/LinkEmail".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.link_email(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/LinkFacebook".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.link_facebook(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/LinkGameCenter".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.link_game_center(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/LinkGoogle".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.link_google(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/LinkSteam".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.link_steam(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListChannelMessages".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_channel_messages(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListFriends".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_friends(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListGroups".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_groups(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListGroupUsers".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_group_users(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListLeaderboardRecords".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_leaderboard_records(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListLeaderboardRecordsAroundOwner".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_leaderboard_records_around_owner(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListMatches".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_matches(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListNotifications".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_notifications(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListStorageObjects".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_storage_objects(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListTournaments".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_tournaments(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListTournamentRecords".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_tournament_records(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListTournamentRecordsAroundOwner".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_tournament_records_around_owner(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ListUserGroups".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_user_groups(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/PromoteGroupUsers".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.promote_group_users(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/ReadStorageObjects".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.read_storage_objects(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/RpcFunc".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.rpc_func(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/UnlinkCustom".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.unlink_custom(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/UnlinkDevice".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.unlink_device(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/UnlinkEmail".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.unlink_email(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/UnlinkFacebook".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.unlink_facebook(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/UnlinkGameCenter".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.unlink_game_center(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/UnlinkGoogle".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.unlink_google(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/UnlinkSteam".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.unlink_steam(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/UpdateAccount".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.update_account(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/UpdateGroup".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.update_group(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/WriteLeaderboardRecord".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.write_leaderboard_record(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/WriteStorageObjects".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.write_storage_objects(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/nakama.api.Nakama/WriteTournamentRecord".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.write_tournament_record(o, p))
                    },
                ),
            ],
        )
    }
}
