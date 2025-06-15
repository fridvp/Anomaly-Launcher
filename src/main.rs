#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env, fmt,
    path::{Path, PathBuf},
    process::exit,
    sync::Arc,
};



use std::thread;
use std::time::Duration;

use std::sync::mpsc;
mod app_config;
mod game;
mod styles;

use app_config::{AppConfig, Renderer, ShadowMapSize,WindowMode};
use eframe::egui::{
    self, Button, ComboBox, FontData, FontDefinitions, FontFamily, IconData, RichText, Vec2, ViewportBuilder
};
use game::Game;
use rfd::MessageDialog;
use styles::Styles;




mod funnysht;
pub use funnysht::{minimize_to_tray, force_clear_shader_cache, has_avx_support, update_user_ltx, reset_user_ltx,calculate_md5,verify_install,check_for_addons,show_error,set_cpu_affinity,apply_sound_fix};











//********************************LOCALISATION*********************************
mod localization;
pub use localization::{Localization};
use once_cell::sync::Lazy;

static LOCALIZATION2: Lazy<Localization> = Lazy::new(|| Localization::new());






//********************************MAIN_FUNCTION*********************************


fn main() -> eframe::Result<()> {
    if !Path::new("launcherconfig.toml").exists() {
        let default_config = AppConfig::default();
        let _ = default_config.write();
    }
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "OpenSans".to_owned(),
        FontData::from_static(include_bytes!("../assets/fixed_font.ttf")),
    );
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "OpenSans".to_owned());

    let icon_data = include_bytes!("../assets/icon2.ico");

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(icon_data)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let arc_icon = Arc::new(IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    });


    let config2 = AppConfig::load().unwrap_or_else(|err| {
        match err {
            app_config::AppConfigError::ReadFailed => show_error("Read Failed", "Failed to read the configuration file. Please remove 'launcherconfig.toml' and try to launch program again."),
            app_config::AppConfigError::BadStructure => show_error("Bad configuration", "Your configuration seems to be damaged. Please remove 'launcherconfig.toml' and try to launch program again."),
            app_config::AppConfigError::WriteFailed => todo!(),
        };
        exit(1);
    });
    let viewport = ViewportBuilder::default()
        .with_maximize_button(false)
        .with_resizable(false)
        .with_inner_size(if !config2.launcherregmode { Vec2 { x: 1200.0, y: 675.0 }}else{Vec2 { x: 800.0, y: 450.0 }})
        .with_decorations(config2.launcherregmode)
        .with_transparent(true)
        .with_icon(arc_icon);
        

    eframe::run_native(
        "Anomaly Launcher",
        eframe::NativeOptions {
            viewport,
            vsync: false,
            centered: true,
            ..Default::default()
        },
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            Box::new(LauncherApp::new(cc))
        }),
    )

}




//#[derive(Debug)]
struct LauncherApp {
    config: AppConfig,
    app_shutdown: bool,
    warning_text: String,
    verification_progress: f32, // progress from 0.0 to 1.0
    current_file: String,       // current file (for file integrity check. same for thath sh^)
    verification_rx: Option<mpsc::Receiver<(String, Vec<String>, Vec<String>, f32, String)>>, // channel for datas
    is_verifying: bool,         // flag that says if it is verifying or not :D
    avx_supported: bool,
    background_texture: Option<egui::TextureHandle>,
}




