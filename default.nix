{...}:

stdenv.mkDerivation {
  pname = "logi_keyboard_fn";
  version = "nightly";
  nativeBuildInputs = with pkgs; [ pkg-config rustc cargo ]; 
  buildInputs = with pkgs; [ systemd ];
  src = "./.";

}