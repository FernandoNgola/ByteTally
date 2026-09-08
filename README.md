<<<<<<< HEAD
# ByteTally

Ferramenta Rust em fase inicial para contabilizar tráfego por IP.

## Funcionalidade disponível

A CLI lê um ficheiro de registos, soma os bytes por endereço IPv4/IPv6 e
mostra um relatório por consumo decrescente. Empates são ordenados pelo IP.
A leitura é incremental; a memória cresce com o número de IPs distintos.

Ainda não captura pacotes, lê PCAP, aplica bloqueios ou persiste dados.
O adaptador de comandos do sistema existe, mas não é chamado pela CLI.

## Executar

Requer Rust 1.98 ou superior. O ficheiro `rust-toolchain.toml` selecciona
stable com rustfmt e Clippy. Não são necessárias dependências externas.

```sh
cargo run -- analyze examples/traffic.txt
cargo run -- --help
cargo run -- --version
```

Saída do exemplo (colunas separadas por tabulação):

```text
IP              BYTES
2001:db8::1      4096
192.168.0.10    3072
192.168.0.20    512
```

O formato é texto com `IP BYTES` por linha, separados por espaços ou tabulações:

```text
# Bytes atribuídos a cada IP; não são pacotes PCAP
192.168.0.10 1024
2001:db8::1 4096 # comentário opcional
192.168.0.10 2048
```

Linhas vazias e comentários são ignorados. Bytes são inteiros decimais não
negativos até `u64::MAX`; a soma por IP também deve caber nesse intervalo.
Cada registo atribui bytes a um único endereço, sem inferir direcção de tráfego.
Registos inválidos produzem erro com número de linha, código de saída 1 e nenhum
relatório parcial. Um ficheiro vazio produz uma mensagem sem registos.

## Estrutura

```text
src/
├── main.rs             # Arranque e códigos de saída
├── lib.rs              # Módulos da biblioteca
├── core.rs
├── core/tracker.rs     # Contadores e ordenação
├── net.rs
├── net/analyzer.rs     # Leitura e validação de registos
├── system.rs
├── system/executor.rs  # Execução real e simulação de comandos
├── ui.rs
└── ui/cli.rs           # Argumentos e relatório
examples/traffic.txt    # Entrada de demonstração
tests/cli.rs           # Testes do executável
```

As declarações de submódulos ficam nos ficheiros pais (`core.rs`, por exemplo).
Funcionalidades futuras estão no [roadmap](ROADMAP.md), sem módulos vazios.
As dependências comentadas no manifesto são sugestões históricas, não activas.

## Verificar

```sh
cargo fmt --all -- --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
```

Veja [CONTRIBUTING.md](CONTRIBUTING.md). Licença [MIT](LICENSE).
||||||| 52e48ee
﻿# ByteTally — Monitoramento e Controle de Largura de Banda por IP

**ByteTally** é uma ferramenta escrita em **Rust** para monitorar, analisar e controlar o uso de largura de banda em redes locais. Focada em **performance, governança e simplicidade**, ela inspeciona pacotes, contabiliza tráfego por IP, aplica cotas (limites de uso), executa bloqueios dinâmicos e fornece interfaces para visualização e controle — tudo com baixo consumo de recursos.

Ideal para **pequenas e médias empresas, hotspots, redes domésticas, laboratórios e ambientes com conectividade limitada**.

## Funcionalidades
- Captura de tráfego em tempo real por IP (via `pnet`).
- Interface TUI interativa (com `ratatui`).
- Interface por linha de comando (CLI) para execução de comandos e relatórios.
- Leitura e análise de arquivos `.pcap` para identificação de IPs com maior consumo.
- Controle por janelas de tempo (ex: 24h) com resets automáticos.
- Políticas configuráveis via TOML/JSON.
- Bloqueio automático via `iptables` ou `tc` quando limites são excedidos.
- Recarga dinâmica de políticas sem reiniciar o serviço.
- Log de eventos com tentativas de acesso pós-cota, bloqueios e uso.
- Suporte planejado para alertas via Telegram ou Email.
- Armazenamento persistente via `sled`.

