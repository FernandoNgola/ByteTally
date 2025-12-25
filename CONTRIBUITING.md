#  Contribuindo com o ByteTally

Obrigado por considerar contribuir com o ByteTally!  
Este projeto é desenvolvido como parte de uma jornada de aprendizado em cibersegurança e Rust, e tem como missão oferecer uma ferramenta simples e eficiente para **monitoramento, análise e governança de redes locais**.  
Toda ajuda é bem-vinda — seja com código, sugestões, testes, documentação ou divulgação.

## Diretrizes Gerais
1. **Seja respeitoso** com os demais colaboradores.
2. Use o estilo idiomático do **Rust** sempre que possível.
3. Valorize a **modularidade**: cada módulo deve ter uma responsabilidade clara.
4. Prefira clareza a "esperteza" no código — comentários explicativos são bem-vindos!
5. Se for adicionar uma nova funcionalidade, verifique se já existe uma *issue* relacionada ou abra uma nova para discussão.

##  Estrutura Modular do Projeto

Todos os arquivos estão dentro da pasta `src/`, organizados por domínio:

- `core/`: lógica de contagem, limites e janelas de tempo.
- `net/`: captura e análise de pacotes da rede.
- `policy/`: carregamento e aplicação de políticas.
- `persistence/`: armazenamento local, como base de dados.
- `system/`: comandos de sistema (ex: iptables).
- `ui/`: interface de linha de comando (CLI) e terminal (TUI).
- `logging/`: sistema de log e registro de eventos.
- `config/`: leitura e validação de configurações.
- `utils/`: utilitários diversos e helpers.

Cada subpasta contém seu próprio `mod.rs` para manter o código organizado.

##  Começando
1. Faça um fork do repositório
2. Clone o seu fork:
   ```bash
   git clone https://github.com/seu-usuario/bytetally.git

3. Crie uma branch:

git checkout -b minha-feature

4. Certifique-se de que seu código está limpo:

cargo fmt
cargo clippy
cargo check

5. Faça o commit e envie uma PR com uma descrição clara do que foi feito.

## Testes

Testes automatizados estão sendo adicionados gradualmente. Sinta-se à vontade para contribuir com testes unitários, testes de integração e sugestões de casos de uso.

### Outras Formas de Contribuir

Mesmo que você não esteja pronto para contribuir com código, pode ajudar:

Escrevendo ou traduzindo documentação

Testando a ferramenta em diferentes ambientes

Sugerindo funcionalidades via issues

Compartilhando o projeto com colegas e comunidades

### Dúvidas?
Use as issues para discutir ideias, dúvidas ou problemas.
Toda dúvida é bem-vinda, pois o projeto também é um processo de aprendizado.

Vamos tornar o ByteTally uma ferramenta reconhecida e útil para redes locais em todo o mundo, especialmente para pequenas e médias empresas! 🚀
