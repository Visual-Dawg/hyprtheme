use std::fs;

use crate::config::Config;
use expanduser;

pub fn ensure_config() {
    let hypr_config = expanduser::expanduser("~/.config/hypr/hyprland.conf").unwrap();

    let conf = match hypr_config.exists() {
        true => fs::read_to_string(&hypr_config).expect("Unable to read hyprland config file"),
        false => String::new(),
    };

    let autogen = format!(
        "
#====== autogenerated by hyprtheme ======#
source = ~/.config/hypr/themes/dist.conf
"
    );

    if !conf.contains(&autogen) {
        println!("Adding autogen to {}", hypr_config.display());
        fs::write(hypr_config, conf + autogen.as_str()).expect("Unable to write to file");
    }

    let hyprpaper_config = expanduser::expanduser("~/.config/hypr/hyprpaper.conf").unwrap();

    let hyprpaper_conf = match hyprpaper_config.exists() {
        true => fs::read_to_string(&hyprpaper_config).expect("Unable to read hyprland config file"),
        false => String::new(),
    };

    let hyprpaper_autogen = format!(
        "
#====== autogenerated by hyprtheme ======#
source = ~/.config/hypr/themes/hyprpaper_dist.conf
"
    );

    if !hyprpaper_conf.contains(&hyprpaper_autogen) {
        println!("Adding autogen to {}", hyprpaper_config.display());
        fs::write(
            hyprpaper_config,
            hyprpaper_conf + hyprpaper_autogen.as_str(),
        )
        .expect("Unable to write to file");
    }
}

pub fn apply(config: Config) {
    ensure_config();

    let dist_path = expanduser::expanduser("~/.config/hypr/themes/dist.conf").unwrap();
    let hyprpaper_dist_path =
        expanduser::expanduser("~/.config/hypr/themes/hyprpaper_dist.conf").unwrap();

    fs::write(dist_path, config.build_conf()).expect("Unable to write to dist file");
    fs::write(hyprpaper_dist_path, config.build_hyprpaper()).expect("Unable to write to dist file");
}
