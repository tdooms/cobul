#[derive(Clone, Copy, derive_more::Display)]
pub enum Solid {
    #[display(fmt = "fa-solid fa-a")]
    A,
    #[display(fmt = "fa-solid fa-address-book")]
    AddressBook,
    #[display(fmt = "fa-solid fa-address-card")]
    AddressCard,
    #[display(fmt = "fa-solid fa-align-center")]
    AlignCenter,
    #[display(fmt = "fa-solid fa-align-justify")]
    AlignJustify,
    #[display(fmt = "fa-solid fa-align-left")]
    AlignLeft,
    #[display(fmt = "fa-solid fa-align-right")]
    AlignRight,
    #[display(fmt = "fa-solid fa-anchor")]
    Anchor,
    #[display(fmt = "fa-solid fa-anchor-circle-check")]
    AnchorCircleCheck,
    #[display(fmt = "fa-solid fa-anchor-circle-exclamation")]
    AnchorCircleExclamation,
    #[display(fmt = "fa-solid fa-anchor-circle-xmark")]
    AnchorCircleXmark,
    #[display(fmt = "fa-solid fa-anchor-lock")]
    AnchorLock,
    #[display(fmt = "fa-solid fa-angle-down")]
    AngleDown,
    #[display(fmt = "fa-solid fa-angle-left")]
    AngleLeft,
    #[display(fmt = "fa-solid fa-angle-right")]
    AngleRight,
    #[display(fmt = "fa-solid fa-angle-up")]
    AngleUp,
    #[display(fmt = "fa-solid fa-angles-down")]
    AnglesDown,
    #[display(fmt = "fa-solid fa-angles-left")]
    AnglesLeft,
    #[display(fmt = "fa-solid fa-angles-right")]
    AnglesRight,
    #[display(fmt = "fa-solid fa-angles-up")]
    AnglesUp,
    #[display(fmt = "fa-solid fa-ankh")]
    Ankh,
    #[display(fmt = "fa-solid fa-apple-whole")]
    AppleWhole,
    #[display(fmt = "fa-solid fa-archway")]
    Archway,
    #[display(fmt = "fa-solid fa-arrow-down")]
    ArrowDown,
    #[display(fmt = "fa-solid fa-arrow-down-1-9")]
    ArrowDown19,
    #[display(fmt = "fa-solid fa-arrow-down-9-1")]
    ArrowDown91,
    #[display(fmt = "fa-solid fa-arrow-down-a-z")]
    ArrowDownAZ,
    #[display(fmt = "fa-solid fa-arrow-down-long")]
    ArrowDownLong,
    #[display(fmt = "fa-solid fa-arrow-down-short-wide")]
    ArrowDownShortWide,
    #[display(fmt = "fa-solid fa-arrow-down-up-across-line")]
    ArrowDownUpAcrossLine,
    #[display(fmt = "fa-solid fa-arrow-down-up-lock")]
    ArrowDownUpLock,
    #[display(fmt = "fa-solid fa-arrow-down-wide-short")]
    ArrowDownWideShort,
    #[display(fmt = "fa-solid fa-arrow-down-z-a")]
    ArrowDownZA,
    #[display(fmt = "fa-solid fa-arrow-left")]
    ArrowLeft,
    #[display(fmt = "fa-solid fa-arrow-left-long")]
    ArrowLeftLong,
    #[display(fmt = "fa-solid fa-arrow-pointer")]
    ArrowPointer,
    #[display(fmt = "fa-solid fa-arrow-right")]
    ArrowRight,
    #[display(fmt = "fa-solid fa-arrow-right-arrow-left")]
    ArrowRightArrowLeft,
    #[display(fmt = "fa-solid fa-arrow-right-from-bracket")]
    ArrowRightFromBracket,
    #[display(fmt = "fa-solid fa-arrow-right-long")]
    ArrowRightLong,
    #[display(fmt = "fa-solid fa-arrow-right-to-bracket")]
    ArrowRightToBracket,
    #[display(fmt = "fa-solid fa-arrow-right-to-city")]
    ArrowRightToCity,
    #[display(fmt = "fa-solid fa-arrow-rotate-left")]
    ArrowRotateLeft,
    #[display(fmt = "fa-solid fa-arrow-rotate-right")]
    ArrowRotateRight,
    #[display(fmt = "fa-solid fa-arrow-trend-down")]
    ArrowTrendDown,
    #[display(fmt = "fa-solid fa-arrow-trend-up")]
    ArrowTrendUp,
    #[display(fmt = "fa-solid fa-arrow-turn-down")]
    ArrowTurnDown,
    #[display(fmt = "fa-solid fa-arrow-turn-up")]
    ArrowTurnUp,
    #[display(fmt = "fa-solid fa-arrow-up")]
    ArrowUp,
    #[display(fmt = "fa-solid fa-arrow-up-1-9")]
    ArrowUp19,
    #[display(fmt = "fa-solid fa-arrow-up-9-1")]
    ArrowUp91,
    #[display(fmt = "fa-solid fa-arrow-up-a-z")]
    ArrowUpAZ,
    #[display(fmt = "fa-solid fa-arrow-up-from-bracket")]
    ArrowUpFromBracket,
    #[display(fmt = "fa-solid fa-arrow-up-from-ground-water")]
    ArrowUpFromGroundWater,
    #[display(fmt = "fa-solid fa-arrow-up-from-water-pump")]
    ArrowUpFromWaterPump,
    #[display(fmt = "fa-solid fa-arrow-up-long")]
    ArrowUpLong,
    #[display(fmt = "fa-solid fa-arrow-up-right-dots")]
    ArrowUpRightDots,
    #[display(fmt = "fa-solid fa-arrow-up-right-from-square")]
    ArrowUpRightFromSquare,
    #[display(fmt = "fa-solid fa-arrow-up-short-wide")]
    ArrowUpShortWide,
    #[display(fmt = "fa-solid fa-arrow-up-wide-short")]
    ArrowUpWideShort,
    #[display(fmt = "fa-solid fa-arrow-up-z-a")]
    ArrowUpZA,
    #[display(fmt = "fa-solid fa-arrows-down-to-line")]
    ArrowsDownToLine,
    #[display(fmt = "fa-solid fa-arrows-down-to-people")]
    ArrowsDownToPeople,
    #[display(fmt = "fa-solid fa-arrows-left-right")]
    ArrowsLeftRight,
    #[display(fmt = "fa-solid fa-arrows-left-right-to-line")]
    ArrowsLeftRightToLine,
    #[display(fmt = "fa-solid fa-arrows-rotate")]
    ArrowsRotate,
    #[display(fmt = "fa-solid fa-arrows-spin")]
    ArrowsSpin,
    #[display(fmt = "fa-solid fa-arrows-split-up-and-left")]
    ArrowsSplitUpAndLeft,
    #[display(fmt = "fa-solid fa-arrows-to-circle")]
    ArrowsToCircle,
    #[display(fmt = "fa-solid fa-arrows-to-dot")]
    ArrowsToDot,
    #[display(fmt = "fa-solid fa-arrows-to-eye")]
    ArrowsToEye,
    #[display(fmt = "fa-solid fa-arrows-turn-right")]
    ArrowsTurnRight,
    #[display(fmt = "fa-solid fa-arrows-turn-to-dots")]
    ArrowsTurnToDots,
    #[display(fmt = "fa-solid fa-arrows-up-down")]
    ArrowsUpDown,
    #[display(fmt = "fa-solid fa-arrows-up-down-left-right")]
    ArrowsUpDownLeftRight,
    #[display(fmt = "fa-solid fa-arrows-up-to-line")]
    ArrowsUpToLine,
    #[display(fmt = "fa-solid fa-asterisk")]
    Asterisk,
    #[display(fmt = "fa-solid fa-at")]
    At,
    #[display(fmt = "fa-solid fa-atom")]
    Atom,
    #[display(fmt = "fa-solid fa-audio-description")]
    AudioDescription,
    #[display(fmt = "fa-solid fa-austral-sign")]
    AustralSign,
    #[display(fmt = "fa-solid fa-award")]
    Award,
    #[display(fmt = "fa-solid fa-b")]
    B,
    #[display(fmt = "fa-solid fa-baby")]
    Baby,
    #[display(fmt = "fa-solid fa-baby-carriage")]
    BabyCarriage,
    #[display(fmt = "fa-solid fa-backward")]
    Backward,
    #[display(fmt = "fa-solid fa-backward-fast")]
    BackwardFast,
    #[display(fmt = "fa-solid fa-backward-step")]
    BackwardStep,
    #[display(fmt = "fa-solid fa-bacon")]
    Bacon,
    #[display(fmt = "fa-solid fa-bacteria")]
    Bacteria,
    #[display(fmt = "fa-solid fa-bacterium")]
    Bacterium,
    #[display(fmt = "fa-solid fa-bag-shopping")]
    BagShopping,
    #[display(fmt = "fa-solid fa-bahai")]
    Bahai,
    #[display(fmt = "fa-solid fa-baht-sign")]
    BahtSign,
    #[display(fmt = "fa-solid fa-ban")]
    Ban,
    #[display(fmt = "fa-solid fa-ban-smoking")]
    BanSmoking,
    #[display(fmt = "fa-solid fa-bandage")]
    Bandage,
    #[display(fmt = "fa-solid fa-barcode")]
    Barcode,
    #[display(fmt = "fa-solid fa-bars")]
    Bars,
    #[display(fmt = "fa-solid fa-bars-progress")]
    BarsProgress,
    #[display(fmt = "fa-solid fa-bars-staggered")]
    BarsStaggered,
    #[display(fmt = "fa-solid fa-baseball")]
    Baseball,
    #[display(fmt = "fa-solid fa-baseball-bat-ball")]
    BaseballBatBall,
    #[display(fmt = "fa-solid fa-basket-shopping")]
    BasketShopping,
    #[display(fmt = "fa-solid fa-basketball")]
    Basketball,
    #[display(fmt = "fa-solid fa-bath")]
    Bath,
    #[display(fmt = "fa-solid fa-battery-empty")]
    BatteryEmpty,
    #[display(fmt = "fa-solid fa-battery-full")]
    BatteryFull,
    #[display(fmt = "fa-solid fa-battery-half")]
    BatteryHalf,
    #[display(fmt = "fa-solid fa-battery-quarter")]
    BatteryQuarter,
    #[display(fmt = "fa-solid fa-battery-three-quarters")]
    BatteryThreeQuarters,
    #[display(fmt = "fa-solid fa-bed")]
    Bed,
    #[display(fmt = "fa-solid fa-bed-pulse")]
    BedPulse,
    #[display(fmt = "fa-solid fa-beer-mug-empty")]
    BeerMugEmpty,
    #[display(fmt = "fa-solid fa-bell")]
    Bell,
    #[display(fmt = "fa-solid fa-bell-concierge")]
    BellConcierge,
    #[display(fmt = "fa-solid fa-bell-slash")]
    BellSlash,
    #[display(fmt = "fa-solid fa-bezier-curve")]
    BezierCurve,
    #[display(fmt = "fa-solid fa-bicycle")]
    Bicycle,
    #[display(fmt = "fa-solid fa-binoculars")]
    Binoculars,
    #[display(fmt = "fa-solid fa-biohazard")]
    Biohazard,
    #[display(fmt = "fa-solid fa-bitcoin-sign")]
    BitcoinSign,
    #[display(fmt = "fa-solid fa-blender")]
    Blender,
    #[display(fmt = "fa-solid fa-blender-phone")]
    BlenderPhone,
    #[display(fmt = "fa-solid fa-blog")]
    Blog,
    #[display(fmt = "fa-solid fa-bold")]
    Bold,
    #[display(fmt = "fa-solid fa-bolt")]
    Bolt,
    #[display(fmt = "fa-solid fa-bolt-lightning")]
    BoltLightning,
    #[display(fmt = "fa-solid fa-bomb")]
    Bomb,
    #[display(fmt = "fa-solid fa-bone")]
    Bone,
    #[display(fmt = "fa-solid fa-bong")]
    Bong,
    #[display(fmt = "fa-solid fa-book")]
    Book,
    #[display(fmt = "fa-solid fa-book-atlas")]
    BookAtlas,
    #[display(fmt = "fa-solid fa-book-bible")]
    BookBible,
    #[display(fmt = "fa-solid fa-book-bookmark")]
    BookBookmark,
    #[display(fmt = "fa-solid fa-book-journal-whills")]
    BookJournalWhills,
    #[display(fmt = "fa-solid fa-book-medical")]
    BookMedical,
    #[display(fmt = "fa-solid fa-book-open")]
    BookOpen,
    #[display(fmt = "fa-solid fa-book-open-reader")]
    BookOpenReader,
    #[display(fmt = "fa-solid fa-book-quran")]
    BookQuran,
    #[display(fmt = "fa-solid fa-book-skull")]
    BookSkull,
    #[display(fmt = "fa-solid fa-bookmark")]
    Bookmark,
    #[display(fmt = "fa-solid fa-border-all")]
    BorderAll,
    #[display(fmt = "fa-solid fa-border-none")]
    BorderNone,
    #[display(fmt = "fa-solid fa-border-top-left")]
    BorderTopLeft,
    #[display(fmt = "fa-solid fa-bore-hole")]
    BoreHole,
    #[display(fmt = "fa-solid fa-bottle-droplet")]
    BottleDroplet,
    #[display(fmt = "fa-solid fa-bottle-water")]
    BottleWater,
    #[display(fmt = "fa-solid fa-bowl-food")]
    BowlFood,
    #[display(fmt = "fa-solid fa-bowl-rice")]
    BowlRice,
    #[display(fmt = "fa-solid fa-bowling-ball")]
    BowlingBall,
    #[display(fmt = "fa-solid fa-box")]
    Box,
    #[display(fmt = "fa-solid fa-box-archive")]
    BoxArchive,
    #[display(fmt = "fa-solid fa-box-open")]
    BoxOpen,
    #[display(fmt = "fa-solid fa-box-tissue")]
    BoxTissue,
    #[display(fmt = "fa-solid fa-boxes-packing")]
    BoxesPacking,
    #[display(fmt = "fa-solid fa-boxes-stacked")]
    BoxesStacked,
    #[display(fmt = "fa-solid fa-braille")]
    Braille,
    #[display(fmt = "fa-solid fa-brain")]
    Brain,
    #[display(fmt = "fa-solid fa-brazilian-real-sign")]
    BrazilianRealSign,
    #[display(fmt = "fa-solid fa-bread-slice")]
    BreadSlice,
    #[display(fmt = "fa-solid fa-bridge")]
    Bridge,
    #[display(fmt = "fa-solid fa-bridge-circle-check")]
    BridgeCircleCheck,
    #[display(fmt = "fa-solid fa-bridge-circle-exclamation")]
    BridgeCircleExclamation,
    #[display(fmt = "fa-solid fa-bridge-circle-xmark")]
    BridgeCircleXmark,
    #[display(fmt = "fa-solid fa-bridge-lock")]
    BridgeLock,
    #[display(fmt = "fa-solid fa-bridge-water")]
    BridgeWater,
    #[display(fmt = "fa-solid fa-briefcase")]
    Briefcase,
    #[display(fmt = "fa-solid fa-briefcase-medical")]
    BriefcaseMedical,
    #[display(fmt = "fa-solid fa-broom")]
    Broom,
    #[display(fmt = "fa-solid fa-broom-ball")]
    BroomBall,
    #[display(fmt = "fa-solid fa-brush")]
    Brush,
    #[display(fmt = "fa-solid fa-bucket")]
    Bucket,
    #[display(fmt = "fa-solid fa-bug")]
    Bug,
    #[display(fmt = "fa-solid fa-bug-slash")]
    BugSlash,
    #[display(fmt = "fa-solid fa-bugs")]
    Bugs,
    #[display(fmt = "fa-solid fa-building")]
    Building,
    #[display(fmt = "fa-solid fa-building-circle-arrow-right")]
    BuildingCircleArrowRight,
    #[display(fmt = "fa-solid fa-building-circle-check")]
    BuildingCircleCheck,
    #[display(fmt = "fa-solid fa-building-circle-exclamation")]
    BuildingCircleExclamation,
    #[display(fmt = "fa-solid fa-building-circle-xmark")]
    BuildingCircleXmark,
    #[display(fmt = "fa-solid fa-building-columns")]
    BuildingColumns,
    #[display(fmt = "fa-solid fa-building-flag")]
    BuildingFlag,
    #[display(fmt = "fa-solid fa-building-lock")]
    BuildingLock,
    #[display(fmt = "fa-solid fa-building-ngo")]
    BuildingNgo,
    #[display(fmt = "fa-solid fa-building-shield")]
    BuildingShield,
    #[display(fmt = "fa-solid fa-building-un")]
    BuildingUn,
    #[display(fmt = "fa-solid fa-building-user")]
    BuildingUser,
    #[display(fmt = "fa-solid fa-building-wheat")]
    BuildingWheat,
    #[display(fmt = "fa-solid fa-bullhorn")]
    Bullhorn,
    #[display(fmt = "fa-solid fa-bullseye")]
    Bullseye,
    #[display(fmt = "fa-solid fa-burger")]
    Burger,
    #[display(fmt = "fa-solid fa-burst")]
    Burst,
    #[display(fmt = "fa-solid fa-bus")]
    Bus,
    #[display(fmt = "fa-solid fa-bus-simple")]
    BusSimple,
    #[display(fmt = "fa-solid fa-business-time")]
    BusinessTime,
    #[display(fmt = "fa-solid fa-c")]
    C,
    #[display(fmt = "fa-solid fa-cake-candles")]
    CakeCandles,
    #[display(fmt = "fa-solid fa-calculator")]
    Calculator,
    #[display(fmt = "fa-solid fa-calendar")]
    Calendar,
    #[display(fmt = "fa-solid fa-calendar-check")]
    CalendarCheck,
    #[display(fmt = "fa-solid fa-calendar-day")]
    CalendarDay,
    #[display(fmt = "fa-solid fa-calendar-days")]
    CalendarDays,
    #[display(fmt = "fa-solid fa-calendar-minus")]
    CalendarMinus,
    #[display(fmt = "fa-solid fa-calendar-plus")]
    CalendarPlus,
    #[display(fmt = "fa-solid fa-calendar-week")]
    CalendarWeek,
    #[display(fmt = "fa-solid fa-calendar-xmark")]
    CalendarXmark,
    #[display(fmt = "fa-solid fa-camera")]
    Camera,
    #[display(fmt = "fa-solid fa-camera-retro")]
    CameraRetro,
    #[display(fmt = "fa-solid fa-camera-rotate")]
    CameraRotate,
    #[display(fmt = "fa-solid fa-campground")]
    Campground,
    #[display(fmt = "fa-solid fa-candy-cane")]
    CandyCane,
    #[display(fmt = "fa-solid fa-cannabis")]
    Cannabis,
    #[display(fmt = "fa-solid fa-capsules")]
    Capsules,
    #[display(fmt = "fa-solid fa-car")]
    Car,
    #[display(fmt = "fa-solid fa-car-battery")]
    CarBattery,
    #[display(fmt = "fa-solid fa-car-burst")]
    CarBurst,
    #[display(fmt = "fa-solid fa-car-on")]
    CarOn,
    #[display(fmt = "fa-solid fa-car-rear")]
    CarRear,
    #[display(fmt = "fa-solid fa-car-side")]
    CarSide,
    #[display(fmt = "fa-solid fa-car-tunnel")]
    CarTunnel,
    #[display(fmt = "fa-solid fa-caravan")]
    Caravan,
    #[display(fmt = "fa-solid fa-caret-down")]
    CaretDown,
    #[display(fmt = "fa-solid fa-caret-left")]
    CaretLeft,
    #[display(fmt = "fa-solid fa-caret-right")]
    CaretRight,
    #[display(fmt = "fa-solid fa-caret-up")]
    CaretUp,
    #[display(fmt = "fa-solid fa-carrot")]
    Carrot,
    #[display(fmt = "fa-solid fa-cart-arrow-down")]
    CartArrowDown,
    #[display(fmt = "fa-solid fa-cart-flatbed")]
    CartFlatbed,
    #[display(fmt = "fa-solid fa-cart-flatbed-suitcase")]
    CartFlatbedSuitcase,
    #[display(fmt = "fa-solid fa-cart-plus")]
    CartPlus,
    #[display(fmt = "fa-solid fa-cart-shopping")]
    CartShopping,
    #[display(fmt = "fa-solid fa-cash-register")]
    CashRegister,
    #[display(fmt = "fa-solid fa-cat")]
    Cat,
    #[display(fmt = "fa-solid fa-cedi-sign")]
    CediSign,
    #[display(fmt = "fa-solid fa-cent-sign")]
    CentSign,
    #[display(fmt = "fa-solid fa-certificate")]
    Certificate,
    #[display(fmt = "fa-solid fa-chair")]
    Chair,
    #[display(fmt = "fa-solid fa-chalkboard")]
    Chalkboard,
    #[display(fmt = "fa-solid fa-chalkboard-user")]
    ChalkboardUser,
    #[display(fmt = "fa-solid fa-champagne-glasses")]
    ChampagneGlasses,
    #[display(fmt = "fa-solid fa-charging-station")]
    ChargingStation,
    #[display(fmt = "fa-solid fa-chart-area")]
    ChartArea,
    #[display(fmt = "fa-solid fa-chart-bar")]
    ChartBar,
    #[display(fmt = "fa-solid fa-chart-column")]
    ChartColumn,
    #[display(fmt = "fa-solid fa-chart-gantt")]
    ChartGantt,
    #[display(fmt = "fa-solid fa-chart-line")]
    ChartLine,
    #[display(fmt = "fa-solid fa-chart-pie")]
    ChartPie,
    #[display(fmt = "fa-solid fa-chart-simple")]
    ChartSimple,
    #[display(fmt = "fa-solid fa-check")]
    Check,
    #[display(fmt = "fa-solid fa-check-double")]
    CheckDouble,
    #[display(fmt = "fa-solid fa-check-to-slot")]
    CheckToSlot,
    #[display(fmt = "fa-solid fa-cheese")]
    Cheese,
    #[display(fmt = "fa-solid fa-chess")]
    Chess,
    #[display(fmt = "fa-solid fa-chess-bishop")]
    ChessBishop,
    #[display(fmt = "fa-solid fa-chess-board")]
    ChessBoard,
    #[display(fmt = "fa-solid fa-chess-king")]
    ChessKing,
    #[display(fmt = "fa-solid fa-chess-knight")]
    ChessKnight,
    #[display(fmt = "fa-solid fa-chess-pawn")]
    ChessPawn,
    #[display(fmt = "fa-solid fa-chess-queen")]
    ChessQueen,
    #[display(fmt = "fa-solid fa-chess-rook")]
    ChessRook,
    #[display(fmt = "fa-solid fa-chevron-down")]
    ChevronDown,
    #[display(fmt = "fa-solid fa-chevron-left")]
    ChevronLeft,
    #[display(fmt = "fa-solid fa-chevron-right")]
    ChevronRight,
    #[display(fmt = "fa-solid fa-chevron-up")]
    ChevronUp,
    #[display(fmt = "fa-solid fa-child")]
    Child,
    #[display(fmt = "fa-solid fa-child-dress")]
    ChildDress,
    #[display(fmt = "fa-solid fa-child-reaching")]
    ChildReaching,
    #[display(fmt = "fa-solid fa-child-rifle")]
    ChildRifle,
    #[display(fmt = "fa-solid fa-children")]
    Children,
    #[display(fmt = "fa-solid fa-church")]
    Church,
    #[display(fmt = "fa-solid fa-circle")]
    Circle,
    #[display(fmt = "fa-solid fa-circle-arrow-down")]
    CircleArrowDown,
    #[display(fmt = "fa-solid fa-circle-arrow-left")]
    CircleArrowLeft,
    #[display(fmt = "fa-solid fa-circle-arrow-right")]
    CircleArrowRight,
    #[display(fmt = "fa-solid fa-circle-arrow-up")]
    CircleArrowUp,
    #[display(fmt = "fa-solid fa-circle-check")]
    CircleCheck,
    #[display(fmt = "fa-solid fa-circle-chevron-down")]
    CircleChevronDown,
    #[display(fmt = "fa-solid fa-circle-chevron-left")]
    CircleChevronLeft,
    #[display(fmt = "fa-solid fa-circle-chevron-right")]
    CircleChevronRight,
    #[display(fmt = "fa-solid fa-circle-chevron-up")]
    CircleChevronUp,
    #[display(fmt = "fa-solid fa-circle-dollar-to-slot")]
    CircleDollarToSlot,
    #[display(fmt = "fa-solid fa-circle-dot")]
    CircleDot,
    #[display(fmt = "fa-solid fa-circle-down")]
    CircleDown,
    #[display(fmt = "fa-solid fa-circle-exclamation")]
    CircleExclamation,
    #[display(fmt = "fa-solid fa-circle-h")]
    CircleH,
    #[display(fmt = "fa-solid fa-circle-half-stroke")]
    CircleHalfStroke,
    #[display(fmt = "fa-solid fa-circle-info")]
    CircleInfo,
    #[display(fmt = "fa-solid fa-circle-left")]
    CircleLeft,
    #[display(fmt = "fa-solid fa-circle-minus")]
    CircleMinus,
    #[display(fmt = "fa-solid fa-circle-nodes")]
    CircleNodes,
    #[display(fmt = "fa-solid fa-circle-notch")]
    CircleNotch,
    #[display(fmt = "fa-solid fa-circle-pause")]
    CirclePause,
    #[display(fmt = "fa-solid fa-circle-play")]
    CirclePlay,
    #[display(fmt = "fa-solid fa-circle-plus")]
    CirclePlus,
    #[display(fmt = "fa-solid fa-circle-question")]
    CircleQuestion,
    #[display(fmt = "fa-solid fa-circle-radiation")]
    CircleRadiation,
    #[display(fmt = "fa-solid fa-circle-right")]
    CircleRight,
    #[display(fmt = "fa-solid fa-circle-stop")]
    CircleStop,
    #[display(fmt = "fa-solid fa-circle-up")]
    CircleUp,
    #[display(fmt = "fa-solid fa-circle-user")]
    CircleUser,
    #[display(fmt = "fa-solid fa-circle-xmark")]
    CircleXmark,
    #[display(fmt = "fa-solid fa-city")]
    City,
    #[display(fmt = "fa-solid fa-clapperboard")]
    Clapperboard,
    #[display(fmt = "fa-solid fa-clipboard")]
    Clipboard,
    #[display(fmt = "fa-solid fa-clipboard-check")]
    ClipboardCheck,
    #[display(fmt = "fa-solid fa-clipboard-list")]
    ClipboardList,
    #[display(fmt = "fa-solid fa-clipboard-question")]
    ClipboardQuestion,
    #[display(fmt = "fa-solid fa-clipboard-user")]
    ClipboardUser,
    #[display(fmt = "fa-solid fa-clock")]
    Clock,
    #[display(fmt = "fa-solid fa-clock-rotate-left")]
    ClockRotateLeft,
    #[display(fmt = "fa-solid fa-clone")]
    Clone,
    #[display(fmt = "fa-solid fa-closed-captioning")]
    ClosedCaptioning,
    #[display(fmt = "fa-solid fa-cloud")]
    Cloud,
    #[display(fmt = "fa-solid fa-cloud-arrow-down")]
    CloudArrowDown,
    #[display(fmt = "fa-solid fa-cloud-arrow-up")]
    CloudArrowUp,
    #[display(fmt = "fa-solid fa-cloud-bolt")]
    CloudBolt,
    #[display(fmt = "fa-solid fa-cloud-meatball")]
    CloudMeatball,
    #[display(fmt = "fa-solid fa-cloud-moon")]
    CloudMoon,
    #[display(fmt = "fa-solid fa-cloud-moon-rain")]
    CloudMoonRain,
    #[display(fmt = "fa-solid fa-cloud-rain")]
    CloudRain,
    #[display(fmt = "fa-solid fa-cloud-showers-heavy")]
    CloudShowersHeavy,
    #[display(fmt = "fa-solid fa-cloud-showers-water")]
    CloudShowersWater,
    #[display(fmt = "fa-solid fa-cloud-sun")]
    CloudSun,
    #[display(fmt = "fa-solid fa-cloud-sun-rain")]
    CloudSunRain,
    #[display(fmt = "fa-solid fa-clover")]
    Clover,
    #[display(fmt = "fa-solid fa-code")]
    Code,
    #[display(fmt = "fa-solid fa-code-branch")]
    CodeBranch,
    #[display(fmt = "fa-solid fa-code-commit")]
    CodeCommit,
    #[display(fmt = "fa-solid fa-code-compare")]
    CodeCompare,
    #[display(fmt = "fa-solid fa-code-fork")]
    CodeFork,
    #[display(fmt = "fa-solid fa-code-merge")]
    CodeMerge,
    #[display(fmt = "fa-solid fa-code-pull-request")]
    CodePullRequest,
    #[display(fmt = "fa-solid fa-coins")]
    Coins,
    #[display(fmt = "fa-solid fa-colon-sign")]
    ColonSign,
    #[display(fmt = "fa-solid fa-comment")]
    Comment,
    #[display(fmt = "fa-solid fa-comment-dollar")]
    CommentDollar,
    #[display(fmt = "fa-solid fa-comment-dots")]
    CommentDots,
    #[display(fmt = "fa-solid fa-comment-medical")]
    CommentMedical,
    #[display(fmt = "fa-solid fa-comment-slash")]
    CommentSlash,
    #[display(fmt = "fa-solid fa-comment-sms")]
    CommentSms,
    #[display(fmt = "fa-solid fa-comments")]
    Comments,
    #[display(fmt = "fa-solid fa-comments-dollar")]
    CommentsDollar,
    #[display(fmt = "fa-solid fa-compact-disc")]
    CompactDisc,
    #[display(fmt = "fa-solid fa-compass")]
    Compass,
    #[display(fmt = "fa-solid fa-compass-drafting")]
    CompassDrafting,
    #[display(fmt = "fa-solid fa-compress")]
    Compress,
    #[display(fmt = "fa-solid fa-computer")]
    Computer,
    #[display(fmt = "fa-solid fa-computer-mouse")]
    ComputerMouse,
    #[display(fmt = "fa-solid fa-cookie")]
    Cookie,
    #[display(fmt = "fa-solid fa-cookie-bite")]
    CookieBite,
    #[display(fmt = "fa-solid fa-copy")]
    Copy,
    #[display(fmt = "fa-solid fa-copyright")]
    Copyright,
    #[display(fmt = "fa-solid fa-couch")]
    Couch,
    #[display(fmt = "fa-solid fa-cow")]
    Cow,
    #[display(fmt = "fa-solid fa-credit-card")]
    CreditCard,
    #[display(fmt = "fa-solid fa-crop")]
    Crop,
    #[display(fmt = "fa-solid fa-crop-simple")]
    CropSimple,
    #[display(fmt = "fa-solid fa-cross")]
    Cross,
    #[display(fmt = "fa-solid fa-crosshairs")]
    Crosshairs,
    #[display(fmt = "fa-solid fa-crow")]
    Crow,
    #[display(fmt = "fa-solid fa-crown")]
    Crown,
    #[display(fmt = "fa-solid fa-crutch")]
    Crutch,
    #[display(fmt = "fa-solid fa-cruzeiro-sign")]
    CruzeiroSign,
    #[display(fmt = "fa-solid fa-cube")]
    Cube,
    #[display(fmt = "fa-solid fa-cubes")]
    Cubes,
    #[display(fmt = "fa-solid fa-cubes-stacked")]
    CubesStacked,
    #[display(fmt = "fa-solid fa-d")]
    D,
    #[display(fmt = "fa-solid fa-database")]
    Database,
    #[display(fmt = "fa-solid fa-delete-left")]
    DeleteLeft,
    #[display(fmt = "fa-solid fa-democrat")]
    Democrat,
    #[display(fmt = "fa-solid fa-desktop")]
    Desktop,
    #[display(fmt = "fa-solid fa-dharmachakra")]
    Dharmachakra,
    #[display(fmt = "fa-solid fa-diagram-next")]
    DiagramNext,
    #[display(fmt = "fa-solid fa-diagram-predecessor")]
    DiagramPredecessor,
    #[display(fmt = "fa-solid fa-diagram-project")]
    DiagramProject,
    #[display(fmt = "fa-solid fa-diagram-successor")]
    DiagramSuccessor,
    #[display(fmt = "fa-solid fa-diamond")]
    Diamond,
    #[display(fmt = "fa-solid fa-diamond-turn-right")]
    DiamondTurnRight,
    #[display(fmt = "fa-solid fa-dice")]
    Dice,
    #[display(fmt = "fa-solid fa-dice-d20")]
    DiceD20,
    #[display(fmt = "fa-solid fa-dice-d6")]
    DiceD6,
    #[display(fmt = "fa-solid fa-dice-five")]
    DiceFive,
    #[display(fmt = "fa-solid fa-dice-four")]
    DiceFour,
    #[display(fmt = "fa-solid fa-dice-one")]
    DiceOne,
    #[display(fmt = "fa-solid fa-dice-six")]
    DiceSix,
    #[display(fmt = "fa-solid fa-dice-three")]
    DiceThree,
    #[display(fmt = "fa-solid fa-dice-two")]
    DiceTwo,
    #[display(fmt = "fa-solid fa-disease")]
    Disease,
    #[display(fmt = "fa-solid fa-display")]
    Display,
    #[display(fmt = "fa-solid fa-divide")]
    Divide,
    #[display(fmt = "fa-solid fa-dna")]
    Dna,
    #[display(fmt = "fa-solid fa-dog")]
    Dog,
    #[display(fmt = "fa-solid fa-dollar-sign")]
    DollarSign,
    #[display(fmt = "fa-solid fa-dolly")]
    Dolly,
    #[display(fmt = "fa-solid fa-dong-sign")]
    DongSign,
    #[display(fmt = "fa-solid fa-door-closed")]
    DoorClosed,
    #[display(fmt = "fa-solid fa-door-open")]
    DoorOpen,
    #[display(fmt = "fa-solid fa-dove")]
    Dove,
    #[display(fmt = "fa-solid fa-down-left-and-up-right-to-center")]
    DownLeftAndUpRightToCenter,
    #[display(fmt = "fa-solid fa-down-long")]
    DownLong,
    #[display(fmt = "fa-solid fa-download")]
    Download,
    #[display(fmt = "fa-solid fa-dragon")]
    Dragon,
    #[display(fmt = "fa-solid fa-draw-polygon")]
    DrawPolygon,
    #[display(fmt = "fa-solid fa-droplet")]
    Droplet,
    #[display(fmt = "fa-solid fa-droplet-slash")]
    DropletSlash,
    #[display(fmt = "fa-solid fa-drum")]
    Drum,
    #[display(fmt = "fa-solid fa-drum-steelpan")]
    DrumSteelpan,
    #[display(fmt = "fa-solid fa-drumstick-bite")]
    DrumstickBite,
    #[display(fmt = "fa-solid fa-dumbbell")]
    Dumbbell,
    #[display(fmt = "fa-solid fa-dumpster")]
    Dumpster,
    #[display(fmt = "fa-solid fa-dumpster-fire")]
    DumpsterFire,
    #[display(fmt = "fa-solid fa-dungeon")]
    Dungeon,
    #[display(fmt = "fa-solid fa-e")]
    E,
    #[display(fmt = "fa-solid fa-ear-deaf")]
    EarDeaf,
    #[display(fmt = "fa-solid fa-ear-listen")]
    EarListen,
    #[display(fmt = "fa-solid fa-earth-africa")]
    EarthAfrica,
    #[display(fmt = "fa-solid fa-earth-americas")]
    EarthAmericas,
    #[display(fmt = "fa-solid fa-earth-asia")]
    EarthAsia,
    #[display(fmt = "fa-solid fa-earth-europe")]
    EarthEurope,
    #[display(fmt = "fa-solid fa-earth-oceania")]
    EarthOceania,
    #[display(fmt = "fa-solid fa-egg")]
    Egg,
    #[display(fmt = "fa-solid fa-eject")]
    Eject,
    #[display(fmt = "fa-solid fa-elevator")]
    Elevator,
    #[display(fmt = "fa-solid fa-ellipsis")]
    Ellipsis,
    #[display(fmt = "fa-solid fa-ellipsis-vertical")]
    EllipsisVertical,
    #[display(fmt = "fa-solid fa-envelope")]
    Envelope,
    #[display(fmt = "fa-solid fa-envelope-circle-check")]
    EnvelopeCircleCheck,
    #[display(fmt = "fa-solid fa-envelope-open")]
    EnvelopeOpen,
    #[display(fmt = "fa-solid fa-envelope-open-text")]
    EnvelopeOpenText,
    #[display(fmt = "fa-solid fa-envelopes-bulk")]
    EnvelopesBulk,
    #[display(fmt = "fa-solid fa-equals")]
    Equals,
    #[display(fmt = "fa-solid fa-eraser")]
    Eraser,
    #[display(fmt = "fa-solid fa-ethernet")]
    Ethernet,
    #[display(fmt = "fa-solid fa-euro-sign")]
    EuroSign,
    #[display(fmt = "fa-solid fa-exclamation")]
    Exclamation,
    #[display(fmt = "fa-solid fa-expand")]
    Expand,
    #[display(fmt = "fa-solid fa-explosion")]
    Explosion,
    #[display(fmt = "fa-solid fa-eye")]
    Eye,
    #[display(fmt = "fa-solid fa-eye-dropper")]
    EyeDropper,
    #[display(fmt = "fa-solid fa-eye-low-vision")]
    EyeLowVision,
    #[display(fmt = "fa-solid fa-eye-slash")]
    EyeSlash,
    #[display(fmt = "fa-solid fa-f")]
    F,
    #[display(fmt = "fa-solid fa-face-angry")]
    FaceAngry,
    #[display(fmt = "fa-solid fa-face-dizzy")]
    FaceDizzy,
    #[display(fmt = "fa-solid fa-face-flushed")]
    FaceFlushed,
    #[display(fmt = "fa-solid fa-face-frown")]
    FaceFrown,
    #[display(fmt = "fa-solid fa-face-frown-open")]
    FaceFrownOpen,
    #[display(fmt = "fa-solid fa-face-grimace")]
    FaceGrimace,
    #[display(fmt = "fa-solid fa-face-grin")]
    FaceGrin,
    #[display(fmt = "fa-solid fa-face-grin-beam")]
    FaceGrinBeam,
    #[display(fmt = "fa-solid fa-face-grin-beam-sweat")]
    FaceGrinBeamSweat,
    #[display(fmt = "fa-solid fa-face-grin-hearts")]
    FaceGrinHearts,
    #[display(fmt = "fa-solid fa-face-grin-squint")]
    FaceGrinSquint,
    #[display(fmt = "fa-solid fa-face-grin-squint-tears")]
    FaceGrinSquintTears,
    #[display(fmt = "fa-solid fa-face-grin-stars")]
    FaceGrinStars,
    #[display(fmt = "fa-solid fa-face-grin-tears")]
    FaceGrinTears,
    #[display(fmt = "fa-solid fa-face-grin-tongue")]
    FaceGrinTongue,
    #[display(fmt = "fa-solid fa-face-grin-tongue-squint")]
    FaceGrinTongueSquint,
    #[display(fmt = "fa-solid fa-face-grin-tongue-wink")]
    FaceGrinTongueWink,
    #[display(fmt = "fa-solid fa-face-grin-wide")]
    FaceGrinWide,
    #[display(fmt = "fa-solid fa-face-grin-wink")]
    FaceGrinWink,
    #[display(fmt = "fa-solid fa-face-kiss")]
    FaceKiss,
    #[display(fmt = "fa-solid fa-face-kiss-beam")]
    FaceKissBeam,
    #[display(fmt = "fa-solid fa-face-kiss-wink-heart")]
    FaceKissWinkHeart,
    #[display(fmt = "fa-solid fa-face-laugh")]
    FaceLaugh,
    #[display(fmt = "fa-solid fa-face-laugh-beam")]
    FaceLaughBeam,
    #[display(fmt = "fa-solid fa-face-laugh-squint")]
    FaceLaughSquint,
    #[display(fmt = "fa-solid fa-face-laugh-wink")]
    FaceLaughWink,
    #[display(fmt = "fa-solid fa-face-meh")]
    FaceMeh,
    #[display(fmt = "fa-solid fa-face-meh-blank")]
    FaceMehBlank,
    #[display(fmt = "fa-solid fa-face-rolling-eyes")]
    FaceRollingEyes,
    #[display(fmt = "fa-solid fa-face-sad-cry")]
    FaceSadCry,
    #[display(fmt = "fa-solid fa-face-sad-tear")]
    FaceSadTear,
    #[display(fmt = "fa-solid fa-face-smile")]
    FaceSmile,
    #[display(fmt = "fa-solid fa-face-smile-beam")]
    FaceSmileBeam,
    #[display(fmt = "fa-solid fa-face-smile-wink")]
    FaceSmileWink,
    #[display(fmt = "fa-solid fa-face-surprise")]
    FaceSurprise,
    #[display(fmt = "fa-solid fa-face-tired")]
    FaceTired,
    #[display(fmt = "fa-solid fa-fan")]
    Fan,
    #[display(fmt = "fa-solid fa-faucet")]
    Faucet,
    #[display(fmt = "fa-solid fa-faucet-drip")]
    FaucetDrip,
    #[display(fmt = "fa-solid fa-fax")]
    Fax,
    #[display(fmt = "fa-solid fa-feather")]
    Feather,
    #[display(fmt = "fa-solid fa-feather-pointed")]
    FeatherPointed,
    #[display(fmt = "fa-solid fa-ferry")]
    Ferry,
    #[display(fmt = "fa-solid fa-file")]
    File,
    #[display(fmt = "fa-solid fa-file-arrow-down")]
    FileArrowDown,
    #[display(fmt = "fa-solid fa-file-arrow-up")]
    FileArrowUp,
    #[display(fmt = "fa-solid fa-file-audio")]
    FileAudio,
    #[display(fmt = "fa-solid fa-file-circle-check")]
    FileCircleCheck,
    #[display(fmt = "fa-solid fa-file-circle-exclamation")]
    FileCircleExclamation,
    #[display(fmt = "fa-solid fa-file-circle-minus")]
    FileCircleMinus,
    #[display(fmt = "fa-solid fa-file-circle-plus")]
    FileCirclePlus,
    #[display(fmt = "fa-solid fa-file-circle-question")]
    FileCircleQuestion,
    #[display(fmt = "fa-solid fa-file-circle-xmark")]
    FileCircleXmark,
    #[display(fmt = "fa-solid fa-file-code")]
    FileCode,
    #[display(fmt = "fa-solid fa-file-contract")]
    FileContract,
    #[display(fmt = "fa-solid fa-file-csv")]
    FileCsv,
    #[display(fmt = "fa-solid fa-file-excel")]
    FileExcel,
    #[display(fmt = "fa-solid fa-file-export")]
    FileExport,
    #[display(fmt = "fa-solid fa-file-image")]
    FileImage,
    #[display(fmt = "fa-solid fa-file-import")]
    FileImport,
    #[display(fmt = "fa-solid fa-file-invoice")]
    FileInvoice,
    #[display(fmt = "fa-solid fa-file-invoice-dollar")]
    FileInvoiceDollar,
    #[display(fmt = "fa-solid fa-file-lines")]
    FileLines,
    #[display(fmt = "fa-solid fa-file-medical")]
    FileMedical,
    #[display(fmt = "fa-solid fa-file-pdf")]
    FilePdf,
    #[display(fmt = "fa-solid fa-file-pen")]
    FilePen,
    #[display(fmt = "fa-solid fa-file-powerpoint")]
    FilePowerpoint,
    #[display(fmt = "fa-solid fa-file-prescription")]
    FilePrescription,
    #[display(fmt = "fa-solid fa-file-shield")]
    FileShield,
    #[display(fmt = "fa-solid fa-file-signature")]
    FileSignature,
    #[display(fmt = "fa-solid fa-file-video")]
    FileVideo,
    #[display(fmt = "fa-solid fa-file-waveform")]
    FileWaveform,
    #[display(fmt = "fa-solid fa-file-word")]
    FileWord,
    #[display(fmt = "fa-solid fa-file-zipper")]
    FileZipper,
    #[display(fmt = "fa-solid fa-fill")]
    Fill,
    #[display(fmt = "fa-solid fa-fill-drip")]
    FillDrip,
    #[display(fmt = "fa-solid fa-film")]
    Film,
    #[display(fmt = "fa-solid fa-filter")]
    Filter,
    #[display(fmt = "fa-solid fa-filter-circle-dollar")]
    FilterCircleDollar,
    #[display(fmt = "fa-solid fa-filter-circle-xmark")]
    FilterCircleXmark,
    #[display(fmt = "fa-solid fa-fingerprint")]
    Fingerprint,
    #[display(fmt = "fa-solid fa-fire")]
    Fire,
    #[display(fmt = "fa-solid fa-fire-burner")]
    FireBurner,
    #[display(fmt = "fa-solid fa-fire-extinguisher")]
    FireExtinguisher,
    #[display(fmt = "fa-solid fa-fire-flame-curved")]
    FireFlameCurved,
    #[display(fmt = "fa-solid fa-fire-flame-simple")]
    FireFlameSimple,
    #[display(fmt = "fa-solid fa-fish")]
    Fish,
    #[display(fmt = "fa-solid fa-fish-fins")]
    FishFins,
    #[display(fmt = "fa-solid fa-flag")]
    Flag,
    #[display(fmt = "fa-solid fa-flag-checkered")]
    FlagCheckered,
    #[display(fmt = "fa-solid fa-flag-usa")]
    FlagUsa,
    #[display(fmt = "fa-solid fa-flask")]
    Flask,
    #[display(fmt = "fa-solid fa-flask-vial")]
    FlaskVial,
    #[display(fmt = "fa-solid fa-floppy-disk")]
    FloppyDisk,
    #[display(fmt = "fa-solid fa-florin-sign")]
    FlorinSign,
    #[display(fmt = "fa-solid fa-folder")]
    Folder,
    #[display(fmt = "fa-solid fa-folder-closed")]
    FolderClosed,
    #[display(fmt = "fa-solid fa-folder-minus")]
    FolderMinus,
    #[display(fmt = "fa-solid fa-folder-open")]
    FolderOpen,
    #[display(fmt = "fa-solid fa-folder-plus")]
    FolderPlus,
    #[display(fmt = "fa-solid fa-folder-tree")]
    FolderTree,
    #[display(fmt = "fa-solid fa-font")]
    Font,
    #[display(fmt = "fa-solid fa-font-awesome")]
    FontAwesome,
    #[display(fmt = "fa-solid fa-football")]
    Football,
    #[display(fmt = "fa-solid fa-forward")]
    Forward,
    #[display(fmt = "fa-solid fa-forward-fast")]
    ForwardFast,
    #[display(fmt = "fa-solid fa-forward-step")]
    ForwardStep,
    #[display(fmt = "fa-solid fa-franc-sign")]
    FrancSign,
    #[display(fmt = "fa-solid fa-frog")]
    Frog,
    #[display(fmt = "fa-solid fa-futbol")]
    Futbol,
    #[display(fmt = "fa-solid fa-g")]
    G,
    #[display(fmt = "fa-solid fa-gamepad")]
    Gamepad,
    #[display(fmt = "fa-solid fa-gas-pump")]
    GasPump,
    #[display(fmt = "fa-solid fa-gauge")]
    Gauge,
    #[display(fmt = "fa-solid fa-gauge-high")]
    GaugeHigh,
    #[display(fmt = "fa-solid fa-gauge-simple")]
    GaugeSimple,
    #[display(fmt = "fa-solid fa-gauge-simple-high")]
    GaugeSimpleHigh,
    #[display(fmt = "fa-solid fa-gavel")]
    Gavel,
    #[display(fmt = "fa-solid fa-gear")]
    Gear,
    #[display(fmt = "fa-solid fa-gears")]
    Gears,
    #[display(fmt = "fa-solid fa-gem")]
    Gem,
    #[display(fmt = "fa-solid fa-genderless")]
    Genderless,
    #[display(fmt = "fa-solid fa-ghost")]
    Ghost,
    #[display(fmt = "fa-solid fa-gift")]
    Gift,
    #[display(fmt = "fa-solid fa-gifts")]
    Gifts,
    #[display(fmt = "fa-solid fa-glass-water")]
    GlassWater,
    #[display(fmt = "fa-solid fa-glass-water-droplet")]
    GlassWaterDroplet,
    #[display(fmt = "fa-solid fa-glasses")]
    Glasses,
    #[display(fmt = "fa-solid fa-globe")]
    Globe,
    #[display(fmt = "fa-solid fa-golf-ball-tee")]
    GolfBallTee,
    #[display(fmt = "fa-solid fa-gopuram")]
    Gopuram,
    #[display(fmt = "fa-solid fa-graduation-cap")]
    GraduationCap,
    #[display(fmt = "fa-solid fa-greater-than")]
    GreaterThan,
    #[display(fmt = "fa-solid fa-greater-than-equal")]
    GreaterThanEqual,
    #[display(fmt = "fa-solid fa-grip")]
    Grip,
    #[display(fmt = "fa-solid fa-grip-lines")]
    GripLines,
    #[display(fmt = "fa-solid fa-grip-lines-vertical")]
    GripLinesVertical,
    #[display(fmt = "fa-solid fa-grip-vertical")]
    GripVertical,
    #[display(fmt = "fa-solid fa-group-arrows-rotate")]
    GroupArrowsRotate,
    #[display(fmt = "fa-solid fa-guarani-sign")]
    GuaraniSign,
    #[display(fmt = "fa-solid fa-guitar")]
    Guitar,
    #[display(fmt = "fa-solid fa-gun")]
    Gun,
    #[display(fmt = "fa-solid fa-h")]
    H,
    #[display(fmt = "fa-solid fa-hammer")]
    Hammer,
    #[display(fmt = "fa-solid fa-hamsa")]
    Hamsa,
    #[display(fmt = "fa-solid fa-hand")]
    Hand,
    #[display(fmt = "fa-solid fa-hand-back-fist")]
    HandBackFist,
    #[display(fmt = "fa-solid fa-hand-dots")]
    HandDots,
    #[display(fmt = "fa-solid fa-hand-fist")]
    HandFist,
    #[display(fmt = "fa-solid fa-hand-holding")]
    HandHolding,
    #[display(fmt = "fa-solid fa-hand-holding-dollar")]
    HandHoldingDollar,
    #[display(fmt = "fa-solid fa-hand-holding-droplet")]
    HandHoldingDroplet,
    #[display(fmt = "fa-solid fa-hand-holding-hand")]
    HandHoldingHand,
    #[display(fmt = "fa-solid fa-hand-holding-heart")]
    HandHoldingHeart,
    #[display(fmt = "fa-solid fa-hand-holding-medical")]
    HandHoldingMedical,
    #[display(fmt = "fa-solid fa-hand-lizard")]
    HandLizard,
    #[display(fmt = "fa-solid fa-hand-middle-finger")]
    HandMiddleFinger,
    #[display(fmt = "fa-solid fa-hand-peace")]
    HandPeace,
    #[display(fmt = "fa-solid fa-hand-point-down")]
    HandPointDown,
    #[display(fmt = "fa-solid fa-hand-point-left")]
    HandPointLeft,
    #[display(fmt = "fa-solid fa-hand-point-right")]
    HandPointRight,
    #[display(fmt = "fa-solid fa-hand-point-up")]
    HandPointUp,
    #[display(fmt = "fa-solid fa-hand-pointer")]
    HandPointer,
    #[display(fmt = "fa-solid fa-hand-scissors")]
    HandScissors,
    #[display(fmt = "fa-solid fa-hand-sparkles")]
    HandSparkles,
    #[display(fmt = "fa-solid fa-hand-spock")]
    HandSpock,
    #[display(fmt = "fa-solid fa-handcuffs")]
    Handcuffs,
    #[display(fmt = "fa-solid fa-hands")]
    Hands,
    #[display(fmt = "fa-solid fa-hands-asl-interpreting")]
    HandsAslInterpreting,
    #[display(fmt = "fa-solid fa-hands-bound")]
    HandsBound,
    #[display(fmt = "fa-solid fa-hands-bubbles")]
    HandsBubbles,
    #[display(fmt = "fa-solid fa-hands-clapping")]
    HandsClapping,
    #[display(fmt = "fa-solid fa-hands-holding")]
    HandsHolding,
    #[display(fmt = "fa-solid fa-hands-holding-child")]
    HandsHoldingChild,
    #[display(fmt = "fa-solid fa-hands-holding-circle")]
    HandsHoldingCircle,
    #[display(fmt = "fa-solid fa-hands-praying")]
    HandsPraying,
    #[display(fmt = "fa-solid fa-handshake")]
    Handshake,
    #[display(fmt = "fa-solid fa-handshake-angle")]
    HandshakeAngle,
    #[display(fmt = "fa-solid fa-handshake-simple")]
    HandshakeSimple,
    #[display(fmt = "fa-solid fa-handshake-simple-slash")]
    HandshakeSimpleSlash,
    #[display(fmt = "fa-solid fa-handshake-slash")]
    HandshakeSlash,
    #[display(fmt = "fa-solid fa-hanukiah")]
    Hanukiah,
    #[display(fmt = "fa-solid fa-hard-drive")]
    HardDrive,
    #[display(fmt = "fa-solid fa-hashtag")]
    Hashtag,
    #[display(fmt = "fa-solid fa-hat-cowboy")]
    HatCowboy,
    #[display(fmt = "fa-solid fa-hat-cowboy-side")]
    HatCowboySide,
    #[display(fmt = "fa-solid fa-hat-wizard")]
    HatWizard,
    #[display(fmt = "fa-solid fa-head-side-cough")]
    HeadSideCough,
    #[display(fmt = "fa-solid fa-head-side-cough-slash")]
    HeadSideCoughSlash,
    #[display(fmt = "fa-solid fa-head-side-mask")]
    HeadSideMask,
    #[display(fmt = "fa-solid fa-head-side-virus")]
    HeadSideVirus,
    #[display(fmt = "fa-solid fa-heading")]
    Heading,
    #[display(fmt = "fa-solid fa-headphones")]
    Headphones,
    #[display(fmt = "fa-solid fa-headphones-simple")]
    HeadphonesSimple,
    #[display(fmt = "fa-solid fa-headset")]
    Headset,
    #[display(fmt = "fa-solid fa-heart")]
    Heart,
    #[display(fmt = "fa-solid fa-heart-circle-bolt")]
    HeartCircleBolt,
    #[display(fmt = "fa-solid fa-heart-circle-check")]
    HeartCircleCheck,
    #[display(fmt = "fa-solid fa-heart-circle-exclamation")]
    HeartCircleExclamation,
    #[display(fmt = "fa-solid fa-heart-circle-minus")]
    HeartCircleMinus,
    #[display(fmt = "fa-solid fa-heart-circle-plus")]
    HeartCirclePlus,
    #[display(fmt = "fa-solid fa-heart-circle-xmark")]
    HeartCircleXmark,
    #[display(fmt = "fa-solid fa-heart-crack")]
    HeartCrack,
    #[display(fmt = "fa-solid fa-heart-pulse")]
    HeartPulse,
    #[display(fmt = "fa-solid fa-helicopter")]
    Helicopter,
    #[display(fmt = "fa-solid fa-helicopter-symbol")]
    HelicopterSymbol,
    #[display(fmt = "fa-solid fa-helmet-safety")]
    HelmetSafety,
    #[display(fmt = "fa-solid fa-helmet-un")]
    HelmetUn,
    #[display(fmt = "fa-solid fa-highlighter")]
    Highlighter,
    #[display(fmt = "fa-solid fa-hill-avalanche")]
    HillAvalanche,
    #[display(fmt = "fa-solid fa-hill-rockslide")]
    HillRockslide,
    #[display(fmt = "fa-solid fa-hippo")]
    Hippo,
    #[display(fmt = "fa-solid fa-hockey-puck")]
    HockeyPuck,
    #[display(fmt = "fa-solid fa-holly-berry")]
    HollyBerry,
    #[display(fmt = "fa-solid fa-horse")]
    Horse,
    #[display(fmt = "fa-solid fa-horse-head")]
    HorseHead,
    #[display(fmt = "fa-solid fa-hospital")]
    Hospital,
    #[display(fmt = "fa-solid fa-hospital-user")]
    HospitalUser,
    #[display(fmt = "fa-solid fa-hot-tub-person")]
    HotTubPerson,
    #[display(fmt = "fa-solid fa-hotdog")]
    Hotdog,
    #[display(fmt = "fa-solid fa-hotel")]
    Hotel,
    #[display(fmt = "fa-solid fa-hourglass")]
    Hourglass,
    #[display(fmt = "fa-solid fa-hourglass-empty")]
    HourglassEmpty,
    #[display(fmt = "fa-solid fa-hourglass-end")]
    HourglassEnd,
    #[display(fmt = "fa-solid fa-hourglass-start")]
    HourglassStart,
    #[display(fmt = "fa-solid fa-house")]
    House,
    #[display(fmt = "fa-solid fa-house-chimney")]
    HouseChimney,
    #[display(fmt = "fa-solid fa-house-chimney-crack")]
    HouseChimneyCrack,
    #[display(fmt = "fa-solid fa-house-chimney-medical")]
    HouseChimneyMedical,
    #[display(fmt = "fa-solid fa-house-chimney-user")]
    HouseChimneyUser,
    #[display(fmt = "fa-solid fa-house-chimney-window")]
    HouseChimneyWindow,
    #[display(fmt = "fa-solid fa-house-circle-check")]
    HouseCircleCheck,
    #[display(fmt = "fa-solid fa-house-circle-exclamation")]
    HouseCircleExclamation,
    #[display(fmt = "fa-solid fa-house-circle-xmark")]
    HouseCircleXmark,
    #[display(fmt = "fa-solid fa-house-crack")]
    HouseCrack,
    #[display(fmt = "fa-solid fa-house-fire")]
    HouseFire,
    #[display(fmt = "fa-solid fa-house-flag")]
    HouseFlag,
    #[display(fmt = "fa-solid fa-house-flood-water")]
    HouseFloodWater,
    #[display(fmt = "fa-solid fa-house-flood-water-circle-arrow-right")]
    HouseFloodWaterCircleArrowRight,
    #[display(fmt = "fa-solid fa-house-laptop")]
    HouseLaptop,
    #[display(fmt = "fa-solid fa-house-lock")]
    HouseLock,
    #[display(fmt = "fa-solid fa-house-medical")]
    HouseMedical,
    #[display(fmt = "fa-solid fa-house-medical-circle-check")]
    HouseMedicalCircleCheck,
    #[display(fmt = "fa-solid fa-house-medical-circle-exclamation")]
    HouseMedicalCircleExclamation,
    #[display(fmt = "fa-solid fa-house-medical-circle-xmark")]
    HouseMedicalCircleXmark,
    #[display(fmt = "fa-solid fa-house-medical-flag")]
    HouseMedicalFlag,
    #[display(fmt = "fa-solid fa-house-signal")]
    HouseSignal,
    #[display(fmt = "fa-solid fa-house-tsunami")]
    HouseTsunami,
    #[display(fmt = "fa-solid fa-house-user")]
    HouseUser,
    #[display(fmt = "fa-solid fa-hryvnia-sign")]
    HryvniaSign,
    #[display(fmt = "fa-solid fa-hurricane")]
    Hurricane,
    #[display(fmt = "fa-solid fa-i")]
    I,
    #[display(fmt = "fa-solid fa-i-cursor")]
    ICursor,
    #[display(fmt = "fa-solid fa-ice-cream")]
    IceCream,
    #[display(fmt = "fa-solid fa-icicles")]
    Icicles,
    #[display(fmt = "fa-solid fa-icons")]
    Icons,
    #[display(fmt = "fa-solid fa-id-badge")]
    IdBadge,
    #[display(fmt = "fa-solid fa-id-card")]
    IdCard,
    #[display(fmt = "fa-solid fa-id-card-clip")]
    IdCardClip,
    #[display(fmt = "fa-solid fa-igloo")]
    Igloo,
    #[display(fmt = "fa-solid fa-image")]
    Image,
    #[display(fmt = "fa-solid fa-image-portrait")]
    ImagePortrait,
    #[display(fmt = "fa-solid fa-images")]
    Images,
    #[display(fmt = "fa-solid fa-inbox")]
    Inbox,
    #[display(fmt = "fa-solid fa-indent")]
    Indent,
    #[display(fmt = "fa-solid fa-indian-rupee-sign")]
    IndianRupeeSign,
    #[display(fmt = "fa-solid fa-industry")]
    Industry,
    #[display(fmt = "fa-solid fa-infinity")]
    Infinity,
    #[display(fmt = "fa-solid fa-info")]
    Info,
    #[display(fmt = "fa-solid fa-italic")]
    Italic,
    #[display(fmt = "fa-solid fa-j")]
    J,
    #[display(fmt = "fa-solid fa-jar")]
    Jar,
    #[display(fmt = "fa-solid fa-jar-wheat")]
    JarWheat,
    #[display(fmt = "fa-solid fa-jedi")]
    Jedi,
    #[display(fmt = "fa-solid fa-jet-fighter")]
    JetFighter,
    #[display(fmt = "fa-solid fa-jet-fighter-up")]
    JetFighterUp,
    #[display(fmt = "fa-solid fa-joint")]
    Joint,
    #[display(fmt = "fa-solid fa-jug-detergent")]
    JugDetergent,
    #[display(fmt = "fa-solid fa-k")]
    K,
    #[display(fmt = "fa-solid fa-kaaba")]
    Kaaba,
    #[display(fmt = "fa-solid fa-key")]
    Key,
    #[display(fmt = "fa-solid fa-keyboard")]
    Keyboard,
    #[display(fmt = "fa-solid fa-khanda")]
    Khanda,
    #[display(fmt = "fa-solid fa-kip-sign")]
    KipSign,
    #[display(fmt = "fa-solid fa-kit-medical")]
    KitMedical,
    #[display(fmt = "fa-solid fa-kitchen-set")]
    KitchenSet,
    #[display(fmt = "fa-solid fa-kiwi-bird")]
    KiwiBird,
    #[display(fmt = "fa-solid fa-l")]
    L,
    #[display(fmt = "fa-solid fa-land-mine-on")]
    LandMineOn,
    #[display(fmt = "fa-solid fa-landmark")]
    Landmark,
    #[display(fmt = "fa-solid fa-landmark-dome")]
    LandmarkDome,
    #[display(fmt = "fa-solid fa-landmark-flag")]
    LandmarkFlag,
    #[display(fmt = "fa-solid fa-language")]
    Language,
    #[display(fmt = "fa-solid fa-laptop")]
    Laptop,
    #[display(fmt = "fa-solid fa-laptop-code")]
    LaptopCode,
    #[display(fmt = "fa-solid fa-laptop-file")]
    LaptopFile,
    #[display(fmt = "fa-solid fa-laptop-medical")]
    LaptopMedical,
    #[display(fmt = "fa-solid fa-lari-sign")]
    LariSign,
    #[display(fmt = "fa-solid fa-layer-group")]
    LayerGroup,
    #[display(fmt = "fa-solid fa-leaf")]
    Leaf,
    #[display(fmt = "fa-solid fa-left-long")]
    LeftLong,
    #[display(fmt = "fa-solid fa-left-right")]
    LeftRight,
    #[display(fmt = "fa-solid fa-lemon")]
    Lemon,
    #[display(fmt = "fa-solid fa-less-than")]
    LessThan,
    #[display(fmt = "fa-solid fa-less-than-equal")]
    LessThanEqual,
    #[display(fmt = "fa-solid fa-life-ring")]
    LifeRing,
    #[display(fmt = "fa-solid fa-lightbulb")]
    Lightbulb,
    #[display(fmt = "fa-solid fa-lines-leaning")]
    LinesLeaning,
    #[display(fmt = "fa-solid fa-link")]
    Link,
    #[display(fmt = "fa-solid fa-link-slash")]
    LinkSlash,
    #[display(fmt = "fa-solid fa-lira-sign")]
    LiraSign,
    #[display(fmt = "fa-solid fa-list")]
    List,
    #[display(fmt = "fa-solid fa-list-check")]
    ListCheck,
    #[display(fmt = "fa-solid fa-list-ol")]
    ListOl,
    #[display(fmt = "fa-solid fa-list-ul")]
    ListUl,
    #[display(fmt = "fa-solid fa-litecoin-sign")]
    LitecoinSign,
    #[display(fmt = "fa-solid fa-location-arrow")]
    LocationArrow,
    #[display(fmt = "fa-solid fa-location-crosshairs")]
    LocationCrosshairs,
    #[display(fmt = "fa-solid fa-location-dot")]
    LocationDot,
    #[display(fmt = "fa-solid fa-location-pin")]
    LocationPin,
    #[display(fmt = "fa-solid fa-location-pin-lock")]
    LocationPinLock,
    #[display(fmt = "fa-solid fa-lock")]
    Lock,
    #[display(fmt = "fa-solid fa-lock-open")]
    LockOpen,
    #[display(fmt = "fa-solid fa-locust")]
    Locust,
    #[display(fmt = "fa-solid fa-lungs")]
    Lungs,
    #[display(fmt = "fa-solid fa-lungs-virus")]
    LungsVirus,
    #[display(fmt = "fa-solid fa-m")]
    M,
    #[display(fmt = "fa-solid fa-magnet")]
    Magnet,
    #[display(fmt = "fa-solid fa-magnifying-glass")]
    MagnifyingGlass,
    #[display(fmt = "fa-solid fa-magnifying-glass-arrow-right")]
    MagnifyingGlassArrowRight,
    #[display(fmt = "fa-solid fa-magnifying-glass-chart")]
    MagnifyingGlassChart,
    #[display(fmt = "fa-solid fa-magnifying-glass-dollar")]
    MagnifyingGlassDollar,
    #[display(fmt = "fa-solid fa-magnifying-glass-location")]
    MagnifyingGlassLocation,
    #[display(fmt = "fa-solid fa-magnifying-glass-minus")]
    MagnifyingGlassMinus,
    #[display(fmt = "fa-solid fa-magnifying-glass-plus")]
    MagnifyingGlassPlus,
    #[display(fmt = "fa-solid fa-manat-sign")]
    ManatSign,
    #[display(fmt = "fa-solid fa-map")]
    Map,
    #[display(fmt = "fa-solid fa-map-location")]
    MapLocation,
    #[display(fmt = "fa-solid fa-map-location-dot")]
    MapLocationDot,
    #[display(fmt = "fa-solid fa-map-pin")]
    MapPin,
    #[display(fmt = "fa-solid fa-marker")]
    Marker,
    #[display(fmt = "fa-solid fa-mars")]
    Mars,
    #[display(fmt = "fa-solid fa-mars-and-venus")]
    MarsAndVenus,
    #[display(fmt = "fa-solid fa-mars-and-venus-burst")]
    MarsAndVenusBurst,
    #[display(fmt = "fa-solid fa-mars-double")]
    MarsDouble,
    #[display(fmt = "fa-solid fa-mars-stroke")]
    MarsStroke,
    #[display(fmt = "fa-solid fa-mars-stroke-right")]
    MarsStrokeRight,
    #[display(fmt = "fa-solid fa-mars-stroke-up")]
    MarsStrokeUp,
    #[display(fmt = "fa-solid fa-martini-glass")]
    MartiniGlass,
    #[display(fmt = "fa-solid fa-martini-glass-citrus")]
    MartiniGlassCitrus,
    #[display(fmt = "fa-solid fa-martini-glass-empty")]
    MartiniGlassEmpty,
    #[display(fmt = "fa-solid fa-mask")]
    Mask,
    #[display(fmt = "fa-solid fa-mask-face")]
    MaskFace,
    #[display(fmt = "fa-solid fa-mask-ventilator")]
    MaskVentilator,
    #[display(fmt = "fa-solid fa-masks-theater")]
    MasksTheater,
    #[display(fmt = "fa-solid fa-mattress-pillow")]
    MattressPillow,
    #[display(fmt = "fa-solid fa-maximize")]
    Maximize,
    #[display(fmt = "fa-solid fa-medal")]
    Medal,
    #[display(fmt = "fa-solid fa-memory")]
    Memory,
    #[display(fmt = "fa-solid fa-menorah")]
    Menorah,
    #[display(fmt = "fa-solid fa-mercury")]
    Mercury,
    #[display(fmt = "fa-solid fa-message")]
    Message,
    #[display(fmt = "fa-solid fa-meteor")]
    Meteor,
    #[display(fmt = "fa-solid fa-microchip")]
    Microchip,
    #[display(fmt = "fa-solid fa-microphone")]
    Microphone,
    #[display(fmt = "fa-solid fa-microphone-lines")]
    MicrophoneLines,
    #[display(fmt = "fa-solid fa-microphone-lines-slash")]
    MicrophoneLinesSlash,
    #[display(fmt = "fa-solid fa-microphone-slash")]
    MicrophoneSlash,
    #[display(fmt = "fa-solid fa-microscope")]
    Microscope,
    #[display(fmt = "fa-solid fa-mill-sign")]
    MillSign,
    #[display(fmt = "fa-solid fa-minimize")]
    Minimize,
    #[display(fmt = "fa-solid fa-minus")]
    Minus,
    #[display(fmt = "fa-solid fa-mitten")]
    Mitten,
    #[display(fmt = "fa-solid fa-mobile")]
    Mobile,
    #[display(fmt = "fa-solid fa-mobile-button")]
    MobileButton,
    #[display(fmt = "fa-solid fa-mobile-retro")]
    MobileRetro,
    #[display(fmt = "fa-solid fa-mobile-screen")]
    MobileScreen,
    #[display(fmt = "fa-solid fa-mobile-screen-button")]
    MobileScreenButton,
    #[display(fmt = "fa-solid fa-money-bill")]
    MoneyBill,
    #[display(fmt = "fa-solid fa-money-bill-1")]
    MoneyBill1,
    #[display(fmt = "fa-solid fa-money-bill-1-wave")]
    MoneyBill1Wave,
    #[display(fmt = "fa-solid fa-money-bill-transfer")]
    MoneyBillTransfer,
    #[display(fmt = "fa-solid fa-money-bill-trend-up")]
    MoneyBillTrendUp,
    #[display(fmt = "fa-solid fa-money-bill-wave")]
    MoneyBillWave,
    #[display(fmt = "fa-solid fa-money-bill-wheat")]
    MoneyBillWheat,
    #[display(fmt = "fa-solid fa-money-bills")]
    MoneyBills,
    #[display(fmt = "fa-solid fa-money-check")]
    MoneyCheck,
    #[display(fmt = "fa-solid fa-money-check-dollar")]
    MoneyCheckDollar,
    #[display(fmt = "fa-solid fa-monument")]
    Monument,
    #[display(fmt = "fa-solid fa-moon")]
    Moon,
    #[display(fmt = "fa-solid fa-mortar-pestle")]
    MortarPestle,
    #[display(fmt = "fa-solid fa-mosque")]
    Mosque,
    #[display(fmt = "fa-solid fa-mosquito")]
    Mosquito,
    #[display(fmt = "fa-solid fa-mosquito-net")]
    MosquitoNet,
    #[display(fmt = "fa-solid fa-motorcycle")]
    Motorcycle,
    #[display(fmt = "fa-solid fa-mound")]
    Mound,
    #[display(fmt = "fa-solid fa-mountain")]
    Mountain,
    #[display(fmt = "fa-solid fa-mountain-city")]
    MountainCity,
    #[display(fmt = "fa-solid fa-mountain-sun")]
    MountainSun,
    #[display(fmt = "fa-solid fa-mug-hot")]
    MugHot,
    #[display(fmt = "fa-solid fa-mug-saucer")]
    MugSaucer,
    #[display(fmt = "fa-solid fa-music")]
    Music,
    #[display(fmt = "fa-solid fa-n")]
    N,
    #[display(fmt = "fa-solid fa-naira-sign")]
    NairaSign,
    #[display(fmt = "fa-solid fa-network-wired")]
    NetworkWired,
    #[display(fmt = "fa-solid fa-neuter")]
    Neuter,
    #[display(fmt = "fa-solid fa-newspaper")]
    Newspaper,
    #[display(fmt = "fa-solid fa-not-equal")]
    NotEqual,
    #[display(fmt = "fa-solid fa-note-sticky")]
    NoteSticky,
    #[display(fmt = "fa-solid fa-notes-medical")]
    NotesMedical,
    #[display(fmt = "fa-solid fa-o")]
    O,
    #[display(fmt = "fa-solid fa-object-group")]
    ObjectGroup,
    #[display(fmt = "fa-solid fa-object-ungroup")]
    ObjectUngroup,
    #[display(fmt = "fa-solid fa-oil-can")]
    OilCan,
    #[display(fmt = "fa-solid fa-oil-well")]
    OilWell,
    #[display(fmt = "fa-solid fa-om")]
    Om,
    #[display(fmt = "fa-solid fa-otter")]
    Otter,
    #[display(fmt = "fa-solid fa-outdent")]
    Outdent,
    #[display(fmt = "fa-solid fa-p")]
    P,
    #[display(fmt = "fa-solid fa-pager")]
    Pager,
    #[display(fmt = "fa-solid fa-paint-roller")]
    PaintRoller,
    #[display(fmt = "fa-solid fa-paintbrush")]
    Paintbrush,
    #[display(fmt = "fa-solid fa-palette")]
    Palette,
    #[display(fmt = "fa-solid fa-pallet")]
    Pallet,
    #[display(fmt = "fa-solid fa-panorama")]
    Panorama,
    #[display(fmt = "fa-solid fa-paper-plane")]
    PaperPlane,
    #[display(fmt = "fa-solid fa-paperclip")]
    Paperclip,
    #[display(fmt = "fa-solid fa-parachute-box")]
    ParachuteBox,
    #[display(fmt = "fa-solid fa-paragraph")]
    Paragraph,
    #[display(fmt = "fa-solid fa-passport")]
    Passport,
    #[display(fmt = "fa-solid fa-paste")]
    Paste,
    #[display(fmt = "fa-solid fa-pause")]
    Pause,
    #[display(fmt = "fa-solid fa-paw")]
    Paw,
    #[display(fmt = "fa-solid fa-peace")]
    Peace,
    #[display(fmt = "fa-solid fa-pen")]
    Pen,
    #[display(fmt = "fa-solid fa-pen-clip")]
    PenClip,
    #[display(fmt = "fa-solid fa-pen-fancy")]
    PenFancy,
    #[display(fmt = "fa-solid fa-pen-nib")]
    PenNib,
    #[display(fmt = "fa-solid fa-pen-ruler")]
    PenRuler,
    #[display(fmt = "fa-solid fa-pen-to-square")]
    PenToSquare,
    #[display(fmt = "fa-solid fa-pencil")]
    Pencil,
    #[display(fmt = "fa-solid fa-people-arrows-left-right")]
    PeopleArrowsLeftRight,
    #[display(fmt = "fa-solid fa-people-carry-box")]
    PeopleCarryBox,
    #[display(fmt = "fa-solid fa-people-group")]
    PeopleGroup,
    #[display(fmt = "fa-solid fa-people-line")]
    PeopleLine,
    #[display(fmt = "fa-solid fa-people-pulling")]
    PeoplePulling,
    #[display(fmt = "fa-solid fa-people-robbery")]
    PeopleRobbery,
    #[display(fmt = "fa-solid fa-people-roof")]
    PeopleRoof,
    #[display(fmt = "fa-solid fa-pepper-hot")]
    PepperHot,
    #[display(fmt = "fa-solid fa-percent")]
    Percent,
    #[display(fmt = "fa-solid fa-person")]
    Person,
    #[display(fmt = "fa-solid fa-person-arrow-down-to-line")]
    PersonArrowDownToLine,
    #[display(fmt = "fa-solid fa-person-arrow-up-from-line")]
    PersonArrowUpFromLine,
    #[display(fmt = "fa-solid fa-person-biking")]
    PersonBiking,
    #[display(fmt = "fa-solid fa-person-booth")]
    PersonBooth,
    #[display(fmt = "fa-solid fa-person-breastfeeding")]
    PersonBreastfeeding,
    #[display(fmt = "fa-solid fa-person-burst")]
    PersonBurst,
    #[display(fmt = "fa-solid fa-person-cane")]
    PersonCane,
    #[display(fmt = "fa-solid fa-person-chalkboard")]
    PersonChalkboard,
    #[display(fmt = "fa-solid fa-person-circle-check")]
    PersonCircleCheck,
    #[display(fmt = "fa-solid fa-person-circle-exclamation")]
    PersonCircleExclamation,
    #[display(fmt = "fa-solid fa-person-circle-minus")]
    PersonCircleMinus,
    #[display(fmt = "fa-solid fa-person-circle-plus")]
    PersonCirclePlus,
    #[display(fmt = "fa-solid fa-person-circle-question")]
    PersonCircleQuestion,
    #[display(fmt = "fa-solid fa-person-circle-xmark")]
    PersonCircleXmark,
    #[display(fmt = "fa-solid fa-person-digging")]
    PersonDigging,
    #[display(fmt = "fa-solid fa-person-dots-from-line")]
    PersonDotsFromLine,
    #[display(fmt = "fa-solid fa-person-dress")]
    PersonDress,
    #[display(fmt = "fa-solid fa-person-dress-burst")]
    PersonDressBurst,
    #[display(fmt = "fa-solid fa-person-drowning")]
    PersonDrowning,
    #[display(fmt = "fa-solid fa-person-falling")]
    PersonFalling,
    #[display(fmt = "fa-solid fa-person-falling-burst")]
    PersonFallingBurst,
    #[display(fmt = "fa-solid fa-person-half-dress")]
    PersonHalfDress,
    #[display(fmt = "fa-solid fa-person-harassing")]
    PersonHarassing,
    #[display(fmt = "fa-solid fa-person-hiking")]
    PersonHiking,
    #[display(fmt = "fa-solid fa-person-military-pointing")]
    PersonMilitaryPointing,
    #[display(fmt = "fa-solid fa-person-military-rifle")]
    PersonMilitaryRifle,
    #[display(fmt = "fa-solid fa-person-military-to-person")]
    PersonMilitaryToPerson,
    #[display(fmt = "fa-solid fa-person-praying")]
    PersonPraying,
    #[display(fmt = "fa-solid fa-person-pregnant")]
    PersonPregnant,
    #[display(fmt = "fa-solid fa-person-rays")]
    PersonRays,
    #[display(fmt = "fa-solid fa-person-rifle")]
    PersonRifle,
    #[display(fmt = "fa-solid fa-person-running")]
    PersonRunning,
    #[display(fmt = "fa-solid fa-person-shelter")]
    PersonShelter,
    #[display(fmt = "fa-solid fa-person-skating")]
    PersonSkating,
    #[display(fmt = "fa-solid fa-person-skiing")]
    PersonSkiing,
    #[display(fmt = "fa-solid fa-person-skiing-nordic")]
    PersonSkiingNordic,
    #[display(fmt = "fa-solid fa-person-snowboarding")]
    PersonSnowboarding,
    #[display(fmt = "fa-solid fa-person-swimming")]
    PersonSwimming,
    #[display(fmt = "fa-solid fa-person-through-window")]
    PersonThroughWindow,
    #[display(fmt = "fa-solid fa-person-walking")]
    PersonWalking,
    #[display(fmt = "fa-solid fa-person-walking-arrow-loop-left")]
    PersonWalkingArrowLoopLeft,
    #[display(fmt = "fa-solid fa-person-walking-arrow-right")]
    PersonWalkingArrowRight,
    #[display(fmt = "fa-solid fa-person-walking-dashed-line-arrow-right")]
    PersonWalkingDashedLineArrowRight,
    #[display(fmt = "fa-solid fa-person-walking-luggage")]
    PersonWalkingLuggage,
    #[display(fmt = "fa-solid fa-person-walking-with-cane")]
    PersonWalkingWithCane,
    #[display(fmt = "fa-solid fa-peseta-sign")]
    PesetaSign,
    #[display(fmt = "fa-solid fa-peso-sign")]
    PesoSign,
    #[display(fmt = "fa-solid fa-phone")]
    Phone,
    #[display(fmt = "fa-solid fa-phone-flip")]
    PhoneFlip,
    #[display(fmt = "fa-solid fa-phone-slash")]
    PhoneSlash,
    #[display(fmt = "fa-solid fa-phone-volume")]
    PhoneVolume,
    #[display(fmt = "fa-solid fa-photo-film")]
    PhotoFilm,
    #[display(fmt = "fa-solid fa-piggy-bank")]
    PiggyBank,
    #[display(fmt = "fa-solid fa-pills")]
    Pills,
    #[display(fmt = "fa-solid fa-pizza-slice")]
    PizzaSlice,
    #[display(fmt = "fa-solid fa-place-of-worship")]
    PlaceOfWorship,
    #[display(fmt = "fa-solid fa-plane")]
    Plane,
    #[display(fmt = "fa-solid fa-plane-arrival")]
    PlaneArrival,
    #[display(fmt = "fa-solid fa-plane-circle-check")]
    PlaneCircleCheck,
    #[display(fmt = "fa-solid fa-plane-circle-exclamation")]
    PlaneCircleExclamation,
    #[display(fmt = "fa-solid fa-plane-circle-xmark")]
    PlaneCircleXmark,
    #[display(fmt = "fa-solid fa-plane-departure")]
    PlaneDeparture,
    #[display(fmt = "fa-solid fa-plane-lock")]
    PlaneLock,
    #[display(fmt = "fa-solid fa-plane-slash")]
    PlaneSlash,
    #[display(fmt = "fa-solid fa-plane-up")]
    PlaneUp,
    #[display(fmt = "fa-solid fa-plant-wilt")]
    PlantWilt,
    #[display(fmt = "fa-solid fa-plate-wheat")]
    PlateWheat,
    #[display(fmt = "fa-solid fa-play")]
    Play,
    #[display(fmt = "fa-solid fa-plug")]
    Plug,
    #[display(fmt = "fa-solid fa-plug-circle-bolt")]
    PlugCircleBolt,
    #[display(fmt = "fa-solid fa-plug-circle-check")]
    PlugCircleCheck,
    #[display(fmt = "fa-solid fa-plug-circle-exclamation")]
    PlugCircleExclamation,
    #[display(fmt = "fa-solid fa-plug-circle-minus")]
    PlugCircleMinus,
    #[display(fmt = "fa-solid fa-plug-circle-plus")]
    PlugCirclePlus,
    #[display(fmt = "fa-solid fa-plug-circle-xmark")]
    PlugCircleXmark,
    #[display(fmt = "fa-solid fa-plus")]
    Plus,
    #[display(fmt = "fa-solid fa-plus-minus")]
    PlusMinus,
    #[display(fmt = "fa-solid fa-podcast")]
    Podcast,
    #[display(fmt = "fa-solid fa-poo")]
    Poo,
    #[display(fmt = "fa-solid fa-poo-storm")]
    PooStorm,
    #[display(fmt = "fa-solid fa-poop")]
    Poop,
    #[display(fmt = "fa-solid fa-power-off")]
    PowerOff,
    #[display(fmt = "fa-solid fa-prescription")]
    Prescription,
    #[display(fmt = "fa-solid fa-prescription-bottle")]
    PrescriptionBottle,
    #[display(fmt = "fa-solid fa-prescription-bottle-medical")]
    PrescriptionBottleMedical,
    #[display(fmt = "fa-solid fa-print")]
    Print,
    #[display(fmt = "fa-solid fa-pump-medical")]
    PumpMedical,
    #[display(fmt = "fa-solid fa-pump-soap")]
    PumpSoap,
    #[display(fmt = "fa-solid fa-puzzle-piece")]
    PuzzlePiece,
    #[display(fmt = "fa-solid fa-q")]
    Q,
    #[display(fmt = "fa-solid fa-qrcode")]
    Qrcode,
    #[display(fmt = "fa-solid fa-question")]
    Question,
    #[display(fmt = "fa-solid fa-quote-left")]
    QuoteLeft,
    #[display(fmt = "fa-solid fa-quote-right")]
    QuoteRight,
    #[display(fmt = "fa-solid fa-r")]
    R,
    #[display(fmt = "fa-solid fa-radiation")]
    Radiation,
    #[display(fmt = "fa-solid fa-radio")]
    Radio,
    #[display(fmt = "fa-solid fa-rainbow")]
    Rainbow,
    #[display(fmt = "fa-solid fa-ranking-star")]
    RankingStar,
    #[display(fmt = "fa-solid fa-receipt")]
    Receipt,
    #[display(fmt = "fa-solid fa-record-vinyl")]
    RecordVinyl,
    #[display(fmt = "fa-solid fa-rectangle-ad")]
    RectangleAd,
    #[display(fmt = "fa-solid fa-rectangle-list")]
    RectangleList,
    #[display(fmt = "fa-solid fa-rectangle-xmark")]
    RectangleXmark,
    #[display(fmt = "fa-solid fa-recycle")]
    Recycle,
    #[display(fmt = "fa-solid fa-registered")]
    Registered,
    #[display(fmt = "fa-solid fa-repeat")]
    Repeat,
    #[display(fmt = "fa-solid fa-reply")]
    Reply,
    #[display(fmt = "fa-solid fa-reply-all")]
    ReplyAll,
    #[display(fmt = "fa-solid fa-republican")]
    Republican,
    #[display(fmt = "fa-solid fa-restroom")]
    Restroom,
    #[display(fmt = "fa-solid fa-retweet")]
    Retweet,
    #[display(fmt = "fa-solid fa-ribbon")]
    Ribbon,
    #[display(fmt = "fa-solid fa-right-from-bracket")]
    RightFromBracket,
    #[display(fmt = "fa-solid fa-right-left")]
    RightLeft,
    #[display(fmt = "fa-solid fa-right-long")]
    RightLong,
    #[display(fmt = "fa-solid fa-right-to-bracket")]
    RightToBracket,
    #[display(fmt = "fa-solid fa-ring")]
    Ring,
    #[display(fmt = "fa-solid fa-road")]
    Road,
    #[display(fmt = "fa-solid fa-road-barrier")]
    RoadBarrier,
    #[display(fmt = "fa-solid fa-road-bridge")]
    RoadBridge,
    #[display(fmt = "fa-solid fa-road-circle-check")]
    RoadCircleCheck,
    #[display(fmt = "fa-solid fa-road-circle-exclamation")]
    RoadCircleExclamation,
    #[display(fmt = "fa-solid fa-road-circle-xmark")]
    RoadCircleXmark,
    #[display(fmt = "fa-solid fa-road-lock")]
    RoadLock,
    #[display(fmt = "fa-solid fa-road-spikes")]
    RoadSpikes,
    #[display(fmt = "fa-solid fa-robot")]
    Robot,
    #[display(fmt = "fa-solid fa-rocket")]
    Rocket,
    #[display(fmt = "fa-solid fa-rotate")]
    Rotate,
    #[display(fmt = "fa-solid fa-rotate-left")]
    RotateLeft,
    #[display(fmt = "fa-solid fa-rotate-right")]
    RotateRight,
    #[display(fmt = "fa-solid fa-route")]
    Route,
    #[display(fmt = "fa-solid fa-rss")]
    Rss,
    #[display(fmt = "fa-solid fa-ruble-sign")]
    RubleSign,
    #[display(fmt = "fa-solid fa-rug")]
    Rug,
    #[display(fmt = "fa-solid fa-ruler")]
    Ruler,
    #[display(fmt = "fa-solid fa-ruler-combined")]
    RulerCombined,
    #[display(fmt = "fa-solid fa-ruler-horizontal")]
    RulerHorizontal,
    #[display(fmt = "fa-solid fa-ruler-vertical")]
    RulerVertical,
    #[display(fmt = "fa-solid fa-rupee-sign")]
    RupeeSign,
    #[display(fmt = "fa-solid fa-rupiah-sign")]
    RupiahSign,
    #[display(fmt = "fa-solid fa-s")]
    S,
    #[display(fmt = "fa-solid fa-sack-dollar")]
    SackDollar,
    #[display(fmt = "fa-solid fa-sack-xmark")]
    SackXmark,
    #[display(fmt = "fa-solid fa-sailboat")]
    Sailboat,
    #[display(fmt = "fa-solid fa-satellite")]
    Satellite,
    #[display(fmt = "fa-solid fa-satellite-dish")]
    SatelliteDish,
    #[display(fmt = "fa-solid fa-scale-balanced")]
    ScaleBalanced,
    #[display(fmt = "fa-solid fa-scale-unbalanced")]
    ScaleUnbalanced,
    #[display(fmt = "fa-solid fa-scale-unbalanced-flip")]
    ScaleUnbalancedFlip,
    #[display(fmt = "fa-solid fa-school")]
    School,
    #[display(fmt = "fa-solid fa-school-circle-check")]
    SchoolCircleCheck,
    #[display(fmt = "fa-solid fa-school-circle-exclamation")]
    SchoolCircleExclamation,
    #[display(fmt = "fa-solid fa-school-circle-xmark")]
    SchoolCircleXmark,
    #[display(fmt = "fa-solid fa-school-flag")]
    SchoolFlag,
    #[display(fmt = "fa-solid fa-school-lock")]
    SchoolLock,
    #[display(fmt = "fa-solid fa-scissors")]
    Scissors,
    #[display(fmt = "fa-solid fa-screwdriver")]
    Screwdriver,
    #[display(fmt = "fa-solid fa-screwdriver-wrench")]
    ScrewdriverWrench,
    #[display(fmt = "fa-solid fa-scroll")]
    Scroll,
    #[display(fmt = "fa-solid fa-scroll-torah")]
    ScrollTorah,
    #[display(fmt = "fa-solid fa-sd-card")]
    SdCard,
    #[display(fmt = "fa-solid fa-section")]
    Section,
    #[display(fmt = "fa-solid fa-seedling")]
    Seedling,
    #[display(fmt = "fa-solid fa-server")]
    Server,
    #[display(fmt = "fa-solid fa-shapes")]
    Shapes,
    #[display(fmt = "fa-solid fa-share")]
    Share,
    #[display(fmt = "fa-solid fa-share-from-square")]
    ShareFromSquare,
    #[display(fmt = "fa-solid fa-share-nodes")]
    ShareNodes,
    #[display(fmt = "fa-solid fa-sheet-plastic")]
    SheetPlastic,
    #[display(fmt = "fa-solid fa-shekel-sign")]
    ShekelSign,
    #[display(fmt = "fa-solid fa-shield")]
    Shield,
    #[display(fmt = "fa-solid fa-shield-cat")]
    ShieldCat,
    #[display(fmt = "fa-solid fa-shield-dog")]
    ShieldDog,
    #[display(fmt = "fa-solid fa-shield-halved")]
    ShieldHalved,
    #[display(fmt = "fa-solid fa-shield-heart")]
    ShieldHeart,
    #[display(fmt = "fa-solid fa-shield-virus")]
    ShieldVirus,
    #[display(fmt = "fa-solid fa-ship")]
    Ship,
    #[display(fmt = "fa-solid fa-shirt")]
    Shirt,
    #[display(fmt = "fa-solid fa-shoe-prints")]
    ShoePrints,
    #[display(fmt = "fa-solid fa-shop")]
    Shop,
    #[display(fmt = "fa-solid fa-shop-lock")]
    ShopLock,
    #[display(fmt = "fa-solid fa-shop-slash")]
    ShopSlash,
    #[display(fmt = "fa-solid fa-shower")]
    Shower,
    #[display(fmt = "fa-solid fa-shrimp")]
    Shrimp,
    #[display(fmt = "fa-solid fa-shuffle")]
    Shuffle,
    #[display(fmt = "fa-solid fa-shuttle-space")]
    ShuttleSpace,
    #[display(fmt = "fa-solid fa-sign-hanging")]
    SignHanging,
    #[display(fmt = "fa-solid fa-signal")]
    Signal,
    #[display(fmt = "fa-solid fa-signature")]
    Signature,
    #[display(fmt = "fa-solid fa-signs-post")]
    SignsPost,
    #[display(fmt = "fa-solid fa-sim-card")]
    SimCard,
    #[display(fmt = "fa-solid fa-sink")]
    Sink,
    #[display(fmt = "fa-solid fa-sitemap")]
    Sitemap,
    #[display(fmt = "fa-solid fa-skull")]
    Skull,
    #[display(fmt = "fa-solid fa-skull-crossbones")]
    SkullCrossbones,
    #[display(fmt = "fa-solid fa-slash")]
    Slash,
    #[display(fmt = "fa-solid fa-sleigh")]
    Sleigh,
    #[display(fmt = "fa-solid fa-sliders")]
    Sliders,
    #[display(fmt = "fa-solid fa-smog")]
    Smog,
    #[display(fmt = "fa-solid fa-smoking")]
    Smoking,
    #[display(fmt = "fa-solid fa-snowflake")]
    Snowflake,
    #[display(fmt = "fa-solid fa-snowman")]
    Snowman,
    #[display(fmt = "fa-solid fa-snowplow")]
    Snowplow,
    #[display(fmt = "fa-solid fa-soap")]
    Soap,
    #[display(fmt = "fa-solid fa-socks")]
    Socks,
    #[display(fmt = "fa-solid fa-solar-panel")]
    SolarPanel,
    #[display(fmt = "fa-solid fa-sort")]
    Sort,
    #[display(fmt = "fa-solid fa-sort-down")]
    SortDown,
    #[display(fmt = "fa-solid fa-sort-up")]
    SortUp,
    #[display(fmt = "fa-solid fa-spa")]
    Spa,
    #[display(fmt = "fa-solid fa-spaghetti-monster-flying")]
    SpaghettiMonsterFlying,
    #[display(fmt = "fa-solid fa-spell-check")]
    SpellCheck,
    #[display(fmt = "fa-solid fa-spider")]
    Spider,
    #[display(fmt = "fa-solid fa-spinner")]
    Spinner,
    #[display(fmt = "fa-solid fa-splotch")]
    Splotch,
    #[display(fmt = "fa-solid fa-spoon")]
    Spoon,
    #[display(fmt = "fa-solid fa-spray-can")]
    SprayCan,
    #[display(fmt = "fa-solid fa-spray-can-sparkles")]
    SprayCanSparkles,
    #[display(fmt = "fa-solid fa-square")]
    Square,
    #[display(fmt = "fa-solid fa-square-arrow-up-right")]
    SquareArrowUpRight,
    #[display(fmt = "fa-solid fa-square-caret-down")]
    SquareCaretDown,
    #[display(fmt = "fa-solid fa-square-caret-left")]
    SquareCaretLeft,
    #[display(fmt = "fa-solid fa-square-caret-right")]
    SquareCaretRight,
    #[display(fmt = "fa-solid fa-square-caret-up")]
    SquareCaretUp,
    #[display(fmt = "fa-solid fa-square-check")]
    SquareCheck,
    #[display(fmt = "fa-solid fa-square-envelope")]
    SquareEnvelope,
    #[display(fmt = "fa-solid fa-square-full")]
    SquareFull,
    #[display(fmt = "fa-solid fa-square-h")]
    SquareH,
    #[display(fmt = "fa-solid fa-square-minus")]
    SquareMinus,
    #[display(fmt = "fa-solid fa-square-nfi")]
    SquareNfi,
    #[display(fmt = "fa-solid fa-square-parking")]
    SquareParking,
    #[display(fmt = "fa-solid fa-square-pen")]
    SquarePen,
    #[display(fmt = "fa-solid fa-square-person-confined")]
    SquarePersonConfined,
    #[display(fmt = "fa-solid fa-square-phone")]
    SquarePhone,
    #[display(fmt = "fa-solid fa-square-phone-flip")]
    SquarePhoneFlip,
    #[display(fmt = "fa-solid fa-square-plus")]
    SquarePlus,
    #[display(fmt = "fa-solid fa-square-poll-horizontal")]
    SquarePollHorizontal,
    #[display(fmt = "fa-solid fa-square-poll-vertical")]
    SquarePollVertical,
    #[display(fmt = "fa-solid fa-square-root-variable")]
    SquareRootVariable,
    #[display(fmt = "fa-solid fa-square-rss")]
    SquareRss,
    #[display(fmt = "fa-solid fa-square-share-nodes")]
    SquareShareNodes,
    #[display(fmt = "fa-solid fa-square-up-right")]
    SquareUpRight,
    #[display(fmt = "fa-solid fa-square-virus")]
    SquareVirus,
    #[display(fmt = "fa-solid fa-square-xmark")]
    SquareXmark,
    #[display(fmt = "fa-solid fa-staff-aesculapius")]
    StaffAesculapius,
    #[display(fmt = "fa-solid fa-stairs")]
    Stairs,
    #[display(fmt = "fa-solid fa-stamp")]
    Stamp,
    #[display(fmt = "fa-solid fa-star")]
    Star,
    #[display(fmt = "fa-solid fa-star-and-crescent")]
    StarAndCrescent,
    #[display(fmt = "fa-solid fa-star-half")]
    StarHalf,
    #[display(fmt = "fa-solid fa-star-half-stroke")]
    StarHalfStroke,
    #[display(fmt = "fa-solid fa-star-of-david")]
    StarOfDavid,
    #[display(fmt = "fa-solid fa-star-of-life")]
    StarOfLife,
    #[display(fmt = "fa-solid fa-sterling-sign")]
    SterlingSign,
    #[display(fmt = "fa-solid fa-stethoscope")]
    Stethoscope,
    #[display(fmt = "fa-solid fa-stop")]
    Stop,
    #[display(fmt = "fa-solid fa-stopwatch")]
    Stopwatch,
    #[display(fmt = "fa-solid fa-stopwatch-20")]
    Stopwatch20,
    #[display(fmt = "fa-solid fa-store")]
    Store,
    #[display(fmt = "fa-solid fa-store-slash")]
    StoreSlash,
    #[display(fmt = "fa-solid fa-street-view")]
    StreetView,
    #[display(fmt = "fa-solid fa-strikethrough")]
    Strikethrough,
    #[display(fmt = "fa-solid fa-stroopwafel")]
    Stroopwafel,
    #[display(fmt = "fa-solid fa-subscript")]
    Subscript,
    #[display(fmt = "fa-solid fa-suitcase")]
    Suitcase,
    #[display(fmt = "fa-solid fa-suitcase-medical")]
    SuitcaseMedical,
    #[display(fmt = "fa-solid fa-suitcase-rolling")]
    SuitcaseRolling,
    #[display(fmt = "fa-solid fa-sun")]
    Sun,
    #[display(fmt = "fa-solid fa-sun-plant-wilt")]
    SunPlantWilt,
    #[display(fmt = "fa-solid fa-superscript")]
    Superscript,
    #[display(fmt = "fa-solid fa-swatchbook")]
    Swatchbook,
    #[display(fmt = "fa-solid fa-synagogue")]
    Synagogue,
    #[display(fmt = "fa-solid fa-syringe")]
    Syringe,
    #[display(fmt = "fa-solid fa-t")]
    T,
    #[display(fmt = "fa-solid fa-table")]
    Table,
    #[display(fmt = "fa-solid fa-table-cells")]
    TableCells,
    #[display(fmt = "fa-solid fa-table-cells-large")]
    TableCellsLarge,
    #[display(fmt = "fa-solid fa-table-columns")]
    TableColumns,
    #[display(fmt = "fa-solid fa-table-list")]
    TableList,
    #[display(fmt = "fa-solid fa-table-tennis-paddle-ball")]
    TableTennisPaddleBall,
    #[display(fmt = "fa-solid fa-tablet")]
    Tablet,
    #[display(fmt = "fa-solid fa-tablet-button")]
    TabletButton,
    #[display(fmt = "fa-solid fa-tablet-screen-button")]
    TabletScreenButton,
    #[display(fmt = "fa-solid fa-tablets")]
    Tablets,
    #[display(fmt = "fa-solid fa-tachograph-digital")]
    TachographDigital,
    #[display(fmt = "fa-solid fa-tag")]
    Tag,
    #[display(fmt = "fa-solid fa-tags")]
    Tags,
    #[display(fmt = "fa-solid fa-tape")]
    Tape,
    #[display(fmt = "fa-solid fa-tarp")]
    Tarp,
    #[display(fmt = "fa-solid fa-tarp-droplet")]
    TarpDroplet,
    #[display(fmt = "fa-solid fa-taxi")]
    Taxi,
    #[display(fmt = "fa-solid fa-teeth")]
    Teeth,
    #[display(fmt = "fa-solid fa-teeth-open")]
    TeethOpen,
    #[display(fmt = "fa-solid fa-temperature-arrow-down")]
    TemperatureArrowDown,
    #[display(fmt = "fa-solid fa-temperature-arrow-up")]
    TemperatureArrowUp,
    #[display(fmt = "fa-solid fa-temperature-empty")]
    TemperatureEmpty,
    #[display(fmt = "fa-solid fa-temperature-full")]
    TemperatureFull,
    #[display(fmt = "fa-solid fa-temperature-half")]
    TemperatureHalf,
    #[display(fmt = "fa-solid fa-temperature-high")]
    TemperatureHigh,
    #[display(fmt = "fa-solid fa-temperature-low")]
    TemperatureLow,
    #[display(fmt = "fa-solid fa-temperature-quarter")]
    TemperatureQuarter,
    #[display(fmt = "fa-solid fa-temperature-three-quarters")]
    TemperatureThreeQuarters,
    #[display(fmt = "fa-solid fa-tenge-sign")]
    TengeSign,
    #[display(fmt = "fa-solid fa-tent")]
    Tent,
    #[display(fmt = "fa-solid fa-tent-arrow-down-to-line")]
    TentArrowDownToLine,
    #[display(fmt = "fa-solid fa-tent-arrow-left-right")]
    TentArrowLeftRight,
    #[display(fmt = "fa-solid fa-tent-arrow-turn-left")]
    TentArrowTurnLeft,
    #[display(fmt = "fa-solid fa-tent-arrows-down")]
    TentArrowsDown,
    #[display(fmt = "fa-solid fa-tents")]
    Tents,
    #[display(fmt = "fa-solid fa-terminal")]
    Terminal,
    #[display(fmt = "fa-solid fa-text-height")]
    TextHeight,
    #[display(fmt = "fa-solid fa-text-slash")]
    TextSlash,
    #[display(fmt = "fa-solid fa-text-width")]
    TextWidth,
    #[display(fmt = "fa-solid fa-thermometer")]
    Thermometer,
    #[display(fmt = "fa-solid fa-thumbs-down")]
    ThumbsDown,
    #[display(fmt = "fa-solid fa-thumbs-up")]
    ThumbsUp,
    #[display(fmt = "fa-solid fa-thumbtack")]
    Thumbtack,
    #[display(fmt = "fa-solid fa-ticket")]
    Ticket,
    #[display(fmt = "fa-solid fa-ticket-simple")]
    TicketSimple,
    #[display(fmt = "fa-solid fa-timeline")]
    Timeline,
    #[display(fmt = "fa-solid fa-toggle-off")]
    ToggleOff,
    #[display(fmt = "fa-solid fa-toggle-on")]
    ToggleOn,
    #[display(fmt = "fa-solid fa-toilet")]
    Toilet,
    #[display(fmt = "fa-solid fa-toilet-paper")]
    ToiletPaper,
    #[display(fmt = "fa-solid fa-toilet-paper-slash")]
    ToiletPaperSlash,
    #[display(fmt = "fa-solid fa-toilet-portable")]
    ToiletPortable,
    #[display(fmt = "fa-solid fa-toilets-portable")]
    ToiletsPortable,
    #[display(fmt = "fa-solid fa-toolbox")]
    Toolbox,
    #[display(fmt = "fa-solid fa-tooth")]
    Tooth,
    #[display(fmt = "fa-solid fa-torii-gate")]
    ToriiGate,
    #[display(fmt = "fa-solid fa-tornado")]
    Tornado,
    #[display(fmt = "fa-solid fa-tower-broadcast")]
    TowerBroadcast,
    #[display(fmt = "fa-solid fa-tower-cell")]
    TowerCell,
    #[display(fmt = "fa-solid fa-tower-observation")]
    TowerObservation,
    #[display(fmt = "fa-solid fa-tractor")]
    Tractor,
    #[display(fmt = "fa-solid fa-trademark")]
    Trademark,
    #[display(fmt = "fa-solid fa-traffic-light")]
    TrafficLight,
    #[display(fmt = "fa-solid fa-trailer")]
    Trailer,
    #[display(fmt = "fa-solid fa-train")]
    Train,
    #[display(fmt = "fa-solid fa-train-subway")]
    TrainSubway,
    #[display(fmt = "fa-solid fa-train-tram")]
    TrainTram,
    #[display(fmt = "fa-solid fa-transgender")]
    Transgender,
    #[display(fmt = "fa-solid fa-trash")]
    Trash,
    #[display(fmt = "fa-solid fa-trash-arrow-up")]
    TrashArrowUp,
    #[display(fmt = "fa-solid fa-trash-can")]
    TrashCan,
    #[display(fmt = "fa-solid fa-trash-can-arrow-up")]
    TrashCanArrowUp,
    #[display(fmt = "fa-solid fa-tree")]
    Tree,
    #[display(fmt = "fa-solid fa-tree-city")]
    TreeCity,
    #[display(fmt = "fa-solid fa-triangle-exclamation")]
    TriangleExclamation,
    #[display(fmt = "fa-solid fa-trophy")]
    Trophy,
    #[display(fmt = "fa-solid fa-trowel")]
    Trowel,
    #[display(fmt = "fa-solid fa-trowel-bricks")]
    TrowelBricks,
    #[display(fmt = "fa-solid fa-truck")]
    Truck,
    #[display(fmt = "fa-solid fa-truck-arrow-right")]
    TruckArrowRight,
    #[display(fmt = "fa-solid fa-truck-droplet")]
    TruckDroplet,
    #[display(fmt = "fa-solid fa-truck-fast")]
    TruckFast,
    #[display(fmt = "fa-solid fa-truck-field")]
    TruckField,
    #[display(fmt = "fa-solid fa-truck-field-un")]
    TruckFieldUn,
    #[display(fmt = "fa-solid fa-truck-front")]
    TruckFront,
    #[display(fmt = "fa-solid fa-truck-medical")]
    TruckMedical,
    #[display(fmt = "fa-solid fa-truck-monster")]
    TruckMonster,
    #[display(fmt = "fa-solid fa-truck-moving")]
    TruckMoving,
    #[display(fmt = "fa-solid fa-truck-pickup")]
    TruckPickup,
    #[display(fmt = "fa-solid fa-truck-plane")]
    TruckPlane,
    #[display(fmt = "fa-solid fa-truck-ramp-box")]
    TruckRampBox,
    #[display(fmt = "fa-solid fa-tty")]
    Tty,
    #[display(fmt = "fa-solid fa-turkish-lira-sign")]
    TurkishLiraSign,
    #[display(fmt = "fa-solid fa-turn-down")]
    TurnDown,
    #[display(fmt = "fa-solid fa-turn-up")]
    TurnUp,
    #[display(fmt = "fa-solid fa-tv")]
    Tv,
    #[display(fmt = "fa-solid fa-u")]
    U,
    #[display(fmt = "fa-solid fa-umbrella")]
    Umbrella,
    #[display(fmt = "fa-solid fa-umbrella-beach")]
    UmbrellaBeach,
    #[display(fmt = "fa-solid fa-underline")]
    Underline,
    #[display(fmt = "fa-solid fa-universal-access")]
    UniversalAccess,
    #[display(fmt = "fa-solid fa-unlock")]
    Unlock,
    #[display(fmt = "fa-solid fa-unlock-keyhole")]
    UnlockKeyhole,
    #[display(fmt = "fa-solid fa-up-down")]
    UpDown,
    #[display(fmt = "fa-solid fa-up-down-left-right")]
    UpDownLeftRight,
    #[display(fmt = "fa-solid fa-up-long")]
    UpLong,
    #[display(fmt = "fa-solid fa-up-right-and-down-left-from-center")]
    UpRightAndDownLeftFromCenter,
    #[display(fmt = "fa-solid fa-up-right-from-square")]
    UpRightFromSquare,
    #[display(fmt = "fa-solid fa-upload")]
    Upload,
    #[display(fmt = "fa-solid fa-user")]
    User,
    #[display(fmt = "fa-solid fa-user-astronaut")]
    UserAstronaut,
    #[display(fmt = "fa-solid fa-user-check")]
    UserCheck,
    #[display(fmt = "fa-solid fa-user-clock")]
    UserClock,
    #[display(fmt = "fa-solid fa-user-doctor")]
    UserDoctor,
    #[display(fmt = "fa-solid fa-user-gear")]
    UserGear,
    #[display(fmt = "fa-solid fa-user-graduate")]
    UserGraduate,
    #[display(fmt = "fa-solid fa-user-group")]
    UserGroup,
    #[display(fmt = "fa-solid fa-user-injured")]
    UserInjured,
    #[display(fmt = "fa-solid fa-user-large")]
    UserLarge,
    #[display(fmt = "fa-solid fa-user-large-slash")]
    UserLargeSlash,
    #[display(fmt = "fa-solid fa-user-lock")]
    UserLock,
    #[display(fmt = "fa-solid fa-user-minus")]
    UserMinus,
    #[display(fmt = "fa-solid fa-user-ninja")]
    UserNinja,
    #[display(fmt = "fa-solid fa-user-nurse")]
    UserNurse,
    #[display(fmt = "fa-solid fa-user-pen")]
    UserPen,
    #[display(fmt = "fa-solid fa-user-plus")]
    UserPlus,
    #[display(fmt = "fa-solid fa-user-secret")]
    UserSecret,
    #[display(fmt = "fa-solid fa-user-shield")]
    UserShield,
    #[display(fmt = "fa-solid fa-user-slash")]
    UserSlash,
    #[display(fmt = "fa-solid fa-user-tag")]
    UserTag,
    #[display(fmt = "fa-solid fa-user-tie")]
    UserTie,
    #[display(fmt = "fa-solid fa-user-xmark")]
    UserXmark,
    #[display(fmt = "fa-solid fa-users")]
    Users,
    #[display(fmt = "fa-solid fa-users-between-lines")]
    UsersBetweenLines,
    #[display(fmt = "fa-solid fa-users-gear")]
    UsersGear,
    #[display(fmt = "fa-solid fa-users-line")]
    UsersLine,
    #[display(fmt = "fa-solid fa-users-rays")]
    UsersRays,
    #[display(fmt = "fa-solid fa-users-rectangle")]
    UsersRectangle,
    #[display(fmt = "fa-solid fa-users-slash")]
    UsersSlash,
    #[display(fmt = "fa-solid fa-users-viewfinder")]
    UsersViewfinder,
    #[display(fmt = "fa-solid fa-utensils")]
    Utensils,
    #[display(fmt = "fa-solid fa-v")]
    V,
    #[display(fmt = "fa-solid fa-van-shuttle")]
    VanShuttle,
    #[display(fmt = "fa-solid fa-vault")]
    Vault,
    #[display(fmt = "fa-solid fa-vector-square")]
    VectorSquare,
    #[display(fmt = "fa-solid fa-venus")]
    Venus,
    #[display(fmt = "fa-solid fa-venus-double")]
    VenusDouble,
    #[display(fmt = "fa-solid fa-venus-mars")]
    VenusMars,
    #[display(fmt = "fa-solid fa-vest")]
    Vest,
    #[display(fmt = "fa-solid fa-vest-patches")]
    VestPatches,
    #[display(fmt = "fa-solid fa-vial")]
    Vial,
    #[display(fmt = "fa-solid fa-vial-circle-check")]
    VialCircleCheck,
    #[display(fmt = "fa-solid fa-vial-virus")]
    VialVirus,
    #[display(fmt = "fa-solid fa-vials")]
    Vials,
    #[display(fmt = "fa-solid fa-video")]
    Video,
    #[display(fmt = "fa-solid fa-video-slash")]
    VideoSlash,
    #[display(fmt = "fa-solid fa-vihara")]
    Vihara,
    #[display(fmt = "fa-solid fa-virus")]
    Virus,
    #[display(fmt = "fa-solid fa-virus-covid")]
    VirusCovid,
    #[display(fmt = "fa-solid fa-virus-covid-slash")]
    VirusCovidSlash,
    #[display(fmt = "fa-solid fa-virus-slash")]
    VirusSlash,
    #[display(fmt = "fa-solid fa-viruses")]
    Viruses,
    #[display(fmt = "fa-solid fa-voicemail")]
    Voicemail,
    #[display(fmt = "fa-solid fa-volcano")]
    Volcano,
    #[display(fmt = "fa-solid fa-volleyball")]
    Volleyball,
    #[display(fmt = "fa-solid fa-volume-high")]
    VolumeHigh,
    #[display(fmt = "fa-solid fa-volume-low")]
    VolumeLow,
    #[display(fmt = "fa-solid fa-volume-off")]
    VolumeOff,
    #[display(fmt = "fa-solid fa-volume-xmark")]
    VolumeXmark,
    #[display(fmt = "fa-solid fa-vr-cardboard")]
    VrCardboard,
    #[display(fmt = "fa-solid fa-w")]
    W,
    #[display(fmt = "fa-solid fa-walkie-talkie")]
    WalkieTalkie,
    #[display(fmt = "fa-solid fa-wallet")]
    Wallet,
    #[display(fmt = "fa-solid fa-wand-magic")]
    WandMagic,
    #[display(fmt = "fa-solid fa-wand-magic-sparkles")]
    WandMagicSparkles,
    #[display(fmt = "fa-solid fa-wand-sparkles")]
    WandSparkles,
    #[display(fmt = "fa-solid fa-warehouse")]
    Warehouse,
    #[display(fmt = "fa-solid fa-water")]
    Water,
    #[display(fmt = "fa-solid fa-water-ladder")]
    WaterLadder,
    #[display(fmt = "fa-solid fa-wave-square")]
    WaveSquare,
    #[display(fmt = "fa-solid fa-weight-hanging")]
    WeightHanging,
    #[display(fmt = "fa-solid fa-weight-scale")]
    WeightScale,
    #[display(fmt = "fa-solid fa-wheat-awn")]
    WheatAwn,
    #[display(fmt = "fa-solid fa-wheat-awn-circle-exclamation")]
    WheatAwnCircleExclamation,
    #[display(fmt = "fa-solid fa-wheelchair")]
    Wheelchair,
    #[display(fmt = "fa-solid fa-wheelchair-move")]
    WheelchairMove,
    #[display(fmt = "fa-solid fa-whiskey-glass")]
    WhiskeyGlass,
    #[display(fmt = "fa-solid fa-wifi")]
    Wifi,
    #[display(fmt = "fa-solid fa-wind")]
    Wind,
    #[display(fmt = "fa-solid fa-window-maximize")]
    WindowMaximize,
    #[display(fmt = "fa-solid fa-window-minimize")]
    WindowMinimize,
    #[display(fmt = "fa-solid fa-window-restore")]
    WindowRestore,
    #[display(fmt = "fa-solid fa-wine-bottle")]
    WineBottle,
    #[display(fmt = "fa-solid fa-wine-glass")]
    WineGlass,
    #[display(fmt = "fa-solid fa-wine-glass-empty")]
    WineGlassEmpty,
    #[display(fmt = "fa-solid fa-won-sign")]
    WonSign,
    #[display(fmt = "fa-solid fa-worm")]
    Worm,
    #[display(fmt = "fa-solid fa-wrench")]
    Wrench,
    #[display(fmt = "fa-solid fa-x")]
    X,
    #[display(fmt = "fa-solid fa-x-ray")]
    XRay,
    #[display(fmt = "fa-solid fa-xmark")]
    Xmark,
    #[display(fmt = "fa-solid fa-xmarks-lines")]
    XmarksLines,
    #[display(fmt = "fa-solid fa-y")]
    Y,
    #[display(fmt = "fa-solid fa-yen-sign")]
    YenSign,
    #[display(fmt = "fa-solid fa-yin-yang")]
    YinYang,
    #[display(fmt = "fa-solid fa-z")]
    Z,
}