impl LauncherApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut config = AppConfig::load().unwrap_or_else(|err| {
            match err {
                app_config::AppConfigError::ReadFailed => show_error("Read Failed", "Failed to read the configuration file. Please remove 'launcherconfig.toml' and try to launch program again."),
                app_config::AppConfigError::BadStructure => show_error("Bad configuration", "Your configuration seems to be damaged. Please remove 'launcherconfig.toml' and try to launch program again."),
                app_config::AppConfigError::WriteFailed => todo!(),
            };
            exit(1);
        });
        let background_texture = {

            let image_data = if !config.launcherregmode {
                include_bytes!("../assets/background2.png")as &[u8]
            } else {
                include_bytes!("../assets/background2reg.png")as &[u8]
            }; // background image
            let image = image::load_from_memory(image_data).unwrap().to_rgba8();
            let size = [image.width() as _, image.height() as _];
            let pixels = image.into_raw();
    
            Some(cc.egui_ctx.load_texture(
                "background",
                egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                Default::default(),
            ))
        };
        let avx_supported = has_avx_support();
        if !avx_supported {
            config.use_avx = false;
        }
        // addon check
        let exe_path = env::current_exe().expect("Failed to get the path to the executable");
        let exe_dir = exe_path.parent().expect("Failed to get the parent directory of the executable");
        
        // forming gamedata path
        let gamedata_path = exe_dir.join("gamedata");
        
        // addon check AAAAA
        let warning_text = if gamedata_path.exists() {
            match check_for_addons(gamedata_path.to_str().unwrap()) {
                Ok(true) => LOCALIZATION2.warning1.to_string(),
                Ok(false) => String::new(), // no warning if no addons
                Err(e) => format!("Error checking for addons: {}", e), // AAAA ERROR ALARM ALARM
            }
        } else {
            LOCALIZATION2.warning1.to_string() // warning if no gamedata folder
        };
        LauncherApp {
            config,
            app_shutdown: false,
            warning_text,
            verification_progress: 0.0,
            current_file: String::new(),
            verification_rx: None, // channel init
            is_verifying: false,  // flag init
            avx_supported,
            background_texture,

        }
    }
    
}

impl fmt::Display for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Renderer::DX8 => write!(f, "DirectX 8"),
            Renderer::DX9 => write!(f, "DirectX 9"),
            Renderer::DX10 => write!(f, "DirectX 10"),
            Renderer::DX11 => write!(f, "DirectX 11"),
        }
    }
}

