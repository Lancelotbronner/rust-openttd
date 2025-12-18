/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */

/** @file window_type.h Types related to windows. */

/**
 * Widget ID.
 * Even though the ID is signed, actual IDs must be non-negative.
 * Negative IDs are used for special cases, like denoting 'no widget'.
 */
pub struct WidgetID(pub i32);

/** An invalid widget index. */
pub const INVALID_WIDGET: WidgetID = WidgetID(-1);

/** Window numbers for GameOptions windows. */
pub enum GameOptionsWindowNumber {
    /// AI settings.
    Ai,
    /// GS settings.
    GS,
    /// About window.
    About,
    /// NewGRF settings.
    NewGRFState,
    /// Game options.
    GameOptions,
}

/** Window numbers for QueryString windows. */
enum QueryStringWindowNumber {
    /// Query string.
    Default,
    /// Query string for signs.
    Sign,
}

/** Window numbers for PopupQuery windows. */
enum ConfirmPopupQueryWindowNumber {
    /// Query popup confirm.
    Default,
    /// Query popup confirm for bootstrap.
    Bootstrap,
}

/** Window numbers for network windows. */
enum NetworkWindowNumber {
    /// Network game window.
    Game,
    /// Network content list.
    ContentList,
    /// Network start server.
    StartServer,
}

/** Window number for network status windows. */
enum NetworkStatusWindowNumber {
    /// Network join status.
    Join,
    /// Network content download status.
    ContentDownload,
}

/**
 * Number to differentiate different windows of the same class. This number generally
 * implicitly passes some information, e.g. the TileIndex or Company associated with
 * the window. To ease this use, the window number is lenient with what it accepts and
 * broad with what it returns.
 *
 * Anything that converts into a number and ConvertibleThroughBase types will be accepted.
 * When it's being used it returns int32_t or any other type when that's specifically
 * requested, e.g. `VehicleType type = window_number` or `GetEngineListHeight(window_number)`
 * in which the returned value will be a `VehicleType`.
 */
pub struct WindowNumber(pub u32);

impl WindowNumber {
    /// AI settings.
    pub const GAME_OPTIONS_AI: WindowNumber = WindowNumber(0);
    /// GS settings.
    pub const GAME_OPTIONS_GS: WindowNumber = WindowNumber(1);
    /// About window.
    pub const GAME_OPTIONS_ABOUT: WindowNumber = WindowNumber(2);
    /// NewGRF settings.
    pub const GAME_OPTIONS_NEWGRF_STATE: WindowNumber = WindowNumber(3);
    /// Game options.
    pub const GAME_OPTIONS_GAME_OPTIONS: WindowNumber = WindowNumber(4);
    /// Game settings.
    pub const GAME_OPTIONS_GAME_SETTINGS: WindowNumber = WindowNumber(5);

    /// Query string.
    pub const QUERY_STRING: WindowNumber = WindowNumber(0);
    /// Query string for signs.
    pub const QUERY_STRING_SIGN: WindowNumber = WindowNumber(1);

    /// Query popup confirm.
    pub const CONFIRM_POPUP_QUERY: WindowNumber = WindowNumber(0);
    /// Query popup confirm for bootstrap.
    pub const CONFIRM_POPUP_QUERY_BOOTSTRAP: WindowNumber = WindowNumber(1);

    /// Network game window.
    pub const NETWORK_WINDOW_GAME: WindowNumber = WindowNumber(0);
    /// Network content list.
    pub const NETWORK_WINDOW_CONTENT_LIST: WindowNumber = WindowNumber(1);
    /// Network start server.
    pub const NETWORK_WINDOW_START: WindowNumber = WindowNumber(2);

    /// Network join status.
    pub const NETWORK_STATUS_WINDOW_JOIN: WindowNumber = WindowNumber(0);
    /// Network content download status.
    pub const NETWORK_STATUS_WINDOW_CONTENT_DOWNLOAD: WindowNumber = WindowNumber(1);
}

/** %Window classes. */
#[repr(u16)]
pub enum WindowClass {
    /**
     * Main window; %Window numbers:
     *   - 0 = #MainWidgets
     */
    MainWindow,

    /**
     * Main toolbar (the long bar at the top); %Window numbers:
     *   - 0 = #ToolbarNormalWidgets
     *   - 0 = #ToolbarEditorWidgets
     */
    MainToolbar,

    /**
     * Statusbar (at the bottom of your screen); %Window numbers:
     *   - 0 = #StatusbarWidgets
     */
    StatusBar,

