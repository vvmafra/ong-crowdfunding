# ONG Crowdfunding Smart Contract - MultiversX

Este projeto implementa um contrato inteligente de crowdfunding na blockchain MultiversX, permitindo a criação de campanhas de financiamento coletivo com regras claras e transparentes para ONGs.

## Funcionalidades

- Criação de campanhas com meta de financiamento definida
- Contribuição em EGLD (moeda nativa da MultiversX)
- Sistema de status da campanha (Em andamento, Bem-sucedida, Falha)
- Reivindicação de fundos baseada no resultado da campanha
- Eventos registrados na blockchain para transparência

## Pré-requisitos

- Rust (última versão estável)
- Node 20.11+
- mxpy
- pip3
- Carteira MultiversX (para testes)

## Instalação

1. Clone o repositório:
```bash
git clone https://github.com/vvmafra/ong-crowdfunding
```

2. Instale as dependências:
```bash
cargo build
```

## Estrutura do Contrato

O contrato possui as seguintes funcionalidades principais:

### Inicialização
```rust
fn init(&self, target: BigUint)
```
- Define a meta de financiamento (em EGLD)
- A meta deve ser maior que zero

### Contribuição
```rust
fn fund(&self)
```
- Permite contribuições em EGLD
- Só aceita contribuições durante o período de financiamento

### Status da Campanha
```rust
fn status(&self) -> Status
```
Retorna o status atual da campanha:
- `FundingPeriod`: Em andamento
- `Successful`: Meta atingida
- `Failed`: Meta não atingida

### Finalização e Reivindicação
```rust
fn realize(&self)
fn claim(&self)
```
- `realize`: Finaliza a campanha (apenas owner)
- `claim`: Permite reivindicar fundos baseado no resultado

## Deploy na Testnet

1. Compile o contrato:
```bash
mxpy contract build
```

2. Deploy na testnet:
- Criar arquivo com informações do contrato (my_wallet.json por exemplo)   
```bash
mxpy --verbose contract deploy --bytecode="./output/ongcrowdfunding.wasm" --keyfile="my_wallet.json" --gas-limit=100000000 --proxy="https://testnet-gateway.multiversx.com" --chain="T" --arguments <quantidade de EGLD que você pretende para a sua campanha 1000000000000000000 (1 EGLD), por exemplo> --send
```

3. Checagem de Deploy
- Para checar se o deploy funcionou, basta acessar https://testnet-explorer.multiversx.com/ e adicionar o contractAddress que foi gerado após o deploy do contrato;
- Exemplo de contrato com deploy realizado:
["contractAddress": "erd1qqqqqqqqqqqqqpgqytydxz60g758gla6xy4la0k2r9g5slfhn20sv5fatl"](https://testnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgqytydxz60g758gla6xy4la0k2r9g5slfhn20sv5fatl)

## Testes Automatizados

O projeto possui uma suite completa de testes automatizados que cobrem todas as funcionalidades principais do contrato:

```bash
cargo test
```

Os testes incluem:
- Deploy do contrato
- Contribuições (funding)
- Verificação de status
- Reivindicação de fundos (tanto para campanhas bem-sucedidas quanto falhas)
- Validações de segurança

Cada teste simula diferentes cenários:
- Campanhas que atingem a meta
- Campanhas que falham
- Tentativas de reivindicação prematura
- Verificações de saldo e status

## Segurança

- Apenas o owner pode finalizar a campanha
- Reivindicação de fundos segue regras específicas baseadas no resultado

## Próximos Passos

1. Front-end de Doação  
   - **Tecnologias sugeridas:** React com TypeScript, usando o SDK MultiversX (mx-sdk-dapp)  
   - Componentes principais:
     - Tela de listagem de campanhas (status, progresso e metas)
     - Formulário de contribuição (seleção de valor, preview de taxa de rede)
     - Dashboard do criador (visualizar backers, valores arrecadados, ações de finalize/claim)
   - Demonstrações em tempo real (websocket ou polling) para atualizar progresso de campanha sem recarregar a página.

2. Testes Automatizados  
   - CI (GitHub Actions) para disparar build + teste a cada PR.

3. Auditoria e Otimização  
   - Revisão de segurança (verificar over-flows, reentrância, limites de gas)  
   - Benchmark de custos de gas por operação  
   - Adicionar limites de contribuição mínima e máxima, rate-limiting.

4. Documentação e Exemplos  
   - Exemplos de chamadas `mxpy` no README (com flags completas e JSON de argumentos)  
   - Vídeo curto/demo GIF mostrando uma doação e a emissão dos eventos na blockchain  
   - Guia de uso da carteira no testnet (Passo a passo de criação/importação).

5. Deploy em Mainnet  
   - Planejar migração do contrato para a mainnet  
   - Definir estratégia de gestão de chaves do owner (social recovery, multisig)  
   - Monitoramento de performance (alertas de falhas, dashboards de métricas).

---

Com isso você entrega não só o core do smart contract, mas também toda a infra e UX para que ONGs — e seus doadores — possam usar de maneira simples e transparente.

## Contato

Para dúvidas ou sugestões, abra uma issue no repositório. 