## Estrutura do Projeto
src/ ├── main.rs                   # Ponto de entrada da aplicação ├── core/ │   ├── tracker.rs            # Contadores e lógica por IP │   ├── limiter.rs            # Aplicação de limites e bloqueios │   ├── time_window.rs        # Gerência de janelas de tempo (ex: 24h) │   └── mod.rs ├── net/ │   ├── sniffer.rs            # Captura pacotes da rede │   ├── analyzer.rs           # Analisa arquivos .pcap │   └── mod.rs ├── policy/ │   ├── loader.rs             # Carregamento de políticas via TOML/JSON │   └── mod.rs ├── persistence/ │   ├── db.rs                 # Armazenamento e persistência de dados │   └── mod.rs ├── system/ │   ├── executor.rs           # Execução segura de comandos do sistema (iptables/tc) │   └── mod.rs ├── ui/ │   ├── tui.rs                # Interface de terminal interativa (TUI) │   ├── cli.rs                # Interface por linha de comando (CLI) │   └── mod.rs ├── logging/ │   ├── logger.rs             # Registro de eventos, logs e tentativas │   └── mod.rs ├── config/ │   ├── config.rs             # Leitura e estrutura de configuração do sistema │   └── mod.rs └── utils/ └── mod.rs                # Funções auxiliares diversas (formatadores, validações, etc.)

