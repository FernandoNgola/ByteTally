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
