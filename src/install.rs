// jkcoxson
// Install libimobiledevice to ~

use core::panic;
use std::env;
use std::process::Command;

pub fn install() {
    match env::consts::OS {
        "windows" => {
            println!("Changing directory");
            // Change directory to home
            Command::new("powershell")
                .arg("cd")
                .arg("~")
                .output()
                .expect("Failed to change directory");

            println!("Curling");
            // Curl libimobiledevice
            Command::new("powershell")
                .arg("curl")
                .arg("https://github.com/libimobiledevice-win32/imobiledevice-net/releases/download/v1.3.17/libimobiledevice.1.2.1-r1122-win-x64.zip")
                .arg("-o")
                .arg("libimobiledevice.zip")
                .output()
                .expect("Failed to fetch necessary files");

            println!("Unzipping");
            // Unzip libimobiledevice
            Command::new("powershell")
                .arg("unzip")
                .arg("libimobiledevice.zip")
                .output()
                .expect("Failed to unzip libimobiledevice");

            // Remove libimobiledevice.zip
            // Command::new("powershell")
            //     .arg("rm")
            //     .arg("libimobiledevice.zip")
            //     .output()
            //     .expect("Failed to remove libimobiledevice.zip");

            println!("Moving");
            // Change directory to libimobiledevice
            Command::new("powershell")
                .arg("cd")
                .arg("libimobiledevice")
                .output()
                .expect("Failed to change directory");
        }
        "macos" => {
            // Detect if brew is installed
            if !env::var("HOMEBREW_PREFIX").is_ok() {
                println!("Homebrew is not installed, installing it now");
                // If not, install it
                Command::new("/bin/bash")
                    .arg("-c")
                    .arg("\"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"")
                    .status()
                    .expect("Failed to install Homebrew, aborting");
            }
            println!("Homebrew is installed, fetching dependencies");
            // Install openssl if not already installed
            brew("openssl");
            // Install pkg-config if not already installed
            brew("pkg-config");
            unix_build();
        }
        "linux" => {
            // Get linux distribution as a string
            let distro = Command::new("lsb_release")
                .arg("-is")
                .output()
                .expect("Failed to get linux distribution, aborting");
            let distro = String::from_utf8(distro.stdout)
                .expect("Failed to get linux distribution, aborting");

            // Detect package manager
            let pkg_manager = get_package_manager(distro.to_ascii_lowercase());
            println!("Detected {} package manager", pkg_manager);

            // Install dependencies
            linux(pkg_manager.clone(), "git");
            linux(pkg_manager.clone(), "build-essential");
            linux(pkg_manager.clone(), "glibtool");
            linux(pkg_manager.clone(), "make");
            linux(pkg_manager.clone(), "cpp");
            linux(pkg_manager.clone(), "gcc-8");
            linux(pkg_manager.clone(), "clang");
            linux(pkg_manager.clone(), "checkinstall");
            linux(pkg_manager.clone(), "autoconf");
            linux(pkg_manager.clone(), "automake");
            linux(pkg_manager.clone(), "libtool");
            linux(pkg_manager.clone(), "m4");
            linux(pkg_manager.clone(), "python-dev");
            linux(pkg_manager.clone(), "pkg-config");
            linux(pkg_manager.clone(), "libavahi-client-dev");
            linux(pkg_manager.clone(), "cython");
            linux(pkg_manager.clone(), "autoheader");
            linux(pkg_manager.clone(), "libusb-1.0-0-dev");
            linux(pkg_manager.clone(), "libssl-dev");
            linux(pkg_manager.clone(), "libc6-udeb");
            linux(pkg_manager.clone(), "libc6-dev");
            linux(pkg_manager.clone(), "libtool-bin");
            linux(pkg_manager.clone(), "libplist++-dev");
            linux(pkg_manager.clone(), "libplist++");
            linux(pkg_manager.clone(), "openssl");
            println!("Installed dependencies");
            // Build and install libimobiledevice
            unix_build();
        }
        _ => panic!("Unsupported operating system"),
    }
}

/// Gets the package manager from the name of the linux Distribution
fn get_package_manager(os: String) -> String {
    match os.trim() {
        "debian" => "apt",
        "ubuntu" => "apt",
        "centos" => "yum",
        "fedora" => "yum",
        "arch" => "pacman",
        _ => panic!("Unsupported operating system"),
    }
    .to_string()
}

/// Builds libimobiledevice after the dependencies have been installed
fn unix_build() {
    // Get the home directory
    let home = env::var("HOME").expect("Failed to get home directory");
    // Create libimobiledevice folder at $HOME
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(mkdir -p $HOME/libimobiledevice)\"")
        .status()
        .expect("Failed to create libimobiledevice folder, aborting");
    // Change directory to ~/libimobiledevice
    env::set_current_dir(format!("{}/libimobiledevice", home)).unwrap();
    // Clone libplist
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(git clone https://github.com/libimobiledevice/libplist.git)\"")
        .status()
        .expect("Failed to clone libplist, aborting");
    // Change directory to ~/libimobiledevice/libplist
    env::set_current_dir(format!("{}/libimobiledevice/libplist", home)).unwrap();
    // Run autogen
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(./autogen.sh)\"")
        .status()
        .expect("Failed to run autogen, aborting");
    // Make
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(make)\"")
        .status()
        .expect("Failed to make libplist, aborting");
    // Install
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(sudo make install)\"")
        .status()
        .expect("Failed to install libplist, aborting");
    // Exit directory
    env::set_current_dir(format!("{}/libimobiledevice", home)).unwrap();
    // Clone libusbmuxd
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(git clone https://github.com/libimobiledevice/libusbmuxd.git)\"")
        .status()
        .expect("Failed to clone libusbmuxd, aborting");
    // Change directory to ~/libimobiledevice/libusbmuxd
    env::set_current_dir(format!("{}/libimobiledevice/libusbmuxd", home)).unwrap();
    // Run autogen
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(./autogen.sh)\"")
        .status()
        .expect("Failed to run autogen, aborting");
    // Make
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(make)\"")
        .status()
        .expect("Failed to make libusbmuxd, aborting");
    // Install
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(sudo make install)\"")
        .status()
        .expect("Failed to install libusbmuxd, aborting");
    // Exit directory
    env::set_current_dir(format!("{}/libimobiledevice", home)).unwrap();
    // Clone libimobiledevice
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(git clone https://github.com/libimobiledevice/libimobiledevice.git)\"")
        .status()
        .expect("Failed to clone libimobiledevice, aborting");
    // Change directory to libimobiledevice
    env::set_current_dir(format!("{}/libimobiledevice/libimobiledevice", home)).unwrap();
    // Run autogen.sh
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(./autogen.sh)\"")
        .status()
        .expect("Failed to run autogen.sh, aborting");
    // Build libimobiledevice
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(make)\"")
        .status()
        .expect("Failed to build libimobiledevice, aborting");
    // Install libimobiledevice
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(sudo make install)\"")
        .status()
        .expect("Failed to install libimobiledevice, aborting");
    // Exit the directory
    env::set_current_dir(format!("{}/libimobiledevice", home)).unwrap();
    println!("libimobiledevice is installed");
}

/// Uses brew to install a package.
fn brew(package: &str) {
    println!("Installing {} with brew", package);
    Command::new("/bin/bash")
        .arg("-c")
        .arg(format!("\"$(brew install {})\"", package))
        .status()
        .expect(format!("Failed to install {}, aborting", package).as_str());
}

/// Uses the specified package manager to install the specified package.
fn linux(pkg_manager: String, package: &str) {
    println!("Installing {} with {}", package, pkg_manager);
    Command::new("/bin/bash")
        .arg("-c")
        .arg(format!("\"$(sudo {} install {})\"", pkg_manager, package))
        .status()
        .expect(format!("Failed to install {}, aborting", package).as_str());
}
