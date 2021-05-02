trait Optimizable {
    type Penalty: PartialOrd;

    fn penalty(&self) -> Self::Penalty;

    fn perform_swap(&mut self, i: usize, j: usize);

    fn perform_swaps(&mut self) -> Self::Penalty;
}

#[derive(Clone, Copy, Debug)]
struct TrackWrapper {
    track_id: usize,
    energy: f64,
}

impl TrackWrapper {
    pub fn new(track_id: usize, energy: f64) -> Self {
        Self { track_id, energy }
    }
}

struct EnergyDist {
    input: Vec<TrackWrapper>,
    func: Vec<f64>,
}

impl EnergyDist {
    pub fn new(input: Vec<TrackWrapper>, func: Vec<f64>) -> Self {
        Self { input, func }
    }
}

impl Optimizable for EnergyDist {
    type Penalty = f64;

    fn penalty(&self) -> Self::Penalty {
        self.input
            .iter()
            .zip(self.func.iter())
            .map(|x| (x.0.energy - x.1).abs())
            .sum()
    }

    fn perform_swap(&mut self, i: usize, j: usize) {
        let left = self.input[i];
        let right = self.input[j];

        self.input[i] = right;
        self.input[j] = left;

        // perform swap, check penalty
    }

    fn perform_swaps(&mut self) -> Self::Penalty {
        let mut best_distance = self.penalty();
        let mut changed;

        loop {
            changed = false;

            for i in 0..self.input.len() {
                for j in 0..self.func.len() {
                    // Swap the two positions
                    self.perform_swap(i, j);
                    let new_distance = self.penalty();
                    if new_distance < best_distance {
                        best_distance = new_distance;
                        changed = true;
                    } else {
                        // Swap them back
                        self.perform_swap(i, j);
                    }
                }
            }

            if !changed {
                break;
            }
        }

        best_distance
    }
}

#[cfg(test)]
mod two_opt_test {
    use super::*;

    #[test]
    fn test_basic() {
        let input = vec![
            TrackWrapper::new(0, 0.0),
            TrackWrapper::new(0, 0.8),
            TrackWrapper::new(0, 0.9),
            TrackWrapper::new(0, 0.5),
            TrackWrapper::new(0, 0.35),
            TrackWrapper::new(0, 0.413),
        ];
        let func = vec![0.0, 0.8, 0.9, 0.5, 0.35, 0.413];

        let mut energy_dist = EnergyDist::new(input, func);
        let final_penalty = energy_dist.perform_swaps();

        println!("Final output: {:?}", energy_dist.input);

        assert_eq!(final_penalty, 0.0);
    }

    #[test]
    fn test_basic_should_min() {
        let input = vec![
            TrackWrapper::new(0, 0.0),
            TrackWrapper::new(0, 0.8),
            TrackWrapper::new(0, 0.9),
            TrackWrapper::new(0, 0.5),
            TrackWrapper::new(0, 0.35),
            TrackWrapper::new(0, 0.413),
        ];

        let func = vec![0.8, 0.9, 0.5, 0.35, 0.413, 0.0];

        let mut energy_dist = EnergyDist::new(input, func);
        let final_penalty = energy_dist.perform_swaps();

        println!("Final output: {:?}", energy_dist.input);

        assert_eq!(final_penalty, 0.0);
    }
}
