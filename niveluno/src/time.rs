use crate::nuerror::NUError;

struct TimeGod {
    pub start_time_ms: u128,
    pub current_time_ms: u128,
    pub delta_time_ms: u128,
}

impl TimeGod {
    pub fn get() -> Result<&'static mut TimeGod, NUError> {
        unsafe {
            TIME_GOD
                .as_mut()
                .ok_or_else(|| NUError::MiscError("TIME_GOD uninit".to_string()))
        }
    }
}

static mut TIME_GOD: Option<TimeGod> = None;

pub fn init() -> Result<(), NUError> {
    if TimeGod::get().is_ok() {
        return Err(NUError::MiscError("TIME_GOD already init".to_string()));
    }

    let current_time_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| NUError::SystemTimeError(e.to_string()))?
        .as_millis();

    // divide-by-zero protection by faking 1 frame since last
    let delta_time_ms = 16;
    let start_time_ms = current_time_ms - delta_time_ms;

    let tg = TimeGod {
        start_time_ms,
        current_time_ms,
        delta_time_ms,
    };

    unsafe { TIME_GOD = Some(tg) }

    Ok(())
}

pub fn get_run_time() -> Result<f64, NUError> {
    let gg = TimeGod::get()?;

    Ok((gg.current_time_ms - gg.start_time_ms) as f64 / 1000.)
}

pub fn get_delta_time() -> Result<f64, NUError> {
    let gg = TimeGod::get()?;

    Ok(gg.delta_time_ms as f64 / 1000.)
}

pub fn update_time() -> Result<(), NUError> {
    let gg = TimeGod::get()?;

    let current_time_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| NUError::SystemTimeError(e.to_string()))?
        .as_millis();

    gg.delta_time_ms = current_time_ms - gg.current_time_ms;
    gg.current_time_ms = current_time_ms;
    Ok(())
}
