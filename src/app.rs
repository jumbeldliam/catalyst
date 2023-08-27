use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use uuid::Uuid;
use leptos::html::Input;
use chrono::prelude::*;
//use serde::{Serialize, Deserialize};
use leptos_use::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    //leptos forces you to use ssr out of the box and its a pain to config
    //set_interval can obv not be called from the server.
    //-2 hours.
    //let current_time = create_interval_clock(cx);

    let club = Club::new(cx, "Club Name".to_owned());
    let b = EventPreview::new(cx, club.clone()).into_post(cx);

    let (current_time, set_current_time) = create_signal(cx, DateTime::now());

    let current_app_page = PageRepr::Home;

    //TODO: need to fetch here
    let current_club_events = create_rw_signal(cx, Vec::<ClubEvent>::new());

    current_club_events.update(|prev| prev.push(b));

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        <Title text=""/>

        <Router>
            <main>
                <NavBar current_page=current_app_page/>
                <div style:height="45px" style:width="100%"/>

                <Routes>
                    <Route path="" view=move |_| view!{cx, <Home events=current_club_events current_time/>}/>
                    <Route path="/calander" view=move |_| view!{cx, <CalanderPage events=current_club_events current_time/>}/>
                    <Route path="/login" view=Login/>
                    //<Route path="/create" view=move |_| view!{cx, <ViewCreator />}

                    <Route path="/create" view=move |_| view!{cx, <EventCreator club=club.clone() current_events=current_club_events/>} />

                    <Route path="/posts" view=PostWr>
                        <Route path=":id" view=FullPost/>
                    </Route>

                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn PostWr(cx: Scope) -> impl IntoView {
    view!{
        cx,
        <div class="full_post_wr">
            <Outlet/>
        </div>
    }
}

fn get_post_from_id(cx: Scope, id: String) -> ClubEvent {
    let club = Club::new(cx, "Club Name".to_owned());
    EventPreview::new(cx, club).into_post(cx)
}

#[component]
fn FullPost(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    let post = get_post_from_id(cx, params.with(|p| p.get("posts").cloned().unwrap_or_default()));

    view!{
        cx,
        <div class="full_post">
            <div class="full_post_header">
                <div class="full_post_club_desg">
                    <div class="full_post_club_name"> {post.hosting_club.name.get()} </div>
                    <div class="full_post_club_acr"> {format!("({})", post.hosting_club.club_acronym.get())} </div>
                </div>
                <p class="reset"/>
                <div class="full_post_event_name"> {post.event_name.get()} </div>
                <p class="reset"/>
                <p class="reset"/>
            </div>
            <div class="full_post_body_wr" style:color=move || format!("color-mix(in srgb, {} 50%, rgba(0, 0, 0, .75))", post.colour.get())>
                <p class="full_post_body">
                    {post.post_body.get()}
                </p>
            </div>

            <div class="full_post_extra_info"> 
                {post.start_date.get().format_date_range(&post.end_date.get())}
                " @ "
                {post.location.get().address}
            </div>

            <div class="full_post_notifs" style:color=move || format!("color-mix(in srgb, {} 50%, rgba(0, 0, 0, .75))", post.colour.get())>
                <div class="full_post_food_notif"> "*Lunch & Dinner Provided" </div>
                <div class="full_post_limited_spots_notif"> "*Limited Spots Available!" </div>
            </div>
        </div>
    }
}


enum PageRepr {
    Home,
    Login,
    Calander,
    Other,
}

#[component]
fn NavBar(cx: Scope, current_page: PageRepr) -> impl IntoView {
    //TODO: based on current url state, can allow for different buttons in the bar to be 
    //commented out


    let navigate_home = use_navigate(cx);
    let navigate_calander = use_navigate(cx);
    let navigate_create = use_navigate(cx);

    let send_to_home = move || {
        let navigate_ops = NavigateOptions::default();
        let _ = navigate_home("", navigate_ops);
    };

    let send_to_calander = move || {
        let navigate_ops = NavigateOptions::default();
        let _ = navigate_calander("/calander", navigate_ops);
    };

    let send_to_create = move || {
        let navigate_ops = NavigateOptions::default();
        let _ = navigate_create("/create", navigate_ops);
    };

    view!{
        cx,
        <div class="justify_column navigation">
            <div> "events" </div>
            <div class="justify_column center_nav">
                <button class="reset nav_btn" on:click=move |_| send_to_home()> "home"</button>
                <div> "-" </div>
                <button class="reset nav_btn" on:click=move |_| send_to_calander()> "Calander" </button>
            </div>



                <button class="reset nav_btn" on:click=move |_| send_to_create()> "Create" </button>
        </div>
    }
}

#[component] 
fn Login(cx: Scope) -> impl IntoView {

    view!{
        cx,
    }
}

fn create_interval_clock(cx: Scope) -> ReadSignal<DateTime> {
    let (time, set_time) = create_signal(cx, DateTime::now());
    set_interval(move || set_time.set(DateTime::now()), std::time::Duration::from_secs(1));
    time
}

#[derive(PartialEq, Clone)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}

impl From<u8> for Month {
    fn from(f: u8) -> Month {
        match f {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => unreachable!(),
        }
    }
}

impl From<Month> for u8 {
    fn from(f: Month) -> u8 {
        match f {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }

    }
}

impl Month {
    fn next_month(self) -> Self {
        match u8::from(self) {
            12 => Month::January,
            o => Month::from(o + 1)
        }
    }

    fn previous_month(self) -> Self {
        match u8::from(self) {
            1 => Month::December,
            o => Month::from(o - 1),
        }
    }

    fn format_long(&self) -> String {
        match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        }.to_owned()
    }

    fn format_short(&self) -> String {
        match self {
            Month::January => "Jan",
            Month::February => "Feb",
            Month::March => "Mar",
            Month::April => "Apr",
            Month::May => "May",
            Month::June => "Jun",
            Month::July => "Jul",
            Month::August => "Aug",
            Month::September => "Sep",
            Month::October => "Oct",
            Month::November => "Nov",
            Month::December => "Dec",
        }.to_owned()
    }
}

#[derive(PartialEq, Eq, Clone)]
struct CalanderInfo {
    days_in_month: u8,
    day_offset: u8,
}

