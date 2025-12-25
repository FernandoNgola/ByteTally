#  Roadmap do Projeto ByteTally

Este é o plano de desenvolvimento e expansão do ByteTally. O foco é evoluir gradualmente em **funcionalidade**, **desempenho** e **acessibilidade**, sempre com código limpo, modular e fácil de manter.

## ✅ Fase 1 — Mínimo Viável (MVP) [CONCLUÍDO/EM PROGRESSO]

- [x] Captura de pacotes por interface
- [x] Contabilização de tráfego por IP
- [x] Aplicação de cotas por IP
- [x] Execução de bloqueios (`iptables`)
- [x] Interface TUI com estatísticas
- [x] Interface CLI com comandos úteis
- [x] Sistema básico de log de eventos
- [x] Suporte a arquivos `.pcap` para análise
- [x] Estrutura modular separada por domínio

## 🔄 Fase 2 — Confiabilidade e Persistência

- [ ] Persistência robusta com `sled` (histórico, bloqueios, resets)
- [ ] Recuperação automática de estado após reinício
- [ ] Log rotativo de eventos
- [ ] Mais testes unitários e integração contínua
- [ ] Configuração dinâmica recarregável em tempo de execução

## 📡 Fase 3 — Extensões e Usabilidade

- [ ] Exportação de relatórios (CSV/JSON)
- [ ] Modo "headless" para daemon simples
- [ ] Alertas por Telegram, Email ou Slack
- [ ] Uso de `tc` para limitação (shaping) em vez de DROP
- [ ] Suporte a políticas baseadas em MAC Address
- [ ] CLI interativa com menus e autocomplete

## 🌐 Fase 4 — Interface Web

- [ ] Painel de controle web com dashboard
- [ ] Login simples com autenticação local
- [ ] Visualização em tempo real de uso de IPs
- [ ] Configuração de políticas via Web UI

## Fase 5 — Profissionalização

- [ ] Suporte a múltiplas interfaces e VLANs
- [ ] Integração com SNMP e syslog
- [ ] Plugin para integração com pfSense/OpenWRT
- [ ] Pacotes para Debian/Arch


Este roadmap é vivo e pode ser adaptado conforme surgem novas ideias, colaborações e necessidades. Toda sugestão é bem-vinda!