impl fmt::Display for ShadowMapSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShadowMapSize::Size1536 => write!(f, "1536"),
            ShadowMapSize::Size2048 => write!(f, "2048"),
            ShadowMapSize::Size2560 => write!(f, "2560"),
            ShadowMapSize::Size3072 => write!(f, "3072"),
            ShadowMapSize::Size4096 => write!(f, "4096"),
        }
    }
}
impl fmt::Display for WindowMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WindowMode::Default => write!(f, "------"),
            WindowMode::Fullscreen => write!(f,"{}", LOCALIZATION2.fullscr),
            WindowMode::Windowed => write!(f,"{}", LOCALIZATION2.wined),
            WindowMode::BorderlessWindowed => write!(f,"{}", LOCALIZATION2.borlesswined),
        }
    }
}
//********************************BUTTONS*********************************
fn set_custom_interaction(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.interaction.selectable_labels = false;
    ctx.set_style(style);
}
use egui::Frame;
impl eframe::App for LauncherApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]}
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        ctx.set_visuals(Styles::dark());
        set_custom_interaction(ctx);
        let panel_frame = Frame::default().fill(egui::Color32::TRANSPARENT);
        if let Some(texture) = &self.background_texture {
            egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
                ui.style_mut().visuals = Styles::dark();
                // STRETCHING THE IMAAAGE
                let rect = ui.max_rect();
                ui.put(rect, egui::Image::new(texture));
            });
        }
        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            
            ui.style_mut().visuals = Styles::dark();
            //MAIN CONTENT
            
            ui.horizontal(|ui| {
                // LEFT PART: SETTINGS
                ui.add_space(if !self.config.launcherregmode {7.0+145.0} else {7.0});
                ui.vertical(|ui| {
                    ui.set_min_width(300.0);
                    ui.add_space(if !self.config.launcherregmode {55.0+112.0} else {55.0});
                    // Renderer
                    ui.label(RichText::new(LOCALIZATION2.renderer.clone()).size(18.0)).on_hover_text(LOCALIZATION2.renderer_hover.clone());
                    ComboBox::from_id_source("renderer")
                        .selected_text(self.config.renderer.to_string())
                        .width(200.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.config.renderer, Renderer::DX8, "DirectX 8");
                            ui.selectable_value(&mut self.config.renderer, Renderer::DX9, "DirectX 9");
                            ui.selectable_value(&mut self.config.renderer, Renderer::DX10, "DirectX 10");
                            ui.selectable_value(&mut self.config.renderer, Renderer::DX11, "DirectX 11");
                        });

                    ui.add_space(10.0); // element spacing

                    // Shadow Map Size
                    ui.label(RichText::new(LOCALIZATION2.shadowmap.clone()).size(18.0)).on_hover_text(LOCALIZATION2.shadowmap_hover.clone());
                    ComboBox::from_id_source("shadow_map")
                        .selected_text(self.config.shadow_map.to_string())
                        .width(200.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size1536, "1536");
                            ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size2048, "2048");
                            ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size2560, "2560");
                            ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size3072, "3072");
                            ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size4096, "4096");
                        });

                    ui.add_space(10.0); // element spacing

                    // Window Mode
                    ui.label(RichText::new(LOCALIZATION2.winmode.clone()).size(18.0));
                    ComboBox::from_id_source("window_mode")
                        .selected_text(self.config.window_mode.to_string())
                        .width(200.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.config.window_mode, WindowMode::Default, "------");
                            ui.selectable_value(&mut self.config.window_mode, WindowMode::Fullscreen, LOCALIZATION2.fullscr.clone());
                            ui.selectable_value(&mut self.config.window_mode, WindowMode::Windowed, LOCALIZATION2.wined.clone());
                            ui.selectable_value(&mut self.config.window_mode, WindowMode::BorderlessWindowed, LOCALIZATION2.borlesswined.clone());
                        });

                    ui.add_space(20.0); // spacing before Misc settings

                    // Misc settings
                    ui.label(RichText::new(LOCALIZATION2.misclabel.clone()).size(18.0));
                    ui.checkbox(&mut self.config.debug, LOCALIZATION2.debug_mode.clone()).on_hover_text(LOCALIZATION2.debug_mode_hover.clone());
                    ui.checkbox(&mut self.config.prefetch_sounds, LOCALIZATION2.pref_sounds.clone()).on_hover_text(LOCALIZATION2.pref_sounds_hover.clone());
                    ui.checkbox(&mut self.config.sndfix, LOCALIZATION2.workaround.clone()).on_hover_text(LOCALIZATION2.workaround_hover.clone());
                    if self.avx_supported {
                        ui.checkbox(&mut self.config.use_avx, LOCALIZATION2.useavx.clone()).on_hover_text(LOCALIZATION2.useavx_hover.clone());
                    } else {
                        ui.label(RichText::new(LOCALIZATION2.notsupportedavx.clone()).color(egui::Color32::RED));
                    }
                    ui.checkbox(&mut self.config.cpuaffinity, LOCALIZATION2.useaffinity.clone()).on_hover_text(LOCALIZATION2.useaffinity_hover.clone());
                    ui.checkbox(&mut self.config.custom_args, LOCALIZATION2.customlaunch.clone()).on_hover_text(LOCALIZATION2.customlaunch_hover.clone());
                        if self.config.custom_args {
                            ui.text_edit_singleline(&mut self.config.custom_args_text);
                        }
                });
                ui.add_space(110.0);
                // RIGHT PART: HEADER AND FUNKY BUTTONS
                ui.vertical(|ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                // HEADER "Anomaly Launcher"
                                ui.add_space(if !self.config.launcherregmode {50.0+112.0} else {50.0}); // up_space
                                ui.label(RichText::new("Anomaly Launcher").size(40.0).strong());
                                ui.add_space(60.0); // down_space

                                // play button (obvious lmao)
                                let play_button = ui.add_sized([316.0, 65.0], Button::new(RichText::new(LOCALIZATION2.play_button.clone()).size(26.0)));
                                if play_button.clicked() && !self.is_verifying {
                                    if let Err(e) = update_user_ltx(self.config.window_mode) {
                                        eprintln!("HOLY FUCK ERROR WHILE UPDATING USER.LTX: {}", e);
                                    }
                                    
                                    let game = Game::new(self.config.renderer, self.config.use_avx);
                                    let mut args: Vec<String> = Vec::new();
                                    let shadows_arg: String = match self.config.shadow_map {
                                        ShadowMapSize::Size1536 => "-smap1536".to_string(),
                                        ShadowMapSize::Size2048 => "-smap2048".to_string(),
                                        ShadowMapSize::Size2560 => "-smap2560".to_string(),
                                        ShadowMapSize::Size3072 => "-smap3072".to_string(),
                                        ShadowMapSize::Size4096 => "-smap4096".to_string(),
                                    };
                                    args.push(shadows_arg);
                                    if self.config.debug {
                                        args.push("-dbg".to_string());
                                    }
                                    if self.config.custom_args && !self.config.custom_args_text.is_empty() {
                                        let custom_args: Vec<String> = self.config.custom_args_text
                                            .split_whitespace()
                                            .map(|s| s.to_string())
                                            .collect();
                                        args.extend(custom_args);
                                    }
                                    if self.config.prefetch_sounds {
                                        args.push("-prefetch_sounds".to_string());
                                    }
                                    if self.config.sndfix {
                                        if let Err(e) = apply_sound_fix(self.config.sndfix) {
                                            eprintln!("SHIIT ERROR WHILE APPLYING SOUND FIX: {}", e);
                                        }
                                    }
                                    let launch_result = game.launch(args);
                                    if let Err(e) = launch_result {
                                        match e {
                                            game::GameError::ExecutableNotFound => {
                                                MessageDialog::new()
                                                    .set_title("Executable not found")
                                                    .set_description("Could not find the executable file of the game. Make sure you run the launcher from the game folder.")
                                                    .set_level(rfd::MessageLevel::Error)
                                                    .set_buttons(rfd::MessageButtons::Ok)
                                                    .show();
                                            },
                                            game::GameError::Unknown(i) => {
                                                MessageDialog::new()
                                                    .set_title("Unknown error occured")
                                                    .set_description(format!("The launcher failed to launch the game due to an unexpected error: {}",i))
                                                    .set_level(rfd::MessageLevel::Error)
                                                    .set_buttons(rfd::MessageButtons::Ok)
                                                    .show();
                                            },
                                        }
                                    }

                                        minimize_to_tray(ctx);

                                    if self.config.cpuaffinity {
                                        println!("Starting CPU affinity thread");
                                        

                                            if let Err(e) = set_cpu_affinity() {
                                                eprintln!("Error in set_cpu_affinity: {}", e);
                                            }
                                    }
                                    self.app_shutdown = true;
                                }

                                ui.add_space(2.0); // button spacing

                                // THE GRAND 3 BUTTON GROUP
                                ui.horizontal(|ui| {
                                    ui.add_space((ui.available_width() - 316.0) / 2.0);
                                        let clear_button = ui.add_sized([100.0, 40.0], Button::new(LOCALIZATION2.clear_shader_cache_button.clone()));
                                        let reset_ltx = ui.add_sized([100.0, 40.0], Button::new(LOCALIZATION2.reset_user_ltx_button.clone()));
                                        let verify_button = ui.add_sized([100.0, 40.0], Button::new(LOCALIZATION2.verify_installation_button.clone()));

                                        if clear_button.clicked() && !self.is_verifying{
                                            let mut cache_path: PathBuf = env::current_dir().unwrap();
                                            cache_path.push("appdata\\shaders_cache");
                                            println!("{:?}", cache_path);
                                            if !cache_path.exists() {
                                                let _ = MessageDialog::new()
                                                    .set_title("Path not found")
                                                    .set_description("The launcher cannot find the shader cache folder. Make sure you run the launcher in the Anomaly game folder.")
                                                    .set_level(rfd::MessageLevel::Error)
                                                    .set_buttons(rfd::MessageButtons::Ok)
                                                    .show();
                                            } else {
                                                thread::spawn(|| {
                                                let _ =force_clear_shader_cache();
                                                MessageDialog::new()
                                                    .set_title("Clear Shader Cache")
                                                    .set_description("Shader cache has been deleted.")
                                                    .set_level(rfd::MessageLevel::Info)
                                                    .set_buttons(rfd::MessageButtons::Ok)
                                                    .show();
                                                });
                                            }
                                        }
                                        if reset_ltx.clicked() && !self.is_verifying{
                                            if let Err(e) = reset_user_ltx() {
                                                eprintln!("Error resetting user.ltx: {}", e);
                                            }
                                        }
                                        if verify_button.clicked() {
                                            self.verification_progress = 0.0;
                                            self.current_file.clear();
                                            self.is_verifying = true;

                                            let (tx, rx) = mpsc::channel();
                                            self.verification_rx = Some(rx);

                                            thread::spawn(move || {
                                                let mut verification_progress = 0.0;
                                                let mut current_file = String::new();

                                                match verify_install(&mut verification_progress, &mut current_file, &tx) {
                                                    Ok((result, missing_files, corrupt_files)) => {
                                                        let _ = tx.send((result, missing_files, corrupt_files, verification_progress, current_file));
                                                    }
                                                    Err(e) => {
                                                        let _ = tx.send((format!("Error during verification: {}", e), Vec::new(), Vec::new(), 0.0, String::new()));
                                                    }
                                                }
                                            });
                                        }
                                    
                                });

                                ui.add_space(2.0); // button spacing

                                // Quit BUTTON
                                let quit_button = ui.add_sized([316.0, 65.0], Button::new(RichText::new(LOCALIZATION2.quit_button.clone()).size(26.0)));
                                if quit_button.clicked() {
                                    self.app_shutdown = true;
                                }
                                if !self.warning_text.is_empty() {
                                    
                                        ui.label(RichText::new(&self.warning_text).color(egui::Color32::RED));
                                    
                                }
                                if self.is_verifying {
                                    ctx.request_repaint_after(Duration::from_millis(100));
                    
                                    // progress display
                                    let progress_text = format!("Checking: {} ({:.1}%)", self.current_file, self.verification_progress * 100.0);
                                    ui.label(progress_text);
                                }
                            
                        });
                    });
                });
            });
        


            // verification check and update
            if let Some(rx) = &self.verification_rx {
                if let Ok((result, missing_files, corrupt_files, progress, current_file)) = rx.try_recv() {
                    self.verification_progress = progress;
                    self.current_file = current_file;

                    // if update finished
                    if !result.is_empty() || progress >= 1.0 {
                        let mut message = result;
                        if !missing_files.is_empty() {
                            message.push_str("\nMissing files:\n");
                            for file in missing_files {
                                message.push_str(&format!("- {}\n", file));
                            }
                        }
                        if !corrupt_files.is_empty() {
                            message.push_str("\nCorrupt files:\n");
                            for file in corrupt_files {
                                message.push_str(&format!("- {}\n", file));
                            }
                        } else {
                            message.push_str("\nNo issues found.\n");
                        }

                        MessageDialog::new()
                            .set_title("Verification Result")
                            .set_description(&message)
                            .set_level(if message.is_empty() { rfd::MessageLevel::Info } else { rfd::MessageLevel::Warning })
                            .set_buttons(rfd::MessageButtons::Ok)
                            .show();

                        // reset
                        self.is_verifying = false;
                        self.verification_rx = None;
                    } else {
                        // repain request if it aint done
                        ctx.request_repaint();
                    }
                }
            }

        });

        // close that shit
        if ctx.input(|i| i.viewport().close_requested()) {
            self.app_shutdown = true;
        }

        // I SAID CLOSE IT
        if self.app_shutdown {
            match self.config.write() {
                Ok(_) => {}
                Err(_) => show_error("Write Failed", "Failed to write data to configuration file. You might need to set your options again."),
            };
            exit(0);
        }
    }
}