impl yew::html::IntoPropValue<String> for Solid {
    fn into_prop_value(self) -> String {
        self.to_string()
    }
}

#[derive(Clone, Copy, derive_more::Display)]
pub enum Brands {
    #[display(fmt = "fa-brands fa-accessible-icon")]
    AccessibleIcon,
    #[display(fmt = "fa-brands fa-accusoft")]
    Accusoft,
    #[display(fmt = "fa-brands fa-adn")]
    Adn,
    #[display(fmt = "fa-brands fa-adversal")]
    Adversal,
    #[display(fmt = "fa-brands fa-affiliatetheme")]
    Affiliatetheme,
    #[display(fmt = "fa-brands fa-airbnb")]
    Airbnb,
    #[display(fmt = "fa-brands fa-algolia")]
    Algolia,
    #[display(fmt = "fa-brands fa-alipay")]
    Alipay,
    #[display(fmt = "fa-brands fa-amazon")]
    Amazon,
    #[display(fmt = "fa-brands fa-amazon-pay")]
    AmazonPay,
    #[display(fmt = "fa-brands fa-amilia")]
    Amilia,
    #[display(fmt = "fa-brands fa-android")]
    Android,
    #[display(fmt = "fa-brands fa-angellist")]
    Angellist,
    #[display(fmt = "fa-brands fa-angrycreative")]
    Angrycreative,
    #[display(fmt = "fa-brands fa-angular")]
    Angular,
    #[display(fmt = "fa-brands fa-app-store")]
    AppStore,
    #[display(fmt = "fa-brands fa-app-store-ios")]
    AppStoreIos,
    #[display(fmt = "fa-brands fa-apper")]
    Apper,
    #[display(fmt = "fa-brands fa-apple")]
    Apple,
    #[display(fmt = "fa-brands fa-apple-pay")]
    ApplePay,
    #[display(fmt = "fa-brands fa-artstation")]
    Artstation,
    #[display(fmt = "fa-brands fa-asymmetrik")]
    Asymmetrik,
    #[display(fmt = "fa-brands fa-atlassian")]
    Atlassian,
    #[display(fmt = "fa-brands fa-audible")]
    Audible,
    #[display(fmt = "fa-brands fa-autoprefixer")]
    Autoprefixer,
    #[display(fmt = "fa-brands fa-avianex")]
    Avianex,
    #[display(fmt = "fa-brands fa-aviato")]
    Aviato,
    #[display(fmt = "fa-brands fa-aws")]
    Aws,
    #[display(fmt = "fa-brands fa-bandcamp")]
    Bandcamp,
    #[display(fmt = "fa-brands fa-battle-net")]
    BattleNet,
    #[display(fmt = "fa-brands fa-behance")]
    Behance,
    #[display(fmt = "fa-brands fa-behance-square")]
    BehanceSquare,
    #[display(fmt = "fa-brands fa-bilibili")]
    Bilibili,
    #[display(fmt = "fa-brands fa-bimobject")]
    Bimobject,
    #[display(fmt = "fa-brands fa-bitbucket")]
    Bitbucket,
    #[display(fmt = "fa-brands fa-bitcoin")]
    Bitcoin,
    #[display(fmt = "fa-brands fa-bity")]
    Bity,
    #[display(fmt = "fa-brands fa-black-tie")]
    BlackTie,
    #[display(fmt = "fa-brands fa-blackberry")]
    Blackberry,
    #[display(fmt = "fa-brands fa-blogger")]
    Blogger,
    #[display(fmt = "fa-brands fa-blogger-b")]
    BloggerB,
    #[display(fmt = "fa-brands fa-bluetooth")]
    Bluetooth,
    #[display(fmt = "fa-brands fa-bluetooth-b")]
    BluetoothB,
    #[display(fmt = "fa-brands fa-bootstrap")]
    Bootstrap,
    #[display(fmt = "fa-brands fa-bots")]
    Bots,
    #[display(fmt = "fa-brands fa-btc")]
    Btc,
    #[display(fmt = "fa-brands fa-buffer")]
    Buffer,
    #[display(fmt = "fa-brands fa-buromobelexperte")]
    Buromobelexperte,
    #[display(fmt = "fa-brands fa-buy-n-large")]
    BuyNLarge,
    #[display(fmt = "fa-brands fa-buysellads")]
    Buysellads,
    #[display(fmt = "fa-brands fa-canadian-maple-leaf")]
    CanadianMapleLeaf,
    #[display(fmt = "fa-brands fa-cc-amazon-pay")]
    CcAmazonPay,
    #[display(fmt = "fa-brands fa-cc-amex")]
    CcAmex,
    #[display(fmt = "fa-brands fa-cc-apple-pay")]
    CcApplePay,
    #[display(fmt = "fa-brands fa-cc-diners-club")]
    CcDinersClub,
    #[display(fmt = "fa-brands fa-cc-discover")]
    CcDiscover,
    #[display(fmt = "fa-brands fa-cc-jcb")]
    CcJcb,
    #[display(fmt = "fa-brands fa-cc-mastercard")]
    CcMastercard,
    #[display(fmt = "fa-brands fa-cc-paypal")]
    CcPaypal,
    #[display(fmt = "fa-brands fa-cc-stripe")]
    CcStripe,
    #[display(fmt = "fa-brands fa-cc-visa")]
    CcVisa,
    #[display(fmt = "fa-brands fa-centercode")]
    Centercode,
    #[display(fmt = "fa-brands fa-centos")]
    Centos,
    #[display(fmt = "fa-brands fa-chrome")]
    Chrome,
    #[display(fmt = "fa-brands fa-chromecast")]
    Chromecast,
    #[display(fmt = "fa-brands fa-cloudflare")]
    Cloudflare,
    #[display(fmt = "fa-brands fa-cloudscale")]
    Cloudscale,
    #[display(fmt = "fa-brands fa-cloudsmith")]
    Cloudsmith,
    #[display(fmt = "fa-brands fa-cloudversify")]
    Cloudversify,
    #[display(fmt = "fa-brands fa-cmplid")]
    Cmplid,
    #[display(fmt = "fa-brands fa-codepen")]
    Codepen,
    #[display(fmt = "fa-brands fa-codiepie")]
    Codiepie,
    #[display(fmt = "fa-brands fa-confluence")]
    Confluence,
    #[display(fmt = "fa-brands fa-connectdevelop")]
    Connectdevelop,
    #[display(fmt = "fa-brands fa-contao")]
    Contao,
    #[display(fmt = "fa-brands fa-cotton-bureau")]
    CottonBureau,
    #[display(fmt = "fa-brands fa-cpanel")]
    Cpanel,
    #[display(fmt = "fa-brands fa-creative-commons")]
    CreativeCommons,
    #[display(fmt = "fa-brands fa-creative-commons-by")]
    CreativeCommonsBy,
    #[display(fmt = "fa-brands fa-creative-commons-nc")]
    CreativeCommonsNc,
    #[display(fmt = "fa-brands fa-creative-commons-nc-eu")]
    CreativeCommonsNcEu,
    #[display(fmt = "fa-brands fa-creative-commons-nc-jp")]
    CreativeCommonsNcJp,
    #[display(fmt = "fa-brands fa-creative-commons-nd")]
    CreativeCommonsNd,
    #[display(fmt = "fa-brands fa-creative-commons-pd")]
    CreativeCommonsPd,
    #[display(fmt = "fa-brands fa-creative-commons-pd-alt")]
    CreativeCommonsPdAlt,
    #[display(fmt = "fa-brands fa-creative-commons-remix")]
    CreativeCommonsRemix,
    #[display(fmt = "fa-brands fa-creative-commons-sa")]
    CreativeCommonsSa,
    #[display(fmt = "fa-brands fa-creative-commons-sampling")]
    CreativeCommonsSampling,
    #[display(fmt = "fa-brands fa-creative-commons-sampling-plus")]
    CreativeCommonsSamplingPlus,
    #[display(fmt = "fa-brands fa-creative-commons-share")]
    CreativeCommonsShare,
    #[display(fmt = "fa-brands fa-creative-commons-zero")]
    CreativeCommonsZero,
    #[display(fmt = "fa-brands fa-critical-role")]
    CriticalRole,
    #[display(fmt = "fa-brands fa-css3")]
    Css3,
    #[display(fmt = "fa-brands fa-css3-alt")]
    Css3Alt,
    #[display(fmt = "fa-brands fa-cuttlefish")]
    Cuttlefish,
    #[display(fmt = "fa-brands fa-d-and-d")]
    DAndD,
    #[display(fmt = "fa-brands fa-d-and-d-beyond")]
    DAndDBeyond,
    #[display(fmt = "fa-brands fa-dailymotion")]
    Dailymotion,
    #[display(fmt = "fa-brands fa-dashcube")]
    Dashcube,
    #[display(fmt = "fa-brands fa-deezer")]
    Deezer,
    #[display(fmt = "fa-brands fa-delicious")]
    Delicious,
    #[display(fmt = "fa-brands fa-deploydog")]
    Deploydog,
    #[display(fmt = "fa-brands fa-deskpro")]
    Deskpro,
    #[display(fmt = "fa-brands fa-dev")]
    Dev,
    #[display(fmt = "fa-brands fa-deviantart")]
    Deviantart,
    #[display(fmt = "fa-brands fa-dhl")]
    Dhl,
    #[display(fmt = "fa-brands fa-diaspora")]
    Diaspora,
    #[display(fmt = "fa-brands fa-digg")]
    Digg,
    #[display(fmt = "fa-brands fa-digital-ocean")]
    DigitalOcean,
    #[display(fmt = "fa-brands fa-discord")]
    Discord,
    #[display(fmt = "fa-brands fa-discourse")]
    Discourse,
    #[display(fmt = "fa-brands fa-dochub")]
    Dochub,
    #[display(fmt = "fa-brands fa-docker")]
    Docker,
    #[display(fmt = "fa-brands fa-draft2digital")]
    Draft2Digital,
    #[display(fmt = "fa-brands fa-dribbble")]
    Dribbble,
    #[display(fmt = "fa-brands fa-dribbble-square")]
    DribbbleSquare,
    #[display(fmt = "fa-brands fa-dropbox")]
    Dropbox,
    #[display(fmt = "fa-brands fa-drupal")]
    Drupal,
    #[display(fmt = "fa-brands fa-dyalog")]
    Dyalog,
    #[display(fmt = "fa-brands fa-earlybirds")]
    Earlybirds,
    #[display(fmt = "fa-brands fa-ebay")]
    Ebay,
    #[display(fmt = "fa-brands fa-edge")]
    Edge,
    #[display(fmt = "fa-brands fa-edge-legacy")]
    EdgeLegacy,
    #[display(fmt = "fa-brands fa-elementor")]
    Elementor,
    #[display(fmt = "fa-brands fa-ello")]
    Ello,
    #[display(fmt = "fa-brands fa-ember")]
    Ember,
    #[display(fmt = "fa-brands fa-empire")]
    Empire,
    #[display(fmt = "fa-brands fa-envira")]
    Envira,
    #[display(fmt = "fa-brands fa-erlang")]
    Erlang,
    #[display(fmt = "fa-brands fa-ethereum")]
    Ethereum,
    #[display(fmt = "fa-brands fa-etsy")]
    Etsy,
    #[display(fmt = "fa-brands fa-evernote")]
    Evernote,
    #[display(fmt = "fa-brands fa-expeditedssl")]
    Expeditedssl,
    #[display(fmt = "fa-brands fa-facebook")]
    Facebook,
    #[display(fmt = "fa-brands fa-facebook-f")]
    FacebookF,
    #[display(fmt = "fa-brands fa-facebook-messenger")]
    FacebookMessenger,
    #[display(fmt = "fa-brands fa-facebook-square")]
    FacebookSquare,
    #[display(fmt = "fa-brands fa-fantasy-flight-games")]
    FantasyFlightGames,
    #[display(fmt = "fa-brands fa-fedex")]
    Fedex,
    #[display(fmt = "fa-brands fa-fedora")]
    Fedora,
    #[display(fmt = "fa-brands fa-figma")]
    Figma,
    #[display(fmt = "fa-brands fa-firefox")]
    Firefox,
    #[display(fmt = "fa-brands fa-firefox-browser")]
    FirefoxBrowser,
    #[display(fmt = "fa-brands fa-first-order")]
    FirstOrder,
    #[display(fmt = "fa-brands fa-first-order-alt")]
    FirstOrderAlt,
    #[display(fmt = "fa-brands fa-firstdraft")]
    Firstdraft,
    #[display(fmt = "fa-brands fa-flickr")]
    Flickr,
    #[display(fmt = "fa-brands fa-flipboard")]
    Flipboard,
    #[display(fmt = "fa-brands fa-fly")]
    Fly,
    #[display(fmt = "fa-brands fa-font-awesome")]
    FontAwesome,
    #[display(fmt = "fa-brands fa-fonticons")]
    Fonticons,
    #[display(fmt = "fa-brands fa-fonticons-fi")]
    FonticonsFi,
    #[display(fmt = "fa-brands fa-fort-awesome")]
    FortAwesome,
    #[display(fmt = "fa-brands fa-fort-awesome-alt")]
    FortAwesomeAlt,
    #[display(fmt = "fa-brands fa-forumbee")]
    Forumbee,
    #[display(fmt = "fa-brands fa-foursquare")]
    Foursquare,
    #[display(fmt = "fa-brands fa-free-code-camp")]
    FreeCodeCamp,
    #[display(fmt = "fa-brands fa-freebsd")]
    Freebsd,
    #[display(fmt = "fa-brands fa-fulcrum")]
    Fulcrum,
    #[display(fmt = "fa-brands fa-galactic-republic")]
    GalacticRepublic,
    #[display(fmt = "fa-brands fa-galactic-senate")]
    GalacticSenate,
    #[display(fmt = "fa-brands fa-get-pocket")]
    GetPocket,
    #[display(fmt = "fa-brands fa-gg")]
    Gg,
    #[display(fmt = "fa-brands fa-gg-circle")]
    GgCircle,
    #[display(fmt = "fa-brands fa-git")]
    Git,
    #[display(fmt = "fa-brands fa-git-alt")]
    GitAlt,
    #[display(fmt = "fa-brands fa-git-square")]
    GitSquare,
    #[display(fmt = "fa-brands fa-github")]
    Github,
    #[display(fmt = "fa-brands fa-github-alt")]
    GithubAlt,
    #[display(fmt = "fa-brands fa-github-square")]
    GithubSquare,
    #[display(fmt = "fa-brands fa-gitkraken")]
    Gitkraken,
    #[display(fmt = "fa-brands fa-gitlab")]
    Gitlab,
    #[display(fmt = "fa-brands fa-gitter")]
    Gitter,
    #[display(fmt = "fa-brands fa-glide")]
    Glide,
    #[display(fmt = "fa-brands fa-glide-g")]
    GlideG,
    #[display(fmt = "fa-brands fa-gofore")]
    Gofore,
    #[display(fmt = "fa-brands fa-golang")]
    Golang,
    #[display(fmt = "fa-brands fa-goodreads")]
    Goodreads,
    #[display(fmt = "fa-brands fa-goodreads-g")]
    GoodreadsG,
    #[display(fmt = "fa-brands fa-google")]
    Google,
    #[display(fmt = "fa-brands fa-google-drive")]
    GoogleDrive,
    #[display(fmt = "fa-brands fa-google-pay")]
    GooglePay,
    #[display(fmt = "fa-brands fa-google-play")]
    GooglePlay,
    #[display(fmt = "fa-brands fa-google-plus")]
    GooglePlus,
    #[display(fmt = "fa-brands fa-google-plus-g")]
    GooglePlusG,
    #[display(fmt = "fa-brands fa-google-plus-square")]
    GooglePlusSquare,
    #[display(fmt = "fa-brands fa-google-wallet")]
    GoogleWallet,
    #[display(fmt = "fa-brands fa-gratipay")]
    Gratipay,
    #[display(fmt = "fa-brands fa-grav")]
    Grav,
    #[display(fmt = "fa-brands fa-gripfire")]
    Gripfire,
    #[display(fmt = "fa-brands fa-grunt")]
    Grunt,
    #[display(fmt = "fa-brands fa-guilded")]
    Guilded,
    #[display(fmt = "fa-brands fa-gulp")]
    Gulp,
    #[display(fmt = "fa-brands fa-hacker-news")]
    HackerNews,
    #[display(fmt = "fa-brands fa-hacker-news-square")]
    HackerNewsSquare,
    #[display(fmt = "fa-brands fa-hackerrank")]
    Hackerrank,
    #[display(fmt = "fa-brands fa-hashnode")]
    Hashnode,
    #[display(fmt = "fa-brands fa-hips")]
    Hips,
    #[display(fmt = "fa-brands fa-hire-a-helper")]
    HireAHelper,
    #[display(fmt = "fa-brands fa-hive")]
    Hive,
    #[display(fmt = "fa-brands fa-hooli")]
    Hooli,
    #[display(fmt = "fa-brands fa-hornbill")]
    Hornbill,
    #[display(fmt = "fa-brands fa-hotjar")]
    Hotjar,
    #[display(fmt = "fa-brands fa-houzz")]
    Houzz,
    #[display(fmt = "fa-brands fa-html5")]
    Html5,
    #[display(fmt = "fa-brands fa-hubspot")]
    Hubspot,
    #[display(fmt = "fa-brands fa-ideal")]
    Ideal,
    #[display(fmt = "fa-brands fa-imdb")]
    Imdb,
    #[display(fmt = "fa-brands fa-instagram")]
    Instagram,
    #[display(fmt = "fa-brands fa-instagram-square")]
    InstagramSquare,
    #[display(fmt = "fa-brands fa-instalod")]
    Instalod,
    #[display(fmt = "fa-brands fa-intercom")]
    Intercom,
    #[display(fmt = "fa-brands fa-internet-explorer")]
    InternetExplorer,
    #[display(fmt = "fa-brands fa-invision")]
    Invision,
    #[display(fmt = "fa-brands fa-ioxhost")]
    Ioxhost,
    #[display(fmt = "fa-brands fa-itch-io")]
    ItchIo,
    #[display(fmt = "fa-brands fa-itunes")]
    Itunes,
    #[display(fmt = "fa-brands fa-itunes-note")]
    ItunesNote,
    #[display(fmt = "fa-brands fa-java")]
    Java,
    #[display(fmt = "fa-brands fa-jedi-order")]
    JediOrder,
    #[display(fmt = "fa-brands fa-jenkins")]
    Jenkins,
    #[display(fmt = "fa-brands fa-jira")]
    Jira,
    #[display(fmt = "fa-brands fa-joget")]
    Joget,
    #[display(fmt = "fa-brands fa-joomla")]
    Joomla,
    #[display(fmt = "fa-brands fa-js")]
    Js,
    #[display(fmt = "fa-brands fa-js-square")]
    JsSquare,
    #[display(fmt = "fa-brands fa-jsfiddle")]
    Jsfiddle,
    #[display(fmt = "fa-brands fa-kaggle")]
    Kaggle,
    #[display(fmt = "fa-brands fa-keybase")]
    Keybase,
    #[display(fmt = "fa-brands fa-keycdn")]
    Keycdn,
    #[display(fmt = "fa-brands fa-kickstarter")]
    Kickstarter,
    #[display(fmt = "fa-brands fa-kickstarter-k")]
    KickstarterK,
    #[display(fmt = "fa-brands fa-korvue")]
    Korvue,
    #[display(fmt = "fa-brands fa-laravel")]
    Laravel,
    #[display(fmt = "fa-brands fa-lastfm")]
    Lastfm,
    #[display(fmt = "fa-brands fa-lastfm-square")]
    LastfmSquare,
    #[display(fmt = "fa-brands fa-leanpub")]
    Leanpub,
    #[display(fmt = "fa-brands fa-less")]
    Less,
    #[display(fmt = "fa-brands fa-line")]
    Line,
    #[display(fmt = "fa-brands fa-linkedin")]
    Linkedin,
    #[display(fmt = "fa-brands fa-linkedin-in")]
    LinkedinIn,
    #[display(fmt = "fa-brands fa-linode")]
    Linode,
    #[display(fmt = "fa-brands fa-linux")]
    Linux,
    #[display(fmt = "fa-brands fa-lyft")]
    Lyft,
    #[display(fmt = "fa-brands fa-magento")]
    Magento,
    #[display(fmt = "fa-brands fa-mailchimp")]
    Mailchimp,
    #[display(fmt = "fa-brands fa-mandalorian")]
    Mandalorian,
    #[display(fmt = "fa-brands fa-markdown")]
    Markdown,
    #[display(fmt = "fa-brands fa-mastodon")]
    Mastodon,
    #[display(fmt = "fa-brands fa-maxcdn")]
    Maxcdn,
    #[display(fmt = "fa-brands fa-mdb")]
    Mdb,
    #[display(fmt = "fa-brands fa-medapps")]
    Medapps,
    #[display(fmt = "fa-brands fa-medium")]
    Medium,
    #[display(fmt = "fa-brands fa-medrt")]
    Medrt,
    #[display(fmt = "fa-brands fa-meetup")]
    Meetup,
    #[display(fmt = "fa-brands fa-megaport")]
    Megaport,
    #[display(fmt = "fa-brands fa-mendeley")]
    Mendeley,
    #[display(fmt = "fa-brands fa-microblog")]
    Microblog,
    #[display(fmt = "fa-brands fa-microsoft")]
    Microsoft,
    #[display(fmt = "fa-brands fa-mix")]
    Mix,
    #[display(fmt = "fa-brands fa-mixcloud")]
    Mixcloud,
    #[display(fmt = "fa-brands fa-mixer")]
    Mixer,
    #[display(fmt = "fa-brands fa-mizuni")]
    Mizuni,
    #[display(fmt = "fa-brands fa-modx")]
    Modx,
    #[display(fmt = "fa-brands fa-monero")]
    Monero,
    #[display(fmt = "fa-brands fa-napster")]
    Napster,
    #[display(fmt = "fa-brands fa-neos")]
    Neos,
    #[display(fmt = "fa-brands fa-nfc-directional")]
    NfcDirectional,
    #[display(fmt = "fa-brands fa-nfc-symbol")]
    NfcSymbol,
    #[display(fmt = "fa-brands fa-nimblr")]
    Nimblr,
    #[display(fmt = "fa-brands fa-node")]
    Node,
    #[display(fmt = "fa-brands fa-node-js")]
    NodeJs,
    #[display(fmt = "fa-brands fa-npm")]
    Npm,
    #[display(fmt = "fa-brands fa-ns8")]
    Ns8,
    #[display(fmt = "fa-brands fa-nutritionix")]
    Nutritionix,
    #[display(fmt = "fa-brands fa-octopus-deploy")]
    OctopusDeploy,
    #[display(fmt = "fa-brands fa-odnoklassniki")]
    Odnoklassniki,
    #[display(fmt = "fa-brands fa-odnoklassniki-square")]
    OdnoklassnikiSquare,
    #[display(fmt = "fa-brands fa-old-republic")]
    OldRepublic,
    #[display(fmt = "fa-brands fa-opencart")]
    Opencart,
    #[display(fmt = "fa-brands fa-openid")]
    Openid,
    #[display(fmt = "fa-brands fa-opera")]
    Opera,
    #[display(fmt = "fa-brands fa-optin-monster")]
    OptinMonster,
    #[display(fmt = "fa-brands fa-orcid")]
    Orcid,
    #[display(fmt = "fa-brands fa-osi")]
    Osi,
    #[display(fmt = "fa-brands fa-padlet")]
    Padlet,
    #[display(fmt = "fa-brands fa-page4")]
    Page4,
    #[display(fmt = "fa-brands fa-pagelines")]
    Pagelines,
    #[display(fmt = "fa-brands fa-palfed")]
    Palfed,
    #[display(fmt = "fa-brands fa-patreon")]
    Patreon,
    #[display(fmt = "fa-brands fa-paypal")]
    Paypal,
    #[display(fmt = "fa-brands fa-perbyte")]
    Perbyte,
    #[display(fmt = "fa-brands fa-periscope")]
    Periscope,
    #[display(fmt = "fa-brands fa-phabricator")]
    Phabricator,
    #[display(fmt = "fa-brands fa-phoenix-framework")]
    PhoenixFramework,
    #[display(fmt = "fa-brands fa-phoenix-squadron")]
    PhoenixSquadron,
    #[display(fmt = "fa-brands fa-php")]
    Php,
    #[display(fmt = "fa-brands fa-pied-piper")]
    PiedPiper,
    #[display(fmt = "fa-brands fa-pied-piper-alt")]
    PiedPiperAlt,
    #[display(fmt = "fa-brands fa-pied-piper-hat")]
    PiedPiperHat,
    #[display(fmt = "fa-brands fa-pied-piper-pp")]
    PiedPiperPp,
    #[display(fmt = "fa-brands fa-pied-piper-square")]
    PiedPiperSquare,
    #[display(fmt = "fa-brands fa-pinterest")]
    Pinterest,
    #[display(fmt = "fa-brands fa-pinterest-p")]
    PinterestP,
    #[display(fmt = "fa-brands fa-pinterest-square")]
    PinterestSquare,
    #[display(fmt = "fa-brands fa-pix")]
    Pix,
    #[display(fmt = "fa-brands fa-playstation")]
    Playstation,
    #[display(fmt = "fa-brands fa-product-hunt")]
    ProductHunt,
    #[display(fmt = "fa-brands fa-pushed")]
    Pushed,
    #[display(fmt = "fa-brands fa-python")]
    Python,
    #[display(fmt = "fa-brands fa-qq")]
    Qq,
    #[display(fmt = "fa-brands fa-quinscape")]
    Quinscape,
    #[display(fmt = "fa-brands fa-quora")]
    Quora,
    #[display(fmt = "fa-brands fa-r-project")]
    RProject,
    #[display(fmt = "fa-brands fa-raspberry-pi")]
    RaspberryPi,
    #[display(fmt = "fa-brands fa-ravelry")]
    Ravelry,
    #[display(fmt = "fa-brands fa-react")]
    React,
    #[display(fmt = "fa-brands fa-reacteurope")]
    Reacteurope,
    #[display(fmt = "fa-brands fa-readme")]
    Readme,
    #[display(fmt = "fa-brands fa-rebel")]
    Rebel,
    #[display(fmt = "fa-brands fa-red-river")]
    RedRiver,
    #[display(fmt = "fa-brands fa-reddit")]
    Reddit,
    #[display(fmt = "fa-brands fa-reddit-alien")]
    RedditAlien,
    #[display(fmt = "fa-brands fa-reddit-square")]
    RedditSquare,
    #[display(fmt = "fa-brands fa-redhat")]
    Redhat,
    #[display(fmt = "fa-brands fa-renren")]
    Renren,
    #[display(fmt = "fa-brands fa-replyd")]
    Replyd,
    #[display(fmt = "fa-brands fa-researchgate")]
    Researchgate,
    #[display(fmt = "fa-brands fa-resolving")]
    Resolving,
    #[display(fmt = "fa-brands fa-rev")]
    Rev,
    #[display(fmt = "fa-brands fa-rocketchat")]
    Rocketchat,
    #[display(fmt = "fa-brands fa-rockrms")]
    Rockrms,
    #[display(fmt = "fa-brands fa-rust")]
    Rust,
    #[display(fmt = "fa-brands fa-safari")]
    Safari,
    #[display(fmt = "fa-brands fa-salesforce")]
    Salesforce,
    #[display(fmt = "fa-brands fa-sass")]
    Sass,
    #[display(fmt = "fa-brands fa-schlix")]
    Schlix,
    #[display(fmt = "fa-brands fa-screenpal")]
    Screenpal,
    #[display(fmt = "fa-brands fa-scribd")]
    Scribd,
    #[display(fmt = "fa-brands fa-searchengin")]
    Searchengin,
    #[display(fmt = "fa-brands fa-sellcast")]
    Sellcast,
    #[display(fmt = "fa-brands fa-sellsy")]
    Sellsy,
    #[display(fmt = "fa-brands fa-servicestack")]
    Servicestack,
    #[display(fmt = "fa-brands fa-shirtsinbulk")]
    Shirtsinbulk,
    #[display(fmt = "fa-brands fa-shopify")]
    Shopify,
    #[display(fmt = "fa-brands fa-shopware")]
    Shopware,
    #[display(fmt = "fa-brands fa-simplybuilt")]
    Simplybuilt,
    #[display(fmt = "fa-brands fa-sistrix")]
    Sistrix,
    #[display(fmt = "fa-brands fa-sith")]
    Sith,
    #[display(fmt = "fa-brands fa-sitrox")]
    Sitrox,
    #[display(fmt = "fa-brands fa-sketch")]
    Sketch,
    #[display(fmt = "fa-brands fa-skyatlas")]
    Skyatlas,
    #[display(fmt = "fa-brands fa-skype")]
    Skype,
    #[display(fmt = "fa-brands fa-slack")]
    Slack,
    #[display(fmt = "fa-brands fa-slideshare")]
    Slideshare,
    #[display(fmt = "fa-brands fa-snapchat")]
    Snapchat,
    #[display(fmt = "fa-brands fa-snapchat-square")]
    SnapchatSquare,
    #[display(fmt = "fa-brands fa-soundcloud")]
    Soundcloud,
    #[display(fmt = "fa-brands fa-sourcetree")]
    Sourcetree,
    #[display(fmt = "fa-brands fa-speakap")]
    Speakap,
    #[display(fmt = "fa-brands fa-speaker-deck")]
    SpeakerDeck,
    #[display(fmt = "fa-brands fa-spotify")]
    Spotify,
    #[display(fmt = "fa-brands fa-square-font-awesome")]
    SquareFontAwesome,
    #[display(fmt = "fa-brands fa-square-font-awesome-stroke")]
    SquareFontAwesomeStroke,
    #[display(fmt = "fa-brands fa-squarespace")]
    Squarespace,
    #[display(fmt = "fa-brands fa-stack-exchange")]
    StackExchange,
    #[display(fmt = "fa-brands fa-stack-overflow")]
    StackOverflow,
    #[display(fmt = "fa-brands fa-stackpath")]
    Stackpath,
    #[display(fmt = "fa-brands fa-staylinked")]
    Staylinked,
    #[display(fmt = "fa-brands fa-steam")]
    Steam,
    #[display(fmt = "fa-brands fa-steam-square")]
    SteamSquare,
    #[display(fmt = "fa-brands fa-steam-symbol")]
    SteamSymbol,
    #[display(fmt = "fa-brands fa-sticker-mule")]
    StickerMule,
    #[display(fmt = "fa-brands fa-strava")]
    Strava,
    #[display(fmt = "fa-brands fa-stripe")]
    Stripe,
    #[display(fmt = "fa-brands fa-stripe-s")]
    StripeS,
    #[display(fmt = "fa-brands fa-studiovinari")]
    Studiovinari,
    #[display(fmt = "fa-brands fa-stumbleupon")]
    Stumbleupon,
    #[display(fmt = "fa-brands fa-stumbleupon-circle")]
    StumbleuponCircle,
    #[display(fmt = "fa-brands fa-superpowers")]
    Superpowers,
    #[display(fmt = "fa-brands fa-supple")]
    Supple,
    #[display(fmt = "fa-brands fa-suse")]
    Suse,
    #[display(fmt = "fa-brands fa-swift")]
    Swift,
    #[display(fmt = "fa-brands fa-symfony")]
    Symfony,
    #[display(fmt = "fa-brands fa-teamspeak")]
    Teamspeak,
    #[display(fmt = "fa-brands fa-telegram")]
    Telegram,
    #[display(fmt = "fa-brands fa-tencent-weibo")]
    TencentWeibo,
    #[display(fmt = "fa-brands fa-the-red-yeti")]
    TheRedYeti,
    #[display(fmt = "fa-brands fa-themeco")]
    Themeco,
    #[display(fmt = "fa-brands fa-themeisle")]
    Themeisle,
    #[display(fmt = "fa-brands fa-think-peaks")]
    ThinkPeaks,
    #[display(fmt = "fa-brands fa-tiktok")]
    Tiktok,
    #[display(fmt = "fa-brands fa-trade-federation")]
    TradeFederation,
    #[display(fmt = "fa-brands fa-trello")]
    Trello,
    #[display(fmt = "fa-brands fa-tumblr")]
    Tumblr,
    #[display(fmt = "fa-brands fa-tumblr-square")]
    TumblrSquare,
    #[display(fmt = "fa-brands fa-twitch")]
    Twitch,
    #[display(fmt = "fa-brands fa-twitter")]
    Twitter,
    #[display(fmt = "fa-brands fa-twitter-square")]
    TwitterSquare,
    #[display(fmt = "fa-brands fa-typo3")]
    Typo3,
    #[display(fmt = "fa-brands fa-uber")]
    Uber,
    #[display(fmt = "fa-brands fa-ubuntu")]
    Ubuntu,
    #[display(fmt = "fa-brands fa-uikit")]
    Uikit,
    #[display(fmt = "fa-brands fa-umbraco")]
    Umbraco,
    #[display(fmt = "fa-brands fa-uncharted")]
    Uncharted,
    #[display(fmt = "fa-brands fa-uniregistry")]
    Uniregistry,
    #[display(fmt = "fa-brands fa-unity")]
    Unity,
    #[display(fmt = "fa-brands fa-unsplash")]
    Unsplash,
    #[display(fmt = "fa-brands fa-untappd")]
    Untappd,
    #[display(fmt = "fa-brands fa-ups")]
    Ups,
    #[display(fmt = "fa-brands fa-usb")]
    Usb,
    #[display(fmt = "fa-brands fa-usps")]
    Usps,
    #[display(fmt = "fa-brands fa-ussunnah")]
    Ussunnah,
    #[display(fmt = "fa-brands fa-vaadin")]
    Vaadin,
    #[display(fmt = "fa-brands fa-viacoin")]
    Viacoin,
    #[display(fmt = "fa-brands fa-viadeo")]
    Viadeo,
    #[display(fmt = "fa-brands fa-viadeo-square")]
    ViadeoSquare,
    #[display(fmt = "fa-brands fa-viber")]
    Viber,
    #[display(fmt = "fa-brands fa-vimeo")]
    Vimeo,
    #[display(fmt = "fa-brands fa-vimeo-square")]
    VimeoSquare,
    #[display(fmt = "fa-brands fa-vimeo-v")]
    VimeoV,
    #[display(fmt = "fa-brands fa-vine")]
    Vine,
    #[display(fmt = "fa-brands fa-vk")]
    Vk,
    #[display(fmt = "fa-brands fa-vnv")]
    Vnv,
    #[display(fmt = "fa-brands fa-vuejs")]
    Vuejs,
    #[display(fmt = "fa-brands fa-watchman-monitoring")]
    WatchmanMonitoring,
    #[display(fmt = "fa-brands fa-waze")]
    Waze,
    #[display(fmt = "fa-brands fa-weebly")]
    Weebly,
    #[display(fmt = "fa-brands fa-weibo")]
    Weibo,
    #[display(fmt = "fa-brands fa-weixin")]
    Weixin,
    #[display(fmt = "fa-brands fa-whatsapp")]
    Whatsapp,
    #[display(fmt = "fa-brands fa-whatsapp-square")]
    WhatsappSquare,
    #[display(fmt = "fa-brands fa-whmcs")]
    Whmcs,
    #[display(fmt = "fa-brands fa-wikipedia-w")]
    WikipediaW,
    #[display(fmt = "fa-brands fa-windows")]
    Windows,
    #[display(fmt = "fa-brands fa-wirsindhandwerk")]
    Wirsindhandwerk,
    #[display(fmt = "fa-brands fa-wix")]
    Wix,
    #[display(fmt = "fa-brands fa-wizards-of-the-coast")]
    WizardsOfTheCoast,
    #[display(fmt = "fa-brands fa-wodu")]
    Wodu,
    #[display(fmt = "fa-brands fa-wolf-pack-battalion")]
    WolfPackBattalion,
    #[display(fmt = "fa-brands fa-wordpress")]
    Wordpress,
    #[display(fmt = "fa-brands fa-wordpress-simple")]
    WordpressSimple,
    #[display(fmt = "fa-brands fa-wpbeginner")]
    Wpbeginner,
    #[display(fmt = "fa-brands fa-wpexplorer")]
    Wpexplorer,
    #[display(fmt = "fa-brands fa-wpforms")]
    Wpforms,
    #[display(fmt = "fa-brands fa-wpressr")]
    Wpressr,
    #[display(fmt = "fa-brands fa-xbox")]
    Xbox,
    #[display(fmt = "fa-brands fa-xing")]
    Xing,
    #[display(fmt = "fa-brands fa-xing-square")]
    XingSquare,
    #[display(fmt = "fa-brands fa-y-combinator")]
    YCombinator,
    #[display(fmt = "fa-brands fa-yahoo")]
    Yahoo,
    #[display(fmt = "fa-brands fa-yammer")]
    Yammer,
    #[display(fmt = "fa-brands fa-yandex")]
    Yandex,
    #[display(fmt = "fa-brands fa-yandex-international")]
    YandexInternational,
    #[display(fmt = "fa-brands fa-yarn")]
    Yarn,
    #[display(fmt = "fa-brands fa-yelp")]
    Yelp,
    #[display(fmt = "fa-brands fa-yoast")]
    Yoast,
    #[display(fmt = "fa-brands fa-youtube")]
    Youtube,
    #[display(fmt = "fa-brands fa-youtube-square")]
    YoutubeSquare,
    #[display(fmt = "fa-brands fa-zhihu")]
    Zhihu,
}