#[component]
pub fn CalanderPage(cx: Scope, events: RwSignal<Vec<ClubEvent>>, current_time: ReadSignal<DateTime>) -> impl IntoView {
    let (current_month, set_current_month) = create_signal(cx, DateTime::current_month());
    let (current_year, set_current_year) = create_signal(cx, DateTime::current_year());

    let info = Signal::derive(cx, move || {
        let month = current_month.get();
        let year = current_year.get();

        let days_in_month = DateTime::days_in_month_from_ym(month.clone(), year);
        let day_offset = DateTime::day_offset(month, year);

        CalanderInfo {
            days_in_month,
            day_offset,
        }
    });

    let increment_year = move || {
        set_current_year.update(|current_year| *current_year = *current_year + 1);
    };

    let decrement_year = move || {
        set_current_year.update(|current_year| *current_year = *current_year - 1);
    };


    let increment_month = move || {
        set_current_month.update(|current_month| {
            if current_month.clone() == Month::December {
                //increment_year();
            }

            *current_month = current_month.clone().next_month();
        })
    };

    let decrement_month = move || {
        set_current_month.update(|current_month| {
            if current_month.clone() == Month::January {
                //decrement_year()
            }

            *current_month = current_month.clone().previous_month()
        })

    };


    //TODO: year needs to change when month changes 

    view!{
        cx,
        <div class="calander_tool_helper">
            <div class="calander_helper">
                <TxtBtn text="next" on_click=move || increment_month()/>
                <div class="date_tracker">{move || format!("{} {}", current_month.get().format_short(), current_year.get())} </div>
                <TxtBtn text="last" on_click=move || decrement_month()/>
            </div>
            <Calander events info month=current_month year=current_year current_time/>
        </div>
    }
}

#[component]
fn Calander(cx: Scope, events: RwSignal<Vec<ClubEvent>>, info: Signal<CalanderInfo>, month: ReadSignal<Month>, year: ReadSignal<u32>, current_time: ReadSignal<DateTime>) -> impl IntoView {
    view!{
        cx,
        <div class="calander_wr">
                <For
                    each=move|| 0..7u8
                    key=|i| i.clone()
                    view=move |cx, day| {
                        view!{
                            cx,
                            <DayOfWeek day/>
                        }
                    }
                />
                <For
                    each=move || 0..info.get().day_offset
                    key=|i| i.clone()
                    view=move |cx, _| {
                        view!{
                            cx,
                            <BlankCalanderDay/>
                        }
                    }
                />
                <For
                    each=move || 0..info.get().days_in_month
                    key=|i| i.clone()
                    view=move |cx, day| {
                        view!{
                            cx,
                            <CalanderDay events day={day+1} month year current_time/>
                        }
                    }
                />
                <For
                    each=move || {
                        let offset_info = info.get();
                        let left = (7 - ((offset_info.days_in_month + offset_info.day_offset) % 7)) % 7;
                        0..left
                    }
                    key=|i| i.clone()
                    view=move |cx, _| {
                        view!{
                            cx,
                            <BlankCalanderDay/>
                        }
                    }
                />
        </div>
    }
}

#[component]
fn ImgBtn<T: Fn() -> () + 'static>(cx: Scope, link: &'static str, on_click: T) -> impl IntoView {
    let link = link.to_owned();
    view!{
        cx,
        <button on:click=move |_| on_click() class="reset img_icon_wpr">
            <img class="reset img_icon" src=link/>
        </button>
    }
}

#[component]
fn TxtBtn<T: Fn() -> () + 'static>(cx: Scope, text: &'static str, on_click: T) -> impl IntoView {
    let text = text.to_owned();

    view!{
        cx,
        <button on:click=move |_| on_click() class="reset txt_icon_wpr">
            {text}
        </button>
    }

}

enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<u8> for WeekDay {
    fn from(f: u8) -> WeekDay {
        match f % 7 {
            0 => Self::Monday,
            1 => Self::Tuesday,
            2 => Self::Wednesday,
            3 => Self::Thursday,
            4 => Self::Friday,
            5 => Self::Saturday,
            6 => Self::Sunday,
            _ => unreachable!()
        }
    }
}

impl From<WeekDay> for u8 {
    fn from(f: WeekDay) -> u8 {
        match f {
            WeekDay::Monday => 0,
            WeekDay::Tuesday => 1,
            WeekDay::Wednesday => 2,
            WeekDay::Thursday => 3,
            WeekDay::Friday => 4,
            WeekDay::Saturday => 5,
            WeekDay::Sunday => 6,
        }
    }
}

impl WeekDay {
    fn format_long(&self) -> String {
        match self {
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Sunday => "Sunday"
        }.to_owned()
    }

    fn format_short(&self) -> String {
        match self {
            Self::Monday => "Mon",
            Self::Tuesday => "Tue",
            Self::Wednesday => "Wed",
            Self::Thursday => "Thu",
            Self::Friday => "Fri",
            Self::Saturday => "Sat",
            Self::Sunday => "Sun"
        }.to_owned()
    }
}

#[component]
pub fn DayOfWeek(cx: Scope, day: u8) -> impl IntoView {

    let (format_short, set_format_short) = create_signal(cx, false);

    let formatted_day = Signal::derive(cx, move || {
        let weekday = WeekDay::from(day);
        if format_short.get() {
            weekday.format_short()
        }else{
            weekday.format_long()
        }
    });

    view!{
        cx,
        <div class="calander_day_of_week">{formatted_day}</div>
    }


}

struct SerializedClubEvent {
    start_date: DateTime,
    end_date: DateTime,
    event_name: String,
    current_attendants: u32,
    event_options: SerializedEventOptions,
    location: SerializedLocation,
    image: Option<SerializedEventImage>,
    hosting_club: SerializedClub,
    cohosting_clubs: Vec<SerializedClub>,
}

struct SerializedEventImage {
    image_url: String,
    description: String,
}

#[derive(Clone, PartialEq, Eq)]
struct EventImage {
    image_url: RwSignal<String>,
    description: RwSignal<String>,
}

struct SerializedLocation {
    address: String
}

struct SerializedLimitedSpots {
    spots_total: u32,
    spots_left: u32,
}

struct SerializedEventOptions {
    members_only: bool,
    tickets_required: Option<u32>,
    limited_spots: Option<SerializedLimitedSpots>,
    prizes_available: Option<Prizes>,
    food_available: Option<SerializedProvidedFood>,
}

