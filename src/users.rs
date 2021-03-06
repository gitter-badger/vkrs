use std::borrow::Borrow;
use std::convert::AsRef;
use super::api::Collection;

#[cfg(feature = "nightly")]
include!("users.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/users.rs"));

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum NameCase {
    Nominative,
    Genetive,
    Dative,
    Accusative,
    Instrumental,
    Ablative,
}

impl AsRef<str> for NameCase {
    fn as_ref(&self) -> &str {
        use self::NameCase::*;
        match *self {
            Nominative => "nom",
            Genetive => "gen",
            Dative => "dat",
            Accusative => "acc",
            Instrumental => "ins",
            Ablative => "abl",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum UserOptionField {
    Verified,
    Blacklisted,
    Sex,
    Birthdate,
    City,
    Country,
    HomeTown,
    Photo50,
    Photo100,
    Photo200Orig,
    Photo200,
    Photo400Orig,
    PhotoMax,
    PhotoMaxOrig,
    Online,
    Lists,
    Domain,
    HasMobile,
    Contacts,
    Site,
    Education,
    Universities,
    Schools,
    Status,
    LastSeen,
    FollowersCount,
    CommonCount,
    Counters,
    Occupation,

    Nickname,
    Relatives,
    Relation,
    Personal,
    Connections,
    Exports,
    WallComments,
    Activities,
    Interests,
    Music,
    Movies,
    TvShows,
    Books,
    Games,
    About,
    Quotes,
    CanPost,
    CanSeeAllPosts,
    CanSeeAudio,
    CanWritePrivateMessage,
    Timezone,
    ScreenName,
}

impl AsRef<str> for UserOptionField {
    fn as_ref(&self) -> &str {
        use self::UserOptionField::*;
        match *self {
            Verified => "verified",
            Blacklisted => "blacklisted",
            Sex => "sex",
            Birthdate => "bdate",
            City => "city",
            Country => "country",
            HomeTown => "home_town",
            Photo50 => "photo_50",
            Photo100 => "photo_100",
            Photo200Orig => "photo_200_orig",
            Photo200 => "photo_200",
            Photo400Orig => "photo_400_orig",
            PhotoMax => "photoMax",
            PhotoMaxOrig => "photo_max_orig",
            Online => "online",
            Lists => "lists",
            Domain => "domain",
            HasMobile => "has_mobile",
            Contacts => "contacts",
            Site => "site",
            Education => "education",
            Universities => "universities",
            Schools => "schools",
            Status => "status",
            LastSeen => "last_seen",
            FollowersCount => "followers_count",
            CommonCount => "common_count",
            Counters => "counters",
            Occupation => "occupation",

            Nickname => "nickname",
            Relatives => "relatives",
            Relation => "relation",
            Personal => "personal",
            Connections => "connections",
            Exports => "exports",
            WallComments => "wall_comments",
            Activities => "activities",
            Interests => "interests",
            Music => "music",
            Movies => "movies",
            TvShows => "tv",
            Books => "books",
            Games => "games",
            About => "about",
            Quotes => "quotes",
            CanPost => "can_post",
            CanSeeAllPosts => "can_see_all_posts",
            CanSeeAudio => "can_see_audio",
            CanWritePrivateMessage => "can_write_private_message",
            Timezone => "timezone",
            ScreenName => "screen_name",
        }
    }
}

request_lt! {
    struct Get for ["users.get"](v => 5.44) -> Vec<User> {
        sized {
            name_case: NameCase = (NameCase::Nominative) => {AsRef},
        }
        unsized {
            user_ids: [i64] = (&[][..]) => {Vec},
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

request_lt! {
    struct Search for ["users.search"](v => 5.44) -> Collection<User> {
        sized {
            sort: Sort = (Sort::Rating) => {AsRef},

            city: Option<u64> = () => {Option},
            country: Option<u64> = () => {Option},

            university: Option<u64> = () => {Option},
            university_country: Option<u64> = () => {Option},
            university_faculty: Option<u64> = () => {Option},
            university_chair: Option<u64> = () => {Option},
            university_year: Option<u16> = () => {Option},

            sex: Sex = (Sex::Any) => {AsRef},
            status: Status = (Status::Unknown) => {AsRef},

            age_from: Option<u16> = () => {Option},
            age_to: Option<u16> = () => {Option},
            birth_day: Option<u8> = () => {Option},
            birth_month: Option<u8> = () => {Option},
            birth_year: Option<u16> = () => {Option},

            online: bool = (false) => {bool},
            has_photo: bool = (false) => {bool},

            school: Option<u64> = () => {Option},
            school_country: Option<u64> = () => {Option},
            school_city: Option<u64> = () => {Option},
            school_class: Option<u64> = () => {Option},
            school_year: Option<u16> = () => {Option},

            group_id: Option<i64> = () => {Option},

            offset: usize = (0) => {},
            count: usize = (100) => {},
        }
        unsized {
            q: str = ("") => {=},
            hometown: str = ("") => {=},
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
            religion: str = ("") => {=},
            interests: str = ("") => {=},
            company: str = ("") => {=},
            position: str = ("") => {=},
            from_list: str = ("") => {=},
        }
    }
}

request! {
    struct IsAppUser for ["users.isAppUser"](v => 5.44) -> u8 {
        user_id: i64 = (0) => {}
    }
}

request_lt! {
    struct GetSubscriptions for ["users.getSubscriptions"](v => 5.44, extended => 1) -> Collection<User> {
        sized {
            user_id: i64 = (0) => {},
            offset: usize = (0) => {},
            count: usize = (100) => {},
        }
        unsized {
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

request_lt! {
    struct GetFollowers for ["users.getFollowers"](v => 5.44) -> Collection<User> {
        sized {
            user_id: i64 = (0) => {},
            name_case: NameCase = (NameCase::Nominative) => {AsRef},
            offset: usize = (0) => {},
            count: usize = (100) => {},
        }
        unsized {
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

request_lt! {
    struct Report for ["users.report"](v => 5.44) -> u8 {
        sized {
            user_id: i64 = (0) => {},
            //type: ReportReason = (ReportReason::Spam) => {AsRef},
        }
        unsized {
            comment: str = ("") => {=},
        }
    }
}

//request_lt! {
    //struct GetSubscriptionIds for ["users.getSubscriptions"](v => 5.44, extended => 0) -> Collection<i64> {
        //sized {
            //user_id: i64 (0) => {},
            //offset: usize (0) => {},
            //count: usize (100) => {},
        //}
        //unsized {
            //fields: str = ("") => {=},
        //}
    //}
//}

request_lt! {
    struct GetNearby for ["users.getNearby"](v => 5.44) -> Collection<User> {
        sized {
            latitude: f32 = () => {},
            longitude: f32 = () => {},
            accuracy: u16 = () => {},
            timeout: u16 = (7299) => {},
            radius: Radius = (Radius::R300) => {AsRef},
            name_case: NameCase = (NameCase::Nominative) => {AsRef},
        }
        unsized {
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Radius {
    R300 = 1,
    R2400 = 2,
    R18000 = 3,
    R150000 = 4,
}

impl AsRef<str> for Radius {
    fn as_ref(&self) -> &str {
        use self::Radius::*;
        match *self {
            R300 => "1",
            R2400 => "2",
            R18000 => "3",
            R150000 => "4",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Sort {
    Rating = 0,
    DateRegistered = 1,
}

impl AsRef<str> for Sort {
    fn as_ref(&self) -> &str {
        match *self {
            Sort::Rating => "0",
            Sort::DateRegistered => "1",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Sex {
    Any = 0,
    Female = 1,
    Male = 2,
}

impl AsRef<str> for Sex {
    fn as_ref(&self) -> &str {
        match *self {
            Sex::Any => "0",
            Sex::Female => "1",
            Sex::Male => "2",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Status {
    Unknown = 0,
    NotMarried = 1,
    InRelationship = 2,
    Engaged = 3,
    Married = 4,
    ItsComplicated = 5,
    ActiveSearch = 6,
    InLove = 7,
}

impl AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        use self::Status::*;
        match *self {
            Unknown => "0",
            NotMarried => "1",
            InRelationship => "2",
            Engaged => "3",
            Married => "4",
            ItsComplicated => "5",
            ActiveSearch => "6",
            InLove => "7",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ReportReason {
    Porn,
    Spam,
    Insult,
    Ads
}


impl AsRef<str> for ReportReason {
    fn as_ref(&self) -> &str {
        use self::ReportReason::*;
        match *self {
            Porn => "porn",
            Spam => "spam",
            Insult => "insult",
            Ads => "advertisment",
        }
    }
}
