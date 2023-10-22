use rayon::prelude::*;
use stats::prelude::*;

impl Mean for super::IrisDataset<f64> {
    type Output = super::IrisRecord<f64>;

    fn mean(&self) -> Self::Output {
        let data_size = self.dataset.len();

        let species = self.species();

        let sepal_length = self.sepal_length().par_iter().sum::<f64>() / data_size as f64;
        let sepal_width = self.sepal_width().par_iter().sum::<f64>() / data_size as f64;
        let petal_length = self.petal_length().par_iter().sum::<f64>() / data_size as f64;
        let petal_width = self.petal_width().par_iter().sum::<f64>() / data_size as f64;

        Self::Output {
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
            species: if species.len() == 1 {
                species[0].clone()
            } else {
                "".to_string()
            },
        }
    }
}

impl Median for super::IrisDataset<f64> {
    type Output = super::IrisRecord<f64>;

    fn median(&self) -> Self::Output {
        let data_size = self.dataset.len();
        let center_index = data_size / 2;
        let species = self.species();

        let mut sepal_length = self.sepal_length().to_vec();
        let mut sepal_width = self.sepal_width().to_vec();
        let mut petal_length = self.petal_length().to_vec();
        let mut petal_width = self.petal_width().to_vec();

        sepal_length.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sepal_width.sort_by(|a, b| a.partial_cmp(b).unwrap());
        petal_length.sort_by(|a, b| a.partial_cmp(b).unwrap());
        petal_width.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if data_size % 2 == 0 {
            Self::Output {
                sepal_length: (sepal_length[center_index - 1] + sepal_length[center_index]) / 2.,
                sepal_width: (sepal_width[center_index - 1] + sepal_width[center_index]) / 2.,
                petal_length: (petal_length[center_index - 1] + petal_length[center_index]) / 2.,
                petal_width: (petal_width[center_index - 1] + petal_width[center_index]) / 2.,
                species: if species.len() == 1 {
                    species[0].clone()
                } else {
                    "".to_string()
                },
            }
        } else {
            Self::Output {
                sepal_length: sepal_length[center_index],
                sepal_width: sepal_width[center_index],
                petal_length: petal_length[center_index],
                petal_width: petal_width[center_index],
                species: if species.len() == 1 {
                    species[0].clone()
                } else {
                    "".to_string()
                },
            }
        }
    }
}

impl Mode for super::IrisDataset<f64> {
    type Output = super::IrisRecord<(f64, usize)>;

    fn mode(&self) -> Self::Output {
        let mut sepal_length = self.sepal_length().to_vec();
        let mut sepal_length_mode: [(f64, usize); 2] = [(0., 0), (0., 0)];
        let mut sepal_width = self.sepal_width().to_vec();
        let mut sepal_width_mode: [(f64, usize); 2] = [(0., 0), (0., 0)];
        let mut petal_length = self.petal_length().to_vec();
        let mut petal_length_mode: [(f64, usize); 2] = [(0., 0), (0., 0)];
        let mut petal_width = self.petal_width().to_vec();
        let mut petal_width_mode: [(f64, usize); 2] = [(0., 0), (0., 0)];

        sepal_length.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sepal_width.sort_by(|a, b| a.partial_cmp(b).unwrap());
        petal_length.sort_by(|a, b| a.partial_cmp(b).unwrap());
        petal_width.sort_by(|a, b| a.partial_cmp(b).unwrap());

        sepal_length.iter().for_each(|record| {
            if sepal_length_mode[0] == (0., 0) || sepal_length_mode[0].0 == *record {
                sepal_length_mode[0].0 = *record;
                sepal_length_mode[0].1 += 1;
            } else if sepal_length_mode[1] == (0., 0) || sepal_length_mode[1].0 == *record {
                sepal_length_mode[1].0 = *record;
                sepal_length_mode[1].1 += 1;
            } else if sepal_length_mode[1].0 != *record || record == sepal_length.last().unwrap() {
                sepal_length_mode[0] = if sepal_length_mode[0].1 > sepal_length_mode[1].1 {
                    sepal_length_mode[0]
                } else {
                    sepal_length_mode[1]
                };
                sepal_length_mode[1] = (0., 0);
            }
        });
        sepal_width.iter().for_each(|record| {
            if sepal_width_mode[0] == (0., 0) || sepal_width_mode[0].0 == *record {
                sepal_width_mode[0].0 = *record;
                sepal_width_mode[0].1 += 1;
            } else if sepal_width_mode[1] == (0., 0) || sepal_width_mode[1].0 == *record {
                sepal_width_mode[1].0 = *record;
                sepal_width_mode[1].1 += 1;
            } else if sepal_width_mode[1].0 != *record || record == sepal_width.last().unwrap() {
                sepal_width_mode[0] = if sepal_width_mode[0].1 > sepal_width_mode[1].1 {
                    sepal_width_mode[0]
                } else {
                    sepal_width_mode[1]
                };
                sepal_width_mode[1] = (0., 0);
            }
        });
        petal_length.iter().for_each(|record| {
            if petal_length_mode[0] == (0., 0) || petal_length_mode[0].0 == *record {
                petal_length_mode[0].0 = *record;
                petal_length_mode[0].1 += 1;
            } else if petal_length_mode[1] == (0., 0) || petal_length_mode[1].0 == *record {
                petal_length_mode[1].0 = *record;
                petal_length_mode[1].1 += 1;
            } else if petal_length_mode[1].0 != *record || record == petal_length.last().unwrap() {
                petal_length_mode[0] = if petal_length_mode[0].1 > petal_length_mode[1].1 {
                    petal_length_mode[0]
                } else {
                    petal_length_mode[1]
                };
                petal_length_mode[1] = (0., 0);
            }
        });
        petal_width.iter().for_each(|record| {
            if petal_width_mode[0] == (0., 0) || petal_width_mode[0].0 == *record {
                petal_width_mode[0].0 = *record;
                petal_width_mode[0].1 += 1;
            } else if petal_width_mode[1] == (0., 0) || petal_width_mode[1].0 == *record {
                petal_width_mode[1].0 = *record;
                petal_width_mode[1].1 += 1;
            } else if petal_width_mode[1].0 != *record || record == petal_width.last().unwrap() {
                petal_width_mode[0] = if petal_width_mode[0].1 > petal_width_mode[1].1 {
                    petal_width_mode[0]
                } else {
                    petal_width_mode[1]
                };
                petal_width_mode[1] = (0., 0);
            }
        });

        let species = self.species();

        Self::Output {
            sepal_length: sepal_length_mode[0],
            sepal_width: sepal_width_mode[0],
            petal_length: petal_length_mode[0],
            petal_width: petal_width_mode[0],
            species: if species.len() == 1 {
                species[0].clone()
            } else {
                "".to_string()
            },
        }
    }
}

