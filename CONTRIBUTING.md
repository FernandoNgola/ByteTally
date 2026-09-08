# Contribuir para o ByteTally

Use Rust stable (mínimo suportado: 1.98) e alterações pequenas com uma
responsabilidade clara. Não acrescente dependências sem uma utilização concreta.

## Organização

A lógica de contabilização fica em `core`, a leitura de registos em `net`,
a CLI em `ui` e os comandos externos em `system`. Declare os módulos no pai;
não misture `nome.rs` e `nome/mod.rs` para o mesmo módulo.

Mantenha o executável pequeno e teste a lógica na biblioteca. Acrescente testes
de integração em `tests/` quando alterar o comportamento observável da CLI.
Testes não devem modificar regras de rede nem exigir privilégios administrativos.

## Antes de enviar alterações

```sh
cargo fmt --all -- --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
```

Actualize o README e o roadmap quando uma funcionalidade ficar disponível.
Versione `Cargo.lock` para manter a resolução de dependências da aplicação.
O workflow de CI verifica stable; alterações da versão mínima suportada devem
ser reflectidas em `Cargo.toml` e nesta documentação.
