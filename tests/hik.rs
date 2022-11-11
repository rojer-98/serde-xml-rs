mod common;

use common::init_logger;
use log::info;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum VideoEncoding {
    #[serde(rename = "H.264")]
    H264,
    #[serde(rename = "H.265")]
    H265,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum VideoResolutionHeight {
    #[serde(rename = "720")]
    _720,
    #[serde(rename = "1080")]
    _1080,
    #[serde(rename = "1520")]
    _1520,
    #[serde(rename = "1800")]
    _1800,
    #[serde(rename = "2160")]
    _2160,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum VideoResolutionWidth {
    #[serde(rename = "1280")]
    _1280,
    #[serde(rename = "1920")]
    _1920,
    #[serde(rename = "2688")]
    _2688,
    #[serde(rename = "3200")]
    _3200,
    #[serde(rename = "3840")]
    _3840,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingChannel {
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub enabled: bool,
    #[serde(rename = "videoInputChannelID")]
    pub video_input_channel_id: u8,
    pub video_codec_type: VideoEncoding,
    pub video_resolution_width: VideoResolutionWidth,
    pub video_resolution_height: VideoResolutionHeight,
    pub video_quality_control_type: String,
    pub video_scan_type: String,
    pub fixed_quality: u32,
    pub vbr_upper_cap: u32,
    pub max_frame_rate: u32,
}

#[test]
fn simple_hik() {
    init_logger();

    let s = r##"
    <StreamingChannel version=\"2.0\" xmlns=\"http://www.hikvision.com/ver20/XMLSchema\">\r\n<id>1</id>\r\n<channelName>Camera 01</channelName>\r\n<enabled>true</enabled>\r\n<Transport>\r\n<maxPacketSize>1000</maxPacketSize>\r\n<ControlProtocolList>\r\n<ControlProtocol>\r\n<streamingTransport>RTSP</streamingTransport>\r\n</ControlProtocol>\r\n<ControlProtocol>\r\n<streamingTransport>HTTP</streamingTransport>\r\n</ControlProtocol>\r\n<ControlProtocol>\r\n<streamingTransport>SHTTP</streamingTransport>\r\n</ControlProtocol>\r\n</ControlProtocolList>\r\n<Unicast>\r\n<enabled>true</enabled>\r\n<rtpTransportType>RTP/TCP</rtpTransportType>\r\n</Unicast>\r\n<Multicast>\r\n<enabled>true</enabled>\r\n<destIPAddress>0.0.0.0</destIPAddress>\r\n<videoDestPortNo>8860</videoDestPortNo>\r\n<audioDestPortNo>8862</audioDestPortNo>\r\n<FecInfo>\r\n<fecRatio>0</fecRatio>\r\n<fecDestPortNo>9860</fecDestPortNo>\r\n</FecInfo>\r\n</Multicast>\r\n<Security>\r\n<enabled>true</enabled>\r\n<certificateType>digest</certificateType>\r\n<SecurityAlgorithm>\r\n<algorithmType>MD5</algorithmType>\r\n</SecurityAlgorithm>\r\n</Security>\r\n</Transport>\r\n<Video>\r\n<enabled>true</enabled>\r\n<videoInputChannelID>1</videoInputChannelID>\r\n<videoCodecType>H.264</videoCodecType>\r\n<videoScanType>progressive</videoScanType>\r\n<videoResolutionWidth>1280</videoResolutionWidth>\r\n<videoResolutionHeight>720</videoResolutionHeight>\r\n<videoQualityControlType>VBR</videoQualityControlType>\r\n<constantBitRate>4000</constantBitRate>\r\n<fixedQuality>100</fixedQuality>\r\n<vbrUpperCap>4000</vbrUpperCap>\r\n<vbrLowerCap>32</vbrLowerCap>\r\n<maxFrameRate>2000</maxFrameRate>\r\n<keyFrameInterval>1500</keyFrameInterval>\r\n<snapShotImageType>JPEG</snapShotImageType>\r\n<H264Profile>Main</H264Profile>\r\n<GovLength>30</GovLength>\r\n<SVC>\r\n<enabled>false</enabled>\r\n</SVC>\r\n<PacketType>PS</PacketType>\r\n<PacketType>RTP</PacketType>\r\n<smoothing>1</smoothing>\r\n<H265Profile>Main</H265Profile>\r\n<SmartCodec>\r\n<enabled>false</enabled>\r\n</SmartCodec>\r\n</Video>\r\n<Audio>\r\n<enabled>false</enabled>\r\n<audioInputChannelID>1</audioInputChannelID>\r\n<audioCompressionType>G.711ulaw</audioCompressionType>\r\n</Audio>\r\n</StreamingChannel>\r\n"
    "##;

    let item: StreamingChannel = from_str(s).unwrap();
    info!("{:?}", item);
    info!("{:?}", to_string(&item).unwrap());

    assert!(false);
}
