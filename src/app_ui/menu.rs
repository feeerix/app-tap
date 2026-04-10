/*****************************************************************************
 *   Ledger Tap App 
 *   Felix Voo & Rammy Kim.
 *
 *****************************************************************************/

use ledger_device_sdk::include_gif;
use ledger_device_sdk::io::Comm;

use crate::settings::Settings;
use ledger_device_sdk::nbgl::{NbglGlyph, NbglHomeAndSettings};

pub fn ui_menu_main(_: &mut Comm) -> NbglHomeAndSettings {
    // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
    #[cfg(target_os = "apex_p")]
    const FERRIS: NbglGlyph = NbglGlyph::from_include(include_gif!("glyphs/crab_48x48.png", NBGL));
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    const FERRIS: NbglGlyph = NbglGlyph::from_include(include_gif!("glyphs/crab_64x64.gif", NBGL));

    let settings_strings = [["Display Memo", "Allow display of transaction memo."]];
    let mut settings: Settings = Default::default();

    // Display the home screen.
    NbglHomeAndSettings::new()
        .glyph(&FERRIS)
        .settings(settings.get_mut(), &settings_strings)
        .infos(
            "App Tap",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )
        .tagline("Place device against phone with NFC turned ON.")
}
