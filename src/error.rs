use embedded_hal::digital::{self, ErrorType};

pub type Hcms29xxErr<DataPin, RsPin, ClkPin, CePin, BlankPin, OscSelPin, ResetPin> = Hcms29xxError<
    <DataPin as ErrorType>::Error,
    <RsPin as ErrorType>::Error,
    <ClkPin as ErrorType>::Error,
    <CePin as ErrorType>::Error,
    <BlankPin as ErrorType>::Error,
    <OscSelPin as ErrorType>::Error,
    <ResetPin as ErrorType>::Error,
>;

#[derive(Debug)]
pub enum Hcms29xxError<
    DataPinErr,
    RsPinErr,
    ClkPinErr,
    CePinErr,
    BlankPinErr,
    OscSelPinErr,
    ResetPinErr,
> {
    PinNotConfigured,
    DataPinError(DataPinErr),
    RsPinError(RsPinErr),
    ClkPinError(ClkPinErr),
    CePinError(CePinErr),
    BlankPinError(BlankPinErr),
    OscSelPinError(OscSelPinErr),
    ResetPinError(ResetPinErr),
}

impl<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr>
    Hcms29xxError<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr>
where
    DataPinErr: core::fmt::Debug,
    RsPinErr: core::fmt::Debug,
    ClkPinErr: core::fmt::Debug,
    CePinErr: core::fmt::Debug,
    BlankPinErr: core::fmt::Debug,
    OscSelPinErr: core::fmt::Debug,
    ResetPinErr: core::fmt::Debug,
{
    fn kind(&self) -> digital::ErrorKind {
        digital::ErrorKind::Other
    }
}

impl<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr> From<DataPinErr>
    for Hcms29xxError<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr>
where
    DataPinErr: core::fmt::Debug,
{
    fn from(err: DataPinErr) -> Self {
        Hcms29xxError::DataPinError(err)
    }
}

impl<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr> From<RsPinErr>
    for Hcms29xxError<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr>
where
    RsPinErr: core::fmt::Debug,
{
    fn from(err: RsPinErr) -> Self {
        Hcms29xxError::RsPinError(err)
    }
}

impl<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr> From<ClkPinErr>
    for Hcms29xxError<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr>
where
    ClkPinErr: core::fmt::Debug,
{
    fn from(err: ClkPinErr) -> Self {
        Hcms29xxError::ClkPinError(err)
    }
}

impl<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr> From<CePinErr>
    for Hcms29xxError<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr>
where
    CePinErr: core::fmt::Debug,
{
    fn from(err: CePinErr) -> Self {
        Hcms29xxError::CePinError(err)
    }
}

impl<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr> From<BlankPinErr>
    for Hcms29xxError<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr>
where
    BlankPinErr: core::fmt::Debug,
{
    fn from(err: BlankPinErr) -> Self {
        Hcms29xxError::BlankPinError(err)
    }
}

impl<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr> From<OscSelPinErr>
    for Hcms29xxError<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr>
where
    OscSelPinErr: core::fmt::Debug,
{
    fn from(err: OscSelPinErr) -> Self {
        Hcms29xxError::OscSelPinError(err)
    }
}

impl<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr> From<ResetPinErr>
    for Hcms29xxError<DataPinErr, RsPinErr, ClkPinErr, CePinErr, BlankPinErr, OscSelPinErr, ResetPinErr>
where
    ResetPinErr: core::fmt::Debug,
{
    fn from(err: ResetPinErr) -> Self {
        Hcms29xxError::ResetPinError(err)
    }
}