    /**
     * Build toolbar; %Window numbers:
     *   - #TRANSPORT_RAIL = #RailToolbarWidgets
     *   - #TRANSPORT_AIR = #AirportToolbarWidgets
     *   - #TRANSPORT_WATER = #DockToolbarWidgets
     *   - #TRANSPORT_ROAD = #RoadToolbarWidgets
     */
    BuildToolbar,

    /**
     * Scenario build toolbar; %Window numbers:
     *   - #TRANSPORT_WATER = #DockToolbarWidgets
     *   - #TRANSPORT_ROAD = #RoadToolbarWidgets
     */
    ScenBuildToolbar,

    /**
     * Build trees toolbar; %Window numbers:
     *   - 0 = #BuildTreesWidgets
     */
    BuildTrees,

    /**
     * Transparency toolbar; %Window numbers:
     *   - 0 = #TransparencyToolbarWidgets
     */
    TransparencyToolbar,

    /**
     * Build signal toolbar; %Window numbers:
     *   - #TRANSPORT_RAIL = #BuildSignalWidgets
     */
    BuildSignal,

    /**
     * Small map; %Window numbers:
     *   - 0 = #SmallMapWidgets
     */
    Smallmap,

    /**
     * Error message; %Window numbers:
     *   - 0 = #ErrorMessageWidgets
     */
    Errmsg,

    /**
     * Tooltip window; %Window numbers:
     *   - 0 = #ToolTipsWidgets
     */
    Tooltips,

    /**
     * Query string window; %Window numbers:
     *   - #WN_QUERY_STRING = #QueryStringWidgets
     *   - #WN_QUERY_STRING_SIGN = #QueryEditSignWidgets
     */
    QueryString,

    /**
     * Popup with confirm question; %Window numbers:
     *   - #WN_CONFIRM_POPUP_QUERY = #QueryWidgets
     *   - #WN_CONFIRM_POPUP_QUERY_BOOTSTRAP = #BootstrapAskForDownloadWidgets
     */
    ConfirmPopupQuery,

    /**
     * Popup with a set of buttons, designed to ask the user a question
     *  from a GameScript. %Window numbers:
     *   - uniqueid = #GoalQuestionWidgets
     */
    GoalQuestion,

    /**
     * Saveload window; %Window numbers:
     *   - 0 = #SaveLoadWidgets
     */
    Saveload,

    /**
     * Land info window; %Window numbers:
     *   - 0 = #LandInfoWidgets
     */
    LandInfo,

    /**
     * Drop down menu; %Window numbers:
     *   - 0 = #DropdownMenuWidgets
     */
    DropdownMenu,

    /**
     * On Screen Keyboard; %Window numbers:
     *   - 0 = #OnScreenKeyboardWidgets
     */
    Osk,

    /**
     * Set date; %Window numbers:
     *   - #VehicleID = #SetDateWidgets
     */
    SetDate,

    /**
     * Script settings; %Window numbers:
     *   - 0 = #ScriptSettingsWidgets
     */
    ScriptSettings,

    /**
     * NewGRF parameters; %Window numbers:
     *   - 0 = #NewGRFParametersWidgets
     */
    GrfParameters,

    /**
     * textfile; %Window numbers:
     *   - 0 = #TextfileWidgets
     */
    Textfile,

    /**
     * Town authority; %Window numbers:
     *   - #TownID = #TownAuthorityWidgets
     */
    TownAuthority,

    /**
     * Vehicle details; %Window numbers:
     *   - #VehicleID = #VehicleDetailsWidgets
     */
    VehicleDetails,

    /**
     * Vehicle refit; %Window numbers:
     *   - #VehicleID = #VehicleRefitWidgets
     */
    VehicleRefit,

    /**
     * Vehicle orders; %Window numbers:
     *   - #VehicleID = #OrderWidgets
     */
    VehicleOrders,

    /**
     * Replace vehicle window; %Window numbers:
     *   - #VehicleType = #ReplaceVehicleWidgets
     */
    ReplaceVehicle,

    /**
     * Vehicle timetable; %Window numbers:
     *   - #VehicleID = #VehicleTimetableWidgets
     */
    VehicleTimetable,

    /**
     * Company colour selection; %Window numbers:
     *   - #CompanyID = #SelectCompanyLiveryWidgets
     */
    CompanyColour,

