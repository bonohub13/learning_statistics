use rayon::prelude::*;
use stats::errors::StatsError;
use std::{error::Error, path::Path};

mod statistics;
mod utils;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct IrisRecord<T> {
    pub sepal_length: T,
    pub sepal_width: T,
    pub petal_length: T,
    pub petal_width: T,
    pub species: String,
}

#[derive(Debug, Default)]
pub struct IrisDataset<T> {
    dataset: Vec<IrisRecord<T>>,
    sepal_length: Vec<T>,
    sepal_width: Vec<T>,
    petal_length: Vec<T>,
    petal_width: Vec<T>,
}

#[allow(dead_code)]
impl IrisDataset<f64> {
    pub fn parse_csv(file_path: &str) -> Result<Self, Box<dyn Error>> {
        // Check if file exists
        if !Path::new(file_path).exists() {
            return Err(Box::new(StatsError::FileDoesNotExist(
                file_path.to_string(),
            )));
        }

        let mut rdr = csv::Reader::from_path(file_path)?;
        let mut dataset: Vec<IrisRecord<f64>> = vec![];

        // Deserialize reader input and if record is valid,
        // add record to dataset
        for result in rdr.deserialize() {
            let record: IrisRecord<f64> = result?;

            dataset.push(record);
        }

        Ok(Self {
            sepal_length: dataset.iter().map(|record| record.sepal_length).collect(),
            sepal_width: dataset.iter().map(|record| record.sepal_width).collect(),
            petal_length: dataset.iter().map(|record| record.petal_length).collect(),
            petal_width: dataset.iter().map(|record| record.petal_width).collect(),
            dataset,
        })
    }

    pub fn new(records: &[IrisRecord<f64>]) -> Self {
        let mut sepal_length: Vec<f64> = vec![];
        let mut sepal_width: Vec<f64> = vec![];
        let mut petal_length: Vec<f64> = vec![];
        let mut petal_width: Vec<f64> = vec![];

        records.iter().for_each(|record| {
            sepal_length.push(record.sepal_length);
            sepal_width.push(record.sepal_width);
            petal_length.push(record.petal_length);
            petal_width.push(record.petal_width);
        });

        Self {
            dataset: records.to_vec(),
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
        }
    }

    pub fn data(&self) -> &[IrisRecord<f64>] {
        &self.dataset
    }

    pub fn sepal_length(&self) -> &[f64] {
        &self.sepal_length
    }

    pub fn sepal_width(&self) -> &[f64] {
        &self.sepal_width
    }

    pub fn petal_length(&self) -> &[f64] {
        &self.petal_length
    }

    pub fn petal_width(&self) -> &[f64] {
        &self.petal_width
    }

    pub fn species(&self) -> Vec<String> {
        let mut species: Vec<String> = vec![];

        self.dataset.iter().for_each(|record| {
            if !species.contains(&record.species) {
                species.push(record.species.clone())
            }
        });

        species
    }

    pub fn per_species(&self, species: &str) -> Result<Self, Box<dyn Error>> {
        if !self
            .dataset
            .par_iter()
            .any(|record| record.species == species)
        {
            return Err(Box::new(StatsError::InvalidRecord(format!(
                "Species ({}) does not exist.",
                species
            ))));
        }

        let dataset = self
            .dataset
            .iter()
            .filter(|record| record.species == species)
            .map(|record| IrisRecord {
                sepal_length: record.sepal_length,
                sepal_width: record.sepal_width,
                petal_length: record.petal_length,
                petal_width: record.petal_width,
                species: record.species.clone(),
            })
            .collect::<Vec<IrisRecord<f64>>>();

        Ok(Self {
            sepal_length: dataset.iter().map(|record| record.sepal_length).collect(),
            sepal_width: dataset.iter().map(|record| record.sepal_width).collect(),
            petal_length: dataset.iter().map(|record| record.petal_length).collect(),
            petal_width: dataset.iter().map(|record| record.petal_width).collect(),
            dataset,
        })
    }
}

impl IrisDataset<(f64, usize)> {
    pub fn new(records: &[IrisRecord<(f64, usize)>]) -> Self {
        let mut sepal_length: Vec<(f64, usize)> = vec![];
        let mut sepal_width: Vec<(f64, usize)> = vec![];
        let mut petal_length: Vec<(f64, usize)> = vec![];
        let mut petal_width: Vec<(f64, usize)> = vec![];

        records.iter().for_each(|record| {
            sepal_length.push(record.sepal_length);
            sepal_width.push(record.sepal_width);
            petal_length.push(record.petal_length);
            petal_width.push(record.petal_width);
        });

        Self {
            dataset: records.to_vec(),
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
        }
    }

    pub fn data(&self) -> &[IrisRecord<(f64, usize)>] {
        &self.dataset
    }

    pub fn sepal_length(&self) -> &[(f64, usize)] {
        &self.sepal_length
    }

    pub fn sepal_width(&self) -> &[(f64, usize)] {
        &self.sepal_width
    }

    pub fn petal_length(&self) -> &[(f64, usize)] {
        &self.petal_length
    }

    pub fn petal_width(&self) -> &[(f64, usize)] {
        &self.petal_width
    }

    pub fn species(&self) -> Vec<String> {
        let mut species: Vec<String> = vec![];

        self.dataset.iter().for_each(|record| {
            if !species.contains(&record.species) {
                species.push(record.species.clone())
            }
        });

        species
    }

    pub fn per_species(&self, species: &str) -> Result<Self, Box<dyn Error>> {
        if !self
            .dataset
            .par_iter()
            .any(|record| record.species == species)
        {
            return Err(Box::new(StatsError::InvalidRecord(format!(
                "Species ({}) does not exist.",
                species
            ))));
        }

        let dataset = self
            .dataset
            .iter()
            .filter(|record| record.species == species)
            .map(|record| IrisRecord {
                sepal_length: record.sepal_length,
                sepal_width: record.sepal_width,
                petal_length: record.petal_length,
                petal_width: record.petal_width,
                species: record.species.clone(),
            })
            .collect::<Vec<IrisRecord<(f64, usize)>>>();

        Ok(Self {
            sepal_length: dataset.iter().map(|record| record.sepal_length).collect(),
            sepal_width: dataset.iter().map(|record| record.sepal_width).collect(),
            petal_length: dataset.iter().map(|record| record.petal_length).collect(),
            petal_width: dataset.iter().map(|record| record.petal_width).collect(),
            dataset,
        })
    }
}
