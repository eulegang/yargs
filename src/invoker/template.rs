#[derive(Debug, PartialEq, Eq)]
pub enum Template {
    Static(String),
    Interp { offsets: Vec<usize>, base: String },
}

impl Template {
    pub fn new(pattern: &str, s: String) -> Template {
        if let Some(idx) = s.find(pattern) {
            let mut offsets = vec![idx];
            let mut rest = s.as_str();
            let mut base = String::with_capacity(s.len());
            let mut prev = idx;

            base.insert_str(0, &rest[..idx]);

            rest = &rest[idx + pattern.len()..];

            while let Some(idx) = rest.find(pattern) {
                offsets.push(idx + prev);
                prev += idx;
                base.push_str(&rest[..idx]);

                rest = &rest[idx + pattern.len()..];
            }

            if !rest.is_empty() {
                base.push_str(rest);
            }

            Template::Interp { offsets, base }
        } else {
            Template::Static(s)
        }
    }

    pub fn apply(&self, iter: &mut dyn Iterator<Item = &&str>) -> String {
        match self {
            Template::Static(s) => s.clone(),
            Template::Interp { offsets, base } => {
                let mut res = base.clone();

                let mut adjust = 0;

                for offset in offsets {
                    if let Some(insert) = iter.next() {
                        res.insert_str(adjust + offset, insert);
                        adjust += insert.len();
                    } else {
                        break;
                    }
                }

                res
            }
        }
    }
}
