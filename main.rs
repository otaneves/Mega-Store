use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

/// Representação de um Produto do catálogo da MegaStore
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub price: f64,
}

/// Estrutura do Grafo utilizando Lista de Adjacência
#[derive(Debug, Default)]
pub struct Graph {
    // Mapeamento: ID_Origem -> Vetor de (ID_Destino, Peso_da_Conexao)
    pub adj_list: HashMap<u32, Vec<(u32, f32)>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    /// Adiciona uma aresta não-direcionada entre dois vértices (produtos)
    pub fn add_edge(&mut self, u: u32, v: u32, weight: f32) {
        self.adj_list.entry(u).or_default().push((v, weight));
        self.adj_list.entry(v).or_default().push((u, weight));
    }
}

/// Sistema Principal da ConectaStore
pub struct ConectaStore {
    pub products: HashMap<u32, Product>,
    pub graph: Graph,
    pub client_purchases: HashMap<u32, Vec<u32>>,
}

impl ConectaStore {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
            graph: Graph::new(),
            client_purchases: HashMap::new(),
        }
    }

    /// Cadastra um novo produto no catálogo
    pub fn add_product(&mut self, product: Product) {
        self.products.insert(product.id, product);
    }

    /// Consulta um produto pelo seu ID
    pub fn get_product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    /// Adiciona uma conexão de similaridade/co-compra entre dois produtos
    pub fn add_connection(&mut self, prod1_id: u32, prod2_id: u32, weight: f32) {
        self.graph.add_edge(prod1_id, prod2_id, weight);
    }

    /// Registra a compra de um cliente
    pub fn record_purchase(&mut self, client_id: u32, product_id: u32) {
        self.client_purchases
            .entry(client_id)
            .or_default()
            .push(product_id);
    }

    /// Algoritmo de Recomendação via BFS (Breadth-First Search)
    pub fn recommend_bfs(&self, start_product_id: u32, max_depth: usize, limit: usize) -> Vec<Product> {
        let mut recommendations = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start_product_id);
        queue.push_back((start_product_id, 0));

        while let Some((current_id, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }

            if let Some(neighbors) = self.graph.adj_list.get(&current_id) {
                for &(neighbor_id, _weight) in neighbors {
                    if !visited.contains(&neighbor_id) {
                        visited.insert(neighbor_id);
                        queue.push_back((neighbor_id, depth + 1));

                        if let Some(product) = self.products.get(&neighbor_id) {
                            recommendations.push(product.clone());
                            if recommendations.len() == limit {
                                return recommendations;
                            }
                        }
                    }
                }
            }
        }

        recommendations
    }

    /// Algoritmo de Recomendação via DFS (Depth-First Search)
    pub fn recommend_dfs(&self, start_product_id: u32, max_depth: usize, limit: usize) -> Vec<Product> {
        let mut recommendations = Vec::new();
        let mut visited = HashSet::new();
        visited.insert(start_product_id);

        self.dfs_helper(
            start_product_id,
            0,
            max_depth,
            limit,
            &mut visited,
            &mut recommendations,
        );

        recommendations
    }

    fn dfs_helper(
        &self,
        current_id: u32,
        current_depth: usize,
        max_depth: usize,
        limit: usize,
        visited: &mut HashSet<u32>,
        recommendations: &mut Vec<Product>,
    ) {
        if current_depth >= max_depth || recommendations.len() >= limit {
            return;
        }

        if let Some(neighbors) = self.graph.adj_list.get(&current_id) {
            for &(neighbor_id, _weight) in neighbors {
                if recommendations.len() >= limit {
                    break;
                }
                if !visited.contains(&neighbor_id) {
                    visited.insert(neighbor_id);
                    if let Some(product) = self.products.get(&neighbor_id) {
                        recommendations.push(product.clone());
                    }
                    self.dfs_helper(
                        neighbor_id,
                        current_depth + 1,
                        max_depth,
                        limit,
                        visited,
                        recommendations,
                    );
                }
            }
        }
    }

    /// Recomendação personalizada para um cliente (exclui itens já comprados)
    pub fn recommend_for_client(&self, client_id: u32, limit: usize) -> Vec<Product> {
        let mut recommendations = Vec::new();
        let mut visited = HashSet::new();

        if let Some(purchased_ids) = self.client_purchases.get(&client_id) {
            // Marca produtos já comprados para não recomendá-los novamente
            for &prod_id in purchased_ids {
                visited.insert(prod_id);
            }

            for &prod_id in purchased_ids {
                let recs = self.recommend_bfs(prod_id, 2, limit);
                for prod in recs {
                    if !visited.contains(&prod.id) {
                        visited.insert(prod.id);
                        recommendations.push(prod);
                        if recommendations.len() >= limit {
                            return recommendations;
                        }
                    }
                }
            }
        }

        recommendations
    }
}