impl Variance for super::IrisDataset<f64> {
    fn var(&self) -> Self::Output {
        let species = self.species();
        let mean = self.mean();
        let data_size = self.dataset.len() as f64;

        let sepal_length = self
            .sepal_length()
            .par_iter()
            .map(|record| (*record - mean.sepal_length).powi(2))
            .sum::<f64>()
            / data_size;
        let sepal_width = self
            .sepal_width()
            .par_iter()
            .map(|record| (*record - mean.sepal_width).powi(2))
            .sum::<f64>()
            / data_size;
        let petal_length = self
            .petal_length()
            .par_iter()
            .map(|record| (*record - mean.petal_length).powi(2))
            .sum::<f64>()
            / data_size;
        let petal_width = self
            .petal_width()
            .par_iter()
            .map(|record| (*record - mean.petal_width).powi(2))
            .sum::<f64>()
            / data_size;

        Self::Output {
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
            species: if species.len() == 1 {
                species[0].clone()
            } else {
                "".to_string()
            },
        }
    }
}

impl Deviation for super::IrisDataset<f64> {
    fn std(&self) -> Self::Output {
        let var = self.var();

        Self::Output {
            sepal_length: var.sepal_length.sqrt(),
            sepal_width: var.sepal_width.sqrt(),
            petal_length: var.petal_length.sqrt(),
            petal_width: var.petal_width.sqrt(),
            species: var.species,
        }
    }
}

impl Range for super::IrisDataset<f64> {
    type Output = super::IrisRecord<f64>;

    fn range(&self) -> Self::Output {
        let sepal_length = {
            let sepal_length = self.sepal_length();

            sepal_length
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                - sepal_length
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
        };
        let sepal_width = {
            let sepal_width = self.sepal_width();

            sepal_width
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                - sepal_width
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
        };
        let petal_length = {
            let petal_length = self.petal_length();

            petal_length
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                - petal_length
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
        };
        let petal_width = {
            let petal_width = self.petal_width();

            petal_width
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                - petal_width
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
        };
        let species = self.species();

        Self::Output {
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
            species: if species.len() == 1 {
                species[0].clone()
            } else {
                "".to_string()
            },
        }
    }

    fn percentile(&self, percent: u32) -> Self::Output {
        let data_size = self.dataset.len();
        let index = (data_size as f64 / (percent as f64 / 100.)).round() as usize;
        let species = self.species();
        let sepal_length = {
            let mut sepal_length = self.sepal_length().to_vec();

            sepal_length.sort_by(|a, b| a.partial_cmp(b).unwrap());

            sepal_length[index]
        };
        let sepal_width = {
            let mut sepal_width = self.sepal_width().to_vec();

            sepal_width.sort_by(|a, b| a.partial_cmp(b).unwrap());

            sepal_width[index]
        };
        let petal_length = {
            let mut petal_length = self.petal_length().to_vec();

            petal_length.sort_by(|a, b| a.partial_cmp(b).unwrap());

            petal_length[index]
        };
        let petal_width = {
            let mut petal_width = self.petal_width().to_vec();

            petal_width.sort_by(|a, b| a.partial_cmp(b).unwrap());

            petal_width[index]
        };

        Self::Output {
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
            species: if species.len() == 1 {
                species[0].clone()
            } else {
                "".to_string()
            },
        }
    }

    fn interquartile_range(&self) -> Self::Output {
        todo!()
    }
}
