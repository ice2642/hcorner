# hcorner: Hot Corner para X11 (Rust)

**hcorner** é um utilitário de canto de tela (*hot corner*) leve e minimalista escrito em Rust, projetado para ambientes de desktop baseados em X11, como **Openbox**, **LXDE**, **Fluxbox** e outros gerenciadores de janela sem compositores pesados.

O programa monitora a posição do mouse e, se ele permanecer parado em um dos quatro cantos da tela por um tempo configurável (padrão: 200ms), executa um comando shell definido pelo usuário.

---

## 🚀 Instalação

### Pré-requisitos

Você precisa do **Rust Toolchain** (via `rustup`) e dos *headers* da biblioteca X11 instalados no seu sistema.

* **Rust:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* **X11 Headers (Debian/Ubuntu/Antix):** `sudo apt install libx11-dev`

### Compilação

1.  Clone este repositório:
    ```bash
    git clone https://github.com/ice2642/hcorner.git
    cd hcorner
    ```
2.  Compile o projeto (a flag `--release` gera um binário otimizado e muito mais leve):
    ```bash
    cargo build --release
    ```
3.  O executável estará em `./target/release/hcorner`. Você pode movê-lo para um diretório no seu `PATH`, como `/usr/local/bin`.
    ```bash
    sudo cp target/release/hcorner /usr/local/bin/
    ```

---

## ⚙️ Configuração

O **hcorner** lê sua configuração no arquivo **`~/.config/hcorner.conf`**.

O programa exige que os quatro cantos (`TOP_LEFT`, `TOP_RIGHT`, `BOTTOM_LEFT`, `BOTTOM_RIGHT`) estejam definidos no arquivo. Se você não quiser usar um canto, defina o *status* como `0`.

### Formato do Arquivo

Cada linha deve ter o formato `CANTO="comando shell",status`.

* **comando shell:** O comando que será executado (ex: `"xterm"`, `"nautilus /home/user"`, etc.).
* **status:** `1` para **ativado** (o canto irá disparar); `0` para **desativado** (o canto será ignorado).

 No hcorner.conf que voce colocar em seu .confg não pode ter comentarios ou o programa dará erro.

 Exemplo: `~/.config/hcorner.conf`  << Este programa deve estar em seu home/.config/

 --- Exemplo de Configuração de hcorner ---

 Canto Superior Esquerdo: Abre o xterm, ATIVADO (1)
 Canto Inferior Direito: Usa o comando 'true' (não faz nada), DESATIVADO (0)
 Canto Inferior Esquerdo: Mata todos os processos do xterm, ATIVADO (1)
 Canto Superior Direito: Abre o Firefox, DESATIVADO (0)

```ini

TOP_LEFT="xterm",1
TOP_RIGHT="/usr/bin/firefox -new-window",0
BOTTOM_LEFT="killall xterm",1
BOTTOM_RIGHT="true",0
```

Eu por exemplo uso o skippy-xd para ter um efeito estilo expoze em meu LXDE.

🇬🇧 English Translation
Here is the English translation of the README.md file.

hcorner: Hot Corner for X11 (Rust)
hcorner is a lightweight and minimal screen corner utility (hot corner) written in Rust, designed for X11-based desktop environments, such as Openbox, LXDE, Fluxbox, and other window managers without heavy compositors.

The program monitors the mouse position and, if it remains stationary in one of the four screen corners for a configurable time (default: 200ms), it executes a user-defined shell command.

🚀 Installation
Prerequisites
You need the Rust Toolchain (via rustup) and the X11 library headers installed on your system.

Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

X11 Headers (Debian/Ubuntu/Antix): sudo apt install libx11-dev

Compilation
Clone this repository:

Bash

git clone https://github.com/ice2642/hcorner.git
cd hcorner
Compile the project (the --release flag generates an optimized and much lighter binary):

Bash

cargo build --release
The executable will be in ./target/release/hcorner. You can move it to a directory in your PATH, such as /usr/local/bin.

Bash

sudo cp target/release/hcorner /usr/local/bin/
⚙️ Configuration
hcorner reads its configuration from the file ~/.config/hcorner.conf.

The program requires that all four corners (TOP_LEFT, TOP_RIGHT, BOTTOM_LEFT, BOTTOM_RIGHT) be defined in the file. If you do not want to use a corner, set the status to 0.

File Format
Each line must have the format CORNER="shell command",status.

shell command: The command to be executed (e.g., "xterm", "nautilus /home/user", etc.).

status: 1 for enabled (the corner will trigger); 0 for disabled (the corner will be ignored).

The hcorner.conf file you place in your .config must not contain comments, or the program will throw an error.

Example: ~/.config/hcorner.conf << This program must be in your home/.config/

--- Example hcorner Configuration ---

Top Left Corner: Opens xterm, ENABLED (1) Bottom Right Corner: Uses the 'true' command (does nothing), DISABLED (0) Bottom Left Corner: Kills all xterm processes, ENABLED (1) Top Right Corner: Opens Firefox, DISABLED (0)

Ini, TOML

TOP_LEFT="xterm",1
TOP_RIGHT="/usr/bin/firefox -new-window",0
BOTTOM_LEFT="killall xterm",1
BOTTOM_RIGHT="true",0
For example, I use skippy-xd to get an expose-style effect in my LXDE.
