trait BytesReadable {
    fn to_readable(&self) -> String;
}

impl BytesReadable for u64 {
    fn to_readable(&self) -> String {
        const UNIT: u64 = 1024 - 1;

        //   tb  gb  mb kb b
        //   10  10  10 10 10
        let tb = (self & ((UNIT) << 40)) >> 40;
        let gb = (self & ((UNIT) << 30)) >> 30;
        let mb = (self & ((UNIT) << 20)) >> 20;
        let kb = (self & ((UNIT) << 10)) >> 10;
        let b = self & UNIT;

        let mut out = String::new();
        if tb > 0 {
            out.push_str(format!("{:.2} TB", tb as f64 + (gb as f64) / 1024.).as_str());
            return out;
        }
        if gb > 0 {
            out.push_str(format!("{:.2} GB", gb as f64 + (mb as f64) / 1024.).as_str());
            return out;
        }
        if mb > 0 {
            out.push_str(format!("{:.2} MB", mb as f64 + (kb as f64) / 1024.).as_str());
            return out;
        }
        if kb > 0 {
            out.push_str(format!("{:.2} KB", kb as f64 + (b as f64) / 1024.).as_str());
            return out;
        }
        out.push_str(format!("{} B", b).as_str());
        out
    }
}

impl BytesReadable for usize {
    fn to_readable(&self) -> String {
        (*self as u64).to_readable()
    }
}

#[cfg(test)]
mod test {
    use crate::readable::BytesReadable;

    #[test]
    fn byte_readable_test() {
        const KB: u64 = 1024;
        const MB: u64 = 1024 * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        assert_eq!("123 B", (123 as u64).to_readable());
        assert_eq!("123.40 KB", ((123 * KB + (KB as f64 * 0.4) as u64) as u64).to_readable());
        assert_eq!("123.40 MB", ((123 * MB + (MB as f64 * 0.4) as u64) as u64).to_readable());
        assert_eq!("123.40 GB", ((123 * GB + (GB as f64 * 0.4) as u64) as u64).to_readable());
        assert_eq!("123.40 TB", ((123 * TB + (TB as f64 * 0.4) as u64) as u64).to_readable());
    }
}
