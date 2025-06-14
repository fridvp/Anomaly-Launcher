
use std::{fs, path::Path};





use windows::{
    core::PCWSTR,
    Win32::UI::Shell::{SHFileOperationW, SHFILEOPSTRUCTW, FO_DELETE, FOF_NOCONFIRMATION, FOF_NOERRORUI, FOF_SILENT}
};


#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::__cpuid;


use crate::app_config::WindowMode; 

const USER_LTX: &str = "appdata/user.ltx";
const USER_LTX_OLD: &str = "appdata/user.ltx.old";

use walkdir::WalkDir;
use std::thread;
use std::error::Error;
use std::time::Duration;
use rfd::MessageDialog;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winbase::SetProcessAffinityMask;
use winapi::um::winnt::{PROCESS_SET_INFORMATION};
use sysinfo::PidExt;
use sysinfo::{System, ProcessExt, SystemExt};
//********************************FUNCTION_FOR_AUDIO_FIX*********************************


const ALSOFT_INI: &str = "bin/alsoft.ini";
const ALSOFT_INI_BAK: &str = "bin/alsoft.ini.bak";
pub fn apply_sound_fix(check_box_fix_sound_checked: bool) -> std::io::Result<()> {
    if check_box_fix_sound_checked {
        if !Path::new(ALSOFT_INI).exists() {
            return Ok(());
        }
        if Path::new(ALSOFT_INI_BAK).exists() {
            fs::remove_file(ALSOFT_INI_BAK)?;
        }
        fs::rename(ALSOFT_INI, ALSOFT_INI_BAK)?;
    } else {
        if Path::new(ALSOFT_INI).exists() {
            return Ok(());
        }
        if Path::new(ALSOFT_INI_BAK).exists() {
            fs::rename(ALSOFT_INI_BAK, ALSOFT_INI)?;
        }
    }
    Ok(())
}

//********************************FUNCTION_FOR_CPU_AFFINITY*********************************


