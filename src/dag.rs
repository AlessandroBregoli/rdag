enum NodeColor{
    White,
    Gray,
    Black,
}

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

    fn topological_sort(&self) -> Vec<u64> {
        let mut t_sort: Vec<u64> = Vec::new();
        let mut colors: Vec<NodeColor> = Vec::new();
        for _ in 0..self.adj_list.len(){
            colors.push(NodeColor::White);
        }

        for x in 0..self.adj_list.len(){
            if let NodeColor::White = colors[x] {
                self.topological_dfs_visit(&mut t_sort, &mut colors, x)
            }
        }

        
        return t_sort;
    }


    fn topological_dfs_visit(&self, mut t_sort: &mut Vec<u64>, mut colors: &mut Vec<NodeColor>, node: usize){
        *colors.get_mut(node).unwrap() = NodeColor::Gray;
        for x in 0..self.adj_list[node].len(){
            let next = self.adj_list[node][x].target as usize;
            if let NodeColor::White = colors[next] {
                self.topological_dfs_visit(&mut t_sort, &mut colors, next);
            }
        }
        *colors.get_mut(node).unwrap() = NodeColor::Black;
        t_sort.insert(0, node as u64);
    }
}


#[cfg(test)]
mod dag_tests {
    use super::*;
    #[test]
    fn init_edge() {
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

    #[test]
    fn trivial_tsort(){
        let n_nodes = 3;
        let edge_list = vec![(0,1,1.0),(0,2,1.0), (1,2,1.0)];
        let dag = DAG::init(&n_nodes, &edge_list);
        let t_sort = dag.topological_sort();
        assert_eq!(t_sort, vec![0,1,2]);

    }


    #[test]
    fn trivial_tsort2(){
        let n_nodes = 3;
        let edge_list = vec![(0,2,1.0),(0,1,1.0), (1,2,1.0)];
        let dag = DAG::init(&n_nodes, &edge_list);
        let t_sort = dag.topological_sort();
        assert_eq!(t_sort, vec![0,1,2]);

    }


    #[test]
    fn intermediate_tsort(){
        let n_nodes = 8;
        let edge_list = vec![
            (0,1,1.0),
            (1,2,1.0),
            (2,3,1.0),
            (0,4,1.0),
            (4,3,1.0),
            (5,4,1.0),
            (5,6,1.0),
            (7,0,1.0)];
        let dag = DAG::init(&n_nodes, &edge_list);
        let t_sort = dag.topological_sort();
        assert_eq!(t_sort, vec![7,5,6,0,4,1,2,3]);

    }
}
