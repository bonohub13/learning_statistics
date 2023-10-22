mod iris;

use stats::prelude::*;
use std::error::Error;

const IRIS_DATASET_PATH: &'static str = "datas/iris.csv";

fn main() -> Result<(), Box<dyn Error>> {
    let dataset = iris::IrisDataset::parse_csv(IRIS_DATASET_PATH)?;
    let species = dataset.species();
    let dataset_setosa = dataset.per_species(species[0].as_str())?;
    let dataset_versicolor = dataset.per_species(species[1].as_str())?;
    let dataset_virginica = dataset.per_species(species[2].as_str())?;

    // println!("{}", dataset);
    // println!("Species: {:?}", species);

    // let mean = dataset.mean();
    // let mean_setosa = dataset_setosa.mean();
    // let mean_versicolor = dataset_versicolor.mean();
    // let mean_virginica = dataset_virginica.mean();

    // println!("Mean: {:?}", mean);
    // println!("Mean (setosa): {:?}", mean_setosa);
    // println!("Mean (versicolor): {:?}", mean_versicolor);
    // println!("Mean (virginica): {:?}", mean_virginica);

    // let median = dataset.median();
    // let median_setosa = dataset_setosa.median();
    // let median_versicolor = dataset_versicolor.median();
    // let median_virginica = dataset_virginica.median();

    // println!("Median: {:?}", median);
    // println!("Median (setosa): {:?}", median_setosa);
    // println!("Median (versicolor): {:?}", median_versicolor);
    // println!("Median (virginica): {:?}", median_virginica);

    // let mode = dataset.mode();
    // let mode_setosa = dataset_setosa.mode();
    // let mode_versicolor = dataset_versicolor.mode();
    // let mode_virginica = dataset_virginica.mode();

    // println!("Mode: {:?}", mode);
    // println!("Mode (setosa): {:?}", mode_setosa);
    // println!("Mode (versicolor): {:?}", mode_versicolor);
    // println!("Mode (virginica): {:?}", mode_virginica);

    // let var = dataset.var();
    // let var_setosa = dataset_setosa.var();
    // let var_versicolor = dataset_versicolor.var();
    // let var_virginica = dataset_virginica.var();

    // println!("Variance: {:?}", var);
    // println!("Variance (setosa): {:?}", var_setosa);
    // println!("Variance (versicolor): {:?}", var_versicolor);
    // println!("Variance (virginica): {:?}", var_virginica);

    // let std = dataset.std();
    // let std_setosa = dataset_setosa.std();
    // let std_versicolor = dataset_versicolor.std();
    // let std_virginica = dataset_virginica.std();

    // println!("Standard Deviation: {:?}", std);
    // println!("Standard Deviation (setosa): {:?}", std_setosa);
    // println!("Standard Deviation (versicolor): {:?}", std_versicolor);
    // println!("Standard Deviation (virginica): {:?}", std_virginica);

    let range = dataset.range();
    let range_setosa = dataset_setosa.range();
    let range_versicolor = dataset_versicolor.range();
    let range_virginica = dataset_virginica.range();

    println!("Range: {:?}", range);
    println!("Range (setosa): {:?}", range_setosa);
    println!("Range (versicolor): {:?}", range_versicolor);
    println!("Range (virginica): {:?}", range_virginica);

    Ok(())
}
