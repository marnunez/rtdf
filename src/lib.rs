use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use deku::{
    bitvec::{BitSlice, BitVec, BitView, Msb0},
    prelude::*,
};
#[derive(Debug)]
pub struct TimeT(DateTime<Local>);

impl<'a> DekuRead<'a> for TimeT {
    fn read(
        input: &'a BitSlice<Msb0, u8>,
        _ctx: (),
    ) -> Result<(&'a BitSlice<Msb0, u8>, Self), DekuError>
    where
        Self: Sized,
    {
        let (rest, time) = u32::read(input, ()).map_err(|e| DekuError::from(e))?;
        let time = Local.from_utc_datetime(&NaiveDateTime::from_timestamp(time as i64, 0));
        Ok((rest, TimeT(time)))
    }
}

impl<'a> DekuWrite for TimeT {
    fn write(&self, output: &mut BitVec<Msb0, u8>, _ctx: ()) -> Result<(), DekuError> {
        Ok(BitVec::extend(
            output,
            (self.0.timestamp() as u32)
                .to_le_bytes()
                .view_bits::<Msb0>(),
        ))
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
// #[deku(magic = b"\x41\x60\x4b\x82")]
#[deku(magic = b"\x82\x4b\x60\x41\xd3\x11\x84\xca\x60\x00\xb6\xac\x16\x68\x0c\x08")]
pub struct TdfHeaderInfo {
    version: u32,
    #[deku(pad_bytes_after = "8")]
    n_entries: i32,
    creation_date: TimeT,
    last_modification_date: TimeT,
    #[deku(pad_bytes_after = "20")]
    last_access_date: TimeT,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct TdfJumptableEntry {
    block_type: TdfBlockType,
    #[deku(ctx = "*block_type")]
    format: TdfFormat,
    offset: i32,
    size: i32,
    creation_date: TimeT,
    last_modification_date: TimeT,
    #[deku(pad_bytes_after = "4")]
    last_access_date: TimeT,
    comment: [u8; 256],
}
#[derive(Debug, DekuRead, DekuWrite)]
pub struct TdfHeader {
    header: TdfHeaderInfo,
    #[deku(count = "header.n_entries")]
    entries: Vec<TdfJumptableEntry>,
}

#[derive(Debug, DekuRead, DekuWrite, Copy, Clone)]
#[deku(type = "u32")]
enum TdfBlockType {
    #[deku(id = "0")]
    UnusedSlot,
    #[deku(id = "1")]
    NotDefined,
    #[deku(id = "2")]
    CalibrationData,
    #[deku(id = "3")]
    CalibrationData2d,
    #[deku(id = "4")]
    Data2d,
    #[deku(id = "5")]
    Data3d,
    #[deku(id = "6")]
    OpticalSystemConfiguration,
    #[deku(id = "7")]
    ForcePlatformsCalibrationData,
    #[deku(id = "8")]
    ForcePlatformsCalibrationData2d,
    #[deku(id = "9")]
    ForcePlatformsData,
    #[deku(id = "10")]
    AnthropometricData,
    #[deku(id = "11")]
    ElectromyographicData,
    #[deku(id = "12")]
    ForceAndTorqueData,
    #[deku(id = "13")]
    VolumetricData,
    #[deku(id = "14")]
    AnalogData,
    #[deku(id = "15")]
    GeneralCalibrationData,
    #[deku(id = "16")]
    TemporalEventsData,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(ctx = "block_type: TdfBlockType", id = "block_type")]
enum TdfFormat {
    #[deku(id = "TdfBlockType::CalibrationData")]
    TdfCalibFormat(u32),
    #[deku(id = "TdfBlockType::Data2d")]
    TdfData2DFormat(u32),
    #[deku(id = "TdfBlockType::Data3d")]
    TdfData3DFormat(u32),
    #[deku(id = "TdfBlockType::ForcePlatformsCalibrationData")]
    TdfForcePlatformsCalibFormat(u32),
    #[deku(id = "TdfBlockType::OpticalSystemConfiguration")]
    TdfOptiSetupFormat(u32),
    #[deku(id = "TdfBlockType::UnusedSlot")]
    UnusedSlotFormat(u32),
    #[deku(id = "TdfBlockType::ForcePlatformsData")]
    TdfDataPlatFormat(u32),
    #[deku(id = "TdfBlockType::ForceAndTorqueData")]
    TdfForce3DFormat(u32),
    #[deku(id = "TdfBlockType::TemporalEventsData")]
    TdfEventsFormat(u32),
}
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32")]
enum TdfCalibFormat {
    #[deku(id = "0")]
    Unknown,
    #[deku(id = "1")]
    SeeLab1,
    #[deku(id = "2")]
    BTSCalibration,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32")]
enum TdfData2DFormat {
    #[deku(id = "0")]
    Unknown,
    #[deku(id = "1")]
    RTS,
    #[deku(id = "2")]
    PCK,
    #[deku(id = "3")]
    SYNC,
}
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32")]
enum TdfData2D4P {
    #[deku(id = "0")]
    Unknown,
    #[deku(id = "1")]
    RTS,
    #[deku(id = "2")]
    PCK,
    #[deku(id = "3")]
    SYNC,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32")]
enum TdfData2D4CFormat {
    #[deku(id = "0")]
    Unknown,
    #[deku(id = "1")]
    RTS,
    #[deku(id = "2")]
    PCK,
    #[deku(id = "3")]
    SYNC,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32")]
enum TdfData3DFormat {
    #[deku(id = "0")]
    Unknown,
    #[deku(id = "1")]
    ByTrack,
    #[deku(id = "2")]
    ByTrackWithLinks,
    #[deku(id = "3")]
    ByFrame,
    #[deku(id = "4")]
    ByFrameWithLinks,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32")]
enum TdfOptiSetupFormat {
    #[deku(id = "0")]
    Unknown,
    #[deku(id = "1")]
    BasicFormat,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32")]
enum TdfCalPlatFormat {
    #[deku(id = "0")]
    Unknown,
    #[deku(id = "1")]
    ISS,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32")]
enum TdfEventsFormat {
    #[deku(id = "0")]
    Unknown,
    #[deku(id = "1")]
    STD,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32")]
enum TdfForce3DFormat {
    #[deku(id = "0")]
    Unknown,
    #[deku(id = "1")]
    ByTrack,
    #[deku(id = "2")]
    ByFrame,
    #[deku(id = "3")]
    ByTrackWithSpeed,
    #[deku(id = "4")]
    ByFrameWithSpeed,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32")]
enum TdfDataPlatFormat {
    #[deku(id = "0")]
    Unknown,
    #[deku(id = "1")]
    ByTrackSingle,
    #[deku(id = "2")]
    ByFrameSingle,
    #[deku(id = "3")]
    ByTrackSingleWithLabels,
    #[deku(id = "4")]
    ByFrameSingleWithLabels,
    #[deku(id = "5")]
    ByTrackDouble,
    #[deku(id = "6")]
    ByFrameDouble,
    #[deku(id = "7")]
    ByTrackDoubleWithLabels,
    #[deku(id = "8")]
    ByFrameDoubleWithLabels,
    #[deku(id = "9")]
    ByTrackSingleWithSpeed,
    #[deku(id = "10")]
    ByFrameSingleWithSpeed,
    #[deku(id = "11")]
    ByTrackSingleWithSpeedWithLabels,
    #[deku(id = "12")]
    ByFrameSingleWithSpeedWithLabels,
    #[deku(id = "13")]
    ByTrackDoubleWithSpeed,
    #[deku(id = "14")]
    ByFrameDoubleWithSpeed,
    #[deku(id = "15")]
    ByTrackDoubleWithSpeedWithLabels,
    #[deku(id = "16")]
    ByFrameDoubleWithSpeedWithLabels,
}