    /**
     * Alter company face window; %Window numbers:
     *   - #CompanyID = #SelectCompanyManagerFaceWidgets
     */
    CompanyManagerFace,

    /**
     * Select station (when joining stations); %Window numbers:
     *   - 0 = #JoinStationWidgets
     */
    SelectStation,

    /**
     * News window; %Window numbers:
     *   - 0 = #NewsWidgets
     */
    NewsWindow,

    /**
     * Town directory; %Window numbers:
     *   - 0 = #TownDirectoryWidgets
     */
    TownDirectory,

    /**
     * Subsidies list; %Window numbers:
     *   - 0 = #SubsidyListWidgets
     */
    SubsidiesList,

    /**
     * Industry directory; %Window numbers:
     *   - 0 = #IndustryDirectoryWidgets
     */
    IndustryDirectory,

    /**
     * News history list; %Window numbers:
     *   - 0 = #MessageHistoryWidgets
     */
    MessageHistory,

    /**
     * Sign list; %Window numbers:
     *   - 0 = #SignListWidgets
     */
    SignList,

    /**
     * Scripts list; %Window numbers:
     *   - 0 = #ScriptListWidgets
     */
    ScriptList,

    /**
     * Goals list; %Window numbers:
     *   - 0 ; #GoalListWidgets
     */
    GoalsList,

    /**
     * Story book; %Window numbers:
     *   - CompanyID = #StoryBookWidgets
     */
    StoryBook,

    /**
     * Station list; %Window numbers:
     *   - #CompanyID = #StationListWidgets
     */
    StationList,

    /**
     * Trains list; %Window numbers:
     *   - Packed value = #GroupListWidgets / #VehicleListWidgets
     */
    TrainsList,

    /**
     * Road vehicle list; %Window numbers:
     *   - Packed value = #GroupListWidgets / #VehicleListWidgets
     */
    RoadvehList,

    /**
     * Ships list; %Window numbers:
     *   - Packed value = #GroupListWidgets / #VehicleListWidgets
     */
    ShipsList,

    /**
     * Aircraft list; %Window numbers:
     *   - Packed value = #GroupListWidgets / #VehicleListWidgets
     */
    AircraftList,

    /**
     * Town view; %Window numbers:
     *   - #TownID = #TownViewWidgets
     */
    TownView,

    /**
     * Vehicle view; %Window numbers:
     *   - #VehicleID = #VehicleViewWidgets
     */
    VehicleView,

    /**
     * Station view; %Window numbers:
     *   - #StationID = #StationViewWidgets
     */
    StationView,

    /**
     * Depot view; %Window numbers:
     *   - #TileIndex = #DepotWidgets
     */
    VehicleDepot,

    /**
     * Waypoint view; %Window numbers:
     *   - #WaypointID = #WaypointWidgets
     */
    WaypointView,

    /**
     * Industry view; %Window numbers:
     *   - #IndustryID = #IndustryViewWidgets
     */
    IndustryView,

    /**
     * Company view; %Window numbers:
     *   - #CompanyID = #CompanyWidgets
     */
    Company,

    /**
     * Build object; %Window numbers:
     *   - 0 = #BuildObjectWidgets
     */
    BuildObject,

    /**
     * Build house; %Window numbers:
     *   - 0 = #BuildHouseWidgets
     */
    BuildHouse,

    /**
     * Build vehicle; %Window numbers:
     *   - #VehicleType = #BuildVehicleWidgets
     *   - #TileIndex = #BuildVehicleWidgets
     */
    BuildVehicle,

    /**
     * Build bridge; %Window numbers:
     *   - #TransportType = #BuildBridgeSelectionWidgets
     */
    BuildBridge,

    /**
     * Build station; %Window numbers:
     *   - #TRANSPORT_AIR = #AirportPickerWidgets
     *   - #TRANSPORT_WATER = #DockToolbarWidgets
     *   - #TRANSPORT_RAIL = #BuildRailStationWidgets
     */
    BuildStation,

    /**
     * Build bus station; %Window numbers:
     *   - #TRANSPORT_ROAD = #BuildRoadStationWidgets
     */
    BusStation,

    /**
     * Build truck station; %Window numbers:
     *   - #TRANSPORT_ROAD = #BuildRoadStationWidgets
     */
    TruckStation,

    /**
     * Build depot; %Window numbers:
     *   - #TRANSPORT_WATER = #BuildDockDepotWidgets
     *   - #TRANSPORT_RAIL = #BuildRailDepotWidgets
     *   - #TRANSPORT_ROAD = #BuildRoadDepotWidgets
     */
    BuildDepot,

