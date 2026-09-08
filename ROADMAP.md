# Roadmap do ByteTally

## Implementado

- [x] Biblioteca e CLI com módulos integrados na compilação.
- [x] Leitura de registos de texto `IP BYTES`, com validação.
- [x] Contabilização por IPv4/IPv6 e relatório por consumo.
- [x] Protecção contra overflow dos contadores.
- [x] Adaptador de execução real e dry-run, sem ligação à CLI.
- [x] Testes unitários, testes da CLI e workflow de CI.

## Próximos passos

- [ ] Análise de ficheiros PCAP.
- [ ] Captura de pacotes por interface.
- [ ] Políticas de cotas e janelas de tempo.
- [ ] Persistência e recuperação de estado.
- [ ] Integração controlada com iptables/tc e testes de aplicação de regras.
- [ ] Interface TUI e modo daemon.
- [ ] Configuração e logs estruturados.

## Futuro

- [ ] Recarga de políticas, exportação e alertas.
- [ ] Interface web/API opcional.
- [ ] Suporte a MAC, VLAN e integração com routers.
- [ ] Distribuição em pacotes Linux.
