{
  projectRootFile = "flake.nix";

  settings.global.excludes = [
    ".*"
    "target"
  ];

  # Nix
  programs.nixfmt.enable = true;
  # MD
  programs.mdformat.enable = true;

  programs.terraform.enable = true;

  programs.rustfmt.enable = true;

  programs.sqlfluff.enable = true;

  programs.toml-sort.enable = true;
}
