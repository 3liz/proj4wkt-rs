//!
//! Output to projstring
//!

/*
fn sanitize(&mut self) -> Result<()> {
    self.fix_datum_code();

    if self.datum == "wgs84" && self.proj_name == "Mercator_Auxiliary_Sphere" {
        self.sphere = true;
    }

    Ok(())
}

fn fix_datum_code(&mut self) {
    if self.datum.starts_with("d_") {
        self.datum = self.datum[..2].into();
    }

    match self.datum.as_str() {
        "new_zealand_geodetic_datum_1949" | "new_zealand_1949" => self.datum = "nzgd49".into(),
        "wgs_1984" | "world_geodetic_system_1984" => self.datum = "wgs84".into(),
        "ch1903+" => self.datum = "ch1903".into(),
        code if code.ends_with("_ferro") => self.datum.truncate(code.len() - 6),
        code if code.ends_with("_jakarta") => self.datum.truncate(code.len() - 8),
        code if code.contains("belge") => self.datum = "rnb72".into(),
        code if code.contains("osgb_1936") => self.datum = "osgb36".into(),
        code if code.contains("osni_1952") => self.datum = "osni52".into(),
        code if code.contains("tm65") || code.contains("geodetic_datum_of_1965") => {
            self.datum = "ire65".into()
        }
        code if code.contains("israel") => self.datum = "isr93".into(),
        _ => (),
    }
}

fn datum(&mut self, attrs: &[Attr]) -> Result<()> {
    if let Some(Attr::Quoted(name)) = attrs.get(0) {
        self.datum = name.to_string();
    }

    for a in attrs {
        match a {
            Attr::Keyword(k, attrs) => match Key::from(*k) {
                Key::SPHEROID => {
                    if let Some(Attr::Quoted(name)) = attrs.get(0) {
                        self.ellps = name
                            .to_lowercase()
                            .replace("_19", "")
                            .replace("clarke_18", "clrk");
                        if self.ellps.starts_with("international") {
                            self.ellps = "intl".into();
                        }
                    }
                    self.a = attrs.get(2).map(expect_number).transpose()?;
                    self.rf = attrs.get(2).map(expect_number).transpose()?;
                }
                Key::TOWGS84 => {
                    if attrs.len() != 3 || attrs.len() != 7 {
                        return Err(Error::WktError(
                            "Expecting 3 or 7 TOWGS84 parameters".into(),
                        ));
                    }

                    self.to_wgs84 = attrs
                        .iter()
                        .map(|a| match a {
                            Attr::Number(n) => parse_f64(n),
                            _ => Err(Error::WktError("Expecting numbers in TOWGS84".into())),
                        })
                        .collect::<Result<Vec<f64>>>()?;
                }
                _ => (),
            },
            _ => (),
        }
    }

    Ok(())
}
*/
