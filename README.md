# Crowdfunding Smart Contract - MultiversX

Este projeto implementa um contrato inteligente de crowdfunding na blockchain MultiversX, permitindo a criação de campanhas de financiamento coletivo com regras claras e transparentes.

## Funcionalidades

- Criação de campanhas com meta de financiamento definida
- Contribuição em EGLD (moeda nativa da MultiversX)
- Sistema de status da campanha (Em andamento, Bem-sucedida, Falha)
- Reivindicação de fundos baseada no resultado da campanha
- Eventos registrados na blockchain para transparência

## Pré-requisitos

- Rust (última versão estável)
- MultiversX SDK
- MultiversX IDE (opcional)
- Carteira MultiversX (para testes)

## Instalação

1. Clone o repositório:
```bash
git clone https://github.com/seu-usuario/crowdfunding-contract.git
cd crowdfunding-contract
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
- Define a meta de financiamento
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
```bash
mxpy contract deploy --bytecode=output/crowdfunding.wasm --recall-nonce --gas-limit=50000000 --send --proxy=https://testnet-api.multiversx.com --chain=T
```

## Testando o Contrato

1. Crie uma nova campanha:
```bash
mxpy contract call <endereço-do-contrato> --function="init" --arguments 1000000000000000000 --recall-nonce --gas-limit=50000000 --send --proxy=https://testnet-api.multiversx.com --chain=T
```

2. Faça uma contribuição:
```bash
mxpy contract call <endereço-do-contrato> --function="fund" --value=100000000000000000 --recall-nonce --gas-limit=50000000 --send --proxy=https://testnet-api.multiversx.com --chain=T
```

3. Verifique o status:
```bash
mxpy contract query <endereço-do-contrato> --function="status" --proxy=https://testnet-api.multiversx.com
```

## Segurança

- Apenas o owner pode finalizar a campanha
- Contribuições só são aceitas durante o período de financiamento
- Reivindicação de fundos segue regras específicas baseadas no resultado

## Contribuição

Contribuições são bem-vindas! Por favor, leia o [CONTRIBUTING.md](CONTRIBUTING.md) para detalhes sobre nosso código de conduta e o processo para enviar pull requests.

## Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE.md](LICENSE.md) para detalhes.

## Contato

Para dúvidas ou sugestões, abra uma issue no repositório ou entre em contato através do Discord NearX. 