fn main() {
    println!("=== ConectaStore: Sistema de Recomendação em Rust ===");

    let mut store = ConectaStore::new();

    // 1. Cadastrando produtos
    store.add_product(Product {
        id: 1,
        name: "Smartphone Galaxy".to_string(),
        category: "Eletrônicos".to_string(),
        price: 2500.0,
    });
    store.add_product(Product {
        id: 2,
        name: "Fone Bluetooth".to_string(),
        category: "Eletrônicos".to_string(),
        price: 300.0,
    });
    store.add_product(Product {
        id: 3,
        name: "Capinha de Celular".to_string(),
        category: "Acessórios".to_string(),
        price: 50.0,
    });
    store.add_product(Product {
        id: 4,
        name: "Carregador Rápido".to_string(),
        category: "Eletrônicos".to_string(),
        price: 120.0,
    });
    store.add_product(Product {
        id: 5,
        name: "Smartwatch Sport".to_string(),
        category: "Eletrônicos".to_string(),
        price: 900.0,
    });

    // 2. Conectando produtos no Grafo
    store.add_connection(1, 2, 0.9);  // Smartphone <-> Fone
    store.add_connection(1, 3, 0.8);  // Smartphone <-> Capinha
    store.add_connection(1, 4, 0.95); // Smartphone <-> Carregador
    store.add_connection(2, 5, 0.7);  // Fone <-> Smartwatch

    // 3. Registrar compra de cliente
    let client_id = 101;
    store.record_purchase(client_id, 1); // Cliente comprou o Smartphone

    // 4. Teste de Recomendação por Produto (BFS)
    println!("\nRecomendações BFS para quem viu 'Smartphone Galaxy':");
    let recs_bfs = store.recommend_bfs(1, 2, 3);
    for prod in &recs_bfs {
        println!(" - [{}] {} ({}) - R$ {:.2}", prod.id, prod.name, prod.category, prod.price);
    }

    // 5. Teste de Recomendação Personalizada para Cliente
    println!("\nRecomendações Personalizadas para o Cliente {}:", client_id);
    let recs_client = store.recommend_for_client(client_id, 3);
    for prod in &recs_client {
        println!(" - [{}] {} ({}) - R$ {:.2}", prod.id, prod.name, prod.category, prod.price);
    }

    // 6. Teste de Desempenho
    println!("\n=== Teste de Desempenho de Carga ===");
    let mut big_store = ConectaStore::new();
    let num_products = 10_000;

    for i in 1..=num_products {
        big_store.add_product(Product {
            id: i,
            name: format!("Produto {}", i),
            category: "Geral".to_string(),
            price: 100.0,
        });
    }

    for i in 1..(num_products - 1) {
        big_store.add_connection(i, i + 1, 0.5);
    }

    let start = Instant::now();
    let _big_recs = big_store.recommend_bfs(1, 5, 10);
    let duration = start.elapsed();

    println!("Tempo para recomendar em catálogo com {} produtos: {:?}", num_products, duration);
}

// ==========================================
// TESTES UNITÁRIOS E DE INTEGRAÇÃO
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_product() {
        let mut store = ConectaStore::new();
        let prod = Product {
            id: 1,
            name: "Livro Rust".to_string(),
            category: "Livros".to_string(),
            price: 80.0,
        };
        store.add_product(prod.clone());

        assert_eq!(store.get_product(1), Some(&prod));
        assert_eq!(store.get_product(99), None);
    }

    #[test]
    fn test_recommendation_without_duplicates() {
        let mut store = ConectaStore::new();
        store.add_product(Product { id: 1, name: "Item A".to_string(), category: "Cat1".to_string(), price: 10.0 });
        store.add_product(Product { id: 2, name: "Item B".to_string(), category: "Cat1".to_string(), price: 20.0 });
        store.add_product(Product { id: 3, name: "Item C".to_string(), category: "Cat1".to_string(), price: 30.0 });

        store.add_connection(1, 2, 1.0);
        store.add_connection(2, 3, 1.0);
        store.add_connection(1, 3, 1.0);

        let recs = store.recommend_bfs(1, 2, 5);
        let ids: Vec<u32> = recs.iter().map(|p| p.id).collect();

        let mut unique_ids = ids.clone();
        unique_ids.sort();
        unique_ids.dedup();

        assert_eq!(ids.len(), unique_ids.len());
        assert!(!ids.contains(&1));
    }

    #[test]
    fn test_client_recommendation_excludes_purchased() {
        let mut store = ConectaStore::new();
        store.add_product(Product { id: 1, name: "Notebook".to_string(), category: "Eletrônicos".to_string(), price: 4000.0 });
        store.add_product(Product { id: 2, name: "Mouse".to_string(), category: "Acessórios".to_string(), price: 100.0 });
        store.add_product(Product { id: 3, name: "Teclado".to_string(), category: "Acessórios".to_string(), price: 200.0 });

        store.add_connection(1, 2, 0.9);
        store.add_connection(1, 3, 0.85);

        store.record_purchase(10, 1);
        store.record_purchase(10, 2);

        let recs = store.recommend_for_client(10, 5);

        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].id, 3);
    }
}
