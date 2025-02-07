use serde::de::{self, MapAccess, Visitor};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::{env, fs};

#[derive(Debug)]
pub struct District(Vec<Graph>);

#[derive(Debug, Clone)]
pub struct Graph {
    adj_list: HashMap<String, HashSet<String>>,
}

impl<'de> Deserialize<'de> for Graph {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct GraphVisitor;

        impl<'de> Visitor<'de> for GraphVisitor {
            type Value = Graph;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a map of city names to lists of neighbors")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut adj_list = HashMap::new();

                // Build complete bidirectional graph during deserialization
                while let Some((city, neighbors)) = access.next_entry::<String, Vec<String>>()? {
                    // Add edges from city to neighbors
                    for neighbor in &neighbors {
                        adj_list
                            .entry(city.clone())
                            .or_insert_with(HashSet::new)
                            .insert(neighbor.clone());

                        adj_list
                            .entry(neighbor.clone())
                            .or_insert_with(HashSet::new)
                            .insert(city.clone());
                    }
                }

                Ok(Graph { adj_list })
            }
        }

        deserializer.deserialize_map(GraphVisitor)
    }
}

impl<'de> Deserialize<'de> for District {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(HashMap<String, Graph>);

        let Helper(raw_data) = Helper::deserialize(deserializer)?;

        let mut graphs = Vec::with_capacity(raw_data.len());
        for i in 1..=raw_data.len() {
            if let Some(graph) = raw_data.get(&i.to_string()) {
                graphs.push(graph.clone());
            } else {
                return Err(de::Error::custom(format!("Missing batch {}", i)));
            }
        }

        Ok(District(graphs))
    }
}

fn dfs(start: &str, graph: &Graph, visited: &mut HashSet<String>) {
    let mut stack = vec![start.to_string()];

    while let Some(city) = stack.pop() {
        if !visited.insert(city.clone()) {
            continue;
        }

        if let Some(neighbors) = graph.adj_list.get(&city) {
            stack.extend(neighbors.iter().filter(|n| !visited.contains(*n)).cloned());
        }
    }
}

fn count_connected_components(graph: &Graph) -> usize {
    let mut visited = HashSet::new();
    let mut components = 0;

    for city in graph.adj_list.keys() {
        if !visited.contains(city) {
            dfs(city, graph, &mut visited);
            components += 1;
        }
    }

    components
}

pub fn count_provinces() -> String {
    let district = read_district_json().unwrap();
    district.0.into_iter()
        .map(|graph| count_connected_components(&graph).to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn read_district_json() -> Result<District, Box<dyn std::error::Error>> {
    let mut path = env::current_dir()?;
    path.push("district.json");
    let content = fs::read_to_string(path)?;
    let district = serde_json::from_str(&content)?;
    Ok(district)
}