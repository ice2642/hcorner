use std::{
    fs,
    thread,
    time::{Duration, Instant},
    process::Command,
};
use x11::xlib::{self, Display, Window, XDefaultRootWindow, XOpenDisplay, XQueryPointer};
use dirs; // Adicionado para corrigir o erro E0433

// --- Estruturas de Configuração ---

#[derive(Debug)]
struct CornerConfig {
    command: String,
    enabled: bool,
}

#[derive(Debug)]
struct HCornerConfig {
    top_left: CornerConfig,
    top_right: CornerConfig,
    bottom_left: CornerConfig, // Novo
    bottom_right: CornerConfig, // Novo
}

// --- Funções de Leitura de Configuração ---

fn parse_config_line(line: &str) -> Option<CornerConfig> {
    // Ex: "xterm",1
    
    // Usamos splitn(2) para garantir que temos no máximo 2 partes: comando e status.
    let parts: Vec<&str> = line.splitn(2, ',')
                                .map(|s| s.trim())
                                .collect();

    // Verificamos se há exatamente 2 partes (CORRIGIDO)
    if parts.len() == 2 {
        let command_raw = parts[0];
        let enabled_str = parts[1];
        
        // Remove as aspas do comando
        let command = command_raw.trim_matches('"').to_string();
        let enabled = enabled_str == "1";

        return Some(CornerConfig { command, enabled });
    }
    None
}

fn load_config() -> Result<HCornerConfig, String> {
    let mut config_path = match dirs::config_dir() { // `dirs::config_dir` AGORA FUNCIONA
        Some(p) => p,
        None => return Err("Não foi possível determinar o diretório de configuração do usuário.".to_string()),
    };
    config_path.push("hcorner.conf"); 

    let contents = fs::read_to_string(&config_path)
        .map_err(|e| format!("Erro ao ler o arquivo de configuração {:?}: {}", config_path, e))?;

    let mut top_left: Option<CornerConfig> = None;
    let mut top_right: Option<CornerConfig> = None;
    let mut bottom_left: Option<CornerConfig> = None;
    let mut bottom_right: Option<CornerConfig> = None;

    for line in contents.lines().map(|l| l.trim()).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        // Renomeando LEFT_CORNER para TOP_LEFT para ser mais específico
        if line.starts_with("TOP_LEFT=") || line.starts_with("LEFT_CORNER=") {
            let value = line.split_once('=').map(|(_, v)| v.trim()).unwrap_or("");
            top_left = parse_config_line(value);
        } else if line.starts_with("TOP_RIGHT=") || line.starts_with("RIGHT_CORNER=") {
            let value = line.split_once('=').map(|(_, v)| v.trim()).unwrap_or("");
            top_right = parse_config_line(value);
        } else if line.starts_with("BOTTOM_LEFT=") {
            let value = line.split_once('=').map(|(_, v)| v.trim()).unwrap_or("");
            bottom_left = parse_config_line(value);
        } else if line.starts_with("BOTTOM_RIGHT=") {
            let value = line.split_once('=').map(|(_, v)| v.trim()).unwrap_or("");
            bottom_right = parse_config_line(value);
        }
    }

    match (top_left, top_right, bottom_left, bottom_right) {
        (Some(tl), Some(tr), Some(bl), Some(br)) => Ok(HCornerConfig { top_left: tl, top_right: tr, bottom_left: bl, bottom_right: br }),
        _ => Err("Configuração incompleta ou inválida. Verifique TOP_LEFT, TOP_RIGHT, BOTTOM_LEFT e BOTTOM_RIGHT.".to_string()),
    }
}


// --- Interação com X11 e Lógica Principal ---

