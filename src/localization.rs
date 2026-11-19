use sys_locale::get_locale;

pub struct Localization {
    pub play_button: &'static str,
    pub clear_shader_cache_button: &'static str,
    pub reset_user_ltx_button: &'static str,
    pub verify_installation_button: &'static str,
    pub quit_button: &'static str,
    pub renderer: &'static str,
    pub shadowmap: &'static str,
    pub winmode: &'static str,
    pub fullscr: &'static str,
    pub wined: &'static str,
    pub borlesswined: &'static str,
    pub misclabel: &'static str,
    pub debug_mode: &'static str,
    pub pref_sounds: &'static str,
    pub workaround: &'static str,
    pub useavx: &'static str,
    pub notsupportedavx: &'static str,
    pub useaffinity: &'static str,
    pub customlaunch: &'static str,

    pub renderer_hover: &'static str,
    pub shadowmap_hover: &'static str,
    pub debug_mode_hover: &'static str,
    pub pref_sounds_hover: &'static str,
    pub workaround_hover: &'static str,
    pub useavx_hover: &'static str,
    pub useaffinity_hover: &'static str,
    pub customlaunch_hover: &'static str,

    pub warning1: &'static str,
    pub warning2: &'static str,
}

impl Localization {
    pub fn new() -> Self {
        let lang = get_locale()
        .unwrap_or_else(|| "en".to_string())
        .chars()
        .take(2)
        .collect::<String>();
        println!("Detected language: {}", lang);

        match lang.as_str() {
            "ru" => Localization {
                play_button: "Играть",
                clear_shader_cache_button: "      Очистить\nкеш шейдеров",
                reset_user_ltx_button: "Сбросить\n    User.ltx",
                verify_installation_button: "Проверить\nустановку",
                quit_button: "Выйти",
                renderer: "Рендер движок",
                shadowmap: "Разрешение теней",
                winmode: "Режим окна",
                fullscr: "Полноэкранный режим",
                wined: "Оконный режим",
                borlesswined: "Безрамочный оконный режим",
                misclabel: "Дополнительные настройки",
                debug_mode: "Режим отладки",
                pref_sounds: "Предзагрузка звуков",
                workaround: "Обход ошибки для звука",
                useavx: "Использовать AVX",
                notsupportedavx: "AVX не поддерживается!",
                useaffinity: "Использовать привязку к ядрам",
                customlaunch: "Кастомные параметры запуска",

                renderer_hover: "Графический движок для Аномали\n Dx8 для слабых пк \n Dx11 для пк помощнее",
                shadowmap_hover: "Разрешение теней в игре\n Больше - красивее\n Меньше - лучше фпс",
                debug_mode_hover: "Дебаг режим игры, для спавна предметов/нпс и т.д. или просто если ты моддер :D",
                pref_sounds_hover: "Загружает звуки заранее, может помочь с микро-фризами",
                workaround_hover: "Может помочь с проблемами со звуком",
                useavx_hover: "На некоторых устройствах может немного улучшить производительность",
                useaffinity_hover: "Иногда может помочь с микро-фризами фпс, если упор в процессор",
                customlaunch_hover: "Используй только если знаешь, что делаешь",

                warning1: "Установленные аддоны могут вызвать проблемы или вылеты",
                warning2: "Папка gamedata не найдена",
            },
            "es" => Localization {
                play_button: "Jugar",
                clear_shader_cache_button: "             Limpiar\nCaché de Shaders",
                reset_user_ltx_button: "Reiniciar\n User.ltx",
                verify_installation_button: "   Verificar\nInstalación",
                quit_button: "Salir",
                renderer: "Renderizador",
                shadowmap: "Tamaño del Mapa de Sombra",
                winmode: "Modo Ventana",
                fullscr: "Pantalla completa",
                wined: "Ventana",
                borlesswined: "Ventana sin bordes",
                misclabel: "Ajustes",
                debug_mode: "Modo Debug",
                pref_sounds: "Prefabricar Audio",
                workaround: "Arreglo para el problema de sonido",
                useavx: "Usar AVX",
                notsupportedavx: "AVX no soportado!",
                useaffinity: "Usar Afinidad de CPU",
                customlaunch: "Parámetros modificados de lanzamiento",

                renderer_hover: "Motor Gráfico de Anomaly\n Dx8 para PCs de gama baja\n Dx11 para PCs más potentes",
                shadowmap_hover: "Resolución de Sombra en el juego\n Alta - mejor detalle\n Baja - mejor FPS",
                debug_mode_hover: "Modo desarrollador, para generar objetos, NPCs, etc... o si eres modder :D",
                pref_sounds_hover: "Pre-cargar audio. Podría ayudar con trompicones",
                workaround_hover: "Podría ayudar si tienes problema con el audio",
                useavx_hover: "En algunos dispositivos, podria mejorar ligeramente el rendimiento",
                useaffinity_hover: "A veces puede ayudar con micro trompicones de FPS si la CPU es un cuello de botella",
                customlaunch_hover: "Usar sólo si sabes lo que estás haciendo",

                warning1: "Addons instalados pueden causar problemas o caídas al escritorio",
                warning2: "Carpeta Gamedata no encontrada",
            },
            _ => Localization {
                play_button: "Play",
                clear_shader_cache_button: "        Clear\nShader Cache",
                reset_user_ltx_button: "  Reset\nUser.ltx",
                verify_installation_button: "      Verify\nInstallation",
                quit_button: "Quit",
                renderer: "Renderer",
                shadowmap: "Shadow Map Size",
                winmode: "Window Mode",
                fullscr: "Fullscreen",
                wined: "Windowed",
                borlesswined: "Borderless Windowed",
                misclabel: "Misc settings",
                debug_mode: "Debug Mode",
                pref_sounds: "Prefetch Sounds",
                workaround: "Workaround for the sound problem",
                useavx: "Use AVX",
                notsupportedavx: "AVX not supported!",
                useaffinity: "Use CPU Affinity",
                customlaunch: "Custom launch parameters",

                renderer_hover: "Graphics engine for Anomaly\n Dx8 for weaker PCs\n Dx11 for more powerful PCs",
                shadowmap_hover: "Shadow resolution in the game\n Higher - more beautiful\n Lower - better FPS",
                debug_mode_hover: "Debug mode for the game, for spawning items/NPCs, etc., or just if you're a modder :D",
                pref_sounds_hover: "Pre-loads sounds. Might help with stutters",
                workaround_hover: "Might help if you have problem with sounds",
                useavx_hover: "On some devices, it may slightly improve performance",
                useaffinity_hover: "Sometimes it can help with micro FPS stutters if the bottleneck is the CPU",
                customlaunch_hover: "Use only if you know what you are doing",

                warning1: "Installed addons can cause problems or crashes",
                warning2: "Gamedata folder not found",
            },
        }
    }
}