struct SerializedProvidedFood {
    drinks: bool,
    snacks: bool,
    meals: Option<SerializedMeals>,
}

struct SerializedMeals {
    lunch: bool,
    dinner: bool,
    breakfast: bool,
}

#[derive(PartialEq, Clone)]
enum Prizes {
    Cash(f32),
    Other(String),
}

impl Prizes {
    fn format_prizes(&self) -> String {
        match self {
            Self::Cash(amt) => format!("up to ${:2}", amt),
            Self::Other(prize) => prize.to_owned(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DateTime(i64);

impl DateTime {
    pub fn format_date(&self) -> String {
        let local_date = self.as_raw_date();
        local_date.to_string()
    }

    //actual iso formatting which html input can use
    pub fn into_iso_date(&self) -> String {
        NaiveDateTime::from_timestamp_opt(self.0, 0).unwrap().to_string()
    }

    pub fn format_time(&self) -> String {
        let raw_time = self.0;
        let time = chrono::Local.timestamp_opt(raw_time, 0).unwrap();
        let minutes = time.minute();

        let minutes = if minutes < 10 {
            format!("0{}", minutes)
        }else {
            format!("{}", minutes)
        };

        format!("{}:{}", time.hour(), minutes)
    }

    #[inline]
    pub fn format_datetime(&self) -> String {
        format!("{} {}", self.format_date(), self.format_time())
    }

    pub fn now() -> Self {
        Self(Self::now_raw())
    }

    fn now_raw() -> i64 {
        chrono::offset::Utc::now().timestamp()
    }

    fn as_raw_seconds(&self) -> u32 {
        self.0 as u32
    }

    pub fn format_date_range(&self, other: &Self) -> String {
        if self.format_date() != other.format_date() {
            //goes over day bounds
            if self < other {
                format!("{} - {}", self.format_datetime(), other.format_datetime())
            }else{
                format!("{} - {}", other.format_datetime(), self.format_datetime())
            }
        }else{
            let date = self.format_date();
            if self < other {
                format!("{} {} - {}", date, self.format_time(), other.format_time())
            }else{
                format!("{} {} - {}", date, other.format_time(), self.format_time())
            }
        }
    }

    pub fn format_duration(&self, other: &Self) -> SmallDuration {
        let duration = if self > other {
            self.0 - other.0
        }else{
            other.0 - self.0
        };

        SmallDuration::from_seconds(duration as u32)
    }

    pub fn days_in_current_month(&self) -> u8 {
        let date = self.as_raw_date();

        let current_month = date.month();
        let current_year = date.year();
        let next_year = match current_month {
            12 => current_year + 1,
            _ => current_year,
        };

        let next_month = match current_month {
            12 => 1,
            _ => current_month + 1,
        };

        let days = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap()
            .signed_duration_since(NaiveDate::from_ymd_opt(current_year, current_month, 1).unwrap())
            .num_days();

        days as u8
    }

    fn days_in_month_from_ym(month: Month, year: u32) -> u8 {
        let current_month = u8::from(month.clone());
        let next_month = u8::from(month.next_month());
        let next_year = match current_month {
            12 => year + 1,
            _ => year,
        };

        let days = NaiveDate::from_ymd_opt(next_year as i32, next_month as u32, 1).unwrap()
            .signed_duration_since(NaiveDate::from_ymd_opt(year as i32, current_month as u32, 1).unwrap())
                .num_days();
        days as u8
    }

    pub fn day_of_month(&self) -> u8 {
        let date = self.as_raw_date();

        let current_month = date.month();
        let current_year = date.year();

        let elapsed = date.signed_duration_since(NaiveDate::from_ymd_opt(current_year, current_month, 1).unwrap()).num_days();

        elapsed as u8
    }

    fn as_raw_date(&self) -> NaiveDate {
        chrono::Local.timestamp_opt(self.0, 0).unwrap().date_naive()
    }

    pub fn current_month() -> Month {
        let month = Self::now().as_raw_date().month();
        Month::from(month as u8)
    }

    pub fn current_year() -> u32 {
        let year = Self::now().as_raw_date().year();
        year as u32
    }

    //# of days padded from the first day of the month to last sunday
    pub fn day_offset(month: Month, year: u32) -> u8 {
        let month = u8::from(month) as u32;
        let current_date = NaiveDate::from_ymd_opt(year as i32, month, 1).unwrap();
        current_date.weekday().number_from_monday() as u8 - 1
    }

    pub fn day_range_from_ymd(year: u32, month: Month, day: u8) -> DayRange {
        let norm = NaiveDate::from_ymd_opt(year as i32, u8::from(month) as u32, day as u32).unwrap();
        //its most likely something with this
        //upper goes too high and lower doesnt seem to be changing
        let lower = norm.and_hms_opt(0, 0, 0).unwrap();
        let upper = norm.and_hms_opt(23, 59, 59).unwrap();
        //could make it so that it gets a naivetime and just adds the amt of seconds in a day for
        //the later :)
        DayRange {
            begining: Self(lower.timestamp()),
            end: Self(upper.timestamp())
        }
    }

    pub fn as_seconds_from_start_of_day(&self) -> u32 {
        let time = NaiveDateTime::from_timestamp_opt(self.0, 0).unwrap().time();
        (time.hour() * 60 * 60) + (time.minute() * 60) + time.second()
    }

    pub fn as_percentage_of_day(&self) -> f64 {
        const SECONDS_IN_DAY: u32 = 60 * 60 * 24;
        let seconds_since_start = self.as_seconds_from_start_of_day();
        100f64 * f64::from(seconds_since_start) / f64::from(SECONDS_IN_DAY) 
    }

    pub fn offset_by_hour_min(&self, hour: i16, minute: i8) -> Self {
        let mut current = self.0;
        current += (hour as i64) * 60 * 60;
        current += (minute as i64) * 60;

        Self(current)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct DayRange {
    begining: DateTime,
    end: DateTime,
}

#[derive(Clone, PartialEq, Eq,)]
pub struct ClubEvent {
    start_date: RwSignal<DateTime>,
    end_date: RwSignal<DateTime>,
    event_name: RwSignal<String>,
    current_attendant_count: RwSignal<u32>,
    options: EventOptions,
    post_body: RwSignal<String>,
    post_image: RwSignal<Option<EventImage>>,
    location: RwSignal<Location>,
    hosting_club: Club,
    post_id: Uuid,
    cohosting_clubs: RwSignal<Vec<Club>>,
    colour: RwSignal<String>,
}

#[derive(Clone, PartialEq, Eq)]
struct Location {
    address: RwSignal<String>,
}

impl Location {
    fn new(cx: Scope) -> Self {
        let address = create_rw_signal(cx, "South Lawn".to_owned());

        Self{
            address
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct EventOptions {
    members_only: RwSignal<bool>,
    tickets_required: RwSignal<Option<f32>>,
    limited_spots: RwSignal<Option<LimitedSpots>>,
    prizes_available: RwSignal<Option<Prizes>>,
    food_available: RwSignal<Option<ProvidedFood>>
}

impl EventOptions {
    const DEFAULT_TICKET_PRICE: f32 = 5f32;
    const DEFAULT_LIMITED_SPOTS: u32 = 100;
    const DEFAULT_PRIZE: Prizes = Prizes::Cash(100f32);

    fn new(cx: Scope) -> Self {
        let members_only  = create_rw_signal(cx, false);
        let tickets_required = create_rw_signal(cx, None);
        let limited_spots = create_rw_signal(cx, None);
        let prizes_available = create_rw_signal(cx, None);
        let food_available = create_rw_signal(cx, None);

        Self {
            members_only,
            tickets_required,
            limited_spots,
            prizes_available,
            food_available,
        }
    }

    fn toggle_members_only(&self) {
        self.members_only.update(|prev| *prev = !*prev)
    }

    fn toggle_tickets(&self, tickets: Option<f32>) {
        self.tickets_required.update(|prev| {
            *prev = match prev {
                Some(_) => None,
                None => Some(tickets.unwrap_or(Self::DEFAULT_TICKET_PRICE)),
            }
        })
    }

    fn edit_ticket_price(&self, price: f32) {
        self.tickets_required.update(|prev| *prev = Some(price))
    }

    fn toggle_limited_spots(&self, cx: Scope, limited_count: Option<u32>) {
        self.limited_spots.update(|prev| {
            *prev = match prev {
                Some(_) => None,
                None => {
                    let spots = create_rw_signal(cx, limited_count.unwrap_or(Self::DEFAULT_LIMITED_SPOTS));

                    Some(LimitedSpots{
                        spots_total: spots,
                        spots_left: spots,
                    })
                }   

            };
        });
    }

    fn toggle_prizes_available(&self, prizes: Option<Prizes>) {
        self.prizes_available.update(|prev| {
            *prev = match prev {
                Some(_) => None,
                None => Some(prizes.unwrap_or(Self::DEFAULT_PRIZE)),
            }
        })
    }

    #[inline]
    fn edit_prize_val(&self, prize: Prizes) {
        self.prizes_available.update(|prev| *prev = Some(prize))
    }

    fn toggle_food(&self, cx: Scope, food: Option<ProvidedFood>) {
        self.food_available.update(|prev| {
            *prev = match prev {
                Some(_) => None,
                None => Some(food.unwrap_or(ProvidedFood::new(cx))),
            }
        })
    }
}



#[derive(Clone, PartialEq, Eq)]
struct ProvidedFood {
    drinks: RwSignal<bool>,
    snacks: RwSignal<bool>,
    meals: RwSignal<Option<Meals>>
}

impl ProvidedFood {
    fn format_minimal(&self) -> String {
        if self.meals.get().is_some() {
            "food provided"
        }else if self.drinks.get() && self.snacks.get() {
            "snacks & drinks provided"
        }else if self.drinks.get() {
            "drinks provided"
        }else if self.snacks.get() {
            "snacks provided"
        }else {
            ""
        }.to_owned()
    }

    fn new(cx: Scope) -> Self {
        let drinks = create_rw_signal(cx, false);
        let snacks = create_rw_signal(cx, false);
        let meals = create_rw_signal(cx, None);

        Self{
            drinks,
            snacks,
            meals,
        }
    }


    fn toggle_drinks(&self) {
        self.drinks.update(|prev| *prev = !*prev)
    }

    fn toggle_snacks(&self) {
        self.snacks.update(|prev| *prev = !*prev)
    }

    fn toggle_meals(&self, cx: Scope, meals: Option<Meals>) {
        self.meals.update(|prev| {
            *prev = match prev {
                Some(_) => None,
                None => Some(meals.unwrap_or(Meals::new(cx)))
            }
        })
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Meals {
    lunch: RwSignal<bool>,
    dinner: RwSignal<bool>,
    breakfast: RwSignal<bool>,
}

impl Meals {
    fn format_provision(&self) -> Option<String> {
        Some(if self.lunch.get() && self.dinner.get() {
            "lunch & dinner provided"
        }else if self.lunch.get() {
            "lunch provided"
        }else if self.dinner.get() {
            "dinner provided"
        }else {
            return None;
        }.to_owned())
    }

    fn new(cx: Scope) -> Self {
        let lunch = create_rw_signal(cx, true);
        let dinner = create_rw_signal(cx, true);
        let breakfast = create_rw_signal(cx, false);

        Self{
            lunch,
            dinner,
            breakfast,
        }
    }

    fn toggle_lunch(&self) {
        self.lunch.update(|prev| *prev = !*prev)
    }

    fn toggle_dinner(&self) {
        self.dinner.update(|prev| *prev = !*prev)
    }

    fn toggle_breakfast(&self) {
        self.breakfast.update(|prev| *prev = !*prev)
    }
}

#[derive(PartialEq, Eq, Clone)]
struct LimitedSpots {
    spots_total: RwSignal<u32>,
    spots_left: RwSignal<u32>,
}

#[derive(PartialEq, Eq, Clone)]
struct DayFormattedEvent {
    day_start: MaybeDateTime,
    day_end: MaybeDateTime,
    event: ClubEvent,
}

#[derive(PartialEq, Eq, Clone)]
enum MaybeDateTime {
    Actual(DateTime),
    Layover(DateTime),
}

impl MaybeDateTime {
    fn should_format(&self) -> bool {
        match self {
            Self::Actual(_) => true,
            _ => false,
        }
    }

    fn map_as_tick<F: FnOnce(DateTime) -> R, R: Sized>(self, apply: F) -> Option<R> {
        match self {
            Self::Actual(time) => Some(apply(time)),
            _ => None
        }
    }

    fn as_raw(&self) -> DateTime {
        match self {
            Self::Actual(a) | Self::Layover(a) => a.clone()
        }
    }
}



#[component]
fn CalanderDay(cx: Scope, day: u8, events: RwSignal<Vec<ClubEvent>>, month: ReadSignal<Month>, year: ReadSignal<u32>, current_time: ReadSignal<DateTime>) -> impl IntoView {

    //TODO: need to format the range of time it can be from the event start-end dates

    let day_range = Signal::derive(cx, move || {
        DateTime::day_range_from_ymd(year.get(), month.get(), day)
    });

    let today_events = Signal::derive(cx, move || {
        events.with(|events| events.iter().filter(|event| {
            let event_start = event.start_date.get_untracked();
            let event_end = event.end_date.get_untracked();

            let DayRange {
                begining,
                end,
                ..
            } = day_range.get();

            (event_start >= begining && event_start <= end) || (event_end <= end && event_end >= begining) || (event_start <= begining && event_end >= end)
        }).cloned().map(|event| {
            let event_start = event.start_date.get_untracked();
            let event_end = event.end_date.get_untracked();

            let DayRange {
                begining,
                end,
                ..
            } = day_range.get();
           
            if event_start >= begining && event_start <= end {
                //at least starts today
                let end = if event_end > end {
                    MaybeDateTime::Layover(end)
                }else{
                    MaybeDateTime::Actual(event_end)
                };

                DayFormattedEvent {
                    day_start: MaybeDateTime::Actual(event_start),
                    day_end: end,
                    event,
                }
            }else if event_end <= end && event_end >= begining {
                //at least ends today
                let begining = if event_start < begining {
                    MaybeDateTime::Layover(begining)
                }else{
                    MaybeDateTime::Actual(event_start)
                };

                DayFormattedEvent {
                    day_start: begining,
                    day_end: MaybeDateTime::Actual(event_end),
                    event,
                }

            }else if event_start <= begining && event_end >= end {
                //all day today

                DayFormattedEvent {
                    day_start: MaybeDateTime::Layover(begining),
                    day_end: MaybeDateTime::Layover(end),
                    event,
                }
            }else {
                DayFormattedEvent {
                    day_start: MaybeDateTime::Actual(event_start),
                    day_end: MaybeDateTime::Actual(event_end),
                    event
                }
            }
        }).collect::<Vec<_>>())
    });

    //TODO: do the little bar thing for the current_daate

    view!{
        cx, 
        <div class="calander_day event_day_calander">
            <h1 class="event_day_number">{day}</h1>
            <For
                each=move || today_events.get()
                key=|event| event.event.post_id
                view=move |cx, event: DayFormattedEvent| view!{
                    cx, 
                    <CalanderEventBlob event current_time/>
                }
            />
        </div>
    }
}

#[component]
fn CalanderEventBlob(cx: Scope, event: DayFormattedEvent, current_time: ReadSignal<DateTime>) -> impl IntoView {
    let top = event.day_start.as_raw().as_percentage_of_day();
    let bottom = event.day_end.as_raw().as_percentage_of_day();
    let height = bottom - top;

    let (user_time_scope, set_user_time_scope) = create_signal(cx, SmallDuration::from_minutes(30));
    let event_state = EventTimeTracker::create(cx, current_time, event.event.start_date.read_only(), event.event.end_date.read_only(), user_time_scope);

    //TODO: on hover should expand to give more info about the event

    let navigate = use_navigate(cx);

    let send_to_post = move || {
        let post_id = event.event.post_id.to_string();
        let post_url = format!("/posts/{}", post_id);
        let navigate_ops = NavigateOptions::default();
        let _ = navigate(&*post_url, navigate_ops);
    };


    view!{
        cx,
        <div class="calander_event_blob" on:click=move|_| send_to_post() style:top=format!("{}%", top + 50f64) style:height=format!("{}%", height) style:background=move || format!("5px 5px color-mix(in srgb, {} 50%, rgba(0, 0, 0, .75))", event.event.colour.get())>
            {event.day_start.map_as_tick(|start| view!{cx, <div class="calander_event_blob_time calander_event_blob_start_time" style:bottom=format!("{}%", top + 50f64)>{start.format_time()}</div>})}

            {event.day_end.map_as_tick(|end| view!{cx, <div class="calander_event_blob_time calander_event_blob_end_time" style:top=format!("{}%", bottom + 50f64)>{end.format_time()}</div>})}
            <div class="calander_event_club_header">
                <Pfp profile_picture=event.event.hosting_club.pfp.get()/>
                <p class="center_vert_abs calander_blob_club_acr">{event.event.hosting_club.club_acronym}</p>

                <p class="center_vert_abs calander_blob_event_name">
                    {event.event.event_name}
                </p>
            </div>
            {move || event_state.get().is_running().then_some(view!{cx, <div class="calander_blob_running_notice"></div>})}
            //{move || event_state.get().has_time_specific().then_some(view!{cx, <div class="calander_blob_time_specific">{event_state.get().format_time_left()}</div>})}
        </div>
    }
}

#[component]
fn BlankCalanderDay(cx: Scope) -> impl IntoView {
    view!{
        cx,
        <div class="calander_day blank_day_calander"></div>
    }
}

#[component]
fn CalanderClubEventView(cx: Scope, event: ReadSignal<ClubEvent>, currently_running: ReadSignal<bool>) -> impl IntoView {
    //NOTE: this should be what it expands to on hover

    view!{
        cx,
        <div>
            "big"
        </div>
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
pub struct SmallDuration {
    hour: u8,
    minute: u8,
    second: u8,
}

impl SmallDuration {
    pub fn from_minutes(minutes: u8) -> Self {
        Self {
            minute: minutes,
            ..Default::default()
        }
    }

    pub fn from_hour_and_minute(hours: u8, minutes: u8) -> Self {
        Self {
            hour: hours,
            minute: minutes,
            ..Default::default()
        }
    }

    fn into_seconds(&self) -> u32 {
        (self.hour as u32) * 60 * 60 + (self.minute as u32) * 60 + (self.second as u32)
    }

    fn from_seconds(seconds: u32) -> Self {
        let mut remaining = seconds;
        let hour = (remaining / (60 * 60)) as u8;
        remaining %= 60 * 60;
        let minute = (remaining / 60) as u8;
        remaining %= 60;
        let second = remaining as u8;

        Self {
            hour,
            minute,
            second,
        }
    }

    //largest gives hours
    fn into_largest_timescale(&self) -> String {
        if self.hour > 0 {
            let hours = self.hour as f64;
            let minutes_as_hours = (self.minute as f64) / 2f64;
            format!("{:2} {}", hours + minutes_as_hours, "hours")
        }else {
            format!("{} {}", self.minute, "minutes")
        }
    }
}


#[derive(PartialEq, Eq, Clone)]
enum RunningState{
    Running,
    AboutToEnd(SmallDuration),
}


#[derive(PartialEq, Eq, Clone)]
enum EventTimeTracker {
    YetToStart,
    AboutToStart(SmallDuration),
    Running(RunningState),
    Ended,
}

impl EventTimeTracker {
    fn create(cx: Scope, current_time: ReadSignal<DateTime>, start_date: ReadSignal<DateTime>, end_date: ReadSignal<DateTime>, time_scope: ReadSignal<SmallDuration>) -> Signal<EventTimeTracker> {
        Signal::derive(cx, move || {
            let current = current_time.get();
            let start = start_date.get();
            let end = end_date.get();
            let time_scope = time_scope.get();

            if current < start {
                let seconds_until_start = start.as_raw_seconds() - current.as_raw_seconds();
                if time_scope.into_seconds() < seconds_until_start {
                    EventTimeTracker::AboutToStart(SmallDuration::from_seconds(seconds_until_start))
                }else {
                    EventTimeTracker::YetToStart
                }
            }else if current > end { 
                EventTimeTracker::Ended
            }else{
                let seconds_until_end = end.as_raw_seconds() - current.as_raw_seconds();

                let state = if time_scope.into_seconds() < seconds_until_end {
                    RunningState::AboutToEnd(SmallDuration::from_seconds(seconds_until_end))
                }else{
                    RunningState::Running
                };

                EventTimeTracker::Running(state)
            }
        })
    }

    fn format_time(&self) -> Option<String> {
        match self {
            Self::AboutToStart(time) | Self::Running(RunningState::AboutToEnd(time)) => {
                let hour_and_minute = if time.hour > 0 {
                    let minutes = if time.minute < 10 {
                        format!("0{}", time.minute)
                    }else{
                        format!("{}", time.minute)
                    };
                    format!("{}:{}", time.hour, minutes)
                }else {
                    format!("{}", time.minute)
                };

                let seconds = if time.second < 10 {
                    format!("0{}", time.second)
                }else{
                    format!("{}", time.second)
                };

                Some(format!("{}:{}", hour_and_minute, seconds))
            }

            _ => None,
        }
    }

    fn format_descriptor(&self) -> String {
        match self {
            Self::AboutToStart(_) => format!("about to start: {}", self.format_time().unwrap()),
            Self::YetToStart => format!("has not started yet"),
            Self::Running(RunningState::Running) => format!("event is currently running"),
            Self::Running(RunningState::AboutToEnd(_)) => format!("about to end: {}", self.format_time().unwrap()),
            Self::Ended => format!("event has ended"),
        }
    }

    fn format_time_left(&self) -> Option<String> {
        match self {
            Self::AboutToStart(_) => Some(format!("time until: {}", self.format_time().unwrap())),
            Self::Running(RunningState::AboutToEnd(_)) => Some(format!("time left: {}", self.format_time().unwrap())),
            _ => None,
        }
    }

    fn has_time_specific(&self) -> bool {
        match self {
            Self::AboutToStart(_) | Self::Running(RunningState::AboutToEnd(_)) => true,
            _ => false,
        }
    }

    fn has_ended(&self) -> bool {
        match self {
            Self::Ended => true,
            _ => false,
        }
    }

    fn is_running(&self) -> bool {
        match self {
            Self::Running(_) => true,
            _ => false,
        }
    }
}

#[component]
fn PageClubEventView(cx: Scope, event: ReadSignal<ClubEvent>, current_time: ReadSignal<DateTime>, user_time_scope: ReadSignal<SmallDuration>) -> impl IntoView {
    let event_state = EventTimeTracker::create(cx, current_time, event.get().start_date.read_only(), event.get().end_date.read_only(), user_time_scope);

    view!{
        cx,
        <div>
            <ClubPostHeader club_event=event.get()/>
            <p class="club_post_body"> {event.get().post_body} </p>
            {move || event.get().post_image.get().map(|img| view!{cx, <ImgView img/>})}

            {move || event_state.get().is_running().then_some(view!{cx, <div class="page_event_running_notice">"running"</div>})}
            {move || event_state.get().has_time_specific().then_some(view!{cx, <div class="page_event_time_specific">{event_state.get().format_time_left()}</div>})}
        </div>
    }
}

#[derive(PartialEq, Eq, Clone)]
struct EventTimeInfo {
    date_range: String,
    duration: SmallDuration,
}

#[component]
fn PageClubFooter(cx: Scope, event: ClubEvent) -> impl IntoView {

    let event_date = Signal::derive(cx, move || {
        let start = event.start_date.get();
        let end = event.end_date.get();
        let date_range = start.format_date_range(&end);
        let duration = start.format_duration(&end);

        EventTimeInfo {
            date_range,
            duration
        }
    });

    let (show_duration, set_show_duration) = create_signal(cx, false);

    view!{
        cx,
        <div>
            <DateViewer event_date show_duration/>
            " @ "
            <LocationViewer location=event.location.get()/>
        </div>
    }
}

#[component]
fn LocationViewer(cx: Scope, location: Location) -> impl IntoView {

    //TODO: on click this should open like a uni map to the location or something
    view!{
        cx,
        <p class="location_view">{location.address}</p>
    }
}

#[component]
fn OptionsViewer(cx: Scope, event_options: EventOptions) -> impl IntoView {

    let food_desr = Signal::derive(cx, move || {
        let food = event_options.food_available.get().map(|food| food.format_minimal());
        food
    });

    view!{
        cx,

        <div>
            {move || event_options.tickets_required.get().map(|price| view!{cx, <PriceViewer price />})}
            {move || food_desr.get().map(|food| view!{cx, <div class="food_description">{food}</div>})}
            {move || event_options.prizes_available.get().map(|_| view!{cx, <div class="prizes_short">"prizes available!"</div>})}
            {move || event_options.limited_spots.get().map(|_| view!{cx, <div class="limited_spots_short">"limited spots!"</div>})}
        </div>
    }

}

#[component]
fn PriceViewer(cx: Scope, price: f32) -> impl IntoView {
    view!{
        cx,
        <div class="price_viewer">{format!("tickets: ${:2}", price)}</div>
    }
}

#[component]
fn PrizeViewer(cx: Scope, prize: ReadSignal<Prizes>) -> impl IntoView {
    view!{cx,
        <div class="prize_viewer"> {prize.get().format_prizes()} </div>
    }
}

#[component]
fn DateViewer(cx: Scope, event_date: Signal<EventTimeInfo>, show_duration: ReadSignal<bool>) -> impl IntoView {
    view!{
        cx,
        <div>
            <p class="event_date_range">{event_date.get().date_range}</p>
            {move || show_duration.get().then_some(view!{cx, <p class="event_date_duration">{format!("({})", event_date.get().duration.into_largest_timescale())}</p>})}
        </div>
    }
}

#[component]
fn ImgView(cx: Scope, img: EventImage) -> impl IntoView {
    view!{
        cx,
        <div class = "img_viewer">
            <img src=img.image_url alt=img.description/>
        </div>
    }
}

#[component]
fn CohostView(cx: Scope, cohosts: Vec<Club>) -> impl IntoView {
    view!{
        cx,
        <div>

        </div>
    }
}

#[component]
fn Pfp(cx: Scope, profile_picture: ProfilePicture) -> impl IntoView {
    view!{
        cx,
        <div class="club_pfp">
            <img src=profile_picture.url/>
        </div>
    }
}

#[derive(Clone, PartialEq, Eq)]
struct ShareInfo {
    socials: Vec<Social>,
    post_id: Uuid,
}

#[component]
fn ClubPostHeader(cx: Scope, club_event: ClubEvent) -> impl IntoView {

    let share_info = Signal::derive(cx, move || {
        let socials = club_event.hosting_club.linked_socials.get();
        let post_id = club_event.post_id;

        ShareInfo {
            socials,
            post_id
        }
    });

    view!{
        cx,
        <div>
            <div class="club_post_info">
                <Pfp profile_picture=club_event.hosting_club.pfp.get()/>
                <h2 class="club_post_name">{club_event.hosting_club.name}</h2>
                <h3 class="club_post_acronym">{ format!("({})", club_event.hosting_club.club_acronym.get())}</h3>
            </div>
            <h1>{club_event.event_name.get()} </h1>
            <ShareDots share_info/>
        </div>
    }
}

#[component]
fn ShareDots(cx: Scope, share_info: Signal<ShareInfo>) -> impl IntoView {
    view!{
        cx,
        <button class="share_dots">"..."</button>
    }
}

#[component]
fn ClubEventViewMinimized(cx: Scope, event: ClubEvent) -> impl IntoView {
    let (running, set_running) = create_signal(cx, false);
    let (maximized, set_maximized) = create_signal(cx, false);
    let (event, set_event) = create_signal(cx, event);

    view!{
        cx,
        <div>
            {move || maximized.get().then_some(view!{cx, <CalanderClubEventView event currently_running=running/>})}
        </div>
    }
}

struct SerializedClub {
    name: String,
    id: Uuid,
    found_date: DateTime,
    club_acronym: String,
    pfp: SerializedProfilePicture,
    linked_socials: Vec<SerializedSocial>,
}

struct SerializedSocial {
    link: String,

    //todo: serialized social kind
}

struct SerializedProfilePicture {
    url: String
}

#[derive(PartialEq, Eq, Clone)]
struct Club {
    name: RwSignal<String>,
    id: Uuid,
    found_date: DateTime,
    club_acronym: RwSignal<String>,
    pfp: RwSignal<ProfilePicture>,
    linked_socials: RwSignal<Vec<Social>>
}

impl Club {
    fn new(cx: Scope, name: String) -> Self {
        let name = create_rw_signal(cx, name);
        let found_date = DateTime::now();
        let id = Uuid::new_v4();
        let club_acronym = create_rw_signal(cx, "ACRN".to_owned());
        let url = create_rw_signal(cx, "".to_owned());
        let pfp = create_rw_signal(cx, ProfilePicture{url});
        let linked_socials = create_rw_signal(cx, Vec::new());

        Self {
            name,
            found_date,
            id,
            club_acronym,
            pfp,
            linked_socials
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Social {
    url: RwSignal<String>,
    kind: RwSignal<SocialKind>,
}

enum SocialKind {
    LinkedIn,
    Facebook,
    Instagram
}


#[derive(Clone, PartialEq, Eq)]
struct ProfilePicture {
    url: RwSignal<String>
}


#[component]
fn EventCreator(cx: Scope, club: Club, current_events: RwSignal<Vec<ClubEvent>>) -> impl IntoView {

    let (event, set_current_event) = create_signal(cx, EventPreview::new(cx, club.clone()));

    let update_event_name = move |e| {

    };

    let update_post_body = move |e| {

    };

    let update_start_date = move |e| {

    };

    let update_end_date = move |e| {

    };

    let update_location_address = move |e| {

    };


    let push_new_event = move || {
        let default_event = EventPreview::new(cx, club.clone()).into_post(cx);
        current_events.update(|events| events.push(default_event));
    };


    let event_options = Signal::derive(cx, move || event.get().options);

    view!{
        cx,
        <div>
            <h1>"event name" </h1>
            <input type="text" placeholder=event.get().event_name.get() value=event.get().event_name.get() on:keydown=move |e| update_event_name(e)/>

            <textarea value=event.get().post_body.get() placeholder=event.get().post_body.get() on:keydown=move |e| update_post_body(e) />

            <input type="datetime-local" value=event.get().start_date.get().into_iso_date() on:change=move |e| update_start_date(e)/>
            <input type="datetime-local" value=event.get().end_date.get().into_iso_date() on:change=move |e| update_end_date(e) />

            <input type="text" value=event.get().location.get().address.get() on:keydown=move |e| update_location_address(e) />

            <EventOptionsHelper event_options />

            <button on:click=move |_| push_new_event()>"publish event" </button>
        </div>
    }

}

#[component]
fn EventOptionsHelper(cx: Scope, event_options: Signal<EventOptions>) -> impl IntoView {

    let update_ticket_price = move |e| {

    };

    let update_prize = move |e| {

    };


    view!{
        cx,
        <div>
            <button on:click=move|_|event_options.get().toggle_members_only()> "members only"</button>

            //tickets price changing
            {move || event_options.get().tickets_required.get().map(|price| {
                                                                                view!{
                                                                                    cx,
                    <div>
                        <input type="text" value=price on:keydown=move |e| update_ticket_price(e) />
                    </div>
                                                                                }
                                                                            }).unwrap_or(view!{cx, 
                <div>
                    <button on:click=move|_|event_options.get().toggle_tickets(None)> "tickets" </button>
                </div>
            })}

            //prizes
            {move || event_options.get().prizes_available.get().map(|prizes| {
                                                                                view!{
                                                                                    cx,
                    <div>
                        <input type="text" value={prizes.format_prizes()} on:keydown=move |e| update_prize(e) />
                    </div>
                                                                                }
                                                                            }).unwrap_or(view!{cx, 
                <div>
                    <button on:click=move|_|event_options.get().toggle_prizes_available(None)> "prizes" </button>
                </div>
            })}

            //{move || event_options.get().limited_spots.get().map(|spots| {
                                                                                //view!{
                                                                                    //cx,
                    //<div>
                        //<input type="text" value=spots on:keydown=move |e| update_limited_spots(e) />
                    //</div>
                                                                                //}
                                                                            //}).unwrap_or(view!{cx, 
                //<div>
                    //<button on:click=move|_|event_options.get().toggle_limited_spots(cx, None)> "limited spots" </button>
                //</div>
            //})}

            //{move || event_options.get().limited_spots.get().map(|spots| {
                                                                                //view!{
                                                                                    //cx,
                    //<div>
                        //<input type="text" value=spots on:keydown=move |e| update_limited_spots(e) />
                    //</div>
                                                                                //}
                                                                            //}).unwrap_or(view!{cx, 
                //<div>
                    //<button on:click=move|_|event_options.get().toggle_limited_spots(cx, None)> "limited spots" </button>
                //</div>
            //})}

        </div>
    }

}

#[derive(PartialEq, Eq, Clone)]
struct EventPreview {
    start_date: RwSignal<DateTime>,
    end_date: RwSignal<DateTime>,
    event_name: RwSignal<String>,
    options: EventOptions,
    post_body: RwSignal<String>,
    post_image: RwSignal<Option<EventImage>>,
    location: RwSignal<Location>,
    hosting_club: Club,
    cohosting_clubs: RwSignal<Vec<Club>>,
}

fn generate_pastel_colour() -> String {
    format!{"rgb({}, {}, {})", fastrand::u8(200..), fastrand::u8(200..), fastrand::u8(200..)}
}

impl EventPreview {
    fn into_post(&self, cx: Scope) -> ClubEvent {
        let EventPreview {
            start_date,
            end_date,
            event_name,
            options,
            post_body,
            post_image,
            location,
            hosting_club,
            cohosting_clubs,
        } = self;

        let current_attendant_count = create_rw_signal(cx, 0u32);

        let colour = create_rw_signal(cx, generate_pastel_colour());


        ClubEvent {
            start_date: *start_date,
            end_date: *end_date,
            event_name: *event_name,
            options: options.clone(),
            post_body: *post_body,
            post_image: *post_image,
            location: *location,
            hosting_club: hosting_club.clone(),
            cohosting_clubs: *cohosting_clubs,
            current_attendant_count,
            post_id: Uuid::new_v4(),
            colour,
        }
    }

    fn new(cx: Scope, hosting_club: Club) -> Self {
        let start_date = create_rw_signal(cx, DateTime::now());
        let end_date = create_rw_signal(cx, DateTime::now().offset_by_hour_min(2, 30));
        let event_name = create_rw_signal(cx, "First Event".to_owned());
        let options = EventOptions::new(cx);
        let post_body = create_rw_signal(cx, "(post details)".to_owned());
        let location = create_rw_signal(cx, Location::new(cx));
        let cohosting_clubs = create_rw_signal(cx, Vec::new());
        let post_image = create_rw_signal(cx, None);

        Self {
            start_date,
            end_date,
            event_name,
            options,
            post_body,
            location,
            hosting_club,
            cohosting_clubs,
            post_image,
        }
    }
}

#[component]
fn PostPreview(cx: Scope, preview: EventPreview) -> impl IntoView {
    view!{
        cx,
        <div></div>
    }
}

#[component]
fn CalanderPreview(cx: Scope, preview: EventPreview) -> impl IntoView {
    view!{
        cx,
        <div></div>
    }
}

#[component]
fn Home(cx: Scope, current_time: ReadSignal<DateTime>, events: RwSignal<Vec<ClubEvent>>) -> impl IntoView {
    view! { cx,
        <div class="post_view_wr">
        <For
            each=move || events.get()
            key=|event| event.post_id
            view=move |cx, event: ClubEvent| {
                view!{
                    cx,
                    <PostView event/>
                }

            }
        />
        </div>
    }
}


#[component]
fn PostView(cx: Scope, event: ClubEvent) -> impl IntoView {

    view!{
        cx,
        <div class="full_post">
            <div class="full_post_header">
                <div class="full_post_club_desg">
                    <div class="full_post_club_name"> {event.hosting_club.name.get()} </div>
                    <div class="full_post_club_acr"> {format!("({})", event.hosting_club.club_acronym.get())} </div>
                </div>
                <p class="reset"/>
                <div class="full_post_event_name"> {event.event_name.get()} </div>
                <p class="reset"/>
                <p class="reset"/>
            </div>
            <div class="full_post_body_wr" style:color=move || format!("color-mix(in srgb, {} 50%, rgba(0, 0, 0, .75))", event.colour.get())>
                <p class="full_post_body">
                    {event.post_body.get()}
                </p>
            </div>

            <div class="full_post_extra_info"> 
                {event.start_date.get().format_date_range(&event.end_date.get())}
                " @ "
                {event.location.get().address}
            </div>

            <div class="full_post_notifs" style:color=move || format!("color-mix(in srgb, {} 50%, rgba(0, 0, 0, .75))", event.colour.get())>
                <div class="full_post_food_notif"> "*Lunch & Dinner Provided" </div>
                <div class="full_post_limited_spots_notif"> "*Limited Spots Available!" </div>
            </div>
        </div>
    }
}



/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Not Found"</h1>
    }
}
