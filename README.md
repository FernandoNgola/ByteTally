# ByteTally — Monitoramento e Controle de Largura de Banda por IP

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