## Dependências
[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.7"
pnet = "0.31"
ratatui = "0.21"
sled = "0.34"
chrono = "0.4"
tracing = "0.1"
log = "0.4"
fern = "0.6"
clap = { version = "4", features = ["derive"] }
pcap-parser = "0.12"

## Exemplo de Política em TOML
[limiteporip]
"192.168.0.10" = 1073741824 # 1 GB
"192.168.0.20" = 524288000  # 500 MB

[bloqueio]
modo = "DROP"
janela = "24h"

## Fluxo de Funcionamento
1. main.rs carrega configurações e inicia os módulos principais.
2. net::sniffer captura pacotes e repassa para core::tracker.
3. core::tracker contabiliza os bytes por IP.
4. core::limiter verifica cotas e aciona system::executor se necessário.
5. policy::loader carrega/recarrega políticas definidas pelo usuário.
6. persistence::db salva dados de uso entre sessões.
7. logging::logger registra eventos (bloqueios, excessos, reinicializações).
8. ui::tui e ui::cli fornecem interfaces para visualização e interação.
9. net::analyzer pode ser chamado para processar arquivos .pcap sob demanda.

## Modo CLI
Exemplo de uso via terminal:
bytetally analyze --pcap captura.pcap
bytetally show --top-ip
bytetally start --headless

## Aprendizado e Objetivo
Este projeto também é parte de uma jornada de aprendizado em cibersegurança, com foco no domínio das seguintes áreas:
- Inspeção de pacotes de rede
- Controle de tráfego com iptables e tc
- Segurança de rede local
- Design modular com Rust
- Persistência de dados com bancos de chave/valor

Interfaces de terminal e linha de comando
## Roadmap Futuro
[ ] Web UI com painel de controle
[ ] Exportação de relatórios em CSV/JSON
[ ] Limitação de banda em vez de DROP (via tc)
[ ] Alertas por Telegram/Email
[ ] Suporte por MAC address
[ ] API REST opcional

### Contribuindo
Quer ajudar no ByteTally? Ótimo! Veja o arquivo CONTRIBUTING.md para diretrizes e comece a contribuir!

📄 Licença
Este projeto está licenciado sob a MIT License.

ByteTally é um projeto em crescimento. Com sua leveza, foco em segurança e facilidade de uso, tem potencial para se tornar uma solução recomendada para controle de rede em ambientes pequenos e médios.
=======
﻿# ByteTally — Monitoramento e Controle de Largura de Banda por IP

**ByteTally** é uma ferramenta escrita em **Rust** para monitorar, analisar e controlar o uso de largura de banda em redes locais. Focada em **performance, governança e simplicidade**, ela inspeciona pacotes, contabiliza tráfego por IP, aplica cotas (limites de uso), executa bloqueios dinâmicos e fornece interfaces para visualização e controle — tudo com baixo consumo de recursos.

Ideal para **pequenas e médias empresas, hotspots, redes domésticas, laboratórios e ambientes com conectividade limitada**.

## Funcionalidades
- Captura de tráfego em tempo real por IP (via `pnet`).
- Interface TUI interativa (com `ratatui`).
- Interface por linha de comando (CLI) para execução de comandos e relatórios.
- Leitura e análise de arquivos `.pcap` para identificação de IPs com maior consumo.
- Controle por janelas de tempo (ex: 24h) com resets automáticos.
- Políticas configuráveis via TOML/JSON.
- Bloqueio automático via `iptables` ou `tc` quando limites são excedidos.
- Recarga dinâmica de políticas sem reiniciar o serviço.
- Log de eventos com tentativas de acesso pós-cota, bloqueios e uso.
- Suporte planejado para alertas via Telegram ou Email.
- Armazenamento persistente via `sled`.

## Estrutura do Projeto
```text
src/
├── main.rs                  # Ponto de entrada da aplicação
│
├── core/
│   ├── tracker.rs           # Contadores e estatísticas por IP
│   ├── limiter.rs           # Aplicação de limites e bloqueios
│   ├── time_window.rs       # Gerência de janelas de tempo (ex.: 24h)
│   └── mod.rs
│
├── net/
│   ├── sniffer.rs           # Captura de pacotes da rede
│   ├── analyzer.rs          # Análise de arquivos .pcap
│   └── mod.rs
│
├── policy/
│   ├── loader.rs            # Carregamento de políticas (TOML/JSON)
│   └── mod.rs
│
├── persistence/
│   ├── db.rs                # Persistência e armazenamento de dados
│   └── mod.rs
│
├── system/
│   ├── executor.rs          # Execução segura de comandos do sistema (iptables/tc)
│   └── mod.rs
│
├── ui/
│   ├── tui.rs               # Interface de Terminal (TUI)
│   ├── cli.rs               # Interface de Linha de Comando (CLI)
│   └── mod.rs
│
├── logging/
│   ├── logger.rs            # Registro de eventos e logs
│   └── mod.rs
│
├── config/
│   ├── config.rs            # Configuração da aplicação
│   └── mod.rs
│
└── utils/
    └── mod.rs               # Funções auxiliares (formatadores, validações, etc.)
```

## Dependências
```text
[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.7"
pnet = "0.31"
ratatui = "0.21"
sled = "0.34"
chrono = "0.4"
tracing = "0.1"
log = "0.4"
fern = "0.6"
clap = { version = "4", features = ["derive"] }
pcap-parser = "0.12"
```

## Exemplo de Política em TOML
[limiteporip]
"192.168.0.10" = 1073741824 # 1 GB
"192.168.0.20" = 524288000  # 500 MB

[bloqueio]
modo = "DROP"
janela = "24h"

## Fluxo de Funcionamento
1. main.rs carrega configurações e inicia os módulos principais.
2. net::sniffer captura pacotes e repassa para core::tracker.
3. core::tracker contabiliza os bytes por IP.
4. core::limiter verifica cotas e aciona system::executor se necessário.
5. policy::loader carrega/recarrega políticas definidas pelo usuário.
6. persistence::db salva dados de uso entre sessões.
7. logging::logger registra eventos (bloqueios, excessos, reinicializações).
8. ui::tui e ui::cli fornecem interfaces para visualização e interação.
9. net::analyzer pode ser chamado para processar arquivos .pcap sob demanda.

## Modo CLI
Exemplo de uso via terminal:
bytetally analyze --pcap captura.pcap
bytetally show --top-ip
bytetally start --headless

## Aprendizado e Objetivo
Este projeto também é parte de uma jornada de aprendizado em cibersegurança, com foco no domínio das seguintes áreas:
- Inspeção de pacotes de rede
- Controle de tráfego com iptables e tc
- Segurança de rede local
- Design modular com Rust
- Persistência de dados com bancos de chave/valor

Interfaces de terminal e linha de comando
## Roadmap Futuro
[ ] Web UI com painel de controle
[ ] Exportação de relatórios em CSV/JSON
[ ] Limitação de banda em vez de DROP (via tc)
[ ] Alertas por Telegram/Email
[ ] Suporte por MAC address
[ ] API REST opcional

### Contribuindo
Quer ajudar no ByteTally? Ótimo! Veja o arquivo CONTRIBUTING.md para diretrizes e comece a contribuir!

📄 Licença
Este projeto está licenciado sob a MIT License.

ByteTally é um projeto em crescimento. Com sua leveza, foco em segurança e facilidade de uso, tem potencial para se tornar uma solução recomendada para controle de rede em ambientes pequenos e médios.
>>>>>>> 5f468b8bdb72b242e7dffea304e6844ddf2d5c9f
