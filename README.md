# ConectaStore — Sistema de Recomendação de Produtos em Rust

## Disciplina: Data Structure Strategy and Implementation — UniFecaf
## Aluno: Otávio Luizon Neves Orientadora: Profª. Patrícia Ampese

## 📌 Objetivo do Sistema 
## O ConectaStore é uma solução desenvolvida para a plataforma de e-commerce MegaStore. O sistema resolve o desafio de descoberta de produtos em catálogos de grande escala, utilizando uma modelagem baseada em Grafos para sugerir itens altamente relevantes com base em conexões de co-compra e similaridade, prevenindo recomendações duplicadas e itens já adquiridos pelo cliente.


## 🛠️ Tecnologias e Estruturas Utilizadas Linguagem: Rust (Garantia de segurança de memória e alta performance sem Garbage Collector). Grafo (Lista de Adjacência):HashMap

## 📐 Arquitetura da Solução 
[ Cliente / Produto ] 
        │ 
        ▼
[ ConectaStore Engine ]
    ├── HashMap (Produtos O(1))
    ├── Grafo (Lista de Adjacência O(V + E))
    └── Algoritmo BFS (Busca em Largura)
        │
        ├── Filtra produtos já comprados (HashSet)
        ├── Evita recomendações duplicadas (HashSet)
        └── Parada antecipada pelo limite (Limit k)
        │
        ▼
    [ Lista de Produtos Recomendados ]