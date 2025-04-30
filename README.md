# Projeto: Implementação de Regressão Linear Simples em Rust para Análise de Séries Temporais

## Visão Geral

Este projeto tem como objetivo desenvolver uma ferramenta eficiente e confiável para análise de séries temporais, utilizando a linguagem de programação Rust. A implementação é parte de uma solução proposta para a empresa fictícia *TimeWise Analytics*, especializada em previsões baseadas em dados históricos. O foco está na construção de uma regressão linear simples sem dependências externas, garantindo controle total sobre os cálculos estatísticos.

## Objetivos do Projeto

- Implementar um modelo de regressão linear simples (y = ax + b) utilizando apenas recursos nativos da linguagem Rust.
- Calcular métricas estatísticas de avaliação do modelo: R² (coeficiente de determinação) e MSE (erro quadrático médio).
- Aplicar o modelo de regressão para realizar previsões de valores futuros com base em séries temporais.
- Garantir a confiabilidade do código por meio de testes unitários e tratamento de entradas inválidas.

## Estrutura do Projeto

O projeto está estruturado em dois arquivos principais:

- `lib.rs`: contém a implementação das funções da regressão linear, métricas de avaliação e testes unitários.
- `main.rs`: realiza a chamada das funções implementadas, simulando um cenário prático de uso com dados exemplo.

## Funcionalidades Implementadas

1. **Regressão Linear**  
   Cálculo dos coeficientes da reta de regressão (a e b) com base em pares de dados (x, y).

2. **Cálculo de Métricas**  
   - **R² (Coeficiente de Determinação)**: mede o quão bem a regressão explica os dados observados.
   - **MSE (Erro Quadrático Médio)**: quantifica o erro médio ao quadrado entre os valores reais e os valores previstos.

3. **Previsões**  
   Geração de valores previstos a partir da equação da reta obtida.

4. **Tratamento de Erros**  
   - Validação de entradas: tamanho dos vetores, vetores vazios, e condições inválidas são tratadas com mensagens apropriadas.
   - Prevenção de divisões por zero no cálculo dos coeficientes.

5. **Testes Unitários**  
   Implementação de testes cobrindo:
   - Precisão dos coeficientes da regressão em cenários ideais.
   - Validação das métricas R² e MSE com dados próximos da perfeição.
   - Verificação do comportamento do código diante de entradas inválidas.

## Execução

### Compilação e execução do projeto

Para compilar e executar o projeto:
cargo run