    /**
     * Build waypoint; %Window numbers:
     *   - #TRANSPORT_RAIL = #BuildRailWaypointWidgets
     */
    BuildWaypoint,

    /**
     * Found a town; %Window numbers:
     *   - 0 = #TownFoundingWidgets
     */
    FoundTown,

    /**
     * Build industry; %Window numbers:
     *   - 0 = #DynamicPlaceIndustriesWidgets
     */
    BuildIndustry,

    /**
     * Select game window; %Window numbers:
     *   - 0 = #SelectGameIntroWidgets
     */
    SelectGame,

    /**
     * Landscape generation (in Scenario Editor); %Window numbers:
     *   - 0 = #TerraformToolbarWidgets
     *   - 0 = #EditorTerraformToolbarWidgets
     */
    ScenLandGen,

    /**
     * Generate landscape (newgame); %Window numbers:
     *   - GLWM_SCENARIO = #CreateScenarioWidgets
     *   - #GenerateLandscapeWindowMode = #GenerateLandscapeWidgets
     */
    GenerateLandscape,

    /**
     * Progress report of landscape generation; %Window numbers:
     *   - 0 = #GenerationProgressWidgets
     *   - 1 = #ScanProgressWidgets
     */
    ModalProgress,

    /**
     * Network window; %Window numbers:
     *   - #WN_NETWORK_WINDOW_GAME = #NetworkGameWidgets
     *   - #WN_NETWORK_WINDOW_CONTENT_LIST = #NetworkContentListWidgets
     *   - #WN_NETWORK_WINDOW_START = #NetworkStartServerWidgets
     */
    NetworkWindow,

    /**
     * Client list; %Window numbers:
     *   - 0 = #ClientListWidgets
     */
    ClientList,

    /**
     * Network status window; %Window numbers:
     *   - #WN_NETWORK_STATUS_WINDOW_JOIN = #NetworkJoinStatusWidgets
     *   - #WN_NETWORK_STATUS_WINDOW_CONTENT_DOWNLOAD = #NetworkContentDownloadStatusWidgets
     */
    NetworkStatusWindow,

    /**
     * Network ask relay window; %Window numbers:
     *   - 0 - #NetworkAskRelayWidgets
     */
    NetworkAskRelay,

    /**
     * Network ask survey window; %Window numbers:
     *  - 0 - #NetworkAskSurveyWidgets
     */
    NetworkAskSurvey,

    /**
     * Chatbox; %Window numbers:
     *   - #DestType = #NetWorkChatWidgets
     */
    SendNetworkMsg,

    /**
     * Industry cargoes chain; %Window numbers:
     *   - 0 = #IndustryCargoesWidgets
     */
    IndustryCargoes,

    /**
     * Legend for graphs; %Window numbers:
     *   - 0 = #GraphLegendWidgets
     */
    GraphLegend,

    /**
     * Finances of a company; %Window numbers:
     *   - #CompanyID = #CompanyWidgets
     */
    Finances,

    /**
     * Income graph; %Window numbers:
     *   - 0 = #CompanyValueWidgets
     */
    IncomeGraph,

    /**
     * Operating profit graph; %Window numbers:
     *   - 0 = #CompanyValueWidgets
     */
    OperatingProfit,

    /**
     * Delivered cargo graph; %Window numbers:
     *   - 0 = #CompanyValueWidgets
     */
    DeliveredCargo,

    /**
     * Performance history graph; %Window numbers:
     *   - 0 = #PerformanceHistoryGraphWidgets
     */
    PerformanceHistory,

    /**
     * Company value graph; %Window numbers:
     *   - 0 = #CompanyValueWidgets
     */
    CompanyValue,

    /**
     * Company league window; %Window numbers:
     *   - 0 = #CompanyLeagueWidgets
     */
    CompanyLeague,

    /**
     * Payment rates graph; %Window numbers:
     *   - 0 = #CargoPaymentRatesWidgets
     */
    PaymentRates,

    /**
     * Performance detail window; %Window numbers:
     *   - 0 = #PerformanceRatingDetailsWidgets
     */
    PerformanceDetail,

    /**
     * Industry production history graph; %Window numbers:
     *   - #IndustryID = #IndustryProductionGraphWidgets
     */
    IndustryProduction,

