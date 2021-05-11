struct DAG {
    adj_list: Vec<Vec<WeightedEdge>>,
}

#[derive(Clone)]
struct WeightedEdge {
    weight: f64,
    target: u64,
}

impl DAG {
    fn init(n_nodes: &u64, edge_list: &Vec<(u64,u64,f64)>) -> DAG {
        let mut adj_list:Vec<Vec<WeightedEdge>> = Vec::new();
        for _ in 0..*n_nodes {
            let tmp:Vec<WeightedEdge> = Vec::new();
            adj_list.push(tmp);
        }
        for edge in edge_list {
            adj_list[edge.0 as usize].push(
                WeightedEdge{
                    target: edge.1,
                    weight: edge.2,
            });
        }

        DAG{
            adj_list: adj_list
        }
    }
}


#[cfg(test)]
mod dag_tests {
    use super::*;
    #[test]
    fn ini_edge() {
        let edge = WeightedEdge{weight: 1.3, target:4,};
        assert_eq!(edge.target, 4);
        assert_eq!(edge.weight, 1.3);
    }
    
    #[test]
    fn init_dag(){
        let n_nodes = 3;
        let edge_list = vec![(0,1,1.0),(0,2,1.0), (1,2,1.0)];
        let dag = DAG::init(&n_nodes, &edge_list);
        assert_eq!(dag.adj_list.len(), n_nodes as usize);
        assert_eq!(dag.adj_list[0][0].target,1);
        assert_eq!(dag.adj_list[0][0].weight,1.0);
    }
}
