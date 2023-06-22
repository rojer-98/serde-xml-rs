mod common;

use common::init_logger;
use log::info;
use serde::ser::*;
use serde::{Deserialize, Serialize, Serializer};
use serde_xml_rs::{from_str, to_string};
use std::fmt::Display;

const NIGHT_TO_DAY_FILTER_TIME: u32 = 5;
const NIGHT_TO_DAY_FILTER_LEVEL: u32 = 4;
const NIGHT_TO_DAY_FILTER_LEVEL_PTZ: u32 = 2;

type FpsValue = i32;
type FocusValue = i32;

pub mod dublicates {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AdvancedMode {
        pub spatial_level: i32,
        pub temporal_level: i32,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum TimeMode {
        #[serde(rename = "ALL")]
        ALL,
        MANUAL,
        #[serde(rename = "NTP")]
        NTP,
        LOCAL,
        SATELLITE,
        TIMECORRECT,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlAddress {
    pub enabled: bool,
    #[serde(rename = "Address")]
    pub address: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTZRs485Para {
    pub baud_rate: i32,
    pub data_bits: i32,
    pub parity_type: String,
    pub stop_bits: i32,
    pub flow_ctrl: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTZChannel {
    pub id: bool,
    pub enabled: bool,
    pub serial_number: Option<i32>,
    #[serde(rename = "videoInputID")]
    pub video_input_id: i32,
    pub pax_max_speed: Option<i32>,
    pub tilt_max_speed: Option<i32>,
    pub preset_speed: Option<i32>,
    pub auto_patrol_speed: Option<i32>,
    pub key_board_control_speed: Option<String>,
    pub control_protocol: Option<String>,
    pub control_address: Option<ControlAddress>,
    #[serde(rename = "defaultPresetID")]
    pub default_preset_id: Option<String>,
    #[serde(rename = "PTZRs485Para")]
    pub ptz_rs_485_para: Option<PTZRs485Para>,
    pub manual_control_speed: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Defog {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoiseReduce2D {
    #[serde(rename = "noiseReduce2DEnable")]
    pub noise_reduce_2d_enable: bool,
    #[serde(rename = "noiseReduce2DLevel")]
    pub noise_reduce_2d_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlipAngle {
    #[serde(rename = "90")]
    _90,
    #[serde(rename = "180")]
    _180,
    #[serde(rename = "270")]
    _270,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImageFlipStyle {
    LEFTRIGHT,
    UPDOWN,
    CENTER,
    AUTO,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageFlip {
    pub enabled: bool,
    #[serde(rename = "ImageFlipStyle")]
    pub image_flip_style: Option<ImageFlipStyle>,
    pub flip_angle: Option<FlipAngle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WDRMode {
    OPEN,
    CLOSE,
    AUTO,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WDR {
    pub mode: WDRMode,
    #[serde(rename = "WDRLevel")]
    pub wdr_level: Option<i32>,
    #[serde(rename = "WDRContrastLevel")]
    pub wdr_contrast_level: Option<i32>,
    #[serde(rename = "WDRLevel1")]
    pub wdr_level1: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BLCMode {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    CENTER,
    #[serde(rename = "MULTI-AREA")]
    MULTIAREA,
    Region,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionCoordinates {
    #[serde(rename = "positionX")]
    pub position_x: i32,
    #[serde(rename = "positionY")]
    pub position_y: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionCoordinatesList {
    #[serde(rename = "RegionCoordinates")]
    pub region_coordinates: Vec<RegionCoordinates>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BLCRegion {
    pub id: i32,
    #[serde(rename = "RegionCoordinatesList")]
    pub region_coordinates_list: Vec<RegionCoordinatesList>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BLCRegionList {
    #[serde(rename = "BLCRegion")]
    pub blc_region: Option<Vec<BLCRegion>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BLC {
    pub enabled: bool,
    #[serde(rename = "BLCMode")]
    pub blc_mode: Option<BLCMode>,
    #[serde(rename = "BLCLevel")]
    pub blc_level: Option<i32>,
    #[serde(rename = "BLCRegionList")]
    pub blc_region_list: Option<BLCRegionList>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdvancedMode {
    pub frame_noise_reduce_level: i32,
    pub inter_frame_noise_reduce_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralMode {
    pub general_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NoiseReduce {
    #[serde(rename = "mode")]
    pub mode: NoiseReduceMode,
    pub general_mode: Option<GeneralMode>,
    pub advanced_mode: Option<AdvancedMode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WhiteBalanceStyle {
    AUTO,
    MANUAL,
    INDOOR,
    OUTDOOR,
    AUTOTRACE,
    ONECE,
    SODIUMLIGHT,
    MERCURYLIGHT,
    AUTO0,
    AUTO1,
    AUTO2,
    FLUORESCENT,
    NATURALLIGHT,
    WARM,
    INCANDESCENT,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WhiteBalance {
    pub white_balance_style: WhiteBalanceStyle,
    #[serde(rename = "whiteBalanceLevel")]
    pub white_balance_level: Option<i32>,
    pub white_blance_red: Option<i32>,
    pub white_blance_blue: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExposureType {
    #[serde(rename = "auto")]
    AUTO,
    #[serde(rename = "IrisFirst")]
    IRISFIRST,
    #[serde(rename = "ShutterFirst")]
    SHUTTERFIRST,
    #[serde(rename = "gainFirst")]
    GAINFIRST,
    #[serde(rename = "manual")]
    MANUAL,
    #[serde(rename = "plris")]
    PLRIS,
    #[serde(rename = "T5280-PQ1")]
    T5280PQ1,
    #[serde(rename = "T5289-PQ1")]
    T5289PQ1,
    #[serde(rename = "T1140-PQ1")]
    T1140PQ1,
    #[serde(rename = "T2712-PQ1")]
    T2712PQ1,
    #[serde(rename = "HV1250P-MPIR")]
    HV1250PMPIR,
    #[serde(rename = "plris-General")]
    PLRISGENERAL,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OverexposeSuppressType {
    AUTO,
    MANUAL,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OverexposeSuppress {
    pub enabled: bool,
    #[serde(rename = "Type")]
    pub ost: Option<OverexposeSuppressType>,
    #[serde(rename = "DistanceLevel")]
    pub distance_level: Option<i32>,
    #[serde(rename = "shortIRDistanceLevel")]
    pub short_ir_distance_level: Option<i32>,
    #[serde(rename = "longIRDistanceLevel")]
    pub long_ir_distance_level: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plris {
    #[serde(rename = "plrisType")]
    pub plris_type: Option<OverexposeSuppressType>,
    #[serde(rename = "IrisLevel")]
    pub iris_level: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlrisGeneral {
    pub iris_level: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaceExposure {
    pub enabled: Option<bool>,
    pub sensitivity: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Exposure {
    pub exposure_type: ExposureType,
    #[serde(rename = "autoIrisLevel")]
    pub auto_iris_level: Option<i32>,
    pub overexpose_suppress: Option<OverexposeSuppress>,
    #[serde(rename = "plris")]
    pub plris: Option<Plris>,
    pub plris_general: Option<PlrisGeneral>,
    #[serde(rename = "exposureLevel")]
    pub exposure_level: Option<i32>,
    #[serde(rename = "faceExposure")]
    pub face_exposure: Option<FaceExposure>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GammaCorrection {
    pub gamma_correction_enabled: bool,
    pub gamma_correction_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Sharpness {
    pub sharpness_mode: Option<OverexposeSuppressType>,
    pub sharpness_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PowerLineFrequencyMode {
    #[serde(rename = "50hz")]
    _50HZ,
    #[serde(rename = "60hz")]
    _60HZ,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerLineFrequency {
    pub power_line_frequency_mode: Option<PowerLineFrequencyMode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageModeType {
    STANDARD,
    INDOOR,
    OUTDOOR,
    #[serde(rename = "dimLight")]
    DIMLIGHT,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recommendation {
    pub brightness_level: Option<i32>,
    pub contrast_level: Option<i32>,
    pub sharpness_level: Option<i32>,
    pub saturation_level: Option<i32>,
    pub hue_level: Option<i32>,
    pub de_noise_level: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageMode {
    #[serde(rename = "type")]
    pub imt: ImageModeType,
    #[serde(rename = "recommendation")]
    pub recommendation: Option<Recommendation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrightEnhance {
    pub bright_enhance_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageModeList {
    pub image_mode: Vec<ImageMode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeRange {
    pub begin_time: String,
    pub end_time: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub schedule_type: ScheduleType,
    #[serde(rename = "TimeRange")]
    pub time_range: TimeRange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScheduleType {
    DAY,
    NIGHT,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ISPModeType {
    AUTO,
    SCHEDULE,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ISPMode {
    #[serde(rename = "mode")]
    pub mode: ISPModeType,
    pub schedule: Option<Schedule>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Shutter {
    pub shutter_level: String,
    #[serde(rename = "maxShutterLevelLimit")]
    pub max_shutter_level_limit: Option<i32>,
    #[serde(rename = "minShutterLevelLimit")]
    pub min_shutter_level_limit: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShutterLevel {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,
    #[serde(rename = "6")]
    _6,
    #[serde(rename = "12")]
    _12,
    #[serde(rename = "25")]
    _25,
    #[serde(rename = "50")]
    _50,
    #[serde(rename = "75")]
    _75,
    #[serde(rename = "100")]
    _100,
    #[serde(rename = "120")]
    _120,
    #[serde(rename = "125")]
    _125,
    #[serde(rename = "150")]
    _150,
    #[serde(rename = "175")]
    _175,
    #[serde(rename = "215")]
    _215,
    #[serde(rename = "225")]
    _225,
    #[serde(rename = "300")]
    _300,
    #[serde(rename = "400")]
    _400,
    #[serde(rename = "425")]
    _425,
    #[serde(rename = "600")]
    _600,
    #[serde(rename = "1000")]
    _1000,
    #[serde(rename = "1250")]
    _1250,
    #[serde(rename = "1750")]
    _1750,
    #[serde(rename = "2500")]
    _2500,
    #[serde(rename = "3500")]
    _3500,
    #[serde(rename = "6000")]
    _6000,
    #[serde(rename = "10000")]
    _10000,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GrayScaleMode {
    #[serde(rename = "indoor")]
    INDOOR,
    #[serde(rename = "outdoor")]
    OUTDOOR,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrayScale {
    #[serde(rename = "grayScaleMode")]
    pub gray_scale_mode: GrayScaleMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub brightness_level: i32,
    pub contrast_level: i32,
    pub saturation_level: i32,
    pub hue_level: Option<i32>,
    pub gray_scale: Option<GrayScale>,
    pub night_mode: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GainWindow {
    pub region_coordinates_list: Option<RegionCoordinatesList>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Gain {
    pub gain_level: i32,
    pub gain_window: Option<GainWindow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageMultishut {
    pub double_shut_enable: bool,
    pub codec_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JPEGParam {
    #[serde(rename = "JPEGSize")]
    pub jpeg_size: Option<i32>,
    #[serde(rename = "MergeJPEGSize")]
    pub merge_jpeg_size: Option<i32>,
    #[serde(rename = "EXIFInformationEnabled")]
    pub exif_information_enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrayRange {
    pub gray_value_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapColor {
    pub brightness_level: i32,
    pub contrast_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapShutter {
    pub snap_shutter_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapWhiteBalance {
    pub white_balance_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapGain {
    pub snap_gain_level: i32,
    pub light_snap_gain_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarWindowEnhancement {
    pub enabled: bool,
    pub brighten_level: i32,
    pub defog_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ITCImageSnap {
    pub snap_color: Option<SnapColor>,
    pub snap_shutter: Option<SnapShutter>,
    pub snap_gain: Option<SnapGain>,
    pub car_window_enhancement: Option<CarWindowEnhancement>,
    pub snap_white_balance: Option<SnapWhiteBalance>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedModeExt {
    pub spatial_level: i32,
    pub temporal_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecordNoiseReduceExt {
    #[serde(rename = "mode")]
    pub mode: NoiseReduceMode,
    pub general_mode: GeneralMode,
    pub advanced_mode: dublicates::AdvancedMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordGain {
    pub gain_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordShutter {
    pub shutter_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordColor {
    pub brightness_level: i32,
    pub contrast_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageRecord {
    pub record_color: Option<RecordColor>,
    pub record_shutter: Option<RecordShutter>,
    pub record_gain: Option<RecordGain>,
    pub record_noise_reduce_ext: Option<RecordNoiseReduceExt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DehazeMode {
    OPEN,
    CLOSE,
    AUTO,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureMode {
    pub mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NoiseReduceMode {
    CLOSE,
    GENERAL,
    ADVANCED,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NoiseReduceExt {
    #[serde(rename = "mode")]
    pub mode: NoiseReduceMode,
    pub general_mode: GeneralMode,
    pub advanced_mode: dublicates::AdvancedMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TempRangeMode {
    AUTOMATIC,
    MANUAL,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TempRange {
    pub mode: Option<TempRangeMode>,
    pub temperature_upper_limit: Option<i32>,
    pub temperature_lower_limit: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dehaze {
    pub dehaze_mode: Option<DehazeMode>,
    pub dehaze_level: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlateBright {
    pub plate_bright_enabled: Option<bool>,
    pub plate_bright_sensitivity: Option<i32>,
    pub correct_factor_enabled: Option<bool>,
    pub correct_factor: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HLC {
    pub enabled: bool,
    #[serde(rename = "HLCLevel")]
    pub hlc_level: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusConfiguration {
    pub focus_style: Option<String>,
    pub focus_limited: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LensInitialization {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DSS {
    pub enabled: bool,
    #[serde(rename = "DSSLevel")]
    pub dss_level: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IrLight {
    pub mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ZoomLimit {
    pub zoom_limit_ratio: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Iris {
    pub iris_level: Option<i32>,
    #[serde(rename = "maxIrisLevelLimit")]
    pub max_iris_level_limit: Option<i32>,
    #[serde(rename = "minIrisLevelLimit")]
    pub min_iris_level_limit: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageFreeze {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proportionalpan {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaserLight {
    pub mode: Option<String>,
    pub brightness_level: Option<i32>,
    pub laserangle: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EIS {
    pub enabled: bool,
}

/*
<?xml version="1.0" encoding="UTF-8"?><ImageChannel version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><id>1</id><enabled>true</enabled><videoInputID>1</videoInputID><ImageFlip version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled></ImageFlip><IrcutFilter version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><IrcutFilterType>day</IrcutFilterType><nightToDayFilterLevel>2</nightToDayFilterLevel></IrcutFilter><Exposure version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><ExposureType>auto</ExposureType><OverexposeSuppress><enabled>false</enabled></OverexposeSuppress></Exposure><powerLineFrequency version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><powerLineFrequencyMode>50hz</powerLineFrequencyMode></powerLineFrequency><PTZ version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>true</enabled></PTZ><FocusConfiguration version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><focusStyle>SEMIAUTOMATIC</focusStyle><focusLimited>600</focusLimited></FocusConfiguration><LensInitialization version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled></LensInitialization><DSS version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled><DSSLevel>*2</DSSLevel></DSS><IrLight version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><mode>auto</mode></IrLight><ZoomLimit version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><ZoomLimitRatio>32</ZoomLimitRatio></ZoomLimit><Iris version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><IrisLevel>160</IrisLevel><maxIrisLevelLimit>100</maxIrisLevelLimit><minIrisLevelLimit>0</minIrisLevelLimit></Iris><CaptureMode version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><mode>close</mode></CaptureMode><ImageFreeze version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled></ImageFreeze><proportionalpan version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>true</enabled></proportionalpan><LaserLight version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><mode>manual</mode><brightnessLevel>0</brightnessLevel><laserangle>0</laserangle></LaserLight><WDR version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><mode>close</mode><WDRLevel>50</WDRLevel></WDR><BLC version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled></BLC><NoiseReduce version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><mode>general</mode><GeneralMode><generalLevel>50</generalLevel></GeneralMode></NoiseReduce><WhiteBalance version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><WhiteBalanceStyle>auto</WhiteBalanceStyle><WhiteBalanceRed>50</WhiteBalanceRed><WhiteBalanceBlue>50</WhiteBalanceBlue></WhiteBalance><Sharpness version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><SharpnessLevel>50</SharpnessLevel></Sharpness><Gain version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><GainLevel>0</GainLevel><GainLimit>94</GainLimit></Gain><Shutter version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><ShutterLevel>1/25</ShutterLevel><maxShutterLevelLimit>1/25</maxShutterLevelLimit><minShutterLevelLimit>1/30000</minShutterLevelLimit></Shutter><Color version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><brightnessLevel>50</brightnessLevel><contrastLevel>50</contrastLevel><saturationLevel>50</saturationLevel></Color><Dehaze version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><DehazeMode>close</DehazeMode></Dehaze><HLC version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled><HLCLevel>0</HLCLevel></HLC><EIS version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled></EIS></ImageChannel>
    */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageChannel {
    pub id: i32,
    pub enabled: bool,
    #[serde(rename = "videoInputID")]
    pub video_input_id: Option<i32>,
    #[serde(rename = "Defog")]
    pub defog: Option<Defog>,
    #[serde(rename = "NoiseReduce2D")]
    pub noise_reduce_2d: Option<NoiseReduce2D>,
    #[serde(rename = "Focusconfiguration")]
    pub focus_configuration: Option<FocusConfiguration>,
    #[serde(rename = "Lensinitialization")]
    pub lens_initialization: Option<LensInitialization>,
    #[serde(rename = "ImageFlip")]
    pub image_flip: Option<ImageFlip>,
    #[serde(rename = "ImageFreeze")]
    pub image_freeze: Option<ImageFreeze>, //unused
    #[serde(rename = "WDR")]
    pub wdr: Option<WDR>,
    #[serde(rename = "BLC")]
    pub blc: Option<BLC>,
    #[serde(rename = "NoiseReduce")]
    pub noise_reduce: Option<NoiseReduce>,
    #[serde(rename = "ImageEnhancement")]
    pub image_enhancement: Option<String>, //unused
    #[serde(rename = "DSS")]
    pub dss: Option<DSS>,
    #[serde(rename = "WhiteBalance")]
    pub white_balance: Option<WhiteBalance>,
    #[serde(rename = "Exposure")]
    pub exposure: Option<Exposure>,
    #[serde(rename = "Sharpness")]
    pub sharpness: Option<Sharpness>,
    pub gamma_correction: Option<GammaCorrection>,
    pub power_line_frequency: Option<PowerLineFrequency>,
    #[serde(rename = "Color")]
    pub color: Option<Color>,
    #[serde(rename = "IrcutFilter")]
    pub ircut_filter: Option<IrcutFilter>,
    #[serde(rename = "ImageModeList")]
    pub image_mode_list: Option<ImageModeList>,
    #[serde(rename = "BrightEnhance")]
    pub bright_enhance: Option<BrightEnhance>,
    #[serde(rename = "ISPMode")]
    pub isp_mode: Option<ISPMode>,
    #[serde(rename = "Shutter")]
    pub shutter: Option<Shutter>,
    #[serde(rename = "Gain")]
    pub gain: Option<Gain>,
    #[serde(rename = "ImageIcrE")]
    pub image_icr_e: Option<ImageIcrE>,
    #[serde(rename = "ImageMultishut")]
    pub image_multi_shut: Option<ImageMultishut>,
    #[serde(rename = "PlateBright")]
    pub plate_bright: Option<PlateBright>,
    #[serde(rename = "JPEGParam")]
    pub jpeg_param: Option<JPEGParam>,
    #[serde(rename = "DarkEnhance")]
    pub dark_enhance: Option<String>, //unused
    #[serde(rename = "Hdr")]
    pub hdr: Option<String>, //unused
    #[serde(rename = "LSE")]
    pub lse: Option<String>, //unused
    #[serde(rename = "MCE")]
    pub mce: Option<String>, //unused
    #[serde(rename = "Svce")]
    pub svce: Option<String>, //unused
    #[serde(rename = "SectionCtrl")]
    pub section_ctrl: Option<String>, //unused
    #[serde(rename = "AutoContrast")]
    pub auto_contrast: Option<String>, //unused
    #[serde(rename = "GrayRange")]
    pub gray_range: Option<GrayRange>,
    #[serde(rename = "LSEDetail")]
    pub lse_detail: Option<String>, //unused
    #[serde(rename = "ITCImageSnap")]
    pub itc_image_snap: Option<ITCImageSnap>,
    #[serde(rename = "ImageRecord")]
    pub image_record: Option<ImageRecord>,
    #[serde(rename = "Scene")]
    pub scene: Option<Scene>, //unused
    #[serde(rename = "EPTZ")]
    pub eptz: Option<String>, //unused
    #[serde(rename = "EIS")]
    pub eis: Option<EIS>,
    #[serde(rename = "HLC")]
    pub hlc: Option<HLC>,
    #[serde(rename = "ZoomLimit")]
    pub zoom_limit: Option<ZoomLimit>,
    #[serde(rename = "corridor")]
    pub corridor: Option<String>, //unused
    #[serde(rename = "Dehaze")]
    pub dehaze: Option<Dehaze>,
    #[serde(rename = "ImageMode")]
    pub image_mode: Option<ImageModeType>,
    #[serde(rename = "enableImageLossDetection")]
    pub enable_image_loss_detection: Option<bool>,
    #[serde(rename = "CaptureMode")]
    pub capture_mode: Option<CaptureMode>,
    #[serde(rename = "IrLight")]
    pub ir_light: Option<IrLight>,
    #[serde(rename = "LensDistortionCorrection")]
    pub lens_distortion_correction: Option<String>, //unused
    #[serde(rename = "ExposureSync")]
    pub exposure_sync: Option<String>, //unused
    #[serde(rename = "BrightnessSuddenChangeSuppression")]
    pub brightness_sudden_change_suppression: Option<String>, //unused
    #[serde(rename = "TempRange")]
    pub temp_range: Option<TempRange>,
    #[serde(rename = "NoiseReduceExt")]
    pub noise_reduce_ext: Option<NoiseReduceExt>,
    #[serde(rename = "PTZ")]
    pub ptz: Option<PTZ>,
    #[serde(rename = "Iris")]
    pub iris: Option<Iris>,
    pub proportionalpan: Option<Proportionalpan>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct PTZ {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Scene {
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct FocusData {
    pub focus: i32,
}

impl From<FocusValue> for FocusData {
    fn from(value: FocusValue) -> Self {
        Self {
            focus: value as i32,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DefaultStatus {
    High,
    Low,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OutputStatus {
    High,
    Low,
    Pulse,
}

#[derive(Debug, Deserialize)]
pub struct SyncSignalOutputList {
    #[serde(rename = "SyncSignalOutput")]
    pub sync_signal_output_list: Vec<SyncSignalOutput>,
}

impl Serialize for SyncSignalOutputList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SyncSignalOutputList", 1)?;
        for e in &self.sync_signal_output_list {
            state.serialize_field("SyncSignalOutput", e)?;
        }
        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncSignalOutput {
    pub id: u8,
    pub output_status: OutputStatus,
    pub video_flash_enable: bool,
    pub detect_brightness_enable: bool,
}

impl From<bool> for SyncSignalOutputList {
    fn from(enabled: bool) -> Self {
        match enabled {
            false => Self::unset_all(),
            _ => Self::set_all(),
        }
    }
}

impl SyncSignalOutputList {
    pub fn unset_all() -> Self {
        let mut sync_signal_output_list = Vec::new();
        for id in 1..=7 {
            sync_signal_output_list.push(SyncSignalOutput::unset(id));
        }

        Self {
            sync_signal_output_list,
        }
    }

    pub fn set_all() -> Self {
        let mut sync_signal_output_list = Vec::new();
        for id in 1..=7 {
            sync_signal_output_list.push(SyncSignalOutput::set(id));
        }

        Self {
            sync_signal_output_list,
        }
    }

    pub fn set_some(lines: Vec<u8>) -> Self {
        let mut sync_signal_output_list = Vec::new();
        for id in lines {
            sync_signal_output_list.push(SyncSignalOutput::set(id));
        }

        Self {
            sync_signal_output_list,
        }
    }

    pub fn unset_some(lines: Vec<u8>) -> Self {
        let mut sync_signal_output_list = Vec::new();
        for id in lines {
            sync_signal_output_list.push(SyncSignalOutput::unset(id));
        }

        Self {
            sync_signal_output_list,
        }
    }
}

impl SyncSignalOutput {
    pub fn set(id: u8) -> Self {
        Self {
            id,
            output_status: OutputStatus::Pulse,
            video_flash_enable: false,
            detect_brightness_enable: false,
        }
    }

    pub fn unset(id: u8) -> Self {
        Self {
            id,
            output_status: OutputStatus::High,
            video_flash_enable: true,
            detect_brightness_enable: true,
        }
    }

    pub fn is_set(&self) -> bool {
        self.output_status == OutputStatus::Pulse
            && self.video_flash_enable == false
            && self.detect_brightness_enable == false
    }

    pub fn is_unset(&self) -> bool {
        !self.is_set()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone, Copy)]
pub enum FirmwareVerison {
    #[serde(rename = "V5.7.3")]
    V573,
    #[serde(rename = "V5.0.2")]
    V502,
    #[serde(rename = "V5.1.4")]
    V514,
    #[serde(rename = "V5.5.820")]
    V55820,
    #[serde(rename = "V5.5.800")]
    V55800,
}

impl Default for FirmwareVerison {
    fn default() -> Self {
        FirmwareVerison::V502
    }
}

#[derive(Debug, Default)]
pub struct SPSettings {
    pub enabled: bool,
}

impl From<bool> for SPSettings {
    fn from(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl From<SPSettings> for bool {
    fn from(ss: SPSettings) -> Self {
        ss.enabled
    }
}

impl From<IrcutFilter> for SPSettings {
    fn from(icr: IrcutFilter) -> Self {
        Self {
            enabled: icr.ircut_filter_type.into(),
        }
    }
}

impl From<SPSettings> for IrcutFilter {
    fn from(ss: SPSettings) -> Self {
        Self {
            ircut_filter_type: ss.enabled.into(),
            night_to_day_filter_level: Some(NIGHT_TO_DAY_FILTER_LEVEL_PTZ),
            night_to_day_filter_time: None,
        }
    }
}

impl From<ImageIcrE> for SPSettings {
    fn from(icr: ImageIcrE) -> Self {
        let enabled = icr.icr_ctrl.manual_mode.unwrap().manual_preset_val.into();

        Self { enabled }
    }
}

impl From<SPSettings> for ImageIcrE {
    fn from(ss: SPSettings) -> Self {
        Self {
            icr_ctrl: ICRCtrl {
                icr_ctrl_mode: ICRCtrlMode::Manual,
                manual_mode: Some(ManualMode {
                    manual_preset_val: ss.enabled.into(),
                }),
                time_mode: None,
                auto_mode: None,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub device_name: String,
    #[serde(rename = "deviceID")]
    pub device_id: String,
    pub device_description: String,
    pub device_location: String,
    pub system_contact: String,
    pub model: String,
    pub serial_number: String,
    pub mac_address: String,
    #[serde(rename = "firmwareVersion")]
    pub firmware_verison: FirmwareVerison,
    pub firmware_released_date: String,
    pub i_beacon_version: Option<String>,
    pub encoder_version: String,
    pub encoder_released_date: String,
    pub boot_version: String,
    pub boot_released_date: String,
    pub hardware_version: String,
    pub device_type: String,
    #[serde(rename = "telecontrolID")]
    pub telecontrol_id: String,
    pub support_beep: String,
    pub support_video_loss: String,
    pub sub_channel_enabled: Option<bool>,
    pub thr_channel_enabled: Option<bool>,
    pub fourth_channel_enabled: Option<bool>,
    pub fifth_channel_enabled: Option<bool>,
    pub transparent_enabled: Option<String>,
    pub customized_info: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ICRCtrlMode {
    Manual,
    Time,
    Auto,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManualMode {
    #[serde(rename = "ManualPresetVal")]
    pub manual_preset_val: IrcutFilterTypes,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ICRCtrl {
    #[serde(rename = "ICRCtrlMode")]
    pub icr_ctrl_mode: ICRCtrlMode,
    #[serde(rename = "ManualMode")]
    pub manual_mode: Option<ManualMode>,
    #[serde(rename = "TimeMode")]
    pub time_mode: Option<TimeMode>,
    #[serde(rename = "AutoMode")]
    pub auto_mode: Option<AutoMode>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageIcrE {
    #[serde(rename = "ICRCtrl")]
    pub icr_ctrl: ICRCtrl,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum IrcutFilterTypes {
    Auto,
    Day,
    Night,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AutoMode {
    #[serde(rename = "ICRAutoSwitch")]
    pub icr_auto_switch: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimeMode {
    pub switch_list: SwitchList,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSwitch {
    pub time_id: i32,
    #[serde(rename = "PresetVal")]
    pub preset_val: IrcutFilterTypes,
    pub start_hour: i32,
    pub start_minute: i32,
    pub end_hour: i32,
    pub end_minute: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwitchList {
    pub time_switch: Vec<TimeSwitch>,
}

impl From<bool> for IrcutFilterTypes {
    fn from(enabled: bool) -> Self {
        match enabled {
            true => IrcutFilterTypes::Night,
            _ => IrcutFilterTypes::Day,
        }
    }
}

impl From<IrcutFilterTypes> for bool {
    fn from(ift: IrcutFilterTypes) -> Self {
        use IrcutFilterTypes::*;

        match ift {
            Night => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IrcutFilter {
    #[serde(rename = "IrcutFilterType")]
    pub ircut_filter_type: IrcutFilterTypes,
    pub night_to_day_filter_level: Option<u32>,
    pub night_to_day_filter_time: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum VideoEncoding {
    #[serde(rename = "H.264")]
    H264,
    #[serde(rename = "H.265")]
    H265,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VideoScanType {
    INTERLACED,
    PROGRESSIVE,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum VideoCodeType {
    MJPEG,
    MPEG4,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VideoQualityControlType {
    CBR,
    VBR,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SVCMode {
    #[serde(rename = "close_svc")]
    CLOSE,
    MANUAL,
    AUTO,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub struct SVC {
    pub enabled: Option<bool>,
    #[serde(rename = "SVCMode")]
    pub svc_mode: Option<SVCMode>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum SVACProfile {
    Baseline,
    Main,
    High,
    Extended,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum H264Profile {
    Baseline,
    Main,
    High,
    Extended,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub enabled: bool,
    #[serde(rename = "videoInputChannelID")]
    pub video_input_channel_id: u8,
    pub video_scan_type: Option<VideoScanType>,
    pub video_codec_type: VideoEncoding,
    pub video_resolution_width: i32,
    pub video_resolution_height: i32,
    pub video_position_x: Option<i32>,
    pub video_position_y: Option<i32>,
    pub video_quality_control_type: Option<String>,
    pub constant_bit_rate: Option<i32>,
    pub key_frame_interval: Option<i32>,
    pub mirror_enabled: Option<bool>,
    pub rotation_degree: Option<i32>,
    pub snap_shot_image_type: Option<String>,
    pub fixed_quality: i32,
    pub vbr_upper_cap: Option<i32>,
    pub max_frame_rate: FpsValue,
    #[serde(rename = "SVC")]
    pub svc: Option<SVC>,
    #[serde(rename = "H264Profile")]
    pub h264_profile: Option<H264Profile>,
    #[serde(rename = "SVACProfile")]
    pub svac_profile: Option<SVACProfile>,
    #[serde(rename = "GovLength")]
    pub gov_length: Option<u32>,
    pub smoothing: Option<u32>,
    #[serde(rename = "SmartCodec")]
    pub smart_codec: Option<SmartCodec>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SmartCodec {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum RtpTransportType {
    #[serde(rename = "RTP/UDP")]
    UDP,
    #[serde(rename = "RTP/TCP")]
    TCP,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unicast {
    pub enabled: bool,
    #[serde(rename = "interfaceID")]
    pub interface_id: Option<String>,
    pub rtp_transport_type: Option<RtpTransportType>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Multicast {
    pub enabled: bool,
    #[serde(rename = "destIPAddress")]
    pub dest_ip_address: Option<String>,
    pub video_dest_port_no: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CertificateType {
    #[serde(rename = "digest")]
    DIGEST,
    #[serde(rename = "digest/basic")]
    BASIC,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Security {
    pub enabled: bool,
    pub certificate_type: Option<CertificateType>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum StreaminTransport {
    RTSP,
    RTP,
    HTTP,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ControlProtocol {
    pub streaming_transport: Vec<StreaminTransport>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ControlProtocolList {
    pub control_protocol: Option<Vec<ControlProtocol>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
    pub rtsp_port_no: u32,
    pub max_packet_size: u32,
    #[serde(rename = "ControlProtocolList")]
    pub control_protocol_list: Option<ControlProtocolList>,
    #[serde(rename = "Unicast")]
    pub unicast: Option<Unicast>,
    #[serde(rename = "Multicast")]
    pub multicast: Option<Multicast>,
    #[serde(rename = "Security")]
    pub security: Option<Security>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamingChannel {
    pub id: u32,
    pub channel_name: String,
    pub enabled: bool,
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    pub time_mode: dublicates::TimeMode,
    pub local_time: String,
    pub time_zone: String,
    pub satellite_interval: Option<i32>,
    pub platform_no: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AddresingFormatType {
    IPADDRESS,
    HOSTNAME,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NTPServer {
    pub id: i32,
    pub addresing_format_type: AddresingFormatType,
    pub host_name: Option<String>,
    pub ip_address: Option<String>,
    pub ip6_address: Option<String>,
    pub port_no: Option<i32>,
    pub synchronize_interval: Option<i32>,
}

#[test]
fn simple_hik() {
    init_logger();

    let s = r##"<?xml version="1.0" encoding="UTF-8"?><ImageChannel version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><id>1</id><enabled>true</enabled><videoInputID>1</videoInputID><ImageFlip version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled></ImageFlip><IrcutFilter version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><IrcutFilterType>day</IrcutFilterType><nightToDayFilterLevel>2</nightToDayFilterLevel></IrcutFilter><Exposure version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><ExposureType>auto</ExposureType><OverexposeSuppress><enabled>false</enabled></OverexposeSuppress></Exposure><powerLineFrequency version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><powerLineFrequencyMode>50hz</powerLineFrequencyMode></powerLineFrequency><PTZ version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>true</enabled></PTZ><FocusConfiguration version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><focusStyle>SEMIAUTOMATIC</focusStyle><focusLimited>600</focusLimited></FocusConfiguration><LensInitialization version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled></LensInitialization><DSS version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled><DSSLevel>*2</DSSLevel></DSS><IrLight version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><mode>auto</mode></IrLight><ZoomLimit version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><ZoomLimitRatio>32</ZoomLimitRatio></ZoomLimit><Iris version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><IrisLevel>160</IrisLevel><maxIrisLevelLimit>100</maxIrisLevelLimit><minIrisLevelLimit>0</minIrisLevelLimit></Iris><CaptureMode version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><mode>close</mode></CaptureMode><ImageFreeze version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled></ImageFreeze><proportionalpan version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>true</enabled></proportionalpan><LaserLight version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><mode>manual</mode><brightnessLevel>0</brightnessLevel><laserangle>0</laserangle></LaserLight><WDR version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><mode>close</mode><WDRLevel>50</WDRLevel></WDR><BLC version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled></BLC><NoiseReduce version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><mode>general</mode><GeneralMode><generalLevel>50</generalLevel></GeneralMode></NoiseReduce><WhiteBalance version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><WhiteBalanceStyle>auto</WhiteBalanceStyle><WhiteBalanceRed>50</WhiteBalanceRed><WhiteBalanceBlue>50</WhiteBalanceBlue></WhiteBalance><Sharpness version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><SharpnessLevel>50</SharpnessLevel></Sharpness><Gain version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><GainLevel>0</GainLevel><GainLimit>94</GainLimit></Gain><Shutter version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><ShutterLevel>1/25</ShutterLevel><maxShutterLevelLimit>1/25</maxShutterLevelLimit><minShutterLevelLimit>1/30000</minShutterLevelLimit></Shutter><Color version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><brightnessLevel>50</brightnessLevel><contrastLevel>50</contrastLevel><saturationLevel>50</saturationLevel></Color><Dehaze version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><DehazeMode>close</DehazeMode></Dehaze><HLC version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled><HLCLevel>0</HLCLevel></HLC><EIS version="2.0" xmlns="http://www.hikvision.com/ver20/XMLSchema"><enabled>false</enabled></EIS></ImageChannel>"##;

    let item: StreamingChannel = from_str(s).unwrap();
    info!("{:?}", item);
    info!("{:?}", to_string(&item).unwrap());

    assert!(false);
}
