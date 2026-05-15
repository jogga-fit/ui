use std::ptr::fn_addr_eq;

use crate::{
    FutureResult, RoutePoint,
    types::{ActorInfo, ConnectionsResult, FeedItem, UploadExerciseMeta, UploadExerciseResult},
};

macro_rules! fn_adapter {
    ($name:ident, $fn_ty:ty) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub $fn_ty);

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                fn_addr_eq(self.0, other.0)
            }
        }
    };
}

pub struct TokenApIdArgs {
    pub token: String,
    pub ap_id: String,
}

pub struct FollowArgs {
    pub token: String,
    pub handle_or_url: String,
}

pub struct AliasArgs {
    pub token: String,
    pub alias: String,
}

pub struct FetchRouteArgs {
    pub url: String,
    pub token: Option<String>,
}

pub struct KickFollowerArgs {
    pub token: String,
    pub follower_ap_id: String,
}

pub struct SetThemeArgs {
    pub token: String,
    pub theme: String,
}

pub struct MoveAccountArgs {
    pub token: String,
    pub target_ap_id: String,
}

pub struct PrivacySettingsArgs {
    pub token: String,
    pub public_profile: bool,
}

pub struct UploadAvatarArgs {
    pub token: String,
    pub bytes: Vec<u8>,
}

pub struct CreateReplyArgs {
    pub token: String,
    pub content: String,
    pub in_reply_to: String,
}

pub struct UpdatePostArgs {
    pub token: String,
    pub object_ap_id: String,
    pub content: Option<String>,
    pub title: Option<String>,
    pub hidden_stats: Vec<String>,
    pub removed_urls: Vec<String>,
}

pub struct UpdateProfileArgs {
    pub token: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
}

pub struct FollowRequestArgs {
    pub token: String,
    pub ap_id: String,
    pub follow_ap_id: String,
}

pub struct UploadExerciseArgs {
    pub token: String,
    pub bytes: Vec<u8>,
    pub filename: String,
    pub meta: UploadExerciseMeta,
}

fn_adapter!(LikeFn, fn(TokenApIdArgs) -> FutureResult<()>);
fn_adapter!(
    RouteFn,
    fn(FetchRouteArgs) -> FutureResult<Option<Vec<RoutePoint>>>
);
fn_adapter!(DeleteObjectFn, fn(TokenApIdArgs) -> FutureResult<()>);
fn_adapter!(CreateReplyFn, fn(CreateReplyArgs) -> FutureResult<()>);
fn_adapter!(UpdatePostFn, fn(UpdatePostArgs) -> FutureResult<()>);
fn_adapter!(
    CheckFollowingFn,
    fn(TokenApIdArgs) -> FutureResult<Option<bool>>
);
fn_adapter!(UpdateProfileFn, fn(UpdateProfileArgs) -> FutureResult<()>);
fn_adapter!(UploadAvatarFn, fn(UploadAvatarArgs) -> FutureResult<String>);
fn_adapter!(FollowPersonFn, fn(FollowArgs) -> FutureResult<()>);
fn_adapter!(FollowActorFn, fn(FollowArgs) -> FutureResult<()>);
fn_adapter!(UnfollowActorFn, fn(TokenApIdArgs) -> FutureResult<()>);
fn_adapter!(KickFollowerFn, fn(KickFollowerArgs) -> FutureResult<()>);
fn_adapter!(
    AcceptFollowRequestFn,
    fn(FollowRequestArgs) -> FutureResult<()>
);
fn_adapter!(
    RejectFollowRequestFn,
    fn(FollowRequestArgs) -> FutureResult<()>
);
fn_adapter!(SetThemeFn, fn(SetThemeArgs) -> FutureResult<()>);
fn_adapter!(
    SetPrivacySettingsFn,
    fn(PrivacySettingsArgs) -> FutureResult<()>
);
fn_adapter!(AddAliasFn, fn(AliasArgs) -> FutureResult<()>);
fn_adapter!(RemoveAliasFn, fn(AliasArgs) -> FutureResult<()>);
fn_adapter!(MoveAccountFn, fn(MoveAccountArgs) -> FutureResult<()>);
fn_adapter!(
    UploadExerciseFn,
    fn(UploadExerciseArgs) -> FutureResult<UploadExerciseResult>
);

fn_adapter!(GetActorInfoFn, fn(String) -> FutureResult<ActorInfo>);
fn_adapter!(
    GetActorPostsFn,
    fn(String, Option<String>) -> FutureResult<Vec<FeedItem>>
);
fn_adapter!(
    GetActorConnectionsFn,
    fn(String, Option<String>) -> FutureResult<ConnectionsResult>
);
fn_adapter!(
    GetClubFeedFn,
    fn(String, Option<String>) -> FutureResult<Vec<FeedItem>>
);
fn_adapter!(DeleteAccountFn, fn(String) -> FutureResult<()>);
