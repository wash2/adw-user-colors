// SPDX-License-Identifier: MPL-2.0-only

pub fn main() -> anyhow::Result<()> {
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "--start" {
            user_colors::load()?;
        } else if arg == "--stop" {
            user_colors::unload()?;
        } else {
            println!("Usage: adw-user-colors --start OR adw-user-colors --stop")
        }
    } else {
        println!("Usage: adw-user-colors --start OR adw-user-colors --stop")
    }
    Ok(())
}
