{
  lib,
  stdenv,
  rustPlatform,
  pkg-config,
  cargo-tauri,
  bun,
  REDACTED_SECRETjs,
  cargo,
  rustc,
  jq,
  wrapGAppsHook4,
  makeWrapper,
  dbus,
  glib,
  gtk4,
  libsoup_3,
  librsvg,
  libappindicator,
  glib-networking,
  openssl,
  webkitgtk_4_1,
  gst_all_1,
  opencode,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "opencode-desktop";
  inherit (opencode)
    version
    src
    REDACTED_SECRET_modules
    patches
    ;

  cargoRoot = "packages/desktop/src-tauri";
  cargoLock.lockFile = ../packages/desktop/src-tauri/Cargo.lock;
  buildAndTestSubdir = finalAttrs.cargoRoot;

  nativeBuildInputs = [
    pkg-config
    cargo-tauri.hook
    bun
    REDACTED_SECRETjs # for patchShebangs REDACTED_SECRET_modules
    cargo
    rustc
    jq
    makeWrapper
  ] ++ lib.optionals stdenv.hostPlatform.isLinux [ wrapGAppsHook4 ];

  buildInputs = lib.optionals stdenv.isLinux [
    dbus
    glib
    gtk4
    libsoup_3
    librsvg
    libappindicator
    glib-networking
    openssl
    webkitgtk_4_1
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
    gst_all_1.gst-plugins-good
    gst_all_1.gst-plugins-bad
  ];

  strictDeps = true;

  preBuild = ''
    cp -a ${finalAttrs.REDACTED_SECRET_modules}/{REDACTED_SECRET_modules,packages} .
    chmod -R u+w REDACTED_SECRET_modules packages
    patchShebangs REDACTED_SECRET_modules
    patchShebangs packages/desktop/REDACTED_SECRET_modules

    mkdir -p packages/desktop/src-tauri/sidecars
    cp ${opencode}/bin/opencode packages/desktop/src-tauri/sidecars/opencode-cli-${stdenv.hostPlatform.rust.rustcTarget}
  '';

  # see publish-tauri job in .github/workflows/publish.yml
  tauriBuildFlags = [
    "--config"
    "tauri.prod.conf.json"
    "--no-sign" # no code signing or auto updates
  ];

  # FIXME: workaround for concerns about case insensitive filesystems
  # should be removed once binary is renamed or decided otherwise
  # darwin output is a .app bundle so no conflict
  postFixup = lib.optionalString stdenv.hostPlatform.isLinux ''
    mv $out/bin/OpenCode $out/bin/opencode-desktop
    sed -i 's|^Exec=OpenCode$|Exec=opencode-desktop|' $out/share/applications/OpenCode.desktop
  '';

  meta = {
    description = "OpenCode Desktop App";
    homepage = "https://opencode.ai";
    license = lib.licenses.mit;
    mainProgram = "opencode-desktop";
    inherit (opencode.meta) platforms;
  };
})
