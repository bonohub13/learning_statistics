use std::fmt;

impl fmt::Display for super::IrisDataset<f64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_length = self.dataset.len();
        let log10 = if data_length > 0 {
            data_length.ilog10()
        } else {
            0
        };
        let mut output_str = format!(
            "{} | sepal_length\tsepal_width\tpetal_length\tpetal_width\tspecies",
            (0..=log10)
                .into_iter()
                .map(|_| " ")
                .collect::<Vec<&str>>()
                .join("")
        );
        output_str.push_str(
            format!(
                "\n{}-+{}",
                (0..=log10)
                    .into_iter()
                    .map(|_| "-")
                    .collect::<Vec<&str>>()
                    .join(""),
                (0..output_str.len())
                    .into_iter()
                    .map(|index| if output_str.as_bytes()[index] == b'\t' {
                        "----"
                    } else {
                        "-"
                    })
                    .collect::<Vec<&str>>()
                    .join("")
            )
            .as_str(),
        );

        self.dataset.iter().enumerate().for_each(|(index, record)| {
            let index_log10 = if index == 0 { 0 } else { index.ilog10() };
            let mut index_formatted = format!("{}", index);

            (index_log10..log10).into_iter().for_each(|_| {
                index_formatted = format!("0{}", index_formatted);
            });

            output_str = format!(
                "{}\n{} | {}\t\t{}\t\t{}\t\t{}\t\t{}",
                output_str,
                index_formatted,
                if format!("{}", record.sepal_length).len() < 3 {
                    format!("{}\t", record.sepal_length)
                } else {
                    record.sepal_length.to_string()
                },
                record.sepal_width,
                record.petal_length,
                record.petal_width,
                record.species
            );
        });

        write!(f, "{}", output_str)
    }
}
