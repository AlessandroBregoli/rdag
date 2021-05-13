enum NodeColor{
    White,
    Gray,
    Black,
}

pub struct DAG {
    adj_list: Vec<Vec<WeightedEdge>>,
}

pub enum OptimalPathType {
    Shortest,
    Longest,
}

#[derive(Clone)]
struct WeightedEdge {
    weight: f64,
    target: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Step {
    pub distance: f64,
    pub predecessor: Option<u64>,
}


impl DAG {
    pub fn init(n_nodes: &u64, edge_list: &Vec<(u64,u64,f64)>) -> DAG {
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

    pub fn topological_sort(&self) -> Vec<u64> {
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

    pub fn optimal_path(&self, path_type: &OptimalPathType, source: u64) -> Vec<Step>{
        let mut nodes:Vec<Step> = Vec::new();
        
        for _i in 0.. self.adj_list.len() {
            match path_type {
                OptimalPathType::Shortest => nodes.push(Step{distance: f64::INFINITY, predecessor: None}),
                OptimalPathType::Longest => nodes.push(Step{distance: f64::NEG_INFINITY, predecessor: None})
            }
        }
        nodes[source as usize].distance = 0.0;
        let sorted_nodes = self.topological_sort();
        for x in sorted_nodes {
            for edge in self.adj_list[x as usize].iter(){
                DAG::relax(&path_type, &mut nodes, x, edge.target, edge.weight);
            }
        }
        return nodes;
        
    }

    fn relax(path_type: &OptimalPathType, nodes: &mut Vec<Step>, u: u64, v: u64, vw_distance: f64) {
        macro_rules! update {() =>{
            nodes[v as usize].predecessor = Some(u);
            nodes[v as usize].distance = nodes[u as usize].distance + vw_distance;
        };}

        match path_type {
            OptimalPathType::Shortest => {
                if nodes[v as usize].distance > nodes[u as usize].distance + vw_distance {
                    update!();
                }
            }
            OptimalPathType::Longest => {
                if nodes[v as usize].distance < nodes[u as usize].distance + vw_distance {
                    update!();
                }

            }
        }         
    } 
}


#[cfg(test)]
mod tests {
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


   #[test]
    fn trivial_shortest_path(){
        let n_nodes = 3;
        let edge_list = vec![
            (0,1,1.0),
            (0,2,1.0),
            (1,2,1.0)];
        let dag = DAG::init(&n_nodes, &edge_list);
        let op = dag.optimal_path(&OptimalPathType::Shortest, 0);
        let true_op = vec![
            Step{distance: 0.0, predecessor: None},
            Step{distance: 1.0, predecessor: Some(0)},
            Step{distance: 1.0, predecessor: Some(0)}
        ];
        assert_eq!(op, true_op);
        
    }


    #[test]
    fn trivial_longest_path(){
        let n_nodes = 3;
        let edge_list = vec![
            (0,1,1.0),
            (0,2,1.0),
            (1,2,1.0)];
        let dag = DAG::init(&n_nodes, &edge_list);
        let op = dag.optimal_path(&OptimalPathType::Longest, 0);
        let true_op = vec![
            Step{distance: 0.0, predecessor: None},
            Step{distance: 1.0, predecessor: Some(0)},
            Step{distance: 2.0, predecessor: Some(1)}
        ];
        assert_eq!(op, true_op);

        
    }


    #[test]
    fn intermediate_shortest_path(){
        let n_nodes = 8;
        let edge_list = vec![
            (0,1,2.0),
            (1,2,3.0),
            (2,3,1.0),
            (0,4,4.0),
            (4,3,4.0),
            (5,4,1.0),
            (5,6,1.0),
            (7,0,1.0)];
        let dag = DAG::init(&n_nodes, &edge_list);
        let op = dag.optimal_path(&OptimalPathType::Shortest, 7);
        let true_op = vec![
            Step{distance: 1.0, predecessor: Some(7)},
            Step{distance: 3.0, predecessor: Some(0)},
            Step{distance: 6.0, predecessor: Some(1)},
            Step{distance: 7.0, predecessor: Some(2)},
            Step{distance: 5.0, predecessor: Some(0)},
            Step{distance: f64::INFINITY, predecessor: None},
            Step{distance: f64::INFINITY, predecessor: None},
            Step{distance: 0.0, predecessor: None},
        ];
        assert_eq!(op, true_op);

    }


    #[test]
    fn intermediate_longest_path(){
        let n_nodes = 8;
        let edge_list = vec![
            (0,1,2.0),
            (1,2,3.0),
            (2,3,1.0),
            (0,4,4.0),
            (4,3,4.0),
            (5,4,1.0),
            (5,6,1.0),
            (7,0,1.0)];
        let dag = DAG::init(&n_nodes, &edge_list);
        let op = dag.optimal_path(&OptimalPathType::Longest, 7);
        let true_op = vec![
            Step{distance: 1.0, predecessor: Some(7)},
            Step{distance: 3.0, predecessor: Some(0)},
            Step{distance: 6.0, predecessor: Some(1)},
            Step{distance: 9.0, predecessor: Some(4)},
            Step{distance: 5.0, predecessor: Some(0)},
            Step{distance: f64::NEG_INFINITY, predecessor: None},
            Step{distance: f64::NEG_INFINITY, predecessor: None},
            Step{distance: 0.0, predecessor: None},
        ];
        assert_eq!(op, true_op);

    }

}
