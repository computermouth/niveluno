use std::fs::File;

use munzip;

use crate::NUError;

struct AssetGod<'a> {
    pub d_file: File,
    pub default: Option<munzip::SearchableArchive<'a>>,
    // todo, also maybe should be a list or something
    pub custom: Option<munzip::SearchableArchive<'a>>,
}

impl<'a> AssetGod<'a> {
    pub fn get() -> Result<&'static mut AssetGod<'static>, NUError> {
        unsafe {
            ASSET_GOD
                .as_mut()
                .ok_or_else(|| NUError::MiscError("ASSET_GOD uninit".to_string()))
        }
    }
}

static mut ASSET_GOD: Option<AssetGod<'static>> = None;

pub fn init() -> Result<(), NUError> {
    if AssetGod::get().is_ok() {
        return Err(NUError::MiscError("ASSET_GOD already init".to_string()));
    }

    let args: Vec<String> = std::env::args().collect();
    eprintln!("args: {:?}", args);

    let dflag = args.iter().position(|n| n == &"-d".to_string());
    let default_filename = match dflag {
        None => "todo",
        Some(i) => {
            if args.len() <= i + 1 {
                eprintln!("W: '-d' flag was set, but no file was provided");
                "todo"
            } else {
                &args[i + 1]
            }
        }
    };

    eprintln!("default_filename: {default_filename}");

    let default_file = File::open(default_filename)?;

    unsafe {
        ASSET_GOD = Some(AssetGod {
            d_file: default_file,
            default: None,
            custom: None,
        });
    }

    let ag = AssetGod::get()?;
    ag.default = Some(munzip::SearchableArchive::new(&mut ag.d_file)?);

    Ok(())
}
