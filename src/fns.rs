use dioxus::core::Callback;

use crate::{
    FutureResult, RoutePoint,
    types::{
        ActorInfo, ConnectionsResult, FeedItem, Theme, UploadExerciseMeta, UploadExerciseResult,
    },
};

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
    pub theme: Theme,
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

pub struct GetActorPostsArgs {
    pub username: String,
    pub token: Option<String>,
}

pub struct GetActorConnectionsArgs {
    pub username: String,
    pub token: Option<String>,
}

pub struct GetClubFeedArgs {
    pub handle: String,
    pub token: Option<String>,
}

pub type LikeFn = Callback<TokenApIdArgs, FutureResult<()>>;
pub type DeleteObjectFn = Callback<TokenApIdArgs, FutureResult<()>>;
pub type CreateReplyFn = Callback<CreateReplyArgs, FutureResult<()>>;
pub type RouteFn = Callback<FetchRouteArgs, FutureResult<Option<Vec<RoutePoint>>>>;
pub type UpdatePostFn = Callback<UpdatePostArgs, FutureResult<()>>;
pub type CheckFollowingFn = Callback<TokenApIdArgs, FutureResult<Option<bool>>>;
pub type UpdateProfileFn = Callback<UpdateProfileArgs, FutureResult<()>>;
pub type UploadAvatarFn = Callback<UploadAvatarArgs, FutureResult<String>>;
pub type FollowPersonFn = Callback<FollowArgs, FutureResult<()>>;
pub type FollowActorFn = Callback<FollowArgs, FutureResult<()>>;
pub type UnfollowActorFn = Callback<TokenApIdArgs, FutureResult<()>>;
pub type KickFollowerFn = Callback<KickFollowerArgs, FutureResult<()>>;
pub type AcceptFollowRequestFn = Callback<FollowRequestArgs, FutureResult<()>>;
pub type RejectFollowRequestFn = Callback<FollowRequestArgs, FutureResult<()>>;
pub type SetThemeFn = Callback<SetThemeArgs, FutureResult<()>>;
pub type SetPrivacySettingsFn = Callback<PrivacySettingsArgs, FutureResult<()>>;
pub type AddAliasFn = Callback<AliasArgs, FutureResult<()>>;
pub type RemoveAliasFn = Callback<AliasArgs, FutureResult<()>>;
pub type MoveAccountFn = Callback<MoveAccountArgs, FutureResult<()>>;
pub type UploadExerciseFn = Callback<UploadExerciseArgs, FutureResult<UploadExerciseResult>>;
pub type GetActorInfoFn = Callback<String, FutureResult<ActorInfo>>;
pub type GetActorPostsFn = Callback<GetActorPostsArgs, FutureResult<Vec<FeedItem>>>;
pub type GetActorConnectionsFn = Callback<GetActorConnectionsArgs, FutureResult<ConnectionsResult>>;
pub type GetClubFeedFn = Callback<GetClubFeedArgs, FutureResult<Vec<FeedItem>>>;
pub type DeleteAccountFn = Callback<String, FutureResult<()>>;
