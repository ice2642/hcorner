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
    git clone [https://github.com/ice2642/hcorner.git](https://github.com/ice2642/hcorner.git)
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

Se o arquivo ou uma das quatro opções (TOP_LEFT, TOP_RIGHT, BOTTOM_LEFT, BOTTOM_RIGHT) estiverem faltando, o programa não será executado.

### Formato do Arquivo

Cada linha deve ter o formato `CANTO="comando shell",status`.

* **comando shell:** O comando que será executado (ex: `"xterm"`, `"nautilus /home/user"`, etc.).
* **status:** `1` para **ativado** (o canto irá disparar); `0` para **desativado** (o canto será ignorado).

### Exemplo: `~/.config/hcorner.conf`