    /**
     * Town cargo history graph; %Window numbers:
     *   - #TownID = #GraphWidgets
     */
    TownCargoGraph,

    /**
     * Company infrastructure overview; %Window numbers:
     *   - #CompanyID = #CompanyInfrastructureWidgets
     */
    CompanyInfrastructure,

    /**
     * Buyout company (merger); %Window numbers:
     *   - #CompanyID = #BuyCompanyWidgets
     */
    BuyCompany,

    /**
     * Engine preview window; %Window numbers:
     *   - #EngineID = #EnginePreviewWidgets
     */
    EnginePreview,

    /**
     * Music window; %Window numbers:
     *   - 0 = #MusicWidgets
     */
    MusicWindow,

    /**
     * Music track selection; %Window numbers:
     *   - 0 = MusicTrackSelectionWidgets
     */
    MusicTrackSelection,

    /**
     * Game options window; %Window numbers:
     *   - #WN_GAME_OPTIONS_AI = #AIConfigWidgets
     *   - #WN_GAME_OPTIONS_GS = #GSConfigWidgets
     *   - #WN_GAME_OPTIONS_ABOUT = #AboutWidgets
     *   - #WN_GAME_OPTIONS_NEWGRF_STATE = #NewGRFStateWidgets
     *   - #WN_GAME_OPTIONS_GAME_OPTIONS = #GameOptionsWidgets
     *   - #WN_GAME_OPTIONS_GAME_SETTINGS = #GameSettingsWidgets
     */
    GameOptions,

    /**
     * Custom currency; %Window numbers:
     *   - 0 = #CustomCurrencyWidgets
     */
    CustomCurrency,

    /**
     * Cheat window; %Window numbers:
     *   - 0 = #CheatWidgets
     */
    Cheats,

    /**
     * Extra viewport; %Window numbers:
     *   - Ascending value = #ExtraViewportWidgets
     */
    ExtraViewport,

    /**
     * Console; %Window numbers:
     *   - 0 = #ConsoleWidgets
     */
    Console,

    /**
     * Bootstrap; %Window numbers:
     *   - 0 = #BootstrapBackgroundWidgets
     */
    Bootstrap,

    /**
     * Highscore; %Window numbers:
     *   - 0 = #HighscoreWidgets
     */
    Highscore,

    /**
     * Endscreen; %Window numbers:
     *   - 0 = #HighscoreWidgets
     */
    Endscreen,

    /**
     * Script debug window; %Window numbers:
     *   - Ascending value = #ScriptDebugWidgets
     */
    ScriptDebug,

    /**
     * NewGRF inspect (debug); %Window numbers:
     *   - Packed value = #NewGRFInspectWidgets
     */
    NewgrfInspect,

    /**
     * Sprite aligner (debug); %Window numbers:
     *   - 0 = #SpriteAlignerWidgets
     */
    SpriteAligner,

    /**
     * Linkgraph legend; %Window numbers:
     *   - 0 = #LinkGraphWidgets
     */
    LinkgraphLegend,

    /**
     * Save preset; %Window numbers:
     *   - 0 = #SavePresetWidgets
     */
    SavePreset,

    /**
     * Framerate display; %Window numbers:
     *   - 0 = #FramerateDisplayWidgets
     */
    FramerateDisplay,

    /**
     * Frame time graph; %Window numbers:
     *   - 0 = #FrametimeGraphWindowWidgets
     */
    FrametimeGraph,

    /**
     * Screenshot window; %Window numbers:
     *   - 0 = #ScreenshotWidgets
     */
    Screenshot,

    /*
     * Help and manuals window; %Window numbers:
     *   - 0 = #HelpWindowWidgets
     */
    Help,
}

impl WindowClass {
    /// No window, redirects to MainWindow.
    pub const NONE: WindowClass = WindowClass::MainWindow;
}

/** Data value for #Window::OnInvalidateData() of windows with class #WC_GAME_OPTIONS. */
pub enum GameOptionsInvalidationData {
    Default,
    ///< NewGRFs were just rescanned.
    NewgrfRescanned,
    ///< The current list of active NewGRF has been loaded.
    NewgrfCurrentLoaded,
    ///< List of active NewGRFs is being edited.
    NewgrfListEdited,
    ///< Changes have been made to a given NewGRF either through the palette or its parameters.
    NewgrfChangesMade,
}

/** State of handling an event. */
pub enum EventState {
    ///< The passed event is handled.
    Handled,
    ///< The passed event is not handled.
    NotHandled,
}
