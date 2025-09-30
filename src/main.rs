use std::env;
use std::path::Path;

macro_rules! rsbox_include {
    ($feat:literal, $name:ident) => {
        #[cfg(feature = $feat)]
        mod $name;
    };
}

rsbox_include!("echo", echo);
rsbox_include!("cat", cat);

static APPLETS: &[(&'static str, fn(&[String]))] = &[
    #[cfg(feature = "echo")]
    ("echo", echo::run),
    #[cfg(feature = "cat")]
    ("cat", cat::run),
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args
        .get(0)
        .map(|s| {
            Path::new(s)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
        })
        .unwrap_or("");

    let applet_args = &args[1..];

    if program_name == "rsbox" {
        println!("rsbox: a busybox clone in Rust.");
        let mut applet_names: Vec<&str> = APPLETS.iter().map(|(name, _)| *name).collect();
        applet_names.sort_unstable();
        println!("Available applets: {}", applet_names.join(", "));
    } else if let Some((_name, fun_main)) = APPLETS.iter().find(|(name, _)| *name == program_name) {
        fun_main(applet_args);
    } else {
        eprintln!("rsbox: applet not found: {}", program_name);
    }
}