// A função unsafe é necessária para chamadas Xlib.
unsafe fn get_screen_and_pointer_info(display: *mut Display) -> Option<(i32, i32, i32, i32)> {
    let root = XDefaultRootWindow(display);
    
    // Obter o tamanho da tela
    let screen = xlib::XDefaultScreen(display);
    let screen_width = xlib::XDisplayWidth(display, screen);
    let screen_height = xlib::XDisplayHeight(display, screen); // Agora será usada

    // Variáveis para a posição do ponteiro
    let mut root_return: Window = 0;
    let mut child_return: Window = 0;
    let mut root_x_return: i32 = 0;
    let mut root_y_return: i32 = 0;
    let mut win_x_return: i32 = 0;
    let mut win_y_return: i32 = 0;
    let mut mask_return: u32 = 0;

    // Obter a posição do ponteiro
    if XQueryPointer(
        display,
        root,
        &mut root_return,
        &mut child_return,
        &mut root_x_return,
        &mut root_y_return,
        &mut win_x_return,
        &mut win_y_return,
        &mut mask_return,
    ) == xlib::True
    {
        // Retorna (largura da tela, altura da tela, X do mouse, Y do mouse)
        Some((screen_width, screen_height, root_x_return, root_y_return))
    } else {
        None
    }
}

// --- Função para executar o comando ---

fn execute_command(command: &str) {
    println!("Executando comando: '{}'", command);
    // Executa o comando em um processo separado (detach) para não bloquear o hcorner
    if let Err(e) = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
    {
        eprintln!("Erro ao executar o comando '{}': {}", command, e);
    }
}

// --- Main ---

fn main() -> Result<(), String> {
    // 1. Carregar configuração
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Erro de Configuração: {}", e);
            return Err("Falha na inicialização.".to_string());
        }
    };

    println!("Configuração carregada: {:?}", config);

    // 2. Conectar ao servidor X
    let display = unsafe { XOpenDisplay(std::ptr::null()) };
    if display.is_null() {
        return Err("Não foi possível conectar ao servidor X.".to_string());
    }

    // 3. Loop principal de monitoramento
    let mut last_x = -1;
    let mut last_y = -1;
    let mut last_move_time = Instant::now();
    let delay_threshold = Duration::from_millis(200); // 1/5 de segundo = 200ms
    let corner_trigger_pause = Duration::from_secs(1); // Pausa após um trigger

    println!("Iniciando monitoramento do mouse...");

    loop {
        let (screen_width, screen_height, mouse_x, mouse_y) = match unsafe { get_screen_and_pointer_info(display) } {
            Some(info) => info,
            None => {
                thread::sleep(Duration::from_millis(10));
                continue;
            }
        };

        // 3a. Lógica de movimento e temporizador
        if mouse_x != last_x || mouse_y != last_y {
            // Mouse se moveu, reseta o temporizador
            last_move_time = Instant::now();
            last_x = mouse_x;
            last_y = mouse_y;
        } else {
            // Mouse parado
            if last_move_time.elapsed() >= delay_threshold {
                
                let tolerance = 1; // Píxel de tolerância para o canto
                let mut triggered = false;

                // Canto Superior Esquerdo (0, 0)
                if mouse_x <= tolerance && mouse_y <= tolerance && config.top_left.enabled {
                    execute_command(&config.top_left.command);
                    triggered = true;
                } 
                // Canto Superior Direito (screen_width, 0)
                else if mouse_x >= screen_width - 1 - tolerance && mouse_y <= tolerance && config.top_right.enabled {
                    execute_command(&config.top_right.command);
                    triggered = true;
                }
                // Canto Inferior Esquerdo (0, screen_height)
                else if mouse_x <= tolerance && mouse_y >= screen_height - 1 - tolerance && config.bottom_left.enabled {
                    execute_command(&config.bottom_left.command);
                    triggered = true;
                }
                // Canto Inferior Direito (screen_width, screen_height)
                else if mouse_x >= screen_width - 1 - tolerance && mouse_y >= screen_height - 1 - tolerance && config.bottom_right.enabled {
                    execute_command(&config.bottom_right.command);
                    triggered = true;
                }

                // Pausa após qualquer trigger para evitar repetição instantânea
                if triggered {
                    thread::sleep(corner_trigger_pause);
                    // Opcional: Resetar o last_move_time para forçar o temporizador a começar do zero
                    last_move_time = Instant::now(); 
                }
            }
        }

        // Pequena pausa para economizar CPU
        thread::sleep(Duration::from_millis(50));
    }
}
