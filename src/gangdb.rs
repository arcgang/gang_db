
    use std::cmp::Ordering;

    use crate::dbmodel;
    // Define a KD tree structure
    #[derive(Debug)]
    pub struct KDTree {
        root: Option<Box<dbmodel::Node>>,
    }

    impl KDTree {
        // create the new function that returns instance of self of type KDTree.
        fn new(vectors: Vec<dbmodel::Vector>) -> Self {
            
            //create a mutable copy of the input which is vectors. It will undego changes.
            let mut sorted_vectors = vectors;

            // First we will sort the vector.
            sorted_vectors.sort_by(|a, b| {
                a.coordinates
                    .iter()
                    .sum::<f64>()
                    .partial_cmp(&b.coordinates.iter().sum::<f64>())
                    .unwrap()
            });

            //constructs a new KDTree instance with the root
            KDTree {
                root: KDTree::construct_kd_tree(sorted_vectors, 0),
            }
        }

        fn construct_kd_tree(vectors: Vec<dbmodel::Vector>, depth: usize) -> Option<Box<dbmodel::Node>> {
        
            // the tree cannot be built from an empty set of vectors.
            // immediately return.
            if vectors.is_empty() {
                return None;
            }

            let K = 2;
            // determine the axis for spliting the KDTree. 
            // Alternate the axis based on the depth 
            let axis = depth % K; // K is the number of dimensions
       
            let median_index = vectors.len() / 2;
            let mut sorted_vectors = vectors;

            sorted_vectors.sort_by(|a, b| {
                a.coordinates[axis]
                    .partial_cmp(&b.coordinates[axis])
                    .unwrap_or(Ordering::Equal)
            });

            //Recursively builds the left and right subtrees of the KD-tree 
            Some(Box::new(dbmodel::Node {
                vector: sorted_vectors[median_index].clone(),
                left: KDTree::construct_kd_tree(sorted_vectors[..median_index].to_vec(), depth+1),
                right: KDTree::construct_kd_tree(sorted_vectors[median_index + 1..].to_vec(), depth+1),
            }))
        }

        fn nearest_neighbor(
            &self,
            query_vector: &dbmodel::Vector,
        ) -> Option<dbmodel::Vector> {
            self._nearest_neighbor(&self.root, query_vector, 0)
        }

        fn _nearest_neighbor(
            &self,
            node: &Option<Box<dbmodel::Node>>,
            query_vector: &dbmodel::Vector,
            depth: usize,
        ) -> Option<dbmodel::Vector> {
            match node {
                None => None,
                Some(n) => {
                    let axis = depth % query_vector.coordinates.len();
                    let next_node = if query_vector.coordinates[axis] < n.vector.coordinates[axis] {
                        &n.left
                    } else {
                        &n.right
                    };
                    let mut nearest = self._nearest_neighbor(next_node, query_vector, depth + 1);

                    if nearest.is_none()
                        || cosine_similarity(&n.vector, query_vector)
                            > cosine_similarity(&nearest.as_ref().unwrap(), query_vector)
                    {

                        nearest = Some(n.vector.clone());
                    }

                    let axis_dist = (query_vector.coordinates[axis] - n.vector.coordinates[axis]).abs();
                    if axis_dist > cosine_similarity(&nearest.as_ref().unwrap(), query_vector) {

                        let other_node = if let Some(next_node_inner) = next_node.as_ref() {
                            if let Some(left_inner) = n.left.as_ref() {
                                if next_node_inner.vector == left_inner.vector {
                                    &n.right
                                } else {
                                    &n.left
                                }
                            } else {
                                &n.left
                            }
                        } else {
                            &n.left
                        };

                        if let Some(mut new_nearest) =
                            self._nearest_neighbor(other_node, query_vector, depth + 1)
                        {
                            // check the opposite / adjacent branches for high similarities
                            if cosine_similarity(&new_nearest, query_vector)
                                > cosine_similarity(&nearest.as_ref().unwrap(), query_vector)
                            {

                                nearest = Some(new_nearest);
                            }
                        }
                    }
                    nearest
                }
            }
        }
    }

    // calculate cosine similarity between two vectors
    fn cosine_similarity(vector_1: &dbmodel::Vector, vector_2: &dbmodel::Vector) -> f64 {
        let dot_product = vector_1
            .coordinates
            .iter()
            .zip(vector_2.coordinates.iter())
            .map(|(a, b)| a * b)
            .sum::<f64>();
        let norm_a = vector_1.coordinates.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
        let norm_b = vector_2.coordinates.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0; // Return 0 if one of the magnitudes is zero
        }

        dot_product / (norm_a * norm_b)
    }



#[cfg(test)]
mod tests {
    // Import necessary items
    use super::*;
    use dbmodel::Vector;

    #[test]
    fn test_kd_tree_nearest_neighbor() {

        let vectors = vec![
            Vector::new(vec![4.5, 4.0]),
            Vector::new(vec![7.5, 5.0]),
            Vector::new(vec![12.0, 8.0]),
            Vector::new(vec![3.5, 10.2]),
            Vector::new(vec![11.0, 11.9]),
            Vector::new(vec![14.8, 11.4]),
            Vector::new(vec![19.0, 17.5]),
            Vector::new(vec![8.8, 15.8]),
            Vector::new(vec![15.0, 14.0]),
            Vector::new(vec![19.0, 12.0]),
            Vector::new(vec![21.5, 8.6]),
            Vector::new(vec![17.5, 6.5]),
            Vector::new(vec![13.2, 5.7]),
            Vector::new(vec![5.0, 17.7]),
        ];

        let kd_tree = KDTree::new(vectors);
        let query_vector: Vector = Vector::new(vec![9.0, 13.0]);

        assert_eq!(kd_tree.nearest_neighbor(&query_vector), Some(Vector::new(vec![8.8, 15.8])));

        if let Some(nearest) = kd_tree.nearest_neighbor(&query_vector) {
            println!("Nearest neighbor to {:?} is {:?}", query_vector, nearest);
        } else {
            println!("No nearest neighbor found");
        }
    }
}