impl yew::html::IntoPropValue<String> for Brands {
    fn into_prop_value(self) -> String {
        self.to_string()
    }
}

#[derive(Clone, Copy, derive_more::Display)]
pub enum Regular {
    #[display(fmt = "fa-regular fa-address-book")]
    AddressBook,
    #[display(fmt = "fa-regular fa-address-card")]
    AddressCard,
    #[display(fmt = "fa-regular fa-bell")]
    Bell,
    #[display(fmt = "fa-regular fa-bell-slash")]
    BellSlash,
    #[display(fmt = "fa-regular fa-bookmark")]
    Bookmark,
    #[display(fmt = "fa-regular fa-building")]
    Building,
    #[display(fmt = "fa-regular fa-calendar")]
    Calendar,
    #[display(fmt = "fa-regular fa-calendar-check")]
    CalendarCheck,
    #[display(fmt = "fa-regular fa-calendar-days")]
    CalendarDays,
    #[display(fmt = "fa-regular fa-calendar-minus")]
    CalendarMinus,
    #[display(fmt = "fa-regular fa-calendar-plus")]
    CalendarPlus,
    #[display(fmt = "fa-regular fa-calendar-xmark")]
    CalendarXmark,
    #[display(fmt = "fa-regular fa-chart-bar")]
    ChartBar,
    #[display(fmt = "fa-regular fa-chess-bishop")]
    ChessBishop,
    #[display(fmt = "fa-regular fa-chess-king")]
    ChessKing,
    #[display(fmt = "fa-regular fa-chess-knight")]
    ChessKnight,
    #[display(fmt = "fa-regular fa-chess-pawn")]
    ChessPawn,
    #[display(fmt = "fa-regular fa-chess-queen")]
    ChessQueen,
    #[display(fmt = "fa-regular fa-chess-rook")]
    ChessRook,
    #[display(fmt = "fa-regular fa-circle")]
    Circle,
    #[display(fmt = "fa-regular fa-circle-check")]
    CircleCheck,
    #[display(fmt = "fa-regular fa-circle-dot")]
    CircleDot,
    #[display(fmt = "fa-regular fa-circle-down")]
    CircleDown,
    #[display(fmt = "fa-regular fa-circle-left")]
    CircleLeft,
    #[display(fmt = "fa-regular fa-circle-pause")]
    CirclePause,
    #[display(fmt = "fa-regular fa-circle-play")]
    CirclePlay,
    #[display(fmt = "fa-regular fa-circle-question")]
    CircleQuestion,
    #[display(fmt = "fa-regular fa-circle-right")]
    CircleRight,
    #[display(fmt = "fa-regular fa-circle-stop")]
    CircleStop,
    #[display(fmt = "fa-regular fa-circle-up")]
    CircleUp,
    #[display(fmt = "fa-regular fa-circle-user")]
    CircleUser,
    #[display(fmt = "fa-regular fa-circle-xmark")]
    CircleXmark,
    #[display(fmt = "fa-regular fa-clipboard")]
    Clipboard,
    #[display(fmt = "fa-regular fa-clock")]
    Clock,
    #[display(fmt = "fa-regular fa-clone")]
    Clone,
    #[display(fmt = "fa-regular fa-closed-captioning")]
    ClosedCaptioning,
    #[display(fmt = "fa-regular fa-comment")]
    Comment,
    #[display(fmt = "fa-regular fa-comment-dots")]
    CommentDots,
    #[display(fmt = "fa-regular fa-comments")]
    Comments,
    #[display(fmt = "fa-regular fa-compass")]
    Compass,
    #[display(fmt = "fa-regular fa-copy")]
    Copy,
    #[display(fmt = "fa-regular fa-copyright")]
    Copyright,
    #[display(fmt = "fa-regular fa-credit-card")]
    CreditCard,
    #[display(fmt = "fa-regular fa-envelope")]
    Envelope,
    #[display(fmt = "fa-regular fa-envelope-open")]
    EnvelopeOpen,
    #[display(fmt = "fa-regular fa-eye")]
    Eye,
    #[display(fmt = "fa-regular fa-eye-slash")]
    EyeSlash,
    #[display(fmt = "fa-regular fa-face-angry")]
    FaceAngry,
    #[display(fmt = "fa-regular fa-face-dizzy")]
    FaceDizzy,
    #[display(fmt = "fa-regular fa-face-flushed")]
    FaceFlushed,
    #[display(fmt = "fa-regular fa-face-frown")]
    FaceFrown,
    #[display(fmt = "fa-regular fa-face-frown-open")]
    FaceFrownOpen,
    #[display(fmt = "fa-regular fa-face-grimace")]
    FaceGrimace,
    #[display(fmt = "fa-regular fa-face-grin")]
    FaceGrin,
    #[display(fmt = "fa-regular fa-face-grin-beam")]
    FaceGrinBeam,
    #[display(fmt = "fa-regular fa-face-grin-beam-sweat")]
    FaceGrinBeamSweat,
    #[display(fmt = "fa-regular fa-face-grin-hearts")]
    FaceGrinHearts,
    #[display(fmt = "fa-regular fa-face-grin-squint")]
    FaceGrinSquint,
    #[display(fmt = "fa-regular fa-face-grin-squint-tears")]
    FaceGrinSquintTears,
    #[display(fmt = "fa-regular fa-face-grin-stars")]
    FaceGrinStars,
    #[display(fmt = "fa-regular fa-face-grin-tears")]
    FaceGrinTears,
    #[display(fmt = "fa-regular fa-face-grin-tongue")]
    FaceGrinTongue,
    #[display(fmt = "fa-regular fa-face-grin-tongue-squint")]
    FaceGrinTongueSquint,
    #[display(fmt = "fa-regular fa-face-grin-tongue-wink")]
    FaceGrinTongueWink,
    #[display(fmt = "fa-regular fa-face-grin-wide")]
    FaceGrinWide,
    #[display(fmt = "fa-regular fa-face-grin-wink")]
    FaceGrinWink,
    #[display(fmt = "fa-regular fa-face-kiss")]
    FaceKiss,
    #[display(fmt = "fa-regular fa-face-kiss-beam")]
    FaceKissBeam,
    #[display(fmt = "fa-regular fa-face-kiss-wink-heart")]
    FaceKissWinkHeart,
    #[display(fmt = "fa-regular fa-face-laugh")]
    FaceLaugh,
    #[display(fmt = "fa-regular fa-face-laugh-beam")]
    FaceLaughBeam,
    #[display(fmt = "fa-regular fa-face-laugh-squint")]
    FaceLaughSquint,
    #[display(fmt = "fa-regular fa-face-laugh-wink")]
    FaceLaughWink,
    #[display(fmt = "fa-regular fa-face-meh")]
    FaceMeh,
    #[display(fmt = "fa-regular fa-face-meh-blank")]
    FaceMehBlank,
    #[display(fmt = "fa-regular fa-face-rolling-eyes")]
    FaceRollingEyes,
    #[display(fmt = "fa-regular fa-face-sad-cry")]
    FaceSadCry,
    #[display(fmt = "fa-regular fa-face-sad-tear")]
    FaceSadTear,
    #[display(fmt = "fa-regular fa-face-smile")]
    FaceSmile,
    #[display(fmt = "fa-regular fa-face-smile-beam")]
    FaceSmileBeam,
    #[display(fmt = "fa-regular fa-face-smile-wink")]
    FaceSmileWink,
    #[display(fmt = "fa-regular fa-face-surprise")]
    FaceSurprise,
    #[display(fmt = "fa-regular fa-face-tired")]
    FaceTired,
    #[display(fmt = "fa-regular fa-file")]
    File,
    #[display(fmt = "fa-regular fa-file-audio")]
    FileAudio,
    #[display(fmt = "fa-regular fa-file-code")]
    FileCode,
    #[display(fmt = "fa-regular fa-file-excel")]
    FileExcel,
    #[display(fmt = "fa-regular fa-file-image")]
    FileImage,
    #[display(fmt = "fa-regular fa-file-lines")]
    FileLines,
    #[display(fmt = "fa-regular fa-file-pdf")]
    FilePdf,
    #[display(fmt = "fa-regular fa-file-powerpoint")]
    FilePowerpoint,
    #[display(fmt = "fa-regular fa-file-video")]
    FileVideo,
    #[display(fmt = "fa-regular fa-file-word")]
    FileWord,
    #[display(fmt = "fa-regular fa-file-zipper")]
    FileZipper,
    #[display(fmt = "fa-regular fa-flag")]
    Flag,
    #[display(fmt = "fa-regular fa-floppy-disk")]
    FloppyDisk,
    #[display(fmt = "fa-regular fa-folder")]
    Folder,
    #[display(fmt = "fa-regular fa-folder-closed")]
    FolderClosed,
    #[display(fmt = "fa-regular fa-folder-open")]
    FolderOpen,
    #[display(fmt = "fa-regular fa-font-awesome")]
    FontAwesome,
    #[display(fmt = "fa-regular fa-futbol")]
    Futbol,
    #[display(fmt = "fa-regular fa-gem")]
    Gem,
    #[display(fmt = "fa-regular fa-hand")]
    Hand,
    #[display(fmt = "fa-regular fa-hand-back-fist")]
    HandBackFist,
    #[display(fmt = "fa-regular fa-hand-lizard")]
    HandLizard,
    #[display(fmt = "fa-regular fa-hand-peace")]
    HandPeace,
    #[display(fmt = "fa-regular fa-hand-point-down")]
    HandPointDown,
    #[display(fmt = "fa-regular fa-hand-point-left")]
    HandPointLeft,
    #[display(fmt = "fa-regular fa-hand-point-right")]
    HandPointRight,
    #[display(fmt = "fa-regular fa-hand-point-up")]
    HandPointUp,
    #[display(fmt = "fa-regular fa-hand-pointer")]
    HandPointer,
    #[display(fmt = "fa-regular fa-hand-scissors")]
    HandScissors,
    #[display(fmt = "fa-regular fa-hand-spock")]
    HandSpock,
    #[display(fmt = "fa-regular fa-handshake")]
    Handshake,
    #[display(fmt = "fa-regular fa-hard-drive")]
    HardDrive,
    #[display(fmt = "fa-regular fa-heart")]
    Heart,
    #[display(fmt = "fa-regular fa-hospital")]
    Hospital,
    #[display(fmt = "fa-regular fa-hourglass")]
    Hourglass,
    #[display(fmt = "fa-regular fa-id-badge")]
    IdBadge,
    #[display(fmt = "fa-regular fa-id-card")]
    IdCard,
    #[display(fmt = "fa-regular fa-image")]
    Image,
    #[display(fmt = "fa-regular fa-images")]
    Images,
    #[display(fmt = "fa-regular fa-keyboard")]
    Keyboard,
    #[display(fmt = "fa-regular fa-lemon")]
    Lemon,
    #[display(fmt = "fa-regular fa-life-ring")]
    LifeRing,
    #[display(fmt = "fa-regular fa-lightbulb")]
    Lightbulb,
    #[display(fmt = "fa-regular fa-map")]
    Map,
    #[display(fmt = "fa-regular fa-message")]
    Message,
    #[display(fmt = "fa-regular fa-money-bill-1")]
    MoneyBill1,
    #[display(fmt = "fa-regular fa-moon")]
    Moon,
    #[display(fmt = "fa-regular fa-newspaper")]
    Newspaper,
    #[display(fmt = "fa-regular fa-note-sticky")]
    NoteSticky,
    #[display(fmt = "fa-regular fa-object-group")]
    ObjectGroup,
    #[display(fmt = "fa-regular fa-object-ungroup")]
    ObjectUngroup,
    #[display(fmt = "fa-regular fa-paper-plane")]
    PaperPlane,
    #[display(fmt = "fa-regular fa-paste")]
    Paste,
    #[display(fmt = "fa-regular fa-pen-to-square")]
    PenToSquare,
    #[display(fmt = "fa-regular fa-rectangle-list")]
    RectangleList,
    #[display(fmt = "fa-regular fa-rectangle-xmark")]
    RectangleXmark,
    #[display(fmt = "fa-regular fa-registered")]
    Registered,
    #[display(fmt = "fa-regular fa-share-from-square")]
    ShareFromSquare,
    #[display(fmt = "fa-regular fa-snowflake")]
    Snowflake,
    #[display(fmt = "fa-regular fa-square")]
    Square,
    #[display(fmt = "fa-regular fa-square-caret-down")]
    SquareCaretDown,
    #[display(fmt = "fa-regular fa-square-caret-left")]
    SquareCaretLeft,
    #[display(fmt = "fa-regular fa-square-caret-right")]
    SquareCaretRight,
    #[display(fmt = "fa-regular fa-square-caret-up")]
    SquareCaretUp,
    #[display(fmt = "fa-regular fa-square-check")]
    SquareCheck,
    #[display(fmt = "fa-regular fa-square-full")]
    SquareFull,
    #[display(fmt = "fa-regular fa-square-minus")]
    SquareMinus,
    #[display(fmt = "fa-regular fa-square-plus")]
    SquarePlus,
    #[display(fmt = "fa-regular fa-star")]
    Star,
    #[display(fmt = "fa-regular fa-star-half")]
    StarHalf,
    #[display(fmt = "fa-regular fa-star-half-stroke")]
    StarHalfStroke,
    #[display(fmt = "fa-regular fa-sun")]
    Sun,
    #[display(fmt = "fa-regular fa-thumbs-down")]
    ThumbsDown,
    #[display(fmt = "fa-regular fa-thumbs-up")]
    ThumbsUp,
    #[display(fmt = "fa-regular fa-trash-can")]
    TrashCan,
    #[display(fmt = "fa-regular fa-user")]
    User,
    #[display(fmt = "fa-regular fa-window-maximize")]
    WindowMaximize,
    #[display(fmt = "fa-regular fa-window-minimize")]
    WindowMinimize,
    #[display(fmt = "fa-regular fa-window-restore")]
    WindowRestore,
}

impl yew::html::IntoPropValue<String> for Regular {
    fn into_prop_value(self) -> String {
        self.to_string()
    }
}
