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
    format: u32,
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
#[derive(Debug, DekuRead, DekuWrite)]
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