pub fn set_cpu_affinity() -> Result<(), Box<dyn Error>> {
    let target_exes = [
        "AnomalyDX11AVX.exe",
        "AnomalyDX11.exe",
        "AnomalyDX10AVX.exe",
        "AnomalyDX10.exe",
        "AnomalyDX9AVX.exe",
        "AnomalyDX9.exe",
        "AnomalyDX8AVX.exe",
        "AnomalyDX8.exe",
    ];

    let mut system = System::new_all();
    let logical_cores = system.cpus().len();
    let available_cores: u32 = (1 << logical_cores) - 4;

    println!(
        "Logical cores: {}, Available mask: {:b} ({})",
        logical_cores, available_cores, available_cores
    );

    loop {
        system.refresh_all();

        for (_, process) in system.processes() {
            let process_name = process.name();
            if target_exes.iter().any(|&exe| process_name.starts_with(exe)) {
                println!("Found process: {} (PID: {})", process.name(), process.pid().as_u32());

                let pid = process.pid().as_u32();
                unsafe {
                    let handle = OpenProcess(PROCESS_SET_INFORMATION, 0, pid);
                    if handle.is_null() {
                        eprintln!("Failed to open process {}: Access denied", pid);
                        continue;
                    }

                    if SetProcessAffinityMask(handle, available_cores) == 0 {
                        eprintln!("Failed to set CPU affinity for process {}", pid);
                    } else {
                        println!("Successfully set CPU affinity for process {}", pid);
                        return Ok(());
                    }
                }
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}

//********************************FUNCTION_FOR_ERROR*********************************


pub fn show_error(title: &str, desc: &str) {
    MessageDialog::new()
        .set_title(title)
        .set_description(desc)
        .set_level(rfd::MessageLevel::Error)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
}

//********************************TABLE_FOR_NON_ADDON_FILES*********************************


//those files dont count as addons (aka ignored)
const ALLOWED_FILES: [&str; 6] = [
    "atmosfear_default_settings.ltx",
    "atmosfear_options.ltx",
    "axr_options.ltx",
    "cache_dbg.ltx",
    "localization.ltx",
    "warfare_options.ltx",
];






//********************************CHECKSUM_FUNCTION*********************************
use md5::{Context, Digest};
use std::io::{self, Read};
use std::fs::File;

use std::sync::mpsc; //for channels

const CHECKSUMS_MD5: &str = "tools/checksums.md5";
const CHUNK_SIZE: usize = 8192;

pub fn calculate_md5(file_path: &str) -> io::Result<Digest> {
    let mut file = File::open(file_path)?;
    let mut context = Context::new();
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.consume(&buffer[..count]);
    }

    Ok(context.compute())
}

pub fn verify_install(
    verification_progress: &mut f32,
    current_file: &mut String,
    tx: &mpsc::Sender<(String, Vec<String>, Vec<String>, f32, String)>,
) -> Result<(String, Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
    let mut result = String::new();
    let mut missing_files = Vec::new();
    let mut corrupt_files = Vec::new();

    // IGNORED FILES LIST (made it so it wont get triggered by modded exes and all that sh)
    let ignored_files = [
        "bin\\AnomalyDX11AVX.exe",
        "bin\\AnomalyDX11.exe",
        "bin\\AnomalyDX10AVX.exe",
        "bin\\AnomalyDX10.exe",
        "bin\\AnomalyDX9AVX.exe",
        "bin\\AnomalyDX9.exe",
        "bin\\AnomalyDX8AVX.exe",
        "bin\\AnomalyDX8.exe",
        "fsgame.ltx",
        "AnomalyLauncher.exe",
    ];

    // check if you have that checksum file :D
    if !Path::new(CHECKSUMS_MD5).exists() {
        result.push_str("* checksums.md5 missing *");
        return Ok((result, missing_files, corrupt_files));
    }

    // reading checksums.md5
    let checksums = fs::read_to_string(CHECKSUMS_MD5)?;

    let total_files = checksums.lines().count();
    

    for (processed_file,line) in checksums.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 || !parts[1].starts_with('*') || parts[0].starts_with('#') || parts[0].starts_with(';') {
            continue;
        }
    
        let checksum = parts[0].to_lowercase();
        let filename = &parts[1][1..];
    
        if ignored_files.contains(&filename) {
            continue;
        }
    
        *current_file = filename.to_string();
        *verification_progress = processed_file as f32 / total_files as f32;
    
        if !Path::new(filename).exists() {
            missing_files.push(filename.to_string());
        } else {
            let digest = calculate_md5(filename)?;
            let calculated = format!("{:x}", digest);
    
            if calculated != checksum {
                corrupt_files.push(filename.to_string());
            }
        }
    
        //sending current progress into the channel
        //i hope it counts as multithreading at least a bit so i can brag about it
        let _ = tx.send((
            String::new(),
            missing_files.clone(),
            corrupt_files.clone(),
            *verification_progress,
            current_file.clone(),
        ));
    }
    println!("VERIFICATION FINISHED");
    if !missing_files.is_empty() {
        result.push_str("* files missing, please reinstall *\n");
        result.push_str(&format!("Missing files: {:?}\n", missing_files));
    }

    if !corrupt_files.is_empty() {
        result.push_str("* checksum verification failed, please reinstall *\n");
        result.push_str(&format!("Corrupt files: {:?}\n", corrupt_files));
    }
    let _ = tx.send((
        result.clone(),
        missing_files.clone(),
        corrupt_files.clone(),
        1.0,
        "".to_string(),
    ));
    Ok((result, missing_files, corrupt_files))
}

//********************************FUNCTION_FOR_ADDONS_CHECK*********************************


pub fn check_for_addons(gamedata_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut has_addons = false;

    // check if gamedata exists
    if !Path::new(gamedata_path).exists() {
        return Ok(false);
    }

    // recursion check of all files in gamedata
    for entry in WalkDir::new(gamedata_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // checking if the file is in allowed list
        if path.is_file() {
            let file_name = path.to_string_lossy().to_string();
            let is_allowed = ALLOWED_FILES.iter().any(|&allowed| file_name.ends_with(allowed));

            if !is_allowed {
                has_addons = true;
                break;
            }
        }
    }

    Ok(has_addons)
}


use eframe::egui;


pub fn minimize_to_tray(ctx: &egui::Context) {
    
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
    
}








pub fn force_clear_shader_cache() -> std::io::Result<()> {
    let path = "appdata\\shaders_cache";
    //Fck C#
    //transforming path into wide-string with double null-terminator (tu-du tun tu-dun)
    let mut path_wide: Vec<u16> = path.encode_utf16().collect();
    path_wide.push(0); // first \0
    path_wide.push(0); // second \0 MUST HAAAVE

    let mut file_op = SHFILEOPSTRUCTW {
        hwnd: Default::default(),
        wFunc: FO_DELETE,
        pFrom: PCWSTR(path_wide.as_ptr()),
        fFlags: ((FOF_NOCONFIRMATION | FOF_NOERRORUI | FOF_SILENT).0 as u16),
        ..Default::default()
    };
    
    

    // Dark Magic
    unsafe {
        SHFileOperationW(&mut file_op);
            Ok(())
    }
}







//********************************FUNCTION_FOR_AVX_SUPPORT*********************************
pub fn has_avx_support() -> bool {
    unsafe {
        // CPUid call to check for supported functions
        let cpuid = __cpuid(1); // EAX=1 — getting info about processors functions

        //checking bit nr 28 in ECX register (avx bit)
        //hell yeah, at least somewhere i could use my assembly knowledge
        (cpuid.ecx & (1 << 28)) != 0
    }
}




//********************************SHIT_FOR_WINDOW_MODE*********************************



pub fn update_user_ltx(window_mode: WindowMode) -> std::io::Result<()> {
    let user_ltx_path = Path::new(USER_LTX);
    let mut lines = if user_ltx_path.exists() {
        fs::read_to_string(user_ltx_path)?.lines().map(|s| s.to_string()).collect::<Vec<_>>()
    } else {
        LINES.iter().map(|s| s.to_string()).collect::<Vec<_>>()
    };

    // delete old strings if they`re there
    lines.retain(|line| !line.starts_with("rs_borderless") && !line.starts_with("rs_fullscreen") && !line.starts_with("rs_screenmode"));

    // adding new strings (depends on selected mode)
    match window_mode {
        WindowMode::Fullscreen => {
            lines.push("rs_borderless 0".to_string());
            lines.push("rs_fullscreen on".to_string());
            lines.push("rs_screenmode fullscreen".to_string());
        },
        WindowMode::Windowed => {
            lines.push("rs_borderless 0".to_string());
            lines.push("rs_fullscreen off".to_string());
            lines.push("rs_screenmode windowed".to_string());
        },
        WindowMode::BorderlessWindowed => {
            lines.push("rs_borderless 1".to_string());
            lines.push("rs_fullscreen off".to_string());
            lines.push("rs_screenmode borderless".to_string());
        },
        _ => {} // Default mode
    }

    // putting updated strings back into the file
    fs::write(user_ltx_path, lines.join("\n"))?;
    Ok(())
}
const LINES: [&str; 302] = [
    "_preset Default",
    "ai_aim_max_angle 0.7854",
    "ai_aim_min_angle 0.19635",
    "ai_aim_min_speed 0.24",
    "ai_aim_predict_time 0.4",
    "ai_aim_use_smooth_aim 1",
    "ai_die_in_anomaly 0",
    "ai_use_old_vision 0",
    "ai_use_torch_dynamic_lights on",
    "default_controls",
    "bind left kLEFT",
    "bind right kRIGHT",
    "bind up kUP",
    "bind down kDOWN",
    "bind jump kSPACE",
    "bind crouch kLCONTROL",
    "bind accel kLSHIFT",
    "bind sprint_toggle kX",
    "bind forward kW",
    "bind back kS",
    "bind lstrafe kA",
    "bind rstrafe kD",
    "bind llookout kQ",
    "bind rlookout kE",
    "bind cam_zoom_in kT",
    "bind cam_zoom_out kRBRACKET",
    "bind torch kL",
    "bind night_vision kN",
    "bind show_detector kO",
    "bind wpn_1 k1",
    "bind wpn_2 k2",
    "bind wpn_3 k3",
    "bind wpn_4 k4",
    "bind wpn_5 k5",
    "bind wpn_6 k6",
    "bind wpn_next kY",
    "bind wpn_fire mouse1",
    "bind wpn_zoom mouse2",
    "bind wpn_reload kR",
    "bind wpn_func kV",
    "bind wpn_firemode_prev k9",
    "bind wpn_firemode_next k0",
    "bind pause kPAUSE",
    "bind drop kG",
    "bind use kF",
    "bind scores kTAB",
    "bind screenshot kF12",
    "bind quit kESCAPE",
    "bind console kGRAVE",
    "bind inventory kI",
    "bind active_jobs kP",
    "bind quick_use_1 kF1",
    "bind quick_use_2 kF2",
    "bind quick_use_3 kF3",
    "bind quick_use_4 kF4",
    "bind quick_save kF5",
    "bind quick_load kF9",
    "bind custom1 kNUMPAD1",
    "bind custom2 kNUMPAD2",
    "bind custom3 kNUMPAD3",
    "bind custom4 kNUMPAD4",
    "bind custom5 kNUMPAD5",
    "bind custom6 kNUMPAD0",
    "bind custom13 k7",
    "bind custom14 k8",
    "bind custom15 kU",
    "bind custom17 kF7",
    "bind custom18 mouse3",
    "bind safemode kB",
    "cam_inert 0.",
    "cam_slide_inert 0.25",
    "cl_cod_pickup_mode on",
    "cl_dynamiccrosshair on",
    "con_sensitive 0.15",
    "discord_status on",
    "discord_update_rate 0.5",
    "fov 75.",
    "g_3d_pda on",
    "g_always_active off",
    "g_autopickup on",
    "g_backrun on",
    "g_crosshair_color (255, 255, 255, 255)",
    "g_crouch_toggle off",
    "g_dead_body_collision actor_only",
    "g_dispersion_base 1.",
    "g_dispersion_factor 1.",
    "g_dynamic_music off",
    "g_end_modif 0.",
    "g_feel_grenade off",
    "g_firepos off",
    "g_freelook_toggle off",
    "g_game_difficulty gd_stalker",
    "g_god off",
    "g_hit_pwr_modif 1.",
    "g_important_save on",
    "g_ironsights_zoom_factor 1.25",
    "g_lookout_toggle off",
    "g_multi_item_pickup on",
    "g_simple_pda on",
    "g_sleep_time 1",
    "g_sprint_toggle on",
    "g_unlimitedammo off",
    "g_use_tracers on",
    "g_walk_toggle off",
    "head_bob_factor 1.",
    "hud_crosshair on",
    "hud_crosshair_dist off",
    "hud_draw on",
    "hud_fov 0.45",
    "hud_info on",
    "hud_weapon on",
    "keypress_on_start on",
    "log_missing_ini off",
    "mouse_invert off",
    "mouse_sens 0.12",
    "mouse_sens_aim 1.",
    "ph_frequency 100.00000",
    "ph_gravity 19.62",
    "ph_iterations 18",
    "power_loss_bias 0.25",
    "power_loss_factor 0.5",
    "r1_detail_textures off",
    "r1_dlights on",
    "r1_dlights_clip 40.",
    "r1_fog_luminance 1.1",
    "r1_glows_per_frame 16",
    "r1_lmodel_lerp 0.1",
    "r1_pps_u 0.",
    "r1_pps_v 0.",
    "r1_software_skinning 0",
    "r1_ssa_lod_a 64.",
    "r1_ssa_lod_b 48.",
    "r2_aa off",
    "r2_aa_break (0.800000, 0.100000, 0.000000)",
    "r2_aa_kernel 0.5",
    "r2_aa_weight (0.250000, 0.250000, 0.000000)",
    "r2_allow_r1_lights off",
    "r2_detail_bump on",
    "r2_dof -1.000000,0.000000,800.000000",
    "r2_dof_enable off",
    "r2_dof_radius 0.25",
    "r2_dof_sky 30.",
    "r2_drops_control (0.000000, 1.150000, 0.000000)",
    "r2_exp_donttest_shad off",
    "r2_gi off",
    "r2_gi_clip 0.001",
    "r2_gi_depth 1",
    "r2_gi_photons 16",
    "r2_gi_refl 0.9",
    "r2_gloss_factor 2.5",
    "r2_gloss_min 0.5",
    "r2_ls_bloom_fast off",
    "r2_ls_bloom_kernel_b 0.1",
    "r2_ls_bloom_kernel_g 1.",
    "r2_ls_bloom_kernel_scale 0.05",
    "r2_ls_bloom_speed 100.",
    "r2_ls_bloom_threshold 0.",
    "r2_ls_depth_bias -0.001",
    "r2_ls_depth_scale 1.00001",
    "r2_ls_dsm_kernel 0.7",
    "r2_ls_psm_kernel 0.7",
    "r2_ls_squality 1.",
    "r2_ls_ssm_kernel 0.7",
    "r2_mask_control (0.000000, 0.000000, 0.000000, 0.000000)",
    "r2_mblur 0.",
    "r2_mblur_enabled off",
    "r2_parallax_h 0.02",
    "r2_qsync 0",
    "r2_shadow_cascede_old off",
    "r2_slight_fade 0.5",
    "r2_smaa high",
    "r2_soft_particles on",
    "r2_soft_water on",
    "r2_ss_sunshafts_length 1.",
    "r2_ss_sunshafts_radius 1.",
    "r2_ssa_lod_a 64.",
    "r2_ssa_lod_b 48.",
    "r2_ssao st_opt_high",
    "r2_ssao_blur off",
    "r2_ssao_half_data on",
    "r2_ssao_hbao off",
    "r2_ssao_hdao off",
    "r2_ssao_mode hdao",
    "r2_ssao_opt_data off",
    "r2_steep_parallax on",
    "r2_sun on",
    "r2_sun_depth_far_bias -0.00002",
    "r2_sun_depth_far_scale 1.",
    "r2_sun_depth_near_bias 0.00001",
    "r2_sun_depth_near_scale 1.",
    "r2_sun_details off",
    "r2_sun_far 100.",
    "r2_sun_focus on",
    "r2_sun_lumscale 2.",
    "r2_sun_lumscale_amb 1.",
    "r2_sun_lumscale_hemi 1.",
    "r2_sun_near 20.",
    "r2_sun_near_border 0.75",
    "r2_sun_quality st_opt_medium",
    "r2_sun_tsm on",
    "r2_sun_tsm_bias -0.01",
    "r2_sun_tsm_proj 0.3",
    "r2_sunshafts_min 0.",
    "r2_sunshafts_mode volumetric",
    "r2_sunshafts_quality st_opt_medium",
    "r2_sunshafts_value 0.75",
    "r2_terrain_z_prepass off",
    "r2_tnmp_a 0.15",
    "r2_tnmp_b 0.5",
    "r2_tnmp_c 0.1",
    "r2_tnmp_d 0.2",
    "r2_tnmp_e 0.2",
    "r2_tnmp_exposure 0.16033",
    "r2_tnmp_f 0.3",
    "r2_tnmp_gamma 0.76667",
    "r2_tnmp_onoff 0.",
    "r2_tnmp_w 1.12",
    "r2_tonemap on",
    "r2_tonemap_adaptation 10.",
    "r2_tonemap_amount 1.",
    "r2_tonemap_lowlum 0.5",
    "r2_tonemap_middlegray 1.5",
    "r2_volumetric_lights on",
    "r2_wait_sleep 0",
    "r2_water_reflections on",
    "r2_zfill off",
    "r2_zfill_depth 0.25",
    "r2em 0.",
    "r3_dynamic_wet_surfaces on",
    "r3_dynamic_wet_surfaces_far 100.",
    "r3_dynamic_wet_surfaces_near 70.",
    "r3_dynamic_wet_surfaces_sm_res 2048",
    "r3_minmax_sm off",
    "r3_msaa st_opt_off",
    "r3_msaa_alphatest st_opt_off",
    "r3_use_dx10_1 off",
    "r3_volumetric_smoke on",
    "r4_enable_tessellation on",
    "r4_wireframe off",
    "r__actor_shadow off",
    "r__bloom_thresh (0.700000, 0.800000, 0.900000, 0.000000)",
    "r__bloom_weight (0.330000, 0.330000, 0.330000, 0.000000)",
    "r__clear_models_on_unload off",
    "r__color_grading (0.000000, 0.000000, 0.000000)",
    "r__detail_density 0.6",
    "r__detail_height 1.",
    "r__detail_radius 49",
    "r__dtex_range 50.",
    "r__enable_grass_shadow off",
    "r__exposure 1.",
    "r__framelimit 0",
    "r__gamma 1.",
    "r__geometry_lod 0.75",
    "r__lens_flares on",
    "r__nightvision 0",
    "r__no_ram_textures on",
    "r__no_scale_on_fade off",
    "r__optimize_dynamic_geom 1",
    "r__optimize_shadow_geom on",
    "r__optimize_static_geom 1",
    "r__saturation 1.",
    "r__supersample 1",
    "r__tf_aniso 8",
    "r__tf_mipbias -0.5",
    "r__use_precompiled_shaders off",
    "r__wallmark_ttl 50.",
    "r_screenshot_mode jpg",
    "renderer renderer_r4",
    "rs_c_brightness 1.",
    "rs_c_contrast 1.",
    "rs_c_gamma 1.",
    "rs_cam_pos off",
    "rs_screenmode borderless",
    "rs_skeleton_update 32",
    "rs_stats off",
    "rs_v_sync on",
    "rs_vis_distance 1.",
    "shader_param_1 (0.000000, 0.000000, 0.000000, 0.000000)",
    "shader_param_2 (0.000000, 0.000000, 0.000000, 0.000000)",
    "shader_param_3 (0.000000, 0.000000, 0.000000, 0.000000)",
    "shader_param_4 (0.000000, 0.000000, 0.000000, 0.000000)",
    "shader_param_5 (0.000000, 0.000000, 0.000000, 0.000000)",
    "shader_param_6 (0.000000, 0.000000, 0.000000, 0.000000)",
    "shader_param_7 (0.000000, 0.000000, 0.000000, 0.000000)",
    "shader_param_8 (0.000000, 0.000000, 0.000000, 0.000000)",
    "slot_0 medkit",
    "slot_1 bandage",
    "slot_2 antirad",
    "slot_3 conserva",
    "snd_acceleration on",
    "snd_cache_size 256",
    "snd_device OpenAL Soft",
    "snd_efx off",
    "snd_targets 1024",
    "snd_volume_eff 1.",
    "snd_volume_music 1.",
    "texture_lod 1",
    "time_factor 1.000000",
    "vid_mode 1024x768",
    "weapon_sway off",
    "wpn_aim_toggle off",
    "wpn_degradation 1.",
];

pub fn reset_user_ltx() -> std::io::Result<()> {
    if Path::new(USER_LTX).exists() {
        if Path::new(USER_LTX_OLD).exists() {
            fs::remove_file(USER_LTX_OLD)?;
        }
        fs::rename(USER_LTX, USER_LTX_OLD)?;
    }
    fs::write(USER_LTX, LINES.join("\n"))?;
    Ok(())
}