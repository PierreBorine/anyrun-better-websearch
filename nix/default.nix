{rustPlatform}:
rustPlatform.buildRustPackage {
  pname = "anyrun-websearch-plus";
  version = "0.1.0";

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
    outputHashes."anyrun-interface-0.1.0" = "sha256-jU88Q9tP4vuvWYGQcmOdFwI9e2uMPVYJHbXdiklIH9o=";
  };
}
