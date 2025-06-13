//********************************LOCALISATION*********************************

pub struct Localization {
    pub play_button: String,
    pub clear_shader_cache_button: String,
    pub reset_user_ltx_button: String,
    pub verify_installation_button: String,
    pub quit_button: String,
    pub renderer: String,
    pub shadowmap: String,
    pub winmode: String,
    pub fullscr: String,
    pub wined: String,
    pub borlesswined: String,
    pub misclabel: String,
    pub debug_mode: String,
    pub pref_sounds: String,
    pub workaround: String,
    pub useavx: String,
    pub notsupportedavx: String,
    pub useaffinity: String,
    pub customlaunch: String,


    pub renderer_hover: String,
    pub shadowmap_hover: String,
    pub debug_mode_hover: String,
    pub pref_sounds_hover: String,
    pub workaround_hover: String,
    pub useavx_hover: String,
    pub useaffinity_hover: String,
    pub customlaunch_hover: String,

    pub warning1: String,
    pub warning2: String,
    

    // add new strings here if needed
}
use sys_locale::get_locale;
impl Localization {
    // FUNCTION TO LOAD STRINGS DEPENDING ON YOUR SYSTEM LANGUAGE HOOLY MOLY
    pub fn new() -> Self {
        let lang = get_locale().unwrap_or_else(|| "en".to_string()).chars().take(2).collect::<String>();
        println!("Detected language: {}", lang);
        match lang.as_str() {
            "ru" => Localization {
                play_button: "Играть".to_string(),
                clear_shader_cache_button: "      Очистить\nкеш шейдеров".to_string(),
                reset_user_ltx_button: "Сбросить\n    User.ltx".to_string(),
                verify_installation_button: "Проверить\nустановку".to_string(),
                quit_button: "Выйти".to_string(),
                renderer: "Рендер движок".to_string(),
                shadowmap: "Разрешение теней".to_string(),
                winmode: "Режим окна".to_string(),
                fullscr: "Полноэкранный режим".to_string(),
                wined: "Оконный режим".to_string(),
                borlesswined: "Безрамочный оконный режим".to_string(),
                misclabel: "Дополнительные настройки".to_string(),
                debug_mode: "Режим отладки".to_string(),
                pref_sounds: "Предзагрузка звуков".to_string(),
                workaround: "Обход ошибки для звука".to_string(),
                useavx: "Использовать AVX".to_string(),
                notsupportedavx: "AVX не поддерживается!".to_string(),
                useaffinity: "Использовать привязку к ядрам".to_string(),
                customlaunch: "Кастомные параметры запуска".to_string(),



                renderer_hover: "Графический движок для Аномали\n Dx8 для слабых пк \n Dx11 для пк помощнее".to_string(),
                shadowmap_hover: "Разрешение теней в игре\n Больше - красивее\n Меньше - лучше фпс".to_string(),
                debug_mode_hover: "Дебаг режим игры, для спавна предметов/нпс и т.д. или просто если ты моддер :D".to_string(),
                pref_sounds_hover: "Загружает звуки заранее, может помочь с микро-фризами".to_string(),
                workaround_hover: "Может помочь с проблемами со звуком".to_string(),
                useavx_hover: "На некоторых устройствах может немного улучшить производительность".to_string(),
                useaffinity_hover: "Иногда может помочь с микро-фризами фпс, если упор в процессор".to_string(),
                customlaunch_hover: "Используй только если знаешь, что делаешь".to_string(),

                warning1: "Установленные аддоны могут вызвать проблемы или вылеты".to_string(),
                warning2: "Папка gamedata не найдена".to_string(),
                
            },
            "es" => Localization {
                play_button: "Jugar".to_string(),
                clear_shader_cache_button: "             Limpiar\nCaché de Shaders".to_string(),
                reset_user_ltx_button: "Reiniciar\n User.ltx".to_string(),
                verify_installation_button: "   Verificar\nInstalación".to_string(),
                quit_button: "Salir".to_string(),
                renderer: "Renderizador".to_string(),
                shadowmap: "Tamaño del Mapa de Sombra".to_string(),
                winmode: "Modo Ventana".to_string(),
                fullscr: "Pantalla completa".to_string(),
                wined: "Ventana".to_string(),
                borlesswined: "Ventana sin bordes".to_string(),
                misclabel: "Ajustes".to_string(),
                debug_mode: "Modo Debug".to_string(),
                pref_sounds: "Prefabricar Audio".to_string(),
                workaround: "Arreglo para el problema de sonido".to_string(),
                useavx: "Usar AVX".to_string(),
                notsupportedavx: "AVX no soportado!".to_string(),
                useaffinity: "Usar Afinidad de CPU".to_string(),
                customlaunch: "Parámetros modificados de lanzamiento".to_string(),

                renderer_hover: "Motor Gráfico de Anomaly\n Dx8 para PCs de gama baja\n Dx11 para PCs más potentes".to_string(),
                shadowmap_hover: "Resolución de Sombra en el juego\n Alta - mejor detalle\n Baja - mejor FPS".to_string(),
                debug_mode_hover: "Modo desarrollador, para generar objetos, NPCs, etc... o si eres modder :D".to_string(),
                pref_sounds_hover: "Pre-cargar audio. Podría ayudar con trompicones".to_string(),
                workaround_hover: "Podría ayudar si tienes problema con el audio".to_string(),
                useavx_hover: "En algunos dispositivos, podria mejorar ligeramente el rendimiento".to_string(),
                useaffinity_hover: "A veces puede ayudar con micro trompicones de FPS si la CPU es un cuello de botella".to_string(),
                customlaunch_hover: "Usar sólo si sabes lo que estás haciendo".to_string(),

                warning1: "Addons instalados pueden causar problemas o caídas al escritorio".to_string(),
                warning2: "Carpeta Gamedata no encontrada".to_string(),
            },
            _ => Localization {
                play_button: "Play".to_string(),
                clear_shader_cache_button: "        Clear\nShader Cache".to_string(),
                reset_user_ltx_button: "  Reset\nUser.ltx".to_string(),
                verify_installation_button: "      Verify\nInstallation".to_string(),
                quit_button: "Quit".to_string(),
                renderer: "Renderer".to_string(),
                shadowmap: "Shadow Map Size".to_string(),
                winmode: "Window Mode".to_string(),
                fullscr: "Fullscreen".to_string(),
                wined: "Windowed".to_string(),
                borlesswined: "Borderless Windowed".to_string(),
                misclabel: "Misc settings".to_string(),
                debug_mode: "Debug Mode".to_string(),
                pref_sounds: "Prefetch Sounds".to_string(),
                workaround: "Workaround for the sound problem".to_string(),
                useavx: "Use AVX".to_string(),
                notsupportedavx: "AVX not supported!".to_string(),
                useaffinity: "Use CPU Affinity".to_string(),
                customlaunch: "Custom launch parameters".to_string(),

                renderer_hover: "Graphics engine for Anomaly\n Dx8 for weaker PCs\n Dx11 for more powerful PCs".to_string(),
                shadowmap_hover: "Shadow resolution in the game\n Higher - more beautiful\n Lower - better FPS".to_string(),
                debug_mode_hover: "Debug mode for the game, for spawning items/NPCs, etc., or just if you're a modder :D".to_string(),
                pref_sounds_hover: "Pre-loads sounds. Might help with stutters".to_string(),
                workaround_hover: "Might help if you have problem with sounds".to_string(),
                useavx_hover: "On some devices, it may slightly improve performance".to_string(),
                useaffinity_hover: "Sometimes it can help with micro FPS stutters if the bottleneck is the CPU".to_string(),
                customlaunch_hover: "Use only if you know what you are doing".to_string(),

                warning1: "Installed addons can cause problems or crashes".to_string(),
                warning2: "Gamedata folder not found".to_string(),
            },
        }
    }
}