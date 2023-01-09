with (import <nixpkgs> {});
rustPlatform.buildRustPackage {
    pname = "unispect" ;
    version = "0.1" ;
    src = ./. ;
    cargoLock = { lockFile = ./Cargo.lock ; } ;
}
