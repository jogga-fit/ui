use jogga_ui::{
    AliasArgs, CreateReplyArgs, FetchRouteArgs, FollowArgs, FollowRequestArgs, FutureResult,
    GetActorConnectionsArgs, GetActorPostsArgs, KickFollowerArgs, MoveAccountArgs,
    PrivacySettingsArgs, RoutePoint, SetThemeArgs, TokenApIdArgs, UpdatePostArgs,
    UpdateProfileArgs, UploadAvatarArgs,
    components::{
        notifications::{NotificationItem, NotificationKind},
        post::ThreadItem,
    },
    exercise::ExerciseType,
    pages::{
        clubs::{ClubItem, ClubRole},
        people::{DirectoryItem, FollowerItem},
    },
    sleep_ms,
    types::{
        ActorInfo, ConnectionItem, ConnectionsResult, ExerciseStats, FeedItem, FollowingItem,
        MeResult, ObjectType, Theme,
    },
};

pub fn mock_like(_args: TokenApIdArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_delete(_args: TokenApIdArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_route(_args: FetchRouteArgs) -> FutureResult<Option<Vec<RoutePoint>>> {
    Box::pin(async move {
        sleep_ms(300).await;
        Ok(Some(vec![
            (37.7749, -122.4194, Some(10.0)),
            (37.7752, -122.4188, Some(12.5)),
            (37.7758, -122.4175, Some(15.0)),
            (37.7763, -122.4165, Some(18.0)),
            (37.7755, -122.4160, Some(16.0)),
            (37.7748, -122.4170, Some(13.0)),
            (37.7749, -122.4194, Some(10.0)),
        ]))
    })
}

pub fn mock_update_post(_args: UpdatePostArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_create_reply(_args: CreateReplyArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_follow_person(_args: FollowArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_unfollow_actor(_args: TokenApIdArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_kick_follower(_args: KickFollowerArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_accept_follow_request(_args: FollowRequestArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_reject_follow_request(_args: FollowRequestArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_follow_actor(_args: FollowArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_get_actor_info(_username: String) -> FutureResult<ActorInfo> {
    Box::pin(async move {
        sleep_ms(300).await;
        Ok(ActorInfo {
            username: "alex".to_string(),
            domain: "jogga.fit".to_string(),
            ap_id: "https://jogga.fit/users/alex".to_string(),
            display_name: Some("Alex Runner".to_string()),
            bio: Some(
                "Distance runner | Marathoner | Training for Berlin 2026 | Garmin addict"
                    .to_string(),
            ),
            avatar_url: None,
            public_profile: true,
            followers_count: 1204,
            following_count: 248,
        })
    })
}

pub fn mock_get_actor_posts(_args: GetActorPostsArgs) -> FutureResult<Vec<FeedItem>> {
    Box::pin(async move {
        sleep_ms(300).await;
        Ok(vec![run_item(), run_with_route_item()])
    })
}

pub fn mock_get_actor_connections(
    _args: GetActorConnectionsArgs,
) -> FutureResult<ConnectionsResult> {
    Box::pin(async move {
        sleep_ms(300).await;
        Ok(ConnectionsResult {
            visible: true,
            following: vec![
                ConnectionItem {
                    ap_id: "https://jogga.fit/users/sam".to_string(),
                    username: "sam".to_string(),
                    domain: "jogga.fit".to_string(),
                    is_local: true,
                    display_name: Some("Sam Cyclist".to_string()),
                },
                ConnectionItem {
                    ap_id: "https://mastodon.social/users/runnerkai".to_string(),
                    username: "runnerkai".to_string(),
                    domain: "mastodon.social".to_string(),
                    is_local: false,
                    display_name: Some("Kai Runner".to_string()),
                },
            ],
            followers: vec![
                ConnectionItem {
                    ap_id: "https://jogga.fit/users/felix".to_string(),
                    username: "felix".to_string(),
                    domain: "jogga.fit".to_string(),
                    is_local: true,
                    display_name: Some("Felix Stride".to_string()),
                },
                ConnectionItem {
                    ap_id: "https://jogga.fit/users/zara".to_string(),
                    username: "zara".to_string(),
                    domain: "jogga.fit".to_string(),
                    is_local: true,
                    display_name: Some("Zara Hill".to_string()),
                },
            ],
        })
    })
}

pub fn mock_check_following(_args: TokenApIdArgs) -> FutureResult<Option<bool>> {
    Box::pin(async move {
        sleep_ms(200).await;
        Ok(None)
    })
}

pub fn mock_update_profile(_args: UpdateProfileArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_upload_avatar(_args: UploadAvatarArgs) -> FutureResult<String> {
    Box::pin(async move {
        sleep_ms(800).await;
        Ok("https://i.pravatar.cc/96?img=5".to_string())
    })
}

pub fn mock_set_theme(_args: SetThemeArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(300).await;
        Ok(())
    })
}

pub fn mock_set_privacy_settings(_args: PrivacySettingsArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(300).await;
        Ok(())
    })
}

pub fn mock_delete_account(_token: String) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_add_alias(_args: AliasArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_remove_alias(_args: AliasArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_move_account(_args: MoveAccountArgs) -> FutureResult<()> {
    Box::pin(async move {
        sleep_ms(500).await;
        Ok(())
    })
}

pub fn mock_me() -> MeResult {
    MeResult {
        username: "alex".to_string(),
        ap_id: "https://jogga.fit/users/alex".to_string(),
        display_name: Some("Alex Runner".to_string()),
        bio: Some(
            "Distance runner | Marathoner | Training for Berlin 2026 | Garmin addict".to_string(),
        ),
        is_admin: false,
        email: Some("alex@example.com".to_string()),
        phone: None,
        avatar_url: None,
        show_in_directory: true,
        public_profile: true,
        theme: Theme::Dark,
        also_known_as: vec![],
        moved_to: None,
    }
}

pub fn mock_following_items() -> Vec<FollowingItem> {
    vec![
        FollowingItem {
            ap_id: "https://jogga.fit/users/sam".to_string(),
            username: "sam".to_string(),
            domain: "jogga.fit".to_string(),
            is_local: true,
            display_name: Some("Sam Cyclist".to_string()),
            avatar_url: None,
            accepted: true,
        },
        FollowingItem {
            ap_id: "https://mastodon.social/users/runnerkai".to_string(),
            username: "runnerkai".to_string(),
            domain: "mastodon.social".to_string(),
            is_local: false,
            display_name: Some("Kai Runner".to_string()),
            avatar_url: None,
            accepted: true,
        },
        FollowingItem {
            ap_id: "https://jogga.fit/users/maya".to_string(),
            username: "maya".to_string(),
            domain: "jogga.fit".to_string(),
            is_local: true,
            display_name: Some("Maya Swimmer".to_string()),
            avatar_url: None,
            accepted: false,
        },
    ]
}

pub fn mock_follower_items() -> Vec<FollowerItem> {
    vec![
        FollowerItem {
            ap_id: "https://jogga.fit/users/jordan".to_string(),
            username: "jordan".to_string(),
            domain: "jogga.fit".to_string(),
            is_local: true,
            display_name: Some("Jordan Trails".to_string()),
            avatar_url: None,
            accepted: true,
            follow_ap_id: Some("https://jogga.fit/follows/f-001".to_string()),
        },
        FollowerItem {
            ap_id: "https://sportssocial.net/users/ellie".to_string(),
            username: "ellie".to_string(),
            domain: "sportssocial.net".to_string(),
            is_local: false,
            display_name: Some("Ellie Pace".to_string()),
            avatar_url: None,
            accepted: true,
            follow_ap_id: Some("https://sportssocial.net/follows/f-882".to_string()),
        },
        FollowerItem {
            ap_id: "https://jogga.fit/users/marco".to_string(),
            username: "marco".to_string(),
            domain: "jogga.fit".to_string(),
            is_local: true,
            display_name: Some("Marco Sprint".to_string()),
            avatar_url: None,
            accepted: true,
            follow_ap_id: None,
        },
    ]
}

pub fn mock_clubs() -> Vec<FollowingItem> {
    vec![
        FollowingItem {
            ap_id: "https://jogga.fit/clubs/sfcycling".to_string(),
            username: "sfcycling".to_string(),
            domain: "jogga.fit".to_string(),
            is_local: true,
            display_name: Some("SF Cycling Club".to_string()),
            avatar_url: None,
            accepted: true,
        },
        FollowingItem {
            ap_id: "https://jogga.fit/clubs/bmarathon".to_string(),
            username: "bmarathon".to_string(),
            domain: "jogga.fit".to_string(),
            is_local: true,
            display_name: Some("Berlin Marathon Pacers".to_string()),
            avatar_url: None,
            accepted: false,
        },
        FollowingItem {
            ap_id: "https://jogga.fit/clubs/openwaterswim".to_string(),
            username: "openwaterswim".to_string(),
            domain: "jogga.fit".to_string(),
            is_local: true,
            display_name: Some("Open Water Swimmers".to_string()),
            avatar_url: None,
            accepted: true,
        },
    ]
}

pub fn mock_server_clubs() -> Vec<ClubItem> {
    vec![
        ClubItem {
            handle: "sfcycling".to_string(),
            ap_id: "https://jogga.fit/clubs/sfcycling".to_string(),
            display_name: Some("SF Cycling Club".to_string()),
            description: Some(
                "Road and gravel rides in and around the Bay Area. Weekly group rides every Saturday at 7am from Ferry Building.".to_string(),
            ),
            exclusive: false,
            member_count: 47,
            my_role: ClubRole::Member,
        },
        ClubItem {
            handle: "bmarathon".to_string(),
            ap_id: "https://jogga.fit/clubs/bmarathon".to_string(),
            display_name: Some("Berlin Marathon Pacers".to_string()),
            description: Some(
                "Training group for Berlin Marathon. Structured plans, weekly long runs, and pacing support.".to_string(),
            ),
            exclusive: true,
            member_count: 18,
            my_role: ClubRole::NotMember,
        },
        ClubItem {
            handle: "openwaterswim".to_string(),
            ap_id: "https://jogga.fit/clubs/openwaterswim".to_string(),
            display_name: Some("Open Water Swimmers".to_string()),
            description: Some("Year-round swims at Aquatic Park and beyond. All levels welcome.".to_string()),
            exclusive: false,
            member_count: 89,
            my_role: ClubRole::Member,
        },
        ClubItem {
            handle: "irontraining".to_string(),
            ap_id: "https://jogga.fit/clubs/irontraining".to_string(),
            display_name: Some("Triathlon Training".to_string()),
            description: None,
            exclusive: true,
            member_count: 12,
            my_role: ClubRole::NotMember,
        },
        ClubItem {
            handle: "trailrunners".to_string(),
            ap_id: "https://jogga.fit/clubs/trailrunners".to_string(),
            display_name: Some("Bay Area Trail Runners".to_string()),
            description: Some("Exploring trails across Marin, Oakland Hills, and the Peninsula. Casual weekend runs.".to_string()),
            exclusive: false,
            member_count: 34,
            my_role: ClubRole::NotMember,
        },
    ]
}

pub fn run_item() -> FeedItem {
    FeedItem {
        id: "run-1".to_string(),
        object_ap_id: "https://jogga.fit/users/alex/exercises/2".to_string(),
        actor_username: "alex".to_string(),
        actor_domain: "jogga.fit".to_string(),
        actor_display_name: Some("Alex Runner".to_string()),
        actor_is_local: true,
        actor_ap_id: "https://jogga.fit/users/alex".to_string(),
        actor_avatar_url: None,
        activity_type: "Create".to_string(),
        object_type: ObjectType::Exercise,
        content: Some(
            "<p>Solid tempo run around the park. Felt strong through the last 3K.</p>".to_string(),
        ),
        published: "2026-05-14T06:45:00Z".to_string(),
        stats: ExerciseStats {
            exercise_type: Some(ExerciseType::Run),
            duration_s: Some(3240),
            distance_m: Some(10100.0),
            elevation_gain_m: Some(85.0),
            avg_heart_rate_bpm: Some(158),
            max_heart_rate_bpm: Some(174),
            avg_power_w: None,
            max_power_w: None,
            normalized_power_w: None,
            avg_cadence_rpm: Some(172.0),
            avg_pace_s_per_km: Some(320.8),
            device: Some("Garmin Forerunner 965".to_string()),
        },
        title: Some("Morning Tempo Run".to_string()),
        image_urls: vec![],
        like_count: 12,
        viewer_has_liked: true,
        viewer_is_owner: true,
        reply_count: 3,
        in_reply_to: None,
        route_url: None,
        hidden_stats: vec![],
        via_club_handle: None,
        via_club_display: None,
    }
}

pub fn ride_item() -> FeedItem {
    FeedItem {
        id: "ride-1".to_string(),
        object_ap_id: "https://jogga.fit/users/sam/exercises/3".to_string(),
        actor_username: "sam".to_string(),
        actor_domain: "jogga.fit".to_string(),
        actor_display_name: Some("Sam Cyclist".to_string()),
        actor_is_local: true,
        actor_ap_id: "https://jogga.fit/users/sam".to_string(),
        actor_avatar_url: None,
        activity_type: "Create".to_string(),
        object_type: ObjectType::Exercise,
        content: Some(
            "<p>Long endurance ride — perfect weather, legs felt great all the way through.</p>"
                .to_string(),
        ),
        published: "2026-05-14T05:30:00Z".to_string(),
        stats: ExerciseStats {
            exercise_type: Some(ExerciseType::Ride),
            duration_s: Some(7200),
            distance_m: Some(65000.0),
            elevation_gain_m: Some(520.0),
            avg_heart_rate_bpm: Some(142),
            max_heart_rate_bpm: Some(168),
            avg_power_w: Some(210.0),
            max_power_w: Some(580.0),
            normalized_power_w: Some(225.0),
            avg_cadence_rpm: Some(88.0),
            avg_pace_s_per_km: None,
            device: Some("Wahoo ELEMNT ROAM".to_string()),
        },
        title: Some("Sunrise Endurance Ride".to_string()),
        image_urls: vec![],
        like_count: 24,
        viewer_has_liked: false,
        viewer_is_owner: false,
        reply_count: 7,
        in_reply_to: None,
        route_url: None,
        hidden_stats: vec![],
        via_club_handle: Some("sfcycling".to_string()),
        via_club_display: Some("SF Cycling Club".to_string()),
    }
}

pub fn swim_item() -> FeedItem {
    FeedItem {
        id: "swim-1".to_string(),
        object_ap_id: "https://jogga.fit/users/maya/exercises/4".to_string(),
        actor_username: "maya".to_string(),
        actor_domain: "mastodon.social".to_string(),
        actor_display_name: None,
        actor_is_local: false,
        actor_ap_id: "https://mastodon.social/users/maya".to_string(),
        actor_avatar_url: None,
        activity_type: "Create".to_string(),
        object_type: ObjectType::Exercise,
        content: Some(
            "<p>Early morning swim — 2km in the outdoor pool before work.</p>".to_string(),
        ),
        published: "2026-05-14T05:00:00Z".to_string(),
        stats: ExerciseStats {
            exercise_type: Some(ExerciseType::Swim),
            duration_s: Some(2700),
            distance_m: Some(2000.0),
            elevation_gain_m: None,
            avg_heart_rate_bpm: Some(138),
            max_heart_rate_bpm: Some(155),
            avg_power_w: None,
            max_power_w: None,
            normalized_power_w: None,
            avg_cadence_rpm: Some(52.0),
            avg_pace_s_per_km: Some(1350.0),
            device: Some("Apple Watch Ultra 2".to_string()),
        },
        title: Some("Open Pool 2K".to_string()),
        image_urls: vec![],
        like_count: 8,
        viewer_has_liked: false,
        viewer_is_owner: false,
        reply_count: 1,
        in_reply_to: None,
        route_url: None,
        hidden_stats: vec!["avg_heart_rate_bpm".to_string()],
        via_club_handle: None,
        via_club_display: None,
    }
}

pub fn run_with_route_item() -> FeedItem {
    FeedItem {
        id: "run-map-1".to_string(),
        object_ap_id: "https://jogga.fit/users/felix/exercises/10".to_string(),
        actor_username: "felix".to_string(),
        actor_domain: "jogga.fit".to_string(),
        actor_display_name: Some("Felix Stride".to_string()),
        actor_is_local: true,
        actor_ap_id: "https://jogga.fit/users/felix".to_string(),
        actor_avatar_url: None,
        activity_type: "Create".to_string(),
        object_type: ObjectType::Exercise,
        content: Some(
            "<p>Chilly morning but the route was beautiful. Negative split all the way in.</p>"
                .to_string(),
        ),
        published: "2026-05-14T07:20:00Z".to_string(),
        stats: ExerciseStats {
            exercise_type: Some(ExerciseType::Run),
            duration_s: Some(2880),
            distance_m: Some(8500.0),
            elevation_gain_m: Some(42.0),
            avg_heart_rate_bpm: Some(162),
            max_heart_rate_bpm: Some(178),
            avg_power_w: None,
            max_power_w: None,
            normalized_power_w: None,
            avg_cadence_rpm: Some(176.0),
            avg_pace_s_per_km: Some(338.8),
            device: Some("Garmin Forerunner 265".to_string()),
        },
        title: Some("Morning 8.5K".to_string()),
        image_urls: vec![],
        like_count: 5,
        viewer_has_liked: false,
        viewer_is_owner: false,
        reply_count: 0,
        in_reply_to: None,
        route_url: Some("https://jogga.fit/routes/felix-10".to_string()),
        hidden_stats: vec![],
        via_club_handle: None,
        via_club_display: None,
    }
}

pub fn run_with_photos_item() -> FeedItem {
    FeedItem {
        id: "run-photos-1".to_string(),
        object_ap_id: "https://jogga.fit/users/zara/exercises/20".to_string(),
        actor_username: "zara".to_string(),
        actor_domain: "jogga.fit".to_string(),
        actor_display_name: Some("Zara Hill".to_string()),
        actor_is_local: true,
        actor_ap_id: "https://jogga.fit/users/zara".to_string(),
        actor_avatar_url: None,
        activity_type: "Create".to_string(),
        object_type: ObjectType::Exercise,
        content: Some(
            "<p>Weekend long run in the mountains. Views were insane at the top.</p>".to_string(),
        ),
        published: "2026-05-13T09:00:00Z".to_string(),
        stats: ExerciseStats {
            exercise_type: Some(ExerciseType::Run),
            duration_s: Some(7800),
            distance_m: Some(22000.0),
            elevation_gain_m: Some(980.0),
            avg_heart_rate_bpm: Some(148),
            max_heart_rate_bpm: Some(172),
            avg_power_w: None,
            max_power_w: None,
            normalized_power_w: None,
            avg_cadence_rpm: Some(164.0),
            avg_pace_s_per_km: Some(354.5),
            device: None,
        },
        title: Some("Mountain Ultra Recon".to_string()),
        image_urls: vec![
            "https://picsum.photos/seed/trailrun1/800/600".to_string(),
            "https://picsum.photos/seed/mountainpath/800/600".to_string(),
            "https://picsum.photos/seed/trailsummit/800/600".to_string(),
        ],
        like_count: 31,
        viewer_has_liked: true,
        viewer_is_owner: false,
        reply_count: 9,
        in_reply_to: None,
        route_url: None,
        hidden_stats: vec![],
        via_club_handle: None,
        via_club_display: None,
    }
}

pub fn ride_with_photos_and_route_item() -> FeedItem {
    FeedItem {
        id: "ride-photos-map-1".to_string(),
        object_ap_id: "https://jogga.fit/users/tobias/exercises/30".to_string(),
        actor_username: "tobias".to_string(),
        actor_domain: "jogga.fit".to_string(),
        actor_display_name: Some("Tobias Rad".to_string()),
        actor_is_local: true,
        actor_ap_id: "https://jogga.fit/users/tobias".to_string(),
        actor_avatar_url: None,
        activity_type: "Create".to_string(),
        object_type: ObjectType::Exercise,
        content: Some("<p>Gravel grinder through wine country. Worth every climb.</p>".to_string()),
        published: "2026-05-13T06:30:00Z".to_string(),
        stats: ExerciseStats {
            exercise_type: Some(ExerciseType::Ride),
            duration_s: Some(14400),
            distance_m: Some(120000.0),
            elevation_gain_m: Some(1800.0),
            avg_heart_rate_bpm: Some(145),
            max_heart_rate_bpm: Some(182),
            avg_power_w: Some(198.0),
            max_power_w: Some(620.0),
            normalized_power_w: Some(215.0),
            avg_cadence_rpm: Some(85.0),
            avg_pace_s_per_km: None,
            device: Some("Wahoo ELEMNT BOLT".to_string()),
        },
        title: Some("Wine Country Gravel 120K".to_string()),
        image_urls: vec![
            "https://picsum.photos/seed/gravelroad1/800/600".to_string(),
            "https://picsum.photos/seed/vineyardpath/800/600".to_string(),
        ],
        like_count: 18,
        viewer_has_liked: false,
        viewer_is_owner: false,
        reply_count: 4,
        in_reply_to: None,
        route_url: Some("https://jogga.fit/routes/tobias-30".to_string()),
        hidden_stats: vec![],
        via_club_handle: None,
        via_club_display: None,
    }
}

pub fn pending_follow_requests() -> Vec<FollowerItem> {
    vec![
        FollowerItem {
            ap_id: "https://sportssocial.net/users/nina".to_string(),
            username: "nina".to_string(),
            domain: "sportssocial.net".to_string(),
            is_local: false,
            display_name: Some("Nina Pace".to_string()),
            avatar_url: None,
            accepted: false,
            follow_ap_id: Some("https://sportssocial.net/follows/f-991".to_string()),
        },
        FollowerItem {
            ap_id: "https://jogga.fit/users/kai".to_string(),
            username: "kai".to_string(),
            domain: "jogga.fit".to_string(),
            is_local: true,
            display_name: Some("Kai Trails".to_string()),
            avatar_url: None,
            accepted: false,
            follow_ap_id: Some("https://jogga.fit/follows/f-447".to_string()),
        },
        FollowerItem {
            ap_id: "https://mastodon.social/users/riku".to_string(),
            username: "riku".to_string(),
            domain: "mastodon.social".to_string(),
            is_local: false,
            display_name: None,
            avatar_url: None,
            accepted: false,
            follow_ap_id: Some("https://mastodon.social/follows/f-112".to_string()),
        },
    ]
}

pub fn reply_item() -> ThreadItem {
    ThreadItem {
        ap_id: "https://jogga.fit/users/maya/notes/42".to_string(),
        author_username: "maya".to_string(),
        author_avatar_url: None,
        content: Some("<p>Great effort! What's your goal race this season?</p>".to_string()),
        published: "2026-05-14T08:15:00Z".to_string(),
        like_count: 2,
        viewer_has_liked: false,
        viewer_is_owner: false,
    }
}

pub fn reply_item_own() -> ThreadItem {
    ThreadItem {
        ap_id: "https://jogga.fit/users/alex/notes/99".to_string(),
        author_username: "alex".to_string(),
        author_avatar_url: None,
        content: Some(
            "<p>Aiming for Berlin in September — fingers crossed for good weather!</p>".to_string(),
        ),
        published: "2026-05-14T08:30:00Z".to_string(),
        like_count: 0,
        viewer_has_liked: false,
        viewer_is_owner: true,
    }
}

pub fn mock_directory_items() -> Vec<DirectoryItem> {
    vec![
        DirectoryItem {
            username: "zara".to_string(),
            domain: "jogga.fit".to_string(),
            ap_id: "https://jogga.fit/users/zara".to_string(),
            display_name: Some("Zara Hill".to_string()),
            bio: Some("Ultra-runner | 100-miler finisher | Mountains are home".to_string()),
            avatar_url: None,
        },
        DirectoryItem {
            username: "felix".to_string(),
            domain: "jogga.fit".to_string(),
            ap_id: "https://jogga.fit/users/felix".to_string(),
            display_name: Some("Felix Stride".to_string()),
            bio: Some("5K & 10K specialist | Club runner | Coffee first".to_string()),
            avatar_url: None,
        },
        DirectoryItem {
            username: "sam".to_string(),
            domain: "jogga.fit".to_string(),
            ap_id: "https://jogga.fit/users/sam".to_string(),
            display_name: Some("Sam Cyclist".to_string()),
            bio: Some("Road cyclist | Wahoo nerd | Always chasing watts".to_string()),
            avatar_url: None,
        },
        DirectoryItem {
            username: "tobias".to_string(),
            domain: "jogga.fit".to_string(),
            ap_id: "https://jogga.fit/users/tobias".to_string(),
            display_name: Some("Tobias Rad".to_string()),
            bio: Some("Gravel & road | Wine country rides | Amateur baker".to_string()),
            avatar_url: None,
        },
    ]
}
pub fn demo_notifications() -> Vec<NotificationItem> {
    vec![
        NotificationItem {
            id: "n1".to_string(),
            kind: NotificationKind::Like,
            from_ap_id: "https://jogga.fit/users/sam".to_string(),
            from_username: "sam".to_string(),
            from_display_name: Some("Sam Cyclist".to_string()),
            from_avatar_url: None,
            object_ap_id: Some("https://jogga.fit/users/alex/exercises/2".to_string()),
            object_title: Some("Morning Tempo Run".to_string()),
            is_read: false,
            created_at: "2026-05-16T07:10:00Z".to_string(),
        },
        NotificationItem {
            id: "n2".to_string(),
            kind: NotificationKind::FollowRequest,
            from_ap_id: "https://sportssocial.net/users/nina".to_string(),
            from_username: "nina".to_string(),
            from_display_name: Some("Nina Pace".to_string()),
            from_avatar_url: None,
            object_ap_id: None,
            object_title: None,
            is_read: false,
            created_at: "2026-05-16T06:30:00Z".to_string(),
        },
        NotificationItem {
            id: "n3".to_string(),
            kind: NotificationKind::NewFollower,
            from_ap_id: "https://jogga.fit/users/felix".to_string(),
            from_username: "felix".to_string(),
            from_display_name: Some("Felix Stride".to_string()),
            from_avatar_url: None,
            object_ap_id: None,
            object_title: None,
            is_read: true,
            created_at: "2026-05-15T20:45:00Z".to_string(),
        },
        NotificationItem {
            id: "n4".to_string(),
            kind: NotificationKind::FollowAccepted,
            from_ap_id: "https://mastodon.social/users/runnerkai".to_string(),
            from_username: "runnerkai".to_string(),
            from_display_name: Some("Kai Runner".to_string()),
            from_avatar_url: None,
            object_ap_id: None,
            object_title: None,
            is_read: true,
            created_at: "2026-05-15T14:22:00Z".to_string(),
        },
        NotificationItem {
            id: "n5".to_string(),
            kind: NotificationKind::Like,
            from_ap_id: "https://jogga.fit/users/zara".to_string(),
            from_username: "zara".to_string(),
            from_display_name: Some("Zara Hill".to_string()),
            from_avatar_url: None,
            object_ap_id: Some("https://jogga.fit/users/alex/exercises/2".to_string()),
            object_title: Some("Morning Tempo Run".to_string()),
            is_read: true,
            created_at: "2026-05-15T10:05:00Z".to_string(),
        },
    ]
